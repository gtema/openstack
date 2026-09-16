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

//! Compute (Nova) cleanup provider.
//!
//! The block-storage provider (`providers/block_storage.rs`) declares the
//! `VOLUME Blocks SERVER` rule (a volume attached to a live server can't
//! be deleted until the server is gone). Compute's own `before: ["network"]`
//! service dependency (see `dependencies()` below) only affects `discover()`
//! listing order — it is not, by itself, a deletion-ordering guarantee.
//! `apply()`'s actual deletion order comes purely from resource-level
//! `Blocks`/`CascadeGroup` edges built from `relations()` across every
//! provider's combined node set (see `engine.rs`'s "Service-level ordering
//! only gates listing, not selection" comment). Without a resource-level
//! rule, a server and its nova-owned port could be deleted concurrently in
//! the same layer, racing the port delete against the still-live instance.
//! To prevent that, when the `network` feature is also enabled, this
//! provider declares a `PORT Blocks SERVER` rule keyed on the port's
//! `device_id` matching the server's id — mirroring how
//! `block_storage.rs` declares its cross-service `VOLUME Blocks SERVER`
//! rule gated on `feature = "compute"`.

use async_trait::async_trait;
use serde_json::Value;

use crate::api::compute::v2::server;
use crate::api::{Pagination, QueryAsync, paged, raw};

use crate::cleanup::provider::{CleanupContext, CleanupDependency, CleanupError, CleanupProvider};
#[cfg(feature = "network")]
use crate::cleanup::relations::RelationEffect;
use crate::cleanup::relations::RelationRule;
use crate::cleanup::types::{PlannedResource, ResourceKind};

pub const SERVER: ResourceKind = ResourceKind::new("compute", "server");

fn value_str<'a>(v: &'a Value, key: &str) -> Option<&'a str> {
    v.get(key).and_then(|x| x.as_str())
}

fn to_planned(kind: ResourceKind, v: Value) -> PlannedResource {
    let id = value_str(&v, "id").unwrap_or_default().to_string();
    let name = value_str(&v, "name").map(str::to_string);
    PlannedResource {
        kind,
        id,
        name,
        raw: v,
        selected: false,
        reason: None,
    }
}

// Nova returning 404 for the server means the instance record is gone, but
// neutron's own removal of the instance's nova-owned port(s) is a separate,
// slightly-lagging async step. A subnet delete racing that lag fails with
// `SubnetInUse: One or more ports have an IP allocation from this subnet`
// even though the server that owned the port is already gone. Poll neutron
// directly for ports still carrying this server as `device_id` so `delete()`
// only returns once nothing referencing the server remains. Without the
// `network` feature there is no neutron client to poll, so there is nothing
// to wait for.
#[cfg(feature = "network")]
async fn wait_for_ports_gone(
    ctx: &CleanupContext<'_>,
    server_id: &str,
) -> Result<(), CleanupError> {
    use crate::api::network::v2::port;

    for _ in 0..60 {
        let req = port::list::Request::builder()
            .device_id(server_id)
            .build()
            .map_err(|e| CleanupError::Engine(format!("failed to build port list request: {e}")))?;
        let ports: Vec<Value> = paged(req, Pagination::All)
            .query_async(ctx.client)
            .await
            .map_err(|e| CleanupError::Engine(format!("failed to list ports: {e}")))?;
        if ports.is_empty() {
            return Ok(());
        }
        tokio::time::sleep(std::time::Duration::from_secs(2)).await;
    }
    Err(CleanupError::Engine(format!(
        "ports owned by server {server_id} did not disappear within the timeout"
    )))
}

#[cfg(not(feature = "network"))]
async fn wait_for_ports_gone(
    _ctx: &CleanupContext<'_>,
    _server_id: &str,
) -> Result<(), CleanupError> {
    Ok(())
}

#[derive(Debug, Default)]
pub struct ComputeCleanupProvider;

#[async_trait]
impl CleanupProvider for ComputeCleanupProvider {
    fn service_type(&self) -> &'static str {
        "compute"
    }

    fn dependencies(&self) -> CleanupDependency {
        // NOTE: this only affects `discover()`'s listing order (servers
        // are listed before network resources) — it is not a deletion-
        // ordering guarantee. The actual server-before-port deletion
        // safety comes from the `PORT Blocks SERVER` rule in
        // `relations()` below.
        CleanupDependency {
            before: vec!["network"],
            after: vec![],
        }
    }

    // `Vec::new()` + conditional `push` (rather than `vec![...]`) because
    // the only element is entirely feature-gated; with the `network`
    // feature off this must still compile to an empty
    // `Vec<RelationRule>`.
    #[allow(unused_mut, clippy::vec_init_then_push)]
    fn relations(&self) -> Vec<RelationRule> {
        let mut rules = Vec::new();

        #[cfg(feature = "network")]
        rules.push(RelationRule {
            parent_kind: crate::cleanup::providers::network::PORT,
            child_kind: SERVER,
            matches: |child, parent| value_str(&parent.raw, "device_id") == Some(child.id.as_str()),
            effect: RelationEffect::Blocks,
        });

        rules
    }

    async fn discover(
        &self,
        ctx: &CleanupContext<'_>,
    ) -> Result<Vec<PlannedResource>, CleanupError> {
        let list_err = |kind: ResourceKind| {
            move |e: crate::OpenStackError| CleanupError::Provider {
                kind,
                id: String::new(),
                source: e,
            }
        };

        let mut nodes = Vec::new();

        let servers: Vec<Value> = paged(
            server::list_21::Request::builder().build().map_err(|e| {
                CleanupError::Engine(format!("failed to build server list request: {e}"))
            })?,
            Pagination::All,
        )
        .query_async(ctx.client)
        .await
        .map_err(|e| list_err(SERVER)(e.into()))?;
        for v in servers {
            nodes.push(to_planned(SERVER, v));
        }

        Ok(nodes)
    }

    async fn delete(
        &self,
        ctx: &CleanupContext<'_>,
        resource: &PlannedResource,
    ) -> Result<(), CleanupError> {
        let err = |e: crate::OpenStackError| CleanupError::Provider {
            kind: resource.kind,
            id: resource.id.clone(),
            source: e,
        };

        if resource.kind == SERVER {
            let req = server::delete::Request::builder()
                .id(resource.id.clone())
                .build()
                .map_err(|e| {
                    CleanupError::Engine(format!("failed to build server delete request: {e}"))
                })?;
            raw(req)
                .query_async(ctx.client)
                .await
                .map_err(|e| err(e.into()))?;

            // Nova accepts the delete request and returns immediately,
            // while the instance's volume detachment and NIC/port teardown
            // happen asynchronously in the background. A caller that
            // deletes the server's volume or network right after this call
            // returns races that teardown — on a real cloud (unlike
            // devstack's much smaller/faster catalog) this reliably loses,
            // failing with "volume ... must not be attached" or "port(s)
            // have an IP allocation from this subnet". Block here until
            // the server is actually gone so callers relying on this
            // `delete()` to mean "safe to delete its volume/network next"
            // get that guarantee.
            for _ in 0..60 {
                let get_req = server::get_20::Request::builder()
                    .id(resource.id.clone())
                    .build()
                    .map_err(|e| {
                        CleanupError::Engine(format!("failed to build server get request: {e}"))
                    })?;
                match raw(get_req).query_async(ctx.client).await {
                    Err(e) => {
                        let mapped = err(e.into());
                        if mapped.is_not_found() {
                            return wait_for_ports_gone(ctx, &resource.id).await;
                        }
                        return Err(mapped);
                    }
                    Ok(_) => {
                        tokio::time::sleep(std::time::Duration::from_secs(2)).await;
                    }
                }
            }
            return Err(CleanupError::Engine(format!(
                "server {} did not disappear after delete within the timeout",
                resource.id
            )));
        } else {
            return Err(CleanupError::Engine(format!(
                "ComputeCleanupProvider cannot delete resource kind {:?}",
                resource.kind
            )));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cleanup::engine::ProjectCleanupBuilder;
    #[cfg(feature = "network")]
    use crate::cleanup::relations::evaluate_edges;
    use httpmock::MockServer;
    use std::collections::HashMap;

    async fn mock_client(server: &MockServer) -> crate::AsyncOpenStack {
        let base_url = server.base_url();
        server.mock(|when, then| {
            when.method(httpmock::Method::GET).path("/");
            then.status(200).json_body(
                serde_json::json!({"versions": [{"id": "v3", "status": "SUPPORTED",
                "links": [{"rel": "self", "href": format!("{base_url}/v3/")}]}]}),
            );
        });
        server.mock(|when, then| {
            when.method(httpmock::Method::GET).path("/v3/");
            then.status(200).json_body(
                serde_json::json!({"versions": [{"id": "v3", "status": "SUPPORTED",
                "links": [{"rel": "self", "href": format!("{base_url}/v3/")}]}]}),
            );
        });
        let expires = (chrono::Utc::now() + chrono::TimeDelta::hours(1)).to_rfc3339();
        server.mock(|when, then| {
            when.method(httpmock::Method::POST).path("/v3/auth/tokens");
            then.status(201).header("x-subject-token", "test-token").json_body(serde_json::json!({"token": {
                "id": "token-id", "expires_at": expires,
                "project": {"id": "test-project", "name": "TestProject"},
                "user": {"id": "test-user", "name": "test-user"},
                "methods": ["password"], "audit_ids": ["audit-1"],
                "catalog": [
                    {"type": "identity", "name": "keystone", "endpoints": [{"id": "identity-1",
                        "url": format!("{base_url}/v3"), "region": "RegionOne", "interface": "public"}]},
                    {"type": "compute", "name": "nova", "endpoints": [{"id": "compute-1",
                        "url": format!("{base_url}/v2.1"), "region": "RegionOne", "interface": "public"}]},
                    {"type": "network", "name": "neutron", "endpoints": [{"id": "network-1",
                        "url": format!("{base_url}/v2.0"), "region": "RegionOne", "interface": "public"}]}
                ]
            }}));
        });
        let config = openstack_sdk_core::config::CloudConfig {
            auth: Some(openstack_sdk_core::config::Auth {
                auth_url: Some(format!("{base_url}/v3")),
                username: Some("test-user".into()),
                user_domain_name: Some("Default".into()),
                password: Some("test-password".into()),
                project_id: Some("test-project".into()),
                ..Default::default()
            }),
            region_name: Some("RegionOne".into()),
            interface: Some("public".into()),
            auth_cache: Some(false),
            ..Default::default()
        };
        crate::AsyncOpenStack::new_with_authentication_helper(
            &config,
            crate::auth::auth_helper::Noop::default(),
            false,
        )
        .await
        .expect("client creation failed")
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn discover_maps_servers_to_planned_resources() {
        let server = MockServer::start_async().await;
        let client = mock_client(&server).await;

        server.mock(|when, then| {
            when.method(httpmock::Method::GET).path("/v2.1/servers");
            then.status(200).json_body(serde_json::json!({"servers": [
                {"id": "server-1", "name": "vm-1"}
            ]}));
        });

        let cleanup = ProjectCleanupBuilder::new(&client)
            .with_provider(ComputeCleanupProvider)
            .build();

        let plan = cleanup
            .discover(HashMap::new(), None)
            .await
            .expect("discover failed");

        let node = plan.nodes.iter().find(|n| n.id == "server-1").unwrap();
        assert_eq!(node.kind, SERVER);
        assert_eq!(node.name.as_deref(), Some("vm-1"));
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn delete_issues_server_delete_request() {
        let server = MockServer::start_async().await;
        let client = mock_client(&server).await;

        server.mock(|when, then| {
            when.method(httpmock::Method::GET).path("/v2.1/servers");
            then.status(200).json_body(serde_json::json!({"servers": [
                {"id": "server-1", "name": "vm-1"}
            ]}));
        });
        server.mock(|when, then| {
            when.method(httpmock::Method::DELETE)
                .path("/v2.1/servers/server-1");
            then.status(204);
        });
        // `delete()` polls with a GET after issuing the delete to confirm the
        // server actually disappeared before returning — see the comment on
        // that polling loop in `delete()` for why.
        server.mock(|when, then| {
            when.method(httpmock::Method::GET)
                .path("/v2.1/servers/server-1");
            then.status(404);
        });
        // With the `network` feature enabled, `delete()` also waits for
        // neutron to drop any ports still owned by the server — see
        // `wait_for_ports_gone`.
        #[cfg(feature = "network")]
        server.mock(|when, then| {
            when.method(httpmock::Method::GET)
                .path("/v2.0/ports")
                .query_param("device_id", "server-1");
            then.status(200).json_body(serde_json::json!({"ports": []}));
        });

        let cleanup = ProjectCleanupBuilder::new(&client)
            .with_provider(ComputeCleanupProvider)
            .build();

        let eval: crate::cleanup::provider::EvaluationFn =
            std::sync::Arc::new(|r: &PlannedResource| r.kind == SERVER);
        let plan = cleanup
            .discover(HashMap::new(), Some(eval))
            .await
            .expect("discover failed");
        let result = cleanup.apply(plan).await.expect("apply failed");

        assert!(
            result.errors.is_empty(),
            "unexpected errors: {:?}",
            result.errors
        );
        assert!(result.deleted_ids.contains(&"server-1".to_string()));
    }

    #[cfg(feature = "network")]
    #[test]
    fn port_with_matching_device_id_produces_blocks_edge_against_that_server() {
        use crate::cleanup::providers::network::PORT;

        fn node(kind: ResourceKind, id: &str, extra: serde_json::Value) -> PlannedResource {
            PlannedResource {
                kind,
                id: id.into(),
                name: None,
                raw: extra,
                selected: false,
                reason: None,
            }
        }

        let nodes = vec![
            node(SERVER, "server-1", serde_json::json!({"id": "server-1"})),
            node(
                PORT,
                "port-1",
                serde_json::json!({"id": "port-1", "device_id": "server-1"}),
            ),
            node(
                PORT,
                "port-2",
                serde_json::json!({"id": "port-2", "device_id": "other-server"}),
            ),
        ];

        let rules = ComputeCleanupProvider.relations();
        let edges = evaluate_edges(&nodes, &rules);

        let port1_idx = nodes.iter().position(|n| n.id == "port-1").unwrap();
        let port2_idx = nodes.iter().position(|n| n.id == "port-2").unwrap();
        let server_idx = nodes.iter().position(|n| n.id == "server-1").unwrap();

        assert!(
            edges
                .iter()
                .any(|e| matches!(e.effect, RelationEffect::Blocks)
                    && e.child == server_idx
                    && e.parent == port1_idx),
            "server must be blocked by a port whose device_id matches it"
        );
        assert!(
            !edges.iter().any(|e| e.parent == port2_idx),
            "port with a different device_id must not gain a server-blocking edge"
        );
    }
}
