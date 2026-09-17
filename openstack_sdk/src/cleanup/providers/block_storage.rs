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

use crate::api::block_storage::v3::backup;
use crate::api::block_storage::v3::snapshot;
use crate::api::block_storage::v3::volume;
use crate::api::{Pagination, QueryAsync, paged, raw};

use crate::cleanup::provider::{CleanupContext, CleanupDependency, CleanupError, CleanupProvider};
use crate::cleanup::relations::{RelationEffect, RelationRule};
use crate::cleanup::types::{PlannedResource, ResourceKind};

pub const VOLUME: ResourceKind = ResourceKind::new("block-storage", "volume");
pub const SNAPSHOT: ResourceKind = ResourceKind::new("block-storage", "snapshot");
pub const BACKUP: ResourceKind = ResourceKind::new("block-storage", "backup");

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
        #[allow(unused_mut)]
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
            RelationRule {
                parent_kind: VOLUME,
                child_kind: BACKUP,
                matches: |child, parent| {
                    value_str(&child.raw, "volume_id") == value_str(&parent.raw, "id")
                },
                effect: RelationEffect::Blocks,
            },
            RelationRule {
                parent_kind: VOLUME,
                child_kind: BACKUP,
                matches: |child, parent| {
                    value_str(&child.raw, "volume_id") == value_str(&parent.raw, "id")
                },
                effect: RelationEffect::CascadeGroup,
            },
            // A backup taken from a snapshot (rather than directly from a
            // volume) carries that snapshot's id - delete it before its
            // source snapshot, same as it deletes before its volume.
            RelationRule {
                parent_kind: SNAPSHOT,
                child_kind: BACKUP,
                matches: |child, parent| {
                    value_str(&child.raw, "snapshot_id") == value_str(&parent.raw, "id")
                },
                effect: RelationEffect::Blocks,
            },
            RelationRule {
                parent_kind: SNAPSHOT,
                child_kind: BACKUP,
                matches: |child, parent| {
                    value_str(&child.raw, "snapshot_id") == value_str(&parent.raw, "id")
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

        // `list` (non-detailed) only returns `id`/`name`/`links` - the
        // relation rules below need `attachments` (volume), `volume_id`
        // (snapshot/backup) and `snapshot_id` (backup), which only
        // `list_detailed` provides.
        let volumes: Vec<Value> = paged(
            volume::list_detailed::Request::builder()
                .build()
                .map_err(|e| {
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
            snapshot::list_detailed::Request::builder()
                .build()
                .map_err(|e| {
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

        let backups: Vec<Value> = paged(
            backup::list_detailed::Request::builder()
                .build()
                .map_err(|e| {
                    CleanupError::Engine(format!("failed to build backup list request: {e}"))
                })?,
            Pagination::All,
        )
        .query_async(ctx.client)
        .await
        .map_err(|e| list_err(BACKUP)(e.into()))?;
        for v in backups {
            nodes.push(to_planned(BACKUP, v));
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
        } else if resource.kind == BACKUP {
            let req = backup::delete::Request::builder()
                .id(resource.id.clone())
                .build()
                .map_err(|e| {
                    CleanupError::Engine(format!("failed to build backup delete request: {e}"))
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
    #[cfg(feature = "compute")]
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
            when.method(httpmock::Method::GET)
                .path("/v3/test-project/volumes/detail");
            then.status(200).json_body(serde_json::json!({"volumes": [
                {"id": "vol-1", "name": "data"}
            ]}));
        });
        server.mock(|when, then| {
            when.method(httpmock::Method::GET)
                .path("/v3/test-project/snapshots/detail");
            then.status(200).json_body(serde_json::json!({"snapshots": [
                {"id": "snap-1", "name": "data-snap", "volume_id": "vol-1"}
            ]}));
        });
        server.mock(|when, then| {
            when.method(httpmock::Method::GET)
                .path("/v3/test-project/backups/detail");
            then.status(200)
                .json_body(serde_json::json!({"backups": []}));
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
        assert!(
            snap.selected,
            "snapshot must be pulled in by the cascade group"
        );

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
        assert!(
            result.errors.is_empty(),
            "unexpected errors: {:?}",
            result.errors
        );
        let snap_pos = result
            .deleted_ids
            .iter()
            .position(|id| id == "snap-1")
            .unwrap();
        let vol_pos = result
            .deleted_ids
            .iter()
            .position(|id| id == "vol-1")
            .unwrap();
        assert!(snap_pos < vol_pos, "snapshot must delete before its volume");
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn discover_marks_volume_with_backup_blocked_until_backup_selected() {
        let server = MockServer::start_async().await;
        let client = mock_client(&server).await;

        server.mock(|when, then| {
            when.method(httpmock::Method::GET)
                .path("/v3/test-project/volumes/detail");
            then.status(200).json_body(serde_json::json!({"volumes": [
                {"id": "vol-1", "name": "data"}
            ]}));
        });
        server.mock(|when, then| {
            when.method(httpmock::Method::GET)
                .path("/v3/test-project/snapshots/detail");
            then.status(200)
                .json_body(serde_json::json!({"snapshots": []}));
        });
        server.mock(|when, then| {
            when.method(httpmock::Method::GET)
                .path("/v3/test-project/backups/detail");
            then.status(200).json_body(serde_json::json!({"backups": [
                {"id": "backup-1", "name": "data-backup", "volume_id": "vol-1"}
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
        let backup = plan.nodes.iter().find(|n| n.id == "backup-1").unwrap();
        assert!(vol.selected);
        assert!(
            backup.selected,
            "backup must be pulled in by the cascade group"
        );

        server.mock(|when, then| {
            when.method(httpmock::Method::DELETE)
                .path("/v3/test-project/backups/backup-1");
            then.status(202);
        });
        server.mock(|when, then| {
            when.method(httpmock::Method::DELETE)
                .path("/v3/test-project/volumes/vol-1");
            then.status(202);
        });

        let result = cleanup.apply(plan).await.expect("apply failed");
        assert!(
            result.errors.is_empty(),
            "unexpected errors: {:?}",
            result.errors
        );
        let backup_pos = result
            .deleted_ids
            .iter()
            .position(|id| id == "backup-1")
            .unwrap();
        let vol_pos = result
            .deleted_ids
            .iter()
            .position(|id| id == "vol-1")
            .unwrap();
        assert!(backup_pos < vol_pos, "backup must delete before its volume");
    }

    #[tokio::test]
    async fn discover_marks_snapshot_with_backup_blocked_until_backup_selected() {
        let server = MockServer::start_async().await;
        let client = mock_client(&server).await;

        server.mock(|when, then| {
            when.method(httpmock::Method::GET)
                .path("/v3/test-project/volumes/detail");
            then.status(200).json_body(serde_json::json!({"volumes": [
                {"id": "vol-1", "name": "data"}
            ]}));
        });
        server.mock(|when, then| {
            when.method(httpmock::Method::GET)
                .path("/v3/test-project/snapshots/detail");
            then.status(200).json_body(serde_json::json!({"snapshots": [
                {"id": "snap-1", "name": "data-snap", "volume_id": "vol-1"}
            ]}));
        });
        server.mock(|when, then| {
            when.method(httpmock::Method::GET)
                .path("/v3/test-project/backups/detail");
            then.status(200).json_body(serde_json::json!({"backups": [
                {"id": "backup-1", "name": "snap-backup", "volume_id": "vol-1", "snapshot_id": "snap-1"}
            ]}));
        });

        let cleanup = ProjectCleanupBuilder::new(&client)
            .with_provider(BlockStorageCleanupProvider)
            .build();

        let eval: crate::cleanup::provider::EvaluationFn =
            std::sync::Arc::new(|r: &PlannedResource| {
                r.kind == SNAPSHOT && r.name.as_deref() == Some("data-snap")
            });

        let plan = cleanup
            .discover(HashMap::new(), Some(eval))
            .await
            .expect("discover failed");

        let snap = plan.nodes.iter().find(|n| n.id == "snap-1").unwrap();
        let backup = plan.nodes.iter().find(|n| n.id == "backup-1").unwrap();
        assert!(snap.selected);
        assert!(
            backup.selected,
            "backup must be pulled in by the cascade group"
        );

        server.mock(|when, then| {
            when.method(httpmock::Method::DELETE)
                .path("/v3/test-project/backups/backup-1");
            then.status(202);
        });
        server.mock(|when, then| {
            when.method(httpmock::Method::DELETE)
                .path("/v3/test-project/snapshots/snap-1");
            then.status(202);
        });

        let result = cleanup.apply(plan).await.expect("apply failed");
        assert!(
            result.errors.is_empty(),
            "unexpected errors: {:?}",
            result.errors
        );
        let backup_pos = result
            .deleted_ids
            .iter()
            .position(|id| id == "backup-1")
            .unwrap();
        let snap_pos = result
            .deleted_ids
            .iter()
            .position(|id| id == "snap-1")
            .unwrap();
        assert!(
            backup_pos < snap_pos,
            "backup must delete before its source snapshot"
        );
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
            node(
                VOLUME,
                "vol-2",
                serde_json::json!({"id": "vol-2", "attachments": []}),
            ),
        ];

        let rules = BlockStorageCleanupProvider.relations();
        let edges = evaluate_edges(&nodes, &rules);

        let vol1_idx = nodes.iter().position(|n| n.id == "vol-1").unwrap();
        let vol2_idx = nodes.iter().position(|n| n.id == "vol-2").unwrap();
        let server_idx = nodes.iter().position(|n| n.id == "server-1").unwrap();

        assert!(
            edges
                .iter()
                .any(|e| matches!(e.effect, RelationEffect::Blocks)
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
