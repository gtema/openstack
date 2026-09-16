# Project Cleanup: Extended Providers + Functional Test Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add `ComputeCleanupProvider`, `BlockStorageCleanupProvider`, and
`ImageCleanupProvider` to `openstack_sdk/src/cleanup`, wire the
cross-service ordering rules between them and the existing
`NetworkCleanupProvider`, and add one real, devstack/real-cloud-backed
functional test exercising the whole engine across all four services.

**Architecture:** Each new provider is a single file under
`openstack_sdk/src/cleanup/providers/`, following `providers/network.rs`'s
exact shape: `discover()` pages a list endpoint into `PlannedResource`s via
a local `to_planned` helper, `delete()` matches on `resource.kind` and
issues the matching delete request. Cross-service ordering is expressed as
data (`RelationRule`s), not engine changes — the engine already evaluates
every registered provider's `relations()` output against the full merged
node set from every provider (`relations.rs::evaluate_edges`).

**Tech Stack:** Rust, `async_trait`, `serde_json::Value` as the untyped
resource envelope, `crate::api::{compute,block_storage,image}::v2/v3`
generated request/response bindings, `tokio::test` + a real
`AsyncOpenStack` for the functional test.

## Global Constraints

- Every new provider file mirrors `openstack_sdk/src/cleanup/providers/network.rs`'s
  structure exactly: `pub const` `ResourceKind`s, a local `value_str`/`to_planned`
  helper duplicated per file (matching the existing pattern — no shared
  helper module exists yet and this plan does not introduce one), a
  `#[derive(Debug, Default)]` unit struct implementing `CleanupProvider`,
  and a `#[cfg(test)] mod tests` with `httpmock`-backed discover/relation
  tests.
- Every new provider module is feature-gated exactly like `network` is
  today: `#[cfg(feature = "compute")] pub mod compute;` in
  `openstack_sdk/src/cleanup/providers/mod.rs`, and
  `#[cfg(feature = "compute")] pub use providers::compute::ComputeCleanupProvider;`
  in `openstack_sdk/src/cleanup/mod.rs`. Feature names are the crate's
  existing `compute`, `block_storage`, `image` (see
  `openstack_sdk/Cargo.toml:16-29`) — not new feature names.
- Any `RelationRule` whose `child_kind`/`parent_kind` comes from a
  *different* provider's module (the volume/server relation) must be
  wrapped in `#[cfg(feature = "compute")]` inside `block_storage.rs`'s
  `relations()` body, since `block_storage` must still compile with
  `compute` disabled (default-features builds enable both, but
  `--no-default-features --features block_storage` must not break).
- No engine changes. `evaluate_edges`, `service_layers`, `CleanupProvider`,
  `RelationRule`, `RelationEffect` are all final as merged in `32461f9fc`.
- The functional test must not assume devstack: image selection goes
  through `TEST_IMAGE_NAME` (default `"cirros-0.6.2-x86_64-disk"`), and
  every other created resource is self-contained (created and deleted by
  the test itself), so it runs unmodified against `OS_CLOUD` pointed at
  devstack (CI) or a real cloud (manual verification during
  implementation — this sandbox cannot reach any live OpenStack, loopback
  networking is blocked here).
- `cargo clippy -p openstack_sdk --lib --tests --all-features` must stay
  clean after every task.

---

### Task 1: `ComputeCleanupProvider`

**Files:**
- Create: `openstack_sdk/src/cleanup/providers/compute.rs`
- Modify: `openstack_sdk/src/cleanup/providers/mod.rs`
- Modify: `openstack_sdk/src/cleanup/mod.rs`

**Interfaces:**
- Consumes: `crate::cleanup::provider::{CleanupContext, CleanupDependency, CleanupError, CleanupProvider}`,
  `crate::cleanup::relations::RelationRule` (imported but unused by this
  provider — no rules declared here, see Task 2), `crate::cleanup::types::{PlannedResource, ResourceKind}`,
  `crate::api::compute::v2::server::{list, delete}`, `crate::api::{Pagination, QueryAsync, paged, raw}`.
- Produces: `pub const SERVER: ResourceKind = ResourceKind::new("compute", "server");`
  and `pub struct ComputeCleanupProvider;` implementing `CleanupProvider` —
  `Task 2` imports `crate::cleanup::providers::compute::SERVER` by exactly
  this path and name.

- [ ] **Step 1: Write the provider with its discover/delete tests**

Create `openstack_sdk/src/cleanup/providers/compute.rs`:

```rust
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
//! Servers have no resource-level relations of their own to declare here:
//! the network provider's ports already block/cascade on their owning
//! network regardless of which service owns the port, and the
//! block-storage provider (`providers/block_storage.rs`) declares the
//! `VOLUME Blocks SERVER` rule (a volume attached to a live server can't
//! be deleted until the server is gone). Compute only needs a
//! service-level ordering hint: it must run before network, so a
//! `compute:nova`-owned port isn't deleted out from under a still-live
//! instance.

use async_trait::async_trait;
use serde_json::Value;

use crate::api::compute::v2::server;
use crate::api::{Pagination, QueryAsync, paged, raw};

use crate::cleanup::provider::{CleanupContext, CleanupDependency, CleanupError, CleanupProvider};
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

#[derive(Debug, Default)]
pub struct ComputeCleanupProvider;

#[async_trait]
impl CleanupProvider for ComputeCleanupProvider {
    fn service_type(&self) -> &'static str {
        "compute"
    }

    fn dependencies(&self) -> CleanupDependency {
        CleanupDependency {
            before: vec!["network"],
            after: vec![],
        }
    }

    fn relations(&self) -> Vec<RelationRule> {
        Vec::new()
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
            server::list::Request::builder().build().map_err(|e| {
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
        } else {
            return Err(CleanupError::Engine(format!(
                "ComputeCleanupProvider cannot delete resource kind {:?}",
                resource.kind
            )));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cleanup::engine::ProjectCleanupBuilder;
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
                        "url": format!("{base_url}/v2.1"), "region": "RegionOne", "interface": "public"}]}
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

        assert!(result.errors.is_empty(), "unexpected errors: {:?}", result.errors);
        assert!(result.deleted_ids.contains(&"server-1".to_string()));
    }
}
```

- [ ] **Step 2: Register the module and re-export the provider**

In `openstack_sdk/src/cleanup/providers/mod.rs`, add after the existing
`network` block:

```rust
#[cfg(feature = "compute")]
pub mod compute;
```

In `openstack_sdk/src/cleanup/mod.rs`, add after the existing `network`
re-export:

```rust
#[cfg(feature = "compute")]
pub use providers::compute::ComputeCleanupProvider;
```

- [ ] **Step 3: Run the new tests**

Run: `cargo test -p openstack_sdk --lib --all-features cleanup::providers::compute -- --nocapture`
Expected: both tests pass (`discover_maps_servers_to_planned_resources`,
`delete_issues_server_delete_request`).

If the sandbox's loopback-networking block makes `httpmock`-backed tests
fail to connect here (a known, pre-existing environment limitation — see
`docs/superpowers/plans/2026-09-15-project-cleanup-core-engine.md`'s own
notes on this), run `cargo clippy -p openstack_sdk --lib --tests
--all-features` instead to confirm the code compiles and typechecks, and
note in the task report that test execution itself could not be verified
in this sandbox.

- [ ] **Step 4: Commit**

```bash
git add openstack_sdk/src/cleanup/providers/compute.rs openstack_sdk/src/cleanup/providers/mod.rs openstack_sdk/src/cleanup/mod.rs
git commit -m "feat(sdk): Add compute cleanup provider"
```

---

### Task 2: `BlockStorageCleanupProvider` + cross-service relations

**Files:**
- Create: `openstack_sdk/src/cleanup/providers/block_storage.rs`
- Modify: `openstack_sdk/src/cleanup/providers/mod.rs`
- Modify: `openstack_sdk/src/cleanup/mod.rs`

**Interfaces:**
- Consumes: `crate::cleanup::providers::compute::SERVER` (from Task 1,
  `#[cfg(feature = "compute")]`-guarded use), same
  `provider`/`relations`/`types` imports as Task 1, plus
  `crate::api::block_storage::v3::{volume, snapshot}::{list, delete}`.
- Produces: `pub const VOLUME: ResourceKind = ResourceKind::new("block-storage", "volume");`,
  `pub const SNAPSHOT: ResourceKind = ResourceKind::new("block-storage", "snapshot");`,
  `pub struct BlockStorageCleanupProvider;`.

- [ ] **Step 1: Write the provider with its discover/relation/delete tests**

Create `openstack_sdk/src/cleanup/providers/block_storage.rs`:

```rust
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

//! Block storage (Cinder) cleanup provider.
//!
//! Declares two resource-level relations: a snapshot blocks its volume
//! from deleting until the snapshot is itself selected (and is pulled in
//! automatically via `CascadeGroup` when its volume is selected — same
//! shape as the network provider's `NETWORK`/`SUBNET` pair), and — when
//! the `compute` feature is also enabled — a volume attached to a still-
//! live, unselected server is blocked from deleting until that server is
//! gone. There is deliberately no `CascadeGroup` between a volume and its
//! attached server: deleting a server must not imply deleting its
//! volumes.

use async_trait::async_trait;
use serde_json::Value;

use crate::api::block_storage::v3::snapshot;
use crate::api::block_storage::v3::volume;
use crate::api::{Pagination, QueryAsync, paged, raw};

use crate::cleanup::provider::{CleanupContext, CleanupDependency, CleanupError, CleanupProvider};
use crate::cleanup::relations::{RelationEffect, RelationRule};
use crate::cleanup::types::{PlannedResource, ResourceKind};

pub const VOLUME: ResourceKind = ResourceKind::new("block-storage", "volume");
pub const SNAPSHOT: ResourceKind = ResourceKind::new("block-storage", "snapshot");

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

/// True if `volume`'s `attachments` array (each entry an object with a
/// `server_id` field) contains an attachment to `server_id`.
fn volume_attached_to(volume: &Value, server_id: &str) -> bool {
    volume
        .get("attachments")
        .and_then(|v| v.as_array())
        .is_some_and(|attachments| {
            attachments
                .iter()
                .any(|a| value_str(a, "server_id") == Some(server_id))
        })
}

#[derive(Debug, Default)]
pub struct BlockStorageCleanupProvider;

#[async_trait]
impl CleanupProvider for BlockStorageCleanupProvider {
    fn service_type(&self) -> &'static str {
        "block-storage"
    }

    fn dependencies(&self) -> CleanupDependency {
        CleanupDependency::default()
    }

    fn relations(&self) -> Vec<RelationRule> {
        let mut rules = vec![
            RelationRule {
                parent_kind: VOLUME,
                child_kind: SNAPSHOT,
                matches: |child, parent| {
                    value_str(&child.raw, "volume_id") == value_str(&parent.raw, "id")
                },
                effect: RelationEffect::Blocks,
            },
            RelationRule {
                parent_kind: VOLUME,
                child_kind: SNAPSHOT,
                matches: |child, parent| {
                    value_str(&child.raw, "volume_id") == value_str(&parent.raw, "id")
                },
                effect: RelationEffect::CascadeGroup,
            },
        ];

        #[cfg(feature = "compute")]
        rules.push(RelationRule {
            parent_kind: VOLUME,
            child_kind: crate::cleanup::providers::compute::SERVER,
            matches: |child, parent| volume_attached_to(&parent.raw, &child.id),
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

        let volumes: Vec<Value> = paged(
            volume::list::Request::builder().build().map_err(|e| {
                CleanupError::Engine(format!("failed to build volume list request: {e}"))
            })?,
            Pagination::All,
        )
        .query_async(ctx.client)
        .await
        .map_err(|e| list_err(VOLUME)(e.into()))?;
        for v in volumes {
            nodes.push(to_planned(VOLUME, v));
        }

        let snapshots: Vec<Value> = paged(
            snapshot::list::Request::builder().build().map_err(|e| {
                CleanupError::Engine(format!("failed to build snapshot list request: {e}"))
            })?,
            Pagination::All,
        )
        .query_async(ctx.client)
        .await
        .map_err(|e| list_err(SNAPSHOT)(e.into()))?;
        for v in snapshots {
            nodes.push(to_planned(SNAPSHOT, v));
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

        if resource.kind == VOLUME {
            let req = volume::delete::Request::builder()
                .id(resource.id.clone())
                .build()
                .map_err(|e| {
                    CleanupError::Engine(format!("failed to build volume delete request: {e}"))
                })?;
            raw(req)
                .query_async(ctx.client)
                .await
                .map_err(|e| err(e.into()))?;
        } else if resource.kind == SNAPSHOT {
            let req = snapshot::delete::Request::builder()
                .id(resource.id.clone())
                .build()
                .map_err(|e| {
                    CleanupError::Engine(format!("failed to build snapshot delete request: {e}"))
                })?;
            raw(req)
                .query_async(ctx.client)
                .await
                .map_err(|e| err(e.into()))?;
        } else {
            return Err(CleanupError::Engine(format!(
                "BlockStorageCleanupProvider cannot delete resource kind {:?}",
                resource.kind
            )));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cleanup::engine::ProjectCleanupBuilder;
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
                    {"type": "block-storage", "name": "cinderv3", "endpoints": [{"id": "bs-1",
                        "url": format!("{base_url}/v3/test-project"), "region": "RegionOne", "interface": "public"}]}
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
    async fn discover_marks_volume_with_snapshot_blocked_until_snapshot_selected() {
        let server = MockServer::start_async().await;
        let client = mock_client(&server).await;

        server.mock(|when, then| {
            when.method(httpmock::Method::GET).path("/v3/test-project/volumes");
            then.status(200).json_body(serde_json::json!({"volumes": [
                {"id": "vol-1", "name": "data"}
            ]}));
        });
        server.mock(|when, then| {
            when.method(httpmock::Method::GET).path("/v3/test-project/snapshots");
            then.status(200).json_body(serde_json::json!({"snapshots": [
                {"id": "snap-1", "name": "data-snap", "volume_id": "vol-1"}
            ]}));
        });

        let cleanup = ProjectCleanupBuilder::new(&client)
            .with_provider(BlockStorageCleanupProvider)
            .build();

        let eval: crate::cleanup::provider::EvaluationFn =
            std::sync::Arc::new(|r: &PlannedResource| {
                r.kind == VOLUME && r.name.as_deref() == Some("data")
            });

        let plan = cleanup
            .discover(HashMap::new(), Some(eval))
            .await
            .expect("discover failed");

        let vol = plan.nodes.iter().find(|n| n.id == "vol-1").unwrap();
        let snap = plan.nodes.iter().find(|n| n.id == "snap-1").unwrap();
        assert!(vol.selected);
        assert!(snap.selected, "snapshot must be pulled in by the cascade group");

        server.mock(|when, then| {
            when.method(httpmock::Method::DELETE)
                .path("/v3/test-project/snapshots/snap-1");
            then.status(202);
        });
        server.mock(|when, then| {
            when.method(httpmock::Method::DELETE)
                .path("/v3/test-project/volumes/vol-1");
            then.status(202);
        });

        let result = cleanup.apply(plan).await.expect("apply failed");
        assert!(result.errors.is_empty(), "unexpected errors: {:?}", result.errors);
        let snap_pos = result.deleted_ids.iter().position(|id| id == "snap-1").unwrap();
        let vol_pos = result.deleted_ids.iter().position(|id| id == "vol-1").unwrap();
        assert!(snap_pos < vol_pos, "snapshot must delete before its volume");
    }

    #[cfg(feature = "compute")]
    #[test]
    fn volume_attached_to_server_produces_blocks_edge_against_that_server() {
        use crate::cleanup::providers::compute::SERVER;

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
                VOLUME,
                "vol-1",
                serde_json::json!({"id": "vol-1", "attachments": [{"server_id": "server-1"}]}),
            ),
            node(VOLUME, "vol-2", serde_json::json!({"id": "vol-2", "attachments": []})),
        ];

        let rules = BlockStorageCleanupProvider.relations();
        let edges = evaluate_edges(&nodes, &rules);

        let vol1_idx = nodes.iter().position(|n| n.id == "vol-1").unwrap();
        let vol2_idx = nodes.iter().position(|n| n.id == "vol-2").unwrap();
        let server_idx = nodes.iter().position(|n| n.id == "server-1").unwrap();

        assert!(
            edges.iter().any(|e| matches!(e.effect, RelationEffect::Blocks)
                && e.child == server_idx
                && e.parent == vol1_idx),
            "attached volume must be blocked by its server"
        );
        assert!(
            !edges.iter().any(|e| e.parent == vol2_idx),
            "unattached volume must not gain a server-blocking edge"
        );
    }
}
```

- [ ] **Step 2: Register the module and re-export the provider**

In `openstack_sdk/src/cleanup/providers/mod.rs`, add:

```rust
#[cfg(feature = "block_storage")]
pub mod block_storage;
```

In `openstack_sdk/src/cleanup/mod.rs`, add:

```rust
#[cfg(feature = "block_storage")]
pub use providers::block_storage::BlockStorageCleanupProvider;
```

- [ ] **Step 3: Run the new tests**

Run: `cargo test -p openstack_sdk --lib --all-features cleanup::providers::block_storage -- --nocapture`
Expected: all three tests pass. If sandboxed loopback networking blocks
the `httpmock`-backed async tests here, fall back to `cargo clippy -p
openstack_sdk --lib --tests --all-features` to verify compilation, and
also run with `--no-default-features --features
async,block_storage,network` to confirm the `#[cfg(feature = "compute")]`
guard actually keeps the crate compiling with `compute` disabled — this
is the one new correctness property this task introduces, and it needs
this exact command to prove it (clippy with `--all-features` alone would
never exercise the guard's false branch).

- [ ] **Step 4: Commit**

```bash
git add openstack_sdk/src/cleanup/providers/block_storage.rs openstack_sdk/src/cleanup/providers/mod.rs openstack_sdk/src/cleanup/mod.rs
git commit -m "feat(sdk): Add block-storage cleanup provider"
```

---

### Task 3: `ImageCleanupProvider`

**Files:**
- Create: `openstack_sdk/src/cleanup/providers/image.rs`
- Modify: `openstack_sdk/src/cleanup/providers/mod.rs`
- Modify: `openstack_sdk/src/cleanup/mod.rs`

**Interfaces:**
- Consumes: same `provider`/`relations`/`types` imports as Tasks 1-2, plus
  `crate::api::image::v2::image::{list, delete}`.
- Produces: `pub const IMAGE: ResourceKind = ResourceKind::new("image", "image");`,
  `pub struct ImageCleanupProvider;`.

- [ ] **Step 1: Write the provider with its discover/delete test**

Create `openstack_sdk/src/cleanup/providers/image.rs`:

```rust
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

//! Image (Glance) cleanup provider.
//!
//! No resource-level relations and no service-level ordering hint:
//! glance permits deleting an image regardless of what still references
//! it (booted servers, volumes created from it), so there is no ordering
//! constraint to express.

use async_trait::async_trait;
use serde_json::Value;

use crate::api::image::v2::image;
use crate::api::{Pagination, QueryAsync, paged, raw};

use crate::cleanup::provider::{CleanupContext, CleanupDependency, CleanupError, CleanupProvider};
use crate::cleanup::relations::RelationRule;
use crate::cleanup::types::{PlannedResource, ResourceKind};

pub const IMAGE: ResourceKind = ResourceKind::new("image", "image");

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

#[derive(Debug, Default)]
pub struct ImageCleanupProvider;

#[async_trait]
impl CleanupProvider for ImageCleanupProvider {
    fn service_type(&self) -> &'static str {
        "image"
    }

    fn dependencies(&self) -> CleanupDependency {
        CleanupDependency::default()
    }

    fn relations(&self) -> Vec<RelationRule> {
        Vec::new()
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

        let images: Vec<Value> = paged(
            image::list::Request::builder().build().map_err(|e| {
                CleanupError::Engine(format!("failed to build image list request: {e}"))
            })?,
            Pagination::All,
        )
        .query_async(ctx.client)
        .await
        .map_err(|e| list_err(IMAGE)(e.into()))?;
        for v in images {
            nodes.push(to_planned(IMAGE, v));
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

        if resource.kind == IMAGE {
            let req = image::delete::Request::builder()
                .id(resource.id.clone())
                .build()
                .map_err(|e| {
                    CleanupError::Engine(format!("failed to build image delete request: {e}"))
                })?;
            raw(req)
                .query_async(ctx.client)
                .await
                .map_err(|e| err(e.into()))?;
        } else {
            return Err(CleanupError::Engine(format!(
                "ImageCleanupProvider cannot delete resource kind {:?}",
                resource.kind
            )));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cleanup::engine::ProjectCleanupBuilder;
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
                    {"type": "image", "name": "glance", "endpoints": [{"id": "image-1",
                        "url": format!("{base_url}/v2"), "region": "RegionOne", "interface": "public"}]}
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
    async fn discover_and_delete_an_image() {
        let server = MockServer::start_async().await;
        let client = mock_client(&server).await;

        server.mock(|when, then| {
            when.method(httpmock::Method::GET).path("/v2/images");
            then.status(200).json_body(serde_json::json!({"images": [
                {"id": "img-1", "name": "test-image"}
            ]}));
        });
        server.mock(|when, then| {
            when.method(httpmock::Method::DELETE).path("/v2/images/img-1");
            then.status(204);
        });

        let cleanup = ProjectCleanupBuilder::new(&client)
            .with_provider(ImageCleanupProvider)
            .build();

        let eval: crate::cleanup::provider::EvaluationFn =
            std::sync::Arc::new(|r: &PlannedResource| r.kind == IMAGE);
        let plan = cleanup
            .discover(HashMap::new(), Some(eval))
            .await
            .expect("discover failed");
        let result = cleanup.apply(plan).await.expect("apply failed");

        assert!(result.errors.is_empty(), "unexpected errors: {:?}", result.errors);
        assert!(result.deleted_ids.contains(&"img-1".to_string()));
    }
}
```

- [ ] **Step 2: Register the module and re-export the provider**

In `openstack_sdk/src/cleanup/providers/mod.rs`, add:

```rust
#[cfg(feature = "image")]
pub mod image;
```

In `openstack_sdk/src/cleanup/mod.rs`, add:

```rust
#[cfg(feature = "image")]
pub use providers::image::ImageCleanupProvider;
```

- [ ] **Step 3: Run the new test**

Run: `cargo test -p openstack_sdk --lib --all-features cleanup::providers::image -- --nocapture`
Expected: `discover_and_delete_an_image` passes (or, if sandboxed loopback
networking blocks it, fall back to `cargo clippy -p openstack_sdk --lib
--tests --all-features`, as in Tasks 1-2).

- [ ] **Step 4: Commit**

```bash
git add openstack_sdk/src/cleanup/providers/image.rs openstack_sdk/src/cleanup/providers/mod.rs openstack_sdk/src/cleanup/mod.rs
git commit -m "feat(sdk): Add image cleanup provider"
```

---

### Task 4: Functional test scaffolding (creation helpers + cleanup guard)

**Files:**
- Create: `openstack_sdk/tests/cleanup/mod.rs`
- Create: `openstack_sdk/tests/cleanup/helpers.rs`
- Modify: `openstack_sdk/tests/main.rs`

**Interfaces:**
- Consumes: `openstack_sdk::AsyncOpenStack`, `openstack_sdk::config::ConfigFile`,
  `openstack_sdk::api::{compute::v2::server, block_storage::v3::{volume, snapshot},
  image::v2::image, network::v2::{network, subnet, router, port}}` create/get
  endpoints (all under the `openstack_sdk` public crate, same import root
  used by `tests/connection/*.rs`, e.g. `openstack_sdk::api::network::v2::network`).
- Produces: `pub struct CreatedResources<'a> { client: &'a AsyncOpenStack, created: Vec<(&'static str, String)> }`
  with `pub fn track(&mut self, service_type: &'static str, id: impl Into<String>)`
  and a `Drop` impl doing best-effort reverse-order deletes; and one
  `pub async fn create_*` helper per resource type, each returning the
  created resource's id (and, for the server, waiting for `ACTIVE`) —
  `Task 5` calls these by name.

- [ ] **Step 1: Write `openstack_sdk/tests/cleanup/helpers.rs`**

```rust
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
use openstack_sdk::api::block_storage::v3::{snapshot, volume};
use openstack_sdk::api::compute::v2::server;
use openstack_sdk::api::image::v2::image;
use openstack_sdk::api::network::v2::{network, router, subnet};
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
        // `Drop` cannot be async; spawn a blocking-friendly current-thread
        // runtime to run best-effort deletes synchronously before the
        // test process exits. `tokio::test` already runs on a
        // multi-thread runtime, so `Handle::current().block_on` cannot be
        // used from `Drop` (would panic if called from within that same
        // runtime); use a dedicated runtime instead.
        let rt = match tokio::runtime::Runtime::new() {
            Ok(rt) => rt,
            Err(_) => return,
        };
        rt.block_on(async move {
            for (service_type, id) in created.into_iter().rev() {
                let _ = match service_type {
                    "compute" => server::delete::Request::builder()
                        .id(id)
                        .build()
                        .ok()
                        .map(raw)
                        .expect("request build must not fail")
                        .query_async(client)
                        .await
                        .map(|_| ()),
                    "block-storage-snapshot" => snapshot::delete::Request::builder()
                        .id(id)
                        .build()
                        .ok()
                        .map(raw)
                        .expect("request build must not fail")
                        .query_async(client)
                        .await
                        .map(|_| ()),
                    "block-storage-volume" => volume::delete::Request::builder()
                        .id(id)
                        .build()
                        .ok()
                        .map(raw)
                        .expect("request build must not fail")
                        .query_async(client)
                        .await
                        .map(|_| ()),
                    "network" => network::delete::Request::builder()
                        .id(id)
                        .build()
                        .ok()
                        .map(raw)
                        .expect("request build must not fail")
                        .query_async(client)
                        .await
                        .map(|_| ()),
                    _ => Ok(()),
                };
            }
        });
    }
}

/// Resolves the boot image id by name: `TEST_IMAGE_NAME` if set, else
/// `"cirros-0.6.2-x86_64-disk"` (matching python openstacksdk's
/// functional-test default). Does not create or delete this image — it
/// is expected to pre-exist on the target cloud.
pub async fn resolve_test_image_id(
    client: &AsyncOpenStack,
) -> Result<String, Box<dyn std::error::Error>> {
    let name = std::env::var("TEST_IMAGE_NAME")
        .unwrap_or_else(|_| "cirros-0.6.2-x86_64-disk".to_string());
    let images: Vec<serde_json::Value> = paged(
        image::list::Request::builder().build()?,
        Pagination::All,
    )
    .query_async(client)
    .await?;
    let found = images
        .into_iter()
        .find(|img| img.get("name").and_then(|v| v.as_str()) == Some(name.as_str()))
        .ok_or_else(|| format!("no image named '{name}' found on target cloud"))?;
    Ok(found
        .get("id")
        .and_then(|v| v.as_str())
        .expect("image response must carry an id")
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
        .expect("network create response must carry an id")
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
                .cidr(Some(cidr.to_string()))
                .ip_version(4)
                .build()?,
        )
        .build()?;
    let resp: serde_json::Value = req.query_async(client).await?;
    Ok(resp["id"]
        .as_str()
        .expect("subnet create response must carry an id")
        .to_string())
}

pub async fn create_router_with_interface(
    client: &AsyncOpenStack,
    name: &str,
    subnet_id: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let req = router::create::Request::builder()
        .router(router::create::RouterBuilder::default().name(name.to_string()).build()?)
        .build()?;
    let resp: serde_json::Value = req.query_async(client).await?;
    let router_id = resp["id"]
        .as_str()
        .expect("router create response must carry an id")
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
                .name(Some(name.to_string()))
                .image_ref(Some(image_id.to_string()))
                .size(Some(size_gib))
                .build()?,
        )
        .build()?;
    let resp: serde_json::Value = req.query_async(client).await?;
    Ok(resp["id"]
        .as_str()
        .expect("volume create response must carry an id")
        .to_string())
}

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
        if let Some(v) = vols.iter().find(|v| v["id"].as_str() == Some(volume_id))
            && v["status"].as_str() == Some("available")
        {
            return Ok(());
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
                .name(Some(name.to_string()))
                .volume_id(volume_id.to_string())
                .build()?,
        )
        .build()?;
    let resp: serde_json::Value = req.query_async(client).await?;
    Ok(resp["id"]
        .as_str()
        .expect("snapshot create response must carry an id")
        .to_string())
}

pub async fn create_server_from_volume(
    client: &AsyncOpenStack,
    name: &str,
    flavor_id: &str,
    volume_id: &str,
    network_id: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let req = server::create_20::Request::builder()
        .server(
            server::create_20::ServerBuilder::default()
                .name(name.to_string())
                .flavor_ref(flavor_id.to_string())
                .block_device_mapping_v2(vec![
                    // `boot_index` is `Option<Option<Cow<str>>>` on the
                    // generated type — needs `Some(...)`; `uuid`,
                    // `source_type`, `destination_type`,
                    // `delete_on_termination` are single-layer `Option`s
                    // and take their value directly.
                    server::create_20::BlockDeviceMappingV2Builder::default()
                        .uuid(volume_id.to_string())
                        .source_type(server::create_20::SourceType::Volume)
                        .destination_type(server::create_20::DestinationType::Volume)
                        .boot_index(Some("0".to_string()))
                        .delete_on_termination(false)
                        .build()?,
                ])
                .networks(vec![
                    server::create_20::NetworksBuilder::default()
                        .uuid(network_id.to_string())
                        .build()?,
                ])
                .build()?,
        )
        .build()?;
    let resp: serde_json::Value = req.query_async(client).await?;
    Ok(resp["id"]
        .as_str()
        .expect("server create response must carry an id")
        .to_string())
}

pub async fn wait_for_server_active(
    client: &AsyncOpenStack,
    server_id: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    for _ in 0..60 {
        let req = server::get::Request::builder().id(server_id).build()?;
        let resp: serde_json::Value = req.query_async(client).await?;
        match resp["status"].as_str() {
            Some("ACTIVE") => return Ok(()),
            Some("ERROR") => return Err(format!("server {server_id} went to ERROR").into()),
            _ => {}
        }
        tokio::time::sleep(Duration::from_secs(5)).await;
    }
    Err(format!("server {server_id} did not become ACTIVE in time").into())
}

/// Any flavor works for this scenario (no I/O-heavy workload runs on the
/// server) — picks the first one the list endpoint returns.
pub async fn first_flavor_id(client: &AsyncOpenStack) -> Result<String, Box<dyn std::error::Error>> {
    use openstack_sdk::api::compute::v2::flavor;
    let flavors: Vec<serde_json::Value> = paged(
        flavor::list_20::Request::builder().build()?,
        Pagination::All,
    )
    .query_async(client)
    .await?;
    Ok(flavors
        .first()
        .and_then(|f| f["id"].as_str())
        .ok_or("no flavors available on target cloud")?
        .to_string())
}
```

- [ ] **Step 2: Write `openstack_sdk/tests/cleanup/mod.rs`**

```rust
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

mod full_scenario;
mod helpers;
```

- [ ] **Step 3: Wire the new module into `openstack_sdk/tests/main.rs`**

In `openstack_sdk/tests/main.rs`, add alongside the existing `mod
connection;`:

```rust
mod cleanup;
```

- [ ] **Step 4: Verify compilation**

`full_scenario.rs` doesn't exist yet (Task 5 writes it), so `mod
full_scenario;` in `cleanup/mod.rs` will fail to resolve. Create a
placeholder to unblock this task's compile check, which Task 5 replaces
outright:

```rust
// openstack_sdk/tests/cleanup/full_scenario.rs (placeholder, replaced by Task 5)
```

Run: `cargo build -p openstack_sdk --test functional --all-features`
Expected: builds cleanly (no `OS_CLOUD` needed to build — only to run).

- [ ] **Step 5: Commit**

```bash
git add openstack_sdk/tests/cleanup openstack_sdk/tests/main.rs
git commit -m "test(sdk): Add cleanup functional test scaffolding"
```

---

### Task 5: Full cross-service functional scenario

**Files:**
- Modify: `openstack_sdk/tests/cleanup/full_scenario.rs` (replaces Task 4's
  placeholder)

**Interfaces:**
- Consumes: every helper from Task 4's `helpers.rs` (`resolve_test_image_id`,
  `create_network`, `create_subnet`, `create_router_with_interface`,
  `create_volume_from_image`, `wait_for_volume_available`, `create_snapshot`,
  `first_flavor_id`, `create_server_from_volume`, `wait_for_server_active`,
  `CreatedResources`), plus `openstack_sdk::cleanup::{ProjectCleanupBuilder,
  NetworkCleanupProvider, ComputeCleanupProvider, BlockStorageCleanupProvider,
  ImageCleanupProvider, PlannedResource}` (all four providers behind the
  crate's default features, already enabled — see Global Constraints).
- Produces: nothing consumed by a later task — this is the plan's final
  deliverable.

- [ ] **Step 1: Write the scenario**

Replace the placeholder `openstack_sdk/tests/cleanup/full_scenario.rs`
with:

```rust
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

//! Exercises the project-cleanup engine against a real OpenStack cloud
//! across all four built-in providers (network, compute, block-storage,
//! image) in one related scenario, proving the engine's relation-aware
//! cross-resource ordering with real resources — not just per-provider
//! CRUD. Run against devstack in CI (`.github/workflows/functional.yml`)
//! and manually against a real cloud during development (`OS_CLOUD` +
//! optionally `TEST_IMAGE_NAME`).

use std::env;

use openstack_sdk::cleanup::{
    BlockStorageCleanupProvider, ComputeCleanupProvider, ImageCleanupProvider,
    NetworkCleanupProvider, PlannedResource, ProjectCleanupBuilder,
};
use openstack_sdk::{AsyncOpenStack, config::ConfigFile};

use super::helpers::{
    CreatedResources, create_network, create_router_with_interface, create_server_from_volume,
    create_snapshot, create_subnet, create_volume_from_image, first_flavor_id,
    resolve_test_image_id, wait_for_server_active, wait_for_volume_available,
};

#[tokio::test(flavor = "multi_thread")]
async fn cleanup_engine_removes_related_resources_across_all_services()
-> Result<(), Box<dyn std::error::Error>> {
    let cfg = ConfigFile::new().unwrap();
    let profile = cfg
        .get_cloud_config(env::var("OS_CLOUD").expect("OS_CLOUD variable set"))
        .unwrap()
        .unwrap();
    let client = AsyncOpenStack::new(&profile).await?;

    let marker = format!("osc-cleanup-test-{}", uuid::Uuid::new_v4());
    let mut created = CreatedResources::new(&client);

    // 1. Network + subnet + router-with-interface.
    let network_id = create_network(&client, &format!("{marker}-net")).await?;
    created.track("network", network_id.clone());
    let subnet_id = create_subnet(&client, &format!("{marker}-subnet"), &network_id, "10.250.0.0/24").await?;
    let _router_id = create_router_with_interface(&client, &format!("{marker}-router"), &subnet_id).await?;

    // 2. Resolve the pre-existing boot image (never created/deleted by
    //    this test — see `resolve_test_image_id`'s doc comment).
    let image_id = resolve_test_image_id(&client).await?;

    // 3. Volume from that image, then a snapshot of it.
    let volume_id = create_volume_from_image(&client, &format!("{marker}-vol"), &image_id, 1).await?;
    created.track("block-storage-volume", volume_id.clone());
    wait_for_volume_available(&client, &volume_id).await?;
    let snapshot_id = create_snapshot(&client, &format!("{marker}-snap"), &volume_id).await?;
    created.track("block-storage-snapshot", snapshot_id.clone());

    // 4. Server booted from that volume, attached to the created network.
    let flavor_id = first_flavor_id(&client).await?;
    let server_id =
        create_server_from_volume(&client, &format!("{marker}-server"), &flavor_id, &volume_id, &network_id)
            .await?;
    created.track("compute", server_id.clone());
    wait_for_server_active(&client, &server_id).await?;

    // 5. Run the real cleanup engine, selecting everything whose name
    //    carries this test's marker. The network's subnet and
    //    router-interface have no `name` field of their own but are
    //    pulled in by the engine's existing `CascadeGroup` rules once
    //    the network itself is selected (same mechanism the network
    //    provider's own unit tests already exercise against httpmock).
    //    The pre-existing boot image is never selected: its name carries
    //    no marker.
    let cleanup = ProjectCleanupBuilder::new(&client)
        .with_provider(NetworkCleanupProvider)
        .with_provider(ComputeCleanupProvider)
        .with_provider(BlockStorageCleanupProvider)
        .with_provider(ImageCleanupProvider)
        .build();

    let marker_for_eval = marker.clone();
    let eval: openstack_sdk::cleanup::provider::EvaluationFn = std::sync::Arc::new(move |r: &PlannedResource| {
        r.name
            .as_deref()
            .is_some_and(|n| n.starts_with(&marker_for_eval))
    });

    let plan = cleanup
        .discover(std::collections::HashMap::new(), Some(eval))
        .await
        .expect("discover failed");
    let result = cleanup.apply(plan).await.expect("apply failed");

    assert!(result.errors.is_empty(), "unexpected cleanup errors: {:?}", result.errors);

    for id in [&network_id, &volume_id, &snapshot_id, &server_id] {
        assert!(
            result.deleted_ids.contains(id),
            "expected {id} to be deleted, deleted_ids = {:?}",
            result.deleted_ids
        );
    }

    let pos = |id: &str| {
        result
            .deleted_ids
            .iter()
            .position(|d| d == id)
            .unwrap_or_else(|| panic!("{id} missing from deleted_ids"))
    };
    assert!(pos(&snapshot_id) < pos(&volume_id), "snapshot must delete before its volume");
    assert!(pos(&server_id) < pos(&volume_id), "server must delete before its attached volume");

    // Everything above was deleted by the engine under test; `created`'s
    // `Drop` runs now and its deletes are expected to no-op against
    // already-gone resources.
    drop(created);

    Ok(())
}
```

- [ ] **Step 2: Verify compilation**

Run: `cargo build -p openstack_sdk --test functional --all-features`
Expected: builds cleanly. This cannot be run to completion in this
sandbox (loopback networking blocked — no live OpenStack reachable), so
compilation is the verification bar here; actual execution happens per
the next step.

- [ ] **Step 3: Run against a real cloud (implementation-time verification)**

Run, against a real OpenStack cloud reachable from wherever
implementation happens (not this sandbox):
```bash
OS_CLOUD=<your-real-cloud> TEST_IMAGE_NAME=<an-image-name-on-that-cloud> \
  cargo nextest run -p openstack_sdk --test functional cleanup:: --all-features
```
Expected: `cleanup_engine_removes_related_resources_across_all_services`
passes, and no test resources remain on the target cloud afterward
(spot-check via `openstack server list`, `openstack volume list`,
`openstack network list` for the run's marker prefix, or your cloud's
equivalent).

- [ ] **Step 4: Commit**

```bash
git add openstack_sdk/tests/cleanup/full_scenario.rs
git commit -m "test(sdk): Add cross-service cleanup functional scenario"
```

---

## Final Verification

- [ ] `cargo clippy -p openstack_sdk --lib --tests --all-features` clean.
- [ ] `cargo clippy -p openstack_sdk --lib --tests --no-default-features --features async,network,block_storage` clean (proves the `#[cfg(feature = "compute")]` guard in `block_storage.rs` actually holds with `compute` disabled).
- [ ] `cargo test -p openstack_sdk --lib --all-features cleanup::` passes (or, if this sandbox's loopback-networking block prevents `httpmock`-backed async tests from connecting, note that explicitly and rely on the clippy checks above plus Task 5's real-cloud run).
- [ ] `openstack_sdk/tests/cleanup/full_scenario.rs` passed against a real cloud per Task 5, Step 3.
