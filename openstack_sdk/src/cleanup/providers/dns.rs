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

//! DNS (Designate) cleanup provider.
//!
//! No resource-level relations: a zone's own records go with it when the
//! zone is deleted, and this provider does not otherwise discover
//! individual recordsets. Runs before `network` (a floating IP can carry a
//! PTR record pointing at a zone; the python SDK proxy unsets those PTRs
//! as part of its own DNS cleanup, but this crate's floating IP resource
//! has no such reverse-DNS association today, so there is nothing to
//! order against on that side).
//!
//! `zone::list` is implicitly scoped to the authenticated project by
//! Designate itself (unlike Glance's image list or Neutron's network
//! list), so no explicit project filter is needed here.

use async_trait::async_trait;
use serde_json::Value;

use crate::api::dns::v2::zone;
use crate::api::{Pagination, QueryAsync, paged, raw};

use crate::cleanup::provider::{CleanupContext, CleanupDependency, CleanupError, CleanupProvider};
use crate::cleanup::relations::RelationRule;
use crate::cleanup::types::{PlannedResource, ResourceKind};

pub const ZONE: ResourceKind = ResourceKind::new("dns", "zone");

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
pub struct DnsCleanupProvider;

#[async_trait]
impl CleanupProvider for DnsCleanupProvider {
    fn service_type(&self) -> &'static str {
        "dns"
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

        let zones: Vec<Value> = paged(
            zone::list::Request::builder().build().map_err(|e| {
                CleanupError::Engine(format!("failed to build zone list request: {e}"))
            })?,
            Pagination::All,
        )
        .query_async(ctx.client)
        .await
        .map_err(|e| list_err(ZONE)(e.into()))?;
        for v in zones {
            nodes.push(to_planned(ZONE, v));
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

        if resource.kind == ZONE {
            let req = zone::delete::Request::builder()
                .id(resource.id.clone())
                .build()
                .map_err(|e| {
                    CleanupError::Engine(format!("failed to build zone delete request: {e}"))
                })?;
            raw(req)
                .query_async(ctx.client)
                .await
                .map_err(|e| err(e.into()))?;
        } else {
            return Err(CleanupError::Engine(format!(
                "DnsCleanupProvider cannot delete resource kind {:?}",
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
                    {"type": "dns", "name": "designate", "endpoints": [{"id": "dns-1",
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
    async fn discover_and_delete_a_zone() {
        let server = MockServer::start_async().await;
        let client = mock_client(&server).await;

        server.mock(|when, then| {
            when.method(httpmock::Method::GET).path("/v2/zones");
            then.status(200).json_body(serde_json::json!({"zones": [
                {"id": "zone-1", "name": "example.com."}
            ]}));
        });
        server.mock(|when, then| {
            when.method(httpmock::Method::DELETE)
                .path("/v2/zones/zone-1");
            then.status(202);
        });

        let cleanup = ProjectCleanupBuilder::new(&client)
            .with_provider(DnsCleanupProvider)
            .build();

        let eval: crate::cleanup::provider::EvaluationFn =
            std::sync::Arc::new(|r: &PlannedResource| r.kind == ZONE);
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
        assert!(result.deleted_ids.contains(&"zone-1".to_string()));
    }
}
