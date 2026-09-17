// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//
// SPDX-License-Identifier: Apache-2.0

//! Resource creation helpers and a best-effort cleanup guard for the
//! project-cleanup functional test. These call the real service APIs
//! directly (not through the cleanup engine) to set up a known scenario
//! for the engine to then tear down.

use std::time::Duration;

use openstack_sdk::AsyncOpenStack;
use openstack_sdk::api::block_storage::v3::{backup, snapshot, volume};
use openstack_sdk::api::compute::v2::{keypair, server};
use openstack_sdk::api::image::v2::image;
use openstack_sdk::api::network::v2::{
    floatingip, network, port, router, security_group, security_group_rule, subnet,
};
use openstack_sdk::api::{Pagination, QueryAsync, paged, raw};

/// Tracks every resource this test creates directly (bypassing the
/// cleanup engine) so a failed assertion can still best-effort clean up
/// devstack/the real cloud instead of leaking resources. Direct deletes
/// issued here are expected to fail with "not found" on the happy path,
/// since the engine under test will already have deleted everything —
/// that failure is swallowed, not reported.
pub struct CreatedResources<'a> {
    client: &'a AsyncOpenStack,
    created: Vec<(&'static str, String)>,
}

impl<'a> CreatedResources<'a> {
    pub fn new(client: &'a AsyncOpenStack) -> Self {
        Self {
            client,
            created: Vec::new(),
        }
    }

    pub fn track(&mut self, service_type: &'static str, id: impl Into<String>) {
        self.created.push((service_type, id.into()));
    }
}

impl Drop for CreatedResources<'_> {
    fn drop(&mut self) {
        let client = self.client;
        let created = std::mem::take(&mut self.created);
        if created.is_empty() {
            return;
        }
        // `Drop` cannot be async, and `tokio::test` already runs this
        // `Drop` from inside a live multi-thread runtime — spinning up
        // *any* runtime (even a brand-new one) and calling `block_on` on
        // the current thread panics with "Cannot start a runtime from
        // within a runtime", because the nested-runtime guard is
        // per-thread, not per-runtime-instance. Run the dedicated runtime
        // on a plain, scoped OS thread instead: `thread::scope` lets the
        // spawned thread safely borrow `client` (non-'static) and is
        // joined before `drop` returns, so cleanup still completes
        // synchronously.
        std::thread::scope(|s| {
            s.spawn(|| {
                let rt = match tokio::runtime::Runtime::new() {
                    Ok(rt) => rt,
                    Err(_) => return,
                };
                rt.block_on(run_best_effort_cleanup(client, created));
            });
        });
    }
}

async fn run_best_effort_cleanup(client: &AsyncOpenStack, created: Vec<(&'static str, String)>) {
    for (service_type, id) in created.into_iter().rev() {
        // A `build()` failure here is unreachable in practice (only a
        // single mandatory `.id(...)` field is ever set), but this is
        // best-effort cleanup on the `Drop` path, so a failure is
        // silently skipped rather than unwrapped.
        let _: Result<(), _> = match service_type {
            "compute" => match server::delete::Request::builder().id(id).build() {
                Ok(req) => raw(req).query_async(client).await.map(|_| ()),
                Err(_) => Ok(()),
            },
            "block-storage-snapshot" => match snapshot::delete::Request::builder().id(id).build() {
                Ok(req) => raw(req).query_async(client).await.map(|_| ()),
                Err(_) => Ok(()),
            },
            "block-storage-volume" => match volume::delete::Request::builder().id(id).build() {
                Ok(req) => raw(req).query_async(client).await.map(|_| ()),
                Err(_) => Ok(()),
            },
            // Encoded as "<router_id>:<subnet_id>" by `track` (see the
            // call site in `full_scenario.rs`) since a router needs its
            // interface detached before it — and before the network it
            // sits on — can be deleted. Must run before the `"network"`
            // arm below (reverse-creation-order iteration already
            // guarantees this, since the router-with-interface is
            // created after the network).
            "network-router" => {
                if let Some((router_id, subnet_id)) = id.split_once(':')
                    && let Ok(req) = router::remove_router_interface::Request::builder()
                        .id(router_id)
                        .subnet_id(subnet_id)
                        .build()
                {
                    let _ = raw(req).query_async(client).await;
                }
                match router::delete::Request::builder()
                    .id(id.split(':').next().unwrap_or(&id))
                    .build()
                {
                    Ok(req) => raw(req).query_async(client).await.map(|_| ()),
                    Err(_) => Ok(()),
                }
            }
            "network" => match network::delete::Request::builder().id(id).build() {
                Ok(req) => raw(req).query_async(client).await.map(|_| ()),
                Err(_) => Ok(()),
            },
            "network-floatingip" => match floatingip::delete::Request::builder().id(id).build() {
                Ok(req) => raw(req).query_async(client).await.map(|_| ()),
                Err(_) => Ok(()),
            },
            // Deleting the security group also removes its rules (both
            // the auto-created default-egress ones and the custom one
            // this test adds), so no separate rule cleanup arm is needed.
            "network-security-group" => {
                match security_group::delete::Request::builder().id(id).build() {
                    Ok(req) => raw(req).query_async(client).await.map(|_| ()),
                    Err(_) => Ok(()),
                }
            }
            "compute-keypair" => match keypair::delete_20::Request::builder().id(id).build() {
                Ok(req) => raw(req).query_async(client).await.map(|_| ()),
                Err(_) => Ok(()),
            },
            "block-storage-backup" => match backup::delete::Request::builder().id(id).build() {
                Ok(req) => raw(req).query_async(client).await.map(|_| ()),
                Err(_) => Ok(()),
            },
            _ => Ok(()),
        };
    }
}

/// Resolves the boot image id. If `TEST_IMAGE_NAME` is set, requires an
/// exact name match (explicit operator choice, fail loud if absent). If
/// unset, matches any image whose name contains "cirros" case-insensitively
/// — devstack's default cirros image name has drifted across versions
/// (e.g. `cirros-0.6.2-x86_64-disk` vs newer builds), so an exact default
/// string is unreliable in CI. Does not create or delete this image — it
/// is expected to pre-exist on the target cloud.
pub async fn resolve_test_image_id(
    client: &AsyncOpenStack,
) -> Result<String, Box<dyn std::error::Error>> {
    let explicit_name = std::env::var("TEST_IMAGE_NAME").ok();
    let images: Vec<serde_json::Value> =
        paged(image::list::Request::builder().build()?, Pagination::All)
            .query_async(client)
            .await?;
    let found = match &explicit_name {
        Some(name) => images
            .into_iter()
            .find(|img| img.get("name").and_then(|v| v.as_str()) == Some(name.as_str()))
            .ok_or_else(|| format!("no image named '{name}' found on target cloud"))?,
        None => images
            .into_iter()
            .find(|img| {
                img.get("name")
                    .and_then(|v| v.as_str())
                    .is_some_and(|n| n.to_lowercase().contains("cirros"))
            })
            .ok_or("no image with 'cirros' in its name found on target cloud")?,
    };
    Ok(found
        .get("id")
        .and_then(|v| v.as_str())
        .ok_or("image response must carry an id")?
        .to_string())
}

pub async fn create_network(
    client: &AsyncOpenStack,
    name: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let req = network::create::Request::builder()
        .network(
            network::create::NetworkBuilder::default()
                .name(name.to_string())
                .build()?,
        )
        .build()?;
    // `response_key()` ("network") is already unwrapped by the blanket
    // `QueryAsync` impl, so the deserialized value's fields (`id`, `name`,
    // ...) sit at the top level — not nested under a `"network"` key.
    let resp: serde_json::Value = req.query_async(client).await?;
    Ok(resp["id"]
        .as_str()
        .ok_or("network create response must carry an id")?
        .to_string())
}

pub async fn create_subnet(
    client: &AsyncOpenStack,
    name: &str,
    network_id: &str,
    cidr: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let req = subnet::create::Request::builder()
        .subnet(
            subnet::create::SubnetBuilder::default()
                .name(name.to_string())
                .network_id(network_id.to_string())
                // `cidr` is `Option<Option<Cow<str>>>` on the generated
                // type (nullable in the OpenAPI schema) — the builder's
                // `strip_option` only strips the outer layer, so setting
                // it needs an explicit `Some(...)`, same as the
                // generated CLI command does (`cli/network/src/v2/subnet/create.rs`).
                .cidr(Some(cidr.to_string().into()))
                .ip_version(4)
                .build()?,
        )
        .build()?;
    let resp: serde_json::Value = req.query_async(client).await?;
    Ok(resp["id"]
        .as_str()
        .ok_or("subnet create response must carry an id")?
        .to_string())
}

/// Creates a router with its external gateway set to `external_network_id`
/// and an interface on `subnet_id`. The gateway is required for the
/// floating IP this test associates later - without it, Neutron has no
/// path from the external network to the interface's subnet and rejects
/// the floating IP association with `ExternalGatewayForFloatingIPNotFound`.
pub async fn create_router_with_interface(
    client: &AsyncOpenStack,
    name: &str,
    subnet_id: &str,
    external_network_id: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let req = router::create::Request::builder()
        .router(
            router::create::RouterBuilder::default()
                .name(name.to_string())
                .external_gateway_info(Some(
                    router::create::ExternalGatewayInfoBuilder::default()
                        .network_id(external_network_id.to_string())
                        .build()?,
                ))
                .build()?,
        )
        .build()?;
    let resp: serde_json::Value = req.query_async(client).await?;
    let router_id = resp["id"]
        .as_str()
        .ok_or("router create response must carry an id")?
        .to_string();

    let mut iface_builder = router::add_router_interface::Request::builder();
    iface_builder.id(router_id.clone());
    iface_builder.subnet_id(subnet_id.to_string());
    let iface_req = iface_builder.build()?;
    raw(iface_req).query_async(client).await?;

    Ok(router_id)
}

pub async fn create_volume_from_image(
    client: &AsyncOpenStack,
    name: &str,
    image_id: &str,
    size_gib: i32,
) -> Result<String, Box<dyn std::error::Error>> {
    // `name`, `image_ref`, and `size` are all `Option<Option<T>>` on the
    // generated `Volume` type (nullable fields in Cinder's schema) — each
    // needs an explicit `Some(...)`, matching how the generated CLI
    // volume-create command sets them.
    let req = volume::create_30::Request::builder()
        .volume(
            volume::create_30::VolumeBuilder::default()
                .name(Some(name.to_string().into()))
                .image_ref(Some(image_id.to_string().into()))
                .size(Some(size_gib))
                .build()?,
        )
        .build()?;
    let resp: serde_json::Value = req.query_async(client).await?;
    Ok(resp["id"]
        .as_str()
        .ok_or("volume create response must carry an id")?
        .to_string())
}

/// Stops polling as soon as the volume leaves a transient state, whether it
/// lands on `available` or `error` - an errored volume is exactly the kind
/// of resource this test's cleanup engine needs to prove it can still
/// discover and delete (backend flakiness in CI has produced this; see
/// `wait_for_snapshot_available`/`wait_for_backup_available` for the same
/// reasoning). Letting the caller proceed rather than failing here means
/// any real, unrecoverable consequence (e.g. booting a server from an
/// errored volume) surfaces at that later, more specific call instead of a
/// synthetic timeout/error from this polling helper.
pub async fn wait_for_volume_available(
    client: &AsyncOpenStack,
    volume_id: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    for _ in 0..60 {
        let vols: Vec<serde_json::Value> = paged(
            volume::list_detailed::Request::builder().build()?,
            Pagination::All,
        )
        .query_async(client)
        .await?;
        if let Some(v) = vols.iter().find(|v| v["id"].as_str() == Some(volume_id)) {
            match v["status"].as_str() {
                Some("available") => return Ok(()),
                Some("error") => {
                    eprintln!(
                        "warning: volume {volume_id} went to error \
                         (cleanup engine is expected to still discover/delete it)"
                    );
                    return Ok(());
                }
                _ => {}
            }
        }
        tokio::time::sleep(Duration::from_secs(5)).await;
    }
    Err(format!("volume {volume_id} did not become available in time").into())
}

pub async fn create_snapshot(
    client: &AsyncOpenStack,
    name: &str,
    volume_id: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    // `name` is `Option<Option<Cow<str>>>` on the generated `Snapshot`
    // type; `volume_id` is a plain `Option<Cow<str>>`, so it takes the
    // value directly.
    let req = snapshot::create::Request::builder()
        .snapshot(
            snapshot::create::SnapshotBuilder::default()
                .name(Some(name.to_string().into()))
                .volume_id(volume_id.to_string())
                .build()?,
        )
        .build()?;
    let resp: serde_json::Value = req.query_async(client).await?;
    Ok(resp["id"]
        .as_str()
        .ok_or("snapshot create response must carry an id")?
        .to_string())
}

/// See `wait_for_volume_available`'s doc comment for why an `error` status
/// returns `Ok` here instead of failing the test.
pub async fn wait_for_snapshot_available(
    client: &AsyncOpenStack,
    snapshot_id: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    for _ in 0..60 {
        let req = snapshot::get::Request::builder().id(snapshot_id).build()?;
        let resp: serde_json::Value = req.query_async(client).await?;
        match resp["status"].as_str() {
            Some("available") => return Ok(()),
            Some("error") => {
                eprintln!(
                    "warning: snapshot {snapshot_id} went to error \
                     (cleanup engine is expected to still discover/delete it)"
                );
                return Ok(());
            }
            _ => {}
        }
        tokio::time::sleep(Duration::from_secs(5)).await;
    }
    Err(format!("snapshot {snapshot_id} did not become available in time").into())
}

pub async fn create_server_from_volume(
    client: &AsyncOpenStack,
    name: &str,
    flavor_id: &str,
    volume_id: &str,
    network_id: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    // `create_20`'s negotiation window is exactly microversion 2.0, which
    // real clouds increasingly reject outright (e.g. `cloud_min: 2.1`) —
    // `create_21` (window [2.1, 2.18]) has the same builder shape and is
    // universally supported by any cloud still running the legacy
    // `block_device_mapping_v2`/`networks` server-create shape.
    let req = server::create_21::Request::builder()
        .server(
            server::create_21::ServerBuilder::default()
                .name(name.to_string())
                .flavor_ref(flavor_id.to_string())
                .block_device_mapping_v2(vec![
                    // `boot_index` is `Option<Option<Cow<str>>>` on the
                    // generated type — needs `Some(...)`; `uuid`,
                    // `source_type`, `destination_type`,
                    // `delete_on_termination` are single-layer `Option`s
                    // and take their value directly.
                    server::create_21::BlockDeviceMappingV2Builder::default()
                        .uuid(volume_id.to_string())
                        .source_type(server::create_21::SourceType::Volume)
                        .destination_type(server::create_21::DestinationType::Volume)
                        .boot_index(Some("0".to_string().into()))
                        .delete_on_termination(false)
                        .build()?,
                ])
                .networks(vec![
                    server::create_21::NetworksBuilder::default()
                        .uuid(network_id.to_string())
                        .build()?,
                ])
                .build()?,
        )
        .build()?;
    // Booting straight from a volume that Cinder was just doing something
    // else with (this test backs it up via a snapshot right before this
    // call) can race Nova's own Cinder lookup during block-device-mapping
    // validation, observed in CI as a transient
    // "Block Device Mapping is Invalid: failed to get volume ..." 400 -
    // the volume is otherwise fine (a later `GET` immediately succeeds).
    // Retry the create on exactly that message rather than failing the
    // whole test on a race outside this code's control.
    let mut attempt = 0;
    loop {
        match req.clone().query_async(client).await {
            Ok(resp) => {
                let resp: serde_json::Value = resp;
                return Ok(resp["id"]
                    .as_str()
                    .ok_or("server create response must carry an id")?
                    .to_string());
            }
            Err(e) if attempt < 5 && e.to_string().contains("failed to get volume") => {
                attempt += 1;
                eprintln!(
                    "warning: server create hit transient Cinder/Nova race \
                     ({e}), retrying ({attempt}/5)"
                );
                tokio::time::sleep(Duration::from_secs(5)).await;
            }
            Err(e) => return Err(e.into()),
        }
    }
}

/// See `wait_for_volume_available`'s doc comment: an errored server should
/// still be picked up by the cleanup engine under test, so this returns
/// `Ok` rather than failing the test outright, and leaves any real
/// consequence (e.g. no port to attach a floating IP to) to surface at
/// that later, more specific call.
pub async fn wait_for_server_active(
    client: &AsyncOpenStack,
    server_id: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    for _ in 0..60 {
        let req = server::get_20::Request::builder().id(server_id).build()?;
        let resp: serde_json::Value = req.query_async(client).await?;
        match resp["status"].as_str() {
            Some("ACTIVE") => return Ok(()),
            Some("ERROR") => {
                eprintln!(
                    "warning: server {server_id} went to ERROR \
                     (cleanup engine is expected to still discover/delete it)"
                );
                return Ok(());
            }
            _ => {}
        }
        tokio::time::sleep(Duration::from_secs(5)).await;
    }
    Err(format!("server {server_id} did not become ACTIVE in time").into())
}

/// Any flavor works for this scenario (no I/O-heavy workload runs on the
/// server) — picks the one with the fewest vcpus, since the unqualified
/// "first" flavor a real cloud's list endpoint returns can carry a core
/// count that blows a small test project's quota (e.g. a 24-vcpu flavor
/// against a 20-core project quota).
pub async fn first_flavor_id(
    client: &AsyncOpenStack,
) -> Result<String, Box<dyn std::error::Error>> {
    use openstack_sdk::api::compute::v2::flavor;
    let flavors: Vec<serde_json::Value> = paged(
        flavor::list_detailed_20::Request::builder().build()?,
        Pagination::All,
    )
    .query_async(client)
    .await?;
    Ok(flavors
        .iter()
        .min_by_key(|f| f["vcpus"].as_i64().unwrap_or(i64::MAX))
        .and_then(|f| f["id"].as_str())
        .ok_or("no flavors available on target cloud")?
        .to_string())
}

/// Finds an external network to allocate a floating IP from. Required
/// because `create_floating_ip` needs a `floating_network_id`, and unlike
/// the boot image, devstack/real clouds don't expose this via a fixed,
/// well-known name across environments — `router:external=true` is the
/// portable way to identify it (same filter python-openstackclient's own
/// `network list --external` uses).
pub async fn find_external_network_id(
    client: &AsyncOpenStack,
) -> Result<String, Box<dyn std::error::Error>> {
    let nets: Vec<serde_json::Value> = paged(
        network::list::Request::builder()
            .router_external(true)
            .build()?,
        Pagination::All,
    )
    .query_async(client)
    .await?;
    Ok(nets
        .first()
        .and_then(|n| n["id"].as_str())
        .ok_or("no external network found on target cloud")?
        .to_string())
}

/// Returns the id of the first port bound to `server_id`, i.e. the port
/// the server's fixed IP lives on — what a floating IP association needs.
pub async fn first_port_id_for_server(
    client: &AsyncOpenStack,
    server_id: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let ports: Vec<serde_json::Value> = paged(
        port::list::Request::builder()
            .device_id(server_id.to_string())
            .build()?,
        Pagination::All,
    )
    .query_async(client)
    .await?;
    Ok(ports
        .first()
        .and_then(|p| p["id"].as_str())
        .ok_or_else(|| format!("no port found for server {server_id}"))?
        .to_string())
}

/// `description` doubles as this test's marker carrier — floating IPs have
/// no `name` attribute, and the cleanup provider's `discover()` falls back
/// to `description` for exactly this reason (see `network.rs`'s floating
/// IP discovery loop).
pub async fn create_floating_ip(
    client: &AsyncOpenStack,
    external_network_id: &str,
    port_id: &str,
    description: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let req = floatingip::create::Request::builder()
        .floatingip(
            floatingip::create::FloatingipBuilder::default()
                .floating_network_id(external_network_id.to_string())
                .port_id(Some(port_id.to_string().into()))
                .description(description.to_string())
                .build()?,
        )
        .build()?;
    let resp: serde_json::Value = req.query_async(client).await?;
    Ok(resp["id"]
        .as_str()
        .ok_or("floating IP create response must carry an id")?
        .to_string())
}

pub async fn create_security_group(
    client: &AsyncOpenStack,
    name: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let req = security_group::create::Request::builder()
        .security_group(
            security_group::create::SecurityGroupBuilder::default()
                .name(name.to_string())
                .build()?,
        )
        .build()?;
    let resp: serde_json::Value = req.query_async(client).await?;
    Ok(resp["id"]
        .as_str()
        .ok_or("security group create response must carry an id")?
        .to_string())
}

pub async fn create_security_group_rule(
    client: &AsyncOpenStack,
    security_group_id: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let req = security_group_rule::create::Request::builder()
        .security_group_rule(
            security_group_rule::create::SecurityGroupRuleBuilder::default()
                .security_group_id(security_group_id.to_string())
                .direction(security_group_rule::create::Direction::Ingress)
                .ethertype(security_group_rule::create::Ethertype::Ipv4)
                .protocol("tcp".to_string())
                .port_range_min(Some(22))
                .port_range_max(Some(22))
                .build()?,
        )
        .build()?;
    let resp: serde_json::Value = req.query_async(client).await?;
    Ok(resp["id"]
        .as_str()
        .ok_or("security group rule create response must carry an id")?
        .to_string())
}

/// Nova keypairs have no separate numeric id - `name` is the identifier
/// used by both `get`/`delete` and by `PlannedResource::id` in the
/// cleanup provider's discovery.
pub async fn create_keypair(
    client: &AsyncOpenStack,
    name: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let req = keypair::create_21::Request::builder()
        .keypair(
            keypair::create_21::KeypairBuilder::default()
                .name(name.to_string())
                .build()?,
        )
        .build()?;
    let _: serde_json::Value = req.query_async(client).await?;
    Ok(name.to_string())
}

pub async fn create_volume_backup(
    client: &AsyncOpenStack,
    name: &str,
    volume_id: &str,
    snapshot_id: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    // Backing up via `snapshot_id` (rather than a live `volume_id` backup)
    // reads from the already-quiesced snapshot instead of the volume
    // itself, so it works even once the volume is attached/in-use -
    // devstack's backup driver errors out backing up an in-use volume
    // directly even with `force: true` (observed in CI: "backup ... went
    // to error"). `volume_id` is still required by the API alongside
    // `snapshot_id` (it's the snapshot's own parent volume here).
    let req = backup::create_30::Request::builder()
        .backup(
            backup::create_30::BackupBuilder::default()
                .name(Some(name.to_string().into()))
                .volume_id(volume_id.to_string())
                .snapshot_id(Some(snapshot_id.to_string().into()))
                .build()?,
        )
        .build()?;
    let resp: serde_json::Value = req.query_async(client).await?;
    Ok(resp["id"]
        .as_str()
        .ok_or("backup create response must carry an id")?
        .to_string())
}

/// See `wait_for_volume_available`'s doc comment for why an `error` status
/// returns `Ok` here instead of failing the test.
pub async fn wait_for_backup_available(
    client: &AsyncOpenStack,
    backup_id: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    for _ in 0..60 {
        let req = backup::get::Request::builder().id(backup_id).build()?;
        let resp: serde_json::Value = req.query_async(client).await?;
        match resp["status"].as_str() {
            Some("available") => return Ok(()),
            Some("error") => {
                eprintln!(
                    "warning: backup {backup_id} went to error \
                     (cleanup engine is expected to still discover/delete it)"
                );
                return Ok(());
            }
            _ => {}
        }
        tokio::time::sleep(Duration::from_secs(5)).await;
    }
    Err(format!("backup {backup_id} did not become available in time").into())
}
