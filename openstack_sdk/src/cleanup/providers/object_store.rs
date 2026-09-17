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

//! Object Store (Swift) cleanup provider.
//!
//! Unlike every other resource this crate cleans up, Swift resources are
//! not addressed by an opaque `id` — a container and an object are both
//! keyed by their own `name`, and the "account" (`AUTH_<project-id>`) is a
//! URL path segment baked into the service catalog endpoint itself rather
//! than a value returned by any list/get call. `account_name()` below
//! recovers it the same way the generated CLI does (see
//! `cli/object-store/src/v1/container/list.rs`): resolve the Object Store
//! endpoint and take the last non-empty path segment.
//!
//! `discover()` injects a synthetic `container` field onto every object it
//! lists (the container list response carries only the object's own name),
//! so the `CONTAINER Blocks OBJECT` relation below has something to match
//! against - a container cannot be deleted by Swift while it still holds
//! objects.

use async_trait::async_trait;
use serde_json::Value;

use crate::api::object_store::v1::{account, container, object};
use crate::api::{AsyncClient, Pagination, QueryAsync, paged, raw};
use crate::types::{ApiVersion, ServiceType};

use crate::cleanup::provider::{CleanupContext, CleanupDependency, CleanupError, CleanupProvider};
use crate::cleanup::relations::{RelationEffect, RelationRule};
use crate::cleanup::types::{PlannedResource, ResourceKind};

pub const CONTAINER: ResourceKind = ResourceKind::new("object_store", "container");
pub const OBJECT: ResourceKind = ResourceKind::new("object_store", "object");

fn value_str<'a>(v: &'a Value, key: &str) -> Option<&'a str> {
    v.get(key).and_then(|x| x.as_str())
}

fn to_planned(kind: ResourceKind, v: Value) -> PlannedResource {
    // Swift resources have no `id` at all - `name` is their only identity.
    let id = value_str(&v, "name").unwrap_or_default().to_string();
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

async fn account_name(ctx: &CleanupContext<'_>) -> Result<String, CleanupError> {
    let ep = ctx
        .client
        .get_service_endpoint(&ServiceType::ObjectStore, Some(&ApiVersion::new(1, 0)))
        .await
        .map_err(|e| {
            CleanupError::Engine(format!("failed to resolve object-store endpoint: {e}"))
        })?;
    ep.url()
        .path_segments()
        .and_then(|mut segments| segments.rfind(|s| !s.is_empty()))
        .map(str::to_string)
        .ok_or_else(|| {
            CleanupError::Engine("object-store endpoint has no account path segment".to_string())
        })
}

#[derive(Debug, Default)]
pub struct ObjectStoreCleanupProvider;

#[async_trait]
impl CleanupProvider for ObjectStoreCleanupProvider {
    fn service_type(&self) -> &'static str {
        "object_store"
    }

    fn dependencies(&self) -> CleanupDependency {
        CleanupDependency::default()
    }

    fn relations(&self) -> Vec<RelationRule> {
        vec![RelationRule {
            parent_kind: CONTAINER,
            child_kind: OBJECT,
            matches: |child, parent| {
                value_str(&child.raw, "container") == value_str(&parent.raw, "name")
            },
            effect: RelationEffect::Blocks,
        }]
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
        let account = account_name(ctx).await?;

        let containers: Vec<Value> = paged(
            account::get::Request::builder()
                .account(account.clone())
                .build()
                .map_err(|e| {
                    CleanupError::Engine(format!("failed to build container list request: {e}"))
                })?,
            Pagination::All,
        )
        .query_async(ctx.client)
        .await
        .map_err(|e| list_err(CONTAINER)(e.into()))?;

        for container_v in &containers {
            let Some(container_name) = value_str(container_v, "name") else {
                continue;
            };

            let objects: Vec<Value> = paged(
                container::get::Request::builder()
                    .account(account.clone())
                    .container(container_name.to_string())
                    .build()
                    .map_err(|e| {
                        CleanupError::Engine(format!("failed to build object list request: {e}"))
                    })?,
                Pagination::All,
            )
            .query_async(ctx.client)
            .await
            .map_err(|e| list_err(OBJECT)(e.into()))?;

            for mut object_v in objects {
                if let Value::Object(map) = &mut object_v {
                    map.insert(
                        "container".into(),
                        Value::String(container_name.to_string()),
                    );
                }
                nodes.push(to_planned(OBJECT, object_v));
            }
        }

        for container_v in containers {
            nodes.push(to_planned(CONTAINER, container_v));
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
        let account = account_name(ctx).await?;

        if resource.kind == OBJECT {
            let container_name = value_str(&resource.raw, "container").ok_or_else(|| {
                CleanupError::Engine(format!(
                    "object {} is missing its container name",
                    resource.id
                ))
            })?;
            let req = object::delete::Request::builder()
                .account(account)
                .container(container_name.to_string())
                .object(resource.id.clone())
                .build()
                .map_err(|e| {
                    CleanupError::Engine(format!("failed to build object delete request: {e}"))
                })?;
            raw(req)
                .query_async(ctx.client)
                .await
                .map_err(|e| err(e.into()))?;
        } else if resource.kind == CONTAINER {
            let req = container::delete::Request::builder()
                .account(account)
                .container(resource.id.clone())
                .build()
                .map_err(|e| {
                    CleanupError::Engine(format!("failed to build container delete request: {e}"))
                })?;
            raw(req)
                .query_async(ctx.client)
                .await
                .map_err(|e| err(e.into()))?;
        } else {
            return Err(CleanupError::Engine(format!(
                "ObjectStoreCleanupProvider cannot delete resource kind {:?}",
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
                    {"type": "object-store", "name": "swift", "endpoints": [{"id": "swift-1",
                        "url": format!("{base_url}/v1/AUTH_test-project"), "region": "RegionOne", "interface": "public"}]}
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
    async fn discover_and_delete_container_and_object_in_order() {
        let server = MockServer::start_async().await;
        let client = mock_client(&server).await;

        // Swift's `account`/`container` listing uses marker-based
        // pagination with no `use_keyset_pagination` support (see
        // `Pageable` on `account::get`/`container::get`): after a
        // non-empty page, the client always requests a next page keyed off
        // the last item's `name`, so each listing needs an explicit
        // "second page is empty" mock or the paginator loops forever
        // re-fetching the same single-item page from an unconstrained mock.
        server.mock(|when, then| {
            when.method(httpmock::Method::GET)
                .path("/v1/AUTH_test-project")
                .query_param_missing("marker");
            then.status(200)
                .json_body(serde_json::json!([{"name": "bucket-1"}]));
        });
        server.mock(|when, then| {
            when.method(httpmock::Method::GET)
                .path("/v1/AUTH_test-project")
                .query_param_exists("marker");
            then.status(200).json_body(serde_json::json!([]));
        });
        server.mock(|when, then| {
            when.method(httpmock::Method::GET)
                .path("/v1/AUTH_test-project/bucket-1")
                .query_param_missing("marker");
            then.status(200)
                .json_body(serde_json::json!([{"name": "obj-1"}]));
        });
        server.mock(|when, then| {
            when.method(httpmock::Method::GET)
                .path("/v1/AUTH_test-project/bucket-1")
                .query_param_exists("marker");
            then.status(200).json_body(serde_json::json!([]));
        });

        let cleanup = ProjectCleanupBuilder::new(&client)
            .with_provider(ObjectStoreCleanupProvider)
            .build();

        let plan = cleanup
            .discover(HashMap::new(), None)
            .await
            .expect("discover failed");

        let obj = plan.nodes.iter().find(|n| n.id == "obj-1").unwrap();
        let container = plan.nodes.iter().find(|n| n.id == "bucket-1").unwrap();
        assert!(obj.selected);
        assert!(container.selected);

        server.mock(|when, then| {
            when.method(httpmock::Method::DELETE)
                .path("/v1/AUTH_test-project/bucket-1/obj-1");
            then.status(204);
        });
        server.mock(|when, then| {
            when.method(httpmock::Method::DELETE)
                .path("/v1/AUTH_test-project/bucket-1");
            then.status(204);
        });

        let result = cleanup.apply(plan).await.expect("apply failed");
        assert!(
            result.errors.is_empty(),
            "unexpected errors: {:?}",
            result.errors
        );

        let obj_pos = result
            .deleted_ids
            .iter()
            .position(|d| d == "obj-1")
            .unwrap();
        let container_pos = result
            .deleted_ids
            .iter()
            .position(|d| d == "bucket-1")
            .unwrap();
        assert!(
            obj_pos < container_pos,
            "object must delete before its container"
        );
    }
}
