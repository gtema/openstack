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
//!
//! `discover()` scopes the image list to images owned by the current
//! project (`owner=<project_id>`). An unscoped `image::list` returns
//! every image visible to the caller — including public and
//! project-shared images owned by other projects — which would make
//! cleanup attempt to delete images this project doesn't own.

use async_trait::async_trait;
use serde_json::Value;

use crate::api::image::v2::image;
use crate::api::{Pagination, QueryAsync, RestClient, paged, raw};

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

        let project_id = ctx
            .client
            .get_current_project()
            .and_then(|p| p.id)
            .ok_or_else(|| CleanupError::Engine("failed to determine current project id".into()))?;
        let images: Vec<Value> = paged(
            image::list::Request::builder()
                .owner(project_id)
                .build()
                .map_err(|e| {
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

        // Asserts the discover request scopes the list to images owned by
        // the current project ("test-project", per `mock_client`'s token
        // response) — an image owned by another project (e.g. a public
        // image) must never be returned, let alone selected for deletion.
        server.mock(|when, then| {
            when.method(httpmock::Method::GET)
                .path("/v2/images")
                .query_param("owner", "test-project");
            then.status(200).json_body(serde_json::json!({"images": [
                {"id": "img-1", "name": "test-image", "owner": "test-project"}
            ]}));
        });
        server.mock(|when, then| {
            when.method(httpmock::Method::DELETE)
                .path("/v2/images/img-1");
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

        assert!(
            result.errors.is_empty(),
            "unexpected errors: {:?}",
            result.errors
        );
        assert!(result.deleted_ids.contains(&"img-1".to_string()));
    }
}
