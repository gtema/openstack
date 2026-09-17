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

//! Network (Neutron) cleanup provider.
//!
//! Proves the resource-level `Blocks`/`CascadeGroup` primitives against
//! the case the python SDK hand-codes imperatively: a network cannot be
//! deleted while it still has subnets, router interfaces, or other ports
//! allocated to it. Deleting a network cascades to its subnets, its
//! router interfaces (detached, not deleted, via `remove_router_interface`),
//! and its other ports (deleted normally via the port-delete endpoint).
//!
//! A router is cleaned up independently of any network's cascade group: it
//! is only ever deleted if separately selected, and only once every
//! `ROUTER_INTERFACE` attached to it (via `device_id`) has itself been
//! detached/deleted (expressed by the `ROUTER_INTERFACE Blocks ROUTER`
//! rule below). Routers are deliberately NOT part of a network's cascade
//! group.
//!
//! Router interfaces are not exposed by the generated router list/get
//! response (there is no `interfaces_info` field on it in this codebase's
//! generated types — confirmed by grepping `sdk/network/src`). Instead,
//! following the python SDK proxy's own approach
//! (`network/v2/_proxy.py`'s router-interface helpers), router interfaces
//! are discovered by listing ports and classifying each one by its exact
//! `device_owner` value: `network:router_interface`,
//! `network:router_interface_distributed`, and
//! `network:ha_router_replicated_interface` become `ROUTER_INTERFACE`
//! resources; `network:dhcp` is skipped entirely (never a node, never
//! blocking — DHCP-agent-managed ports are not deletable this way); every
//! other port (unowned, or owned by another service such as `compute:nova`)
//! becomes a plain `PORT` resource that blocks and cascades with its
//! network.

use async_trait::async_trait;
use serde_json::Value;

use crate::api::network::v2::floatingip;
use crate::api::network::v2::network;
use crate::api::network::v2::port;
use crate::api::network::v2::router;
use crate::api::network::v2::router::remove_router_interface;
use crate::api::network::v2::security_group;
use crate::api::network::v2::security_group_rule;
use crate::api::network::v2::subnet;
use crate::api::{Pagination, QueryAsync, RestClient, paged, raw};

use crate::cleanup::provider::{CleanupContext, CleanupDependency, CleanupError, CleanupProvider};
use crate::cleanup::relations::{RelationEffect, RelationRule};
use crate::cleanup::types::{PlannedResource, ResourceKind};

pub const NETWORK: ResourceKind = ResourceKind::new("network", "network");
pub const SUBNET: ResourceKind = ResourceKind::new("network", "subnet");
pub const ROUTER: ResourceKind = ResourceKind::new("network", "router");
pub const ROUTER_INTERFACE: ResourceKind = ResourceKind::new("network", "router_interface");
pub const PORT: ResourceKind = ResourceKind::new("network", "port");
pub const FLOATINGIP: ResourceKind = ResourceKind::new("network", "floatingip");
pub const SECURITY_GROUP: ResourceKind = ResourceKind::new("network", "security_group");
pub const SECURITY_GROUP_RULE: ResourceKind = ResourceKind::new("network", "security_group_rule");

const ROUTER_INTERFACE_OWNERS: [&str; 3] = [
    "network:router_interface",
    "network:router_interface_distributed",
    "network:ha_router_replicated_interface",
];

fn is_router_interface_owner(owner: Option<&str>) -> bool {
    owner.is_some_and(|o| ROUTER_INTERFACE_OWNERS.contains(&o))
}

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

/// Extracts the first subnet id from a port's `fixed_ips` array, if any.
fn first_fixed_ip_subnet_id(port: &Value) -> Option<String> {
    port.get("fixed_ips")
        .and_then(|v| v.as_array())
        .and_then(|arr| arr.first())
        .and_then(|fip| fip.get("subnet_id"))
        .and_then(|v| v.as_str())
        .map(str::to_string)
}

#[derive(Debug, Default)]
pub struct NetworkCleanupProvider;

#[async_trait]
impl CleanupProvider for NetworkCleanupProvider {
    fn service_type(&self) -> &'static str {
        "network"
    }

    fn dependencies(&self) -> CleanupDependency {
        CleanupDependency {
            before: vec!["identity"],
            after: vec![],
        }
    }

    fn relations(&self) -> Vec<RelationRule> {
        vec![
            RelationRule {
                parent_kind: NETWORK,
                child_kind: SUBNET,
                matches: |child, parent| {
                    value_str(&child.raw, "network_id") == value_str(&parent.raw, "id")
                },
                effect: RelationEffect::Blocks,
            },
            RelationRule {
                parent_kind: NETWORK,
                child_kind: SUBNET,
                matches: |child, parent| {
                    value_str(&child.raw, "network_id") == value_str(&parent.raw, "id")
                },
                effect: RelationEffect::CascadeGroup,
            },
            RelationRule {
                parent_kind: NETWORK,
                child_kind: ROUTER_INTERFACE,
                matches: |child, parent| {
                    value_str(&child.raw, "network_id") == value_str(&parent.raw, "id")
                },
                effect: RelationEffect::Blocks,
            },
            RelationRule {
                parent_kind: NETWORK,
                child_kind: ROUTER_INTERFACE,
                matches: |child, parent| {
                    value_str(&child.raw, "network_id") == value_str(&parent.raw, "id")
                },
                effect: RelationEffect::CascadeGroup,
            },
            RelationRule {
                parent_kind: ROUTER,
                child_kind: ROUTER_INTERFACE,
                matches: |child, parent| {
                    value_str(&child.raw, "device_id") == value_str(&parent.raw, "id")
                },
                effect: RelationEffect::Blocks,
            },
            // Neutron refuses `remove_router_interface` for a subnet while
            // any floating IP is still bound to that subnet's router
            // (`RouterInterfaceInUseByFloatingIP`), even though the
            // floating IP has no direct reference to the interface port
            // itself - only to the router (`router_id`) the interface
            // belongs to (`device_id`).
            RelationRule {
                parent_kind: ROUTER_INTERFACE,
                child_kind: FLOATINGIP,
                matches: |child, parent| {
                    value_str(&child.raw, "router_id") == value_str(&parent.raw, "device_id")
                },
                effect: RelationEffect::Blocks,
            },
            // `NETWORK Blocks SUBNET` and `NETWORK Blocks ROUTER_INTERFACE`
            // above only order both against their shared network — they
            // don't order the subnet and the interface against each
            // other, so without this rule the engine can place them in the
            // same layer and delete them concurrently. On a real cloud the
            // router-interface port still holds an IP allocation on the
            // subnet until its `remove_router_interface` call completes,
            // so a concurrent subnet delete races it and fails with
            // `SubnetInUse`.
            RelationRule {
                parent_kind: SUBNET,
                child_kind: ROUTER_INTERFACE,
                matches: |child, parent| {
                    value_str(&child.raw, "subnet_id") == value_str(&parent.raw, "id")
                },
                effect: RelationEffect::Blocks,
            },
            RelationRule {
                parent_kind: NETWORK,
                child_kind: PORT,
                matches: |child, parent| {
                    value_str(&child.raw, "network_id") == value_str(&parent.raw, "id")
                },
                effect: RelationEffect::Blocks,
            },
            RelationRule {
                parent_kind: NETWORK,
                child_kind: PORT,
                matches: |child, parent| {
                    value_str(&child.raw, "network_id") == value_str(&parent.raw, "id")
                },
                effect: RelationEffect::CascadeGroup,
            },
            RelationRule {
                parent_kind: SECURITY_GROUP,
                child_kind: SECURITY_GROUP_RULE,
                matches: |child, parent| {
                    value_str(&child.raw, "security_group_id") == value_str(&parent.raw, "id")
                },
                effect: RelationEffect::Blocks,
            },
            RelationRule {
                parent_kind: SECURITY_GROUP,
                child_kind: SECURITY_GROUP_RULE,
                matches: |child, parent| {
                    value_str(&child.raw, "security_group_id") == value_str(&parent.raw, "id")
                },
                effect: RelationEffect::CascadeGroup,
            },
            RelationRule {
                parent_kind: SECURITY_GROUP,
                child_kind: PORT,
                matches: |child, parent| {
                    child
                        .raw
                        .get("security_groups")
                        .and_then(|v| v.as_array())
                        .is_some_and(|groups| {
                            groups
                                .iter()
                                .any(|g| g.as_str() == value_str(&parent.raw, "id"))
                        })
                },
                effect: RelationEffect::Blocks,
            },
        ]
    }

    async fn discover(
        &self,
        ctx: &CleanupContext<'_>,
    ) -> Result<Vec<PlannedResource>, CleanupError> {
        // Listing endpoints have no single resource id to attach to a
        // failure, so errors here are reported against an empty id; only
        // `delete()` (below) attaches a real resource id.
        let list_err = |kind: ResourceKind| {
            move |e: crate::OpenStackError| CleanupError::Provider {
                kind,
                id: String::new(),
                source: e,
            }
        };

        let mut nodes = Vec::new();

        // Unlike subnets, routers, ports, security groups, and floating
        // IPs (all project-scoped by Neutron's own default policy), an
        // unscoped network list also returns networks *shared* with this
        // project and external networks marked `shared` - neither of
        // which this project owns or may delete. Scope explicitly to
        // `tenant_id` (same approach `ImageCleanupProvider` uses for
        // `owner`) so a devstack/real-cloud external network never gets
        // planned for deletion.
        let project_id = ctx
            .client
            .get_current_project()
            .and_then(|p| p.id)
            .ok_or_else(|| CleanupError::Engine("failed to determine current project id".into()))?;
        let networks: Vec<Value> = paged(
            network::list::Request::builder()
                .tenant_id(project_id)
                .build()
                .map_err(|e| {
                    CleanupError::Engine(format!("failed to build network list request: {e}"))
                })?,
            Pagination::All,
        )
        .query_async(ctx.client)
        .await
        .map_err(|e| list_err(NETWORK)(e.into()))?;
        for v in networks {
            nodes.push(to_planned(NETWORK, v));
        }

        let subnets: Vec<Value> = paged(
            subnet::list::Request::builder().build().map_err(|e| {
                CleanupError::Engine(format!("failed to build subnet list request: {e}"))
            })?,
            Pagination::All,
        )
        .query_async(ctx.client)
        .await
        .map_err(|e| list_err(SUBNET)(e.into()))?;
        for v in subnets {
            nodes.push(to_planned(SUBNET, v));
        }

        let routers: Vec<Value> = paged(
            router::list::Request::builder().build().map_err(|e| {
                CleanupError::Engine(format!("failed to build router list request: {e}"))
            })?,
            Pagination::All,
        )
        .query_async(ctx.client)
        .await
        .map_err(|e| list_err(ROUTER)(e.into()))?;
        for router_v in &routers {
            nodes.push(to_planned(ROUTER, router_v.clone()));
        }

        // Router interfaces are not exposed on the router list/get
        // response in this codebase's generated types (no
        // `interfaces_info` field). Discover them the way the python
        // proxy does: list all ports, and classify each one by its exact
        // `device_owner` value into one of three cases:
        //   - a router-interface owner (see `ROUTER_INTERFACE_OWNERS`)
        //     becomes a `ROUTER_INTERFACE` resource, with `device_id`
        //     giving the owning router, to be detached (not deleted) via
        //     `remove_router_interface`;
        //   - `network:dhcp` is skipped entirely — not a node, never
        //     blocking, matching the python comment "we don't treat DHCP
        //     as a real port";
        //   - everything else (unowned, or owned by another service such
        //     as `compute:nova`) becomes a plain `PORT` resource that
        //     blocks and cascades with its network.
        let ports: Vec<Value> = paged(
            port::list::Request::builder().build().map_err(|e| {
                CleanupError::Engine(format!("failed to build port list request: {e}"))
            })?,
            Pagination::All,
        )
        .query_async(ctx.client)
        .await
        .map_err(|e| list_err(ROUTER_INTERFACE)(e.into()))?;
        for port_v in ports {
            let owner = value_str(&port_v, "device_owner");
            if is_router_interface_owner(owner) {
                let mut iface = port_v;
                let subnet_id = first_fixed_ip_subnet_id(&iface);
                if let Value::Object(map) = &mut iface {
                    // `id` is already the port id, which doubles as
                    // `port_id` for `remove_router_interface`.
                    if let Some(port_id) = map.get("id").cloned() {
                        map.insert("port_id".into(), port_id);
                    }
                    if !map.contains_key("subnet_id")
                        && let Some(subnet_id) = subnet_id
                    {
                        map.insert("subnet_id".into(), Value::String(subnet_id));
                    }
                }
                nodes.push(to_planned(ROUTER_INTERFACE, iface));
            } else if owner == Some("network:dhcp") {
                // DHCP-managed ports are not treated as real ports: never
                // blocking, never deleted directly (Neutron/the DHCP agent
                // manages their lifecycle itself).
                continue;
            } else {
                // Any other port (unowned, or owned by another service
                // such as compute) blocks and cascades with its network,
                // and gets deleted through the normal port-delete
                // endpoint.
                nodes.push(to_planned(PORT, port_v));
            }
        }

        let floatingips: Vec<Value> = paged(
            floatingip::list::Request::builder().build().map_err(|e| {
                CleanupError::Engine(format!("failed to build floating ip list request: {e}"))
            })?,
            Pagination::All,
        )
        .query_async(ctx.client)
        .await
        .map_err(|e| list_err(FLOATINGIP)(e.into()))?;
        for v in floatingips {
            // Floating IPs have no `name` attribute at all (unlike every
            // other resource kind this provider discovers) - `description`
            // is the only user-settable label, so it doubles as `name`
            // here. Without this, a name-prefix `evaluation_fn` (the
            // marker-matching pattern this crate's own functional test and
            // any caller filtering "my resources by name" would use) can
            // never select a floating IP.
            let mut node = to_planned(FLOATINGIP, v);
            if node.name.is_none() {
                node.name = value_str(&node.raw, "description").map(str::to_string);
            }
            nodes.push(node);
        }

        // Neutron auto-creates exactly one security group per project named
        // "default" and refuses to delete it (400 SecurityGroupCannotRemoveDefault)
        // — it must never become a cleanup candidate.
        let security_groups: Vec<Value> = paged(
            security_group::list::Request::builder()
                .build()
                .map_err(|e| {
                    CleanupError::Engine(format!(
                        "failed to build security group list request: {e}"
                    ))
                })?,
            Pagination::All,
        )
        .query_async(ctx.client)
        .await
        .map_err(|e| list_err(SECURITY_GROUP)(e.into()))?;
        let mut default_sg_ids: Vec<String> = Vec::new();
        for v in security_groups {
            if value_str(&v, "name") == Some("default") {
                if let Some(id) = value_str(&v, "id") {
                    default_sg_ids.push(id.to_string());
                }
                continue;
            }
            nodes.push(to_planned(SECURITY_GROUP, v));
        }

        let security_group_rules: Vec<Value> = paged(
            security_group_rule::list::Request::builder()
                .build()
                .map_err(|e| {
                    CleanupError::Engine(format!(
                        "failed to build security group rule list request: {e}"
                    ))
                })?,
            Pagination::All,
        )
        .query_async(ctx.client)
        .await
        .map_err(|e| list_err(SECURITY_GROUP_RULE)(e.into()))?;
        for v in security_group_rules {
            // A rule of the (undeletable, never-discovered) default security
            // group must not be a node either — deleting it would strip the
            // project's default egress/ingress policy while leaving the
            // protected default SG shell behind.
            if value_str(&v, "security_group_id")
                .is_some_and(|id| default_sg_ids.iter().any(|d| d == id))
            {
                continue;
            }
            nodes.push(to_planned(SECURITY_GROUP_RULE, v));
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

        if resource.kind == NETWORK {
            let req = network::delete::Request::builder()
                .id(resource.id.clone())
                .build()
                .map_err(|e| {
                    CleanupError::Engine(format!("failed to build network delete request: {e}"))
                })?;
            raw(req)
                .query_async(ctx.client)
                .await
                .map_err(|e| err(e.into()))?;
        } else if resource.kind == SUBNET {
            // `discover()` deliberately never turns a `network:dhcp` port
            // into a node (Neutron/the DHCP agent owns its lifecycle, and
            // most clouds refuse an explicit delete of one), so nothing
            // upstream ever removes it before this point. On real clouds
            // (unlike devstack, where the agent tends to already be gone by
            // the time cleanup runs) that lingering DHCP port still holds an
            // IP allocation on this subnet, and the delete below fails with
            // `SubnetInUse`. Disabling DHCP on the subnet makes Neutron
            // reclaim that port itself; poll for it to actually disappear
            // before deleting the subnet.
            if let Some(network_id) = value_str(&resource.raw, "network_id") {
                let update_req = subnet::set::Request::builder()
                    .id(resource.id.clone())
                    .subnet(
                        subnet::set::SubnetBuilder::default()
                            .enable_dhcp(false)
                            .build()
                            .map_err(|e| {
                                CleanupError::Engine(format!(
                                    "failed to build subnet update body: {e}"
                                ))
                            })?,
                    )
                    .build()
                    .map_err(|e| {
                        CleanupError::Engine(format!("failed to build subnet update request: {e}"))
                    })?;
                raw(update_req)
                    .query_async(ctx.client)
                    .await
                    .map_err(|e| err(e.into()))?;

                for _ in 0..30 {
                    let list_req = port::list::Request::builder()
                        .network_id(network_id.to_string())
                        .device_owner("network:dhcp")
                        .build()
                        .map_err(|e| {
                            CleanupError::Engine(format!("failed to build port list request: {e}"))
                        })?;
                    let dhcp_ports: Vec<Value> = paged(list_req, Pagination::All)
                        .query_async(ctx.client)
                        .await
                        .map_err(|e| err(e.into()))?;
                    let still_on_subnet = dhcp_ports
                        .iter()
                        .any(|p| first_fixed_ip_subnet_id(p).as_deref() == Some(&resource.id));
                    if !still_on_subnet {
                        break;
                    }
                    tokio::time::sleep(std::time::Duration::from_secs(2)).await;
                }
            }

            let req = subnet::delete::Request::builder()
                .id(resource.id.clone())
                .build()
                .map_err(|e| {
                    CleanupError::Engine(format!("failed to build subnet delete request: {e}"))
                })?;
            raw(req)
                .query_async(ctx.client)
                .await
                .map_err(|e| err(e.into()))?;
        } else if resource.kind == ROUTER {
            let req = router::delete::Request::builder()
                .id(resource.id.clone())
                .build()
                .map_err(|e| {
                    CleanupError::Engine(format!("failed to build router delete request: {e}"))
                })?;
            raw(req)
                .query_async(ctx.client)
                .await
                .map_err(|e| err(e.into()))?;
        } else if resource.kind == ROUTER_INTERFACE {
            let router_id = value_str(&resource.raw, "device_id")
                .unwrap_or_default()
                .to_string();
            let mut builder = remove_router_interface::Request::builder();
            builder.id(router_id);
            if let Some(subnet_id) = value_str(&resource.raw, "subnet_id") {
                builder.subnet_id(subnet_id.to_string());
            }
            if let Some(port_id) = value_str(&resource.raw, "port_id") {
                builder.port_id(port_id.to_string());
            }
            let req = builder.build().map_err(|e| {
                CleanupError::Engine(format!(
                    "failed to build remove_router_interface request: {e}"
                ))
            })?;
            raw(req)
                .query_async(ctx.client)
                .await
                .map_err(|e| err(e.into()))?;
        } else if resource.kind == PORT {
            let req = port::delete::Request::builder()
                .id(resource.id.clone())
                .build()
                .map_err(|e| {
                    CleanupError::Engine(format!("failed to build port delete request: {e}"))
                })?;
            raw(req)
                .query_async(ctx.client)
                .await
                .map_err(|e| err(e.into()))?;
        } else if resource.kind == FLOATINGIP {
            let req = floatingip::delete::Request::builder()
                .id(resource.id.clone())
                .build()
                .map_err(|e| {
                    CleanupError::Engine(format!("failed to build floating ip delete request: {e}"))
                })?;
            raw(req)
                .query_async(ctx.client)
                .await
                .map_err(|e| err(e.into()))?;
        } else if resource.kind == SECURITY_GROUP_RULE {
            let req = security_group_rule::delete::Request::builder()
                .id(resource.id.clone())
                .build()
                .map_err(|e| {
                    CleanupError::Engine(format!(
                        "failed to build security group rule delete request: {e}"
                    ))
                })?;
            raw(req)
                .query_async(ctx.client)
                .await
                .map_err(|e| err(e.into()))?;
        } else if resource.kind == SECURITY_GROUP {
            let req = security_group::delete::Request::builder()
                .id(resource.id.clone())
                .build()
                .map_err(|e| {
                    CleanupError::Engine(format!(
                        "failed to build security group delete request: {e}"
                    ))
                })?;
            raw(req)
                .query_async(ctx.client)
                .await
                .map_err(|e| err(e.into()))?;
        } else {
            return Err(CleanupError::Engine(format!(
                "NetworkCleanupProvider cannot delete resource kind {:?}",
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
    async fn discover_marks_network_with_subnet_as_blocked_until_subnet_selected() {
        let server = MockServer::start_async().await;
        let client = mock_client(&server).await;

        server.mock(|when, then| {
            when.method(httpmock::Method::GET).path("/v2.0/networks");
            then.status(200).json_body(serde_json::json!({"networks": [
                {"id": "net-1", "name": "private"}
            ]}));
        });
        server.mock(|when, then| {
            when.method(httpmock::Method::GET).path("/v2.0/subnets");
            then.status(200).json_body(serde_json::json!({"subnets": [
                {"id": "subnet-1", "name": "private-subnet", "network_id": "net-1"}
            ]}));
        });
        server.mock(|when, then| {
            when.method(httpmock::Method::GET).path("/v2.0/routers");
            then.status(200)
                .json_body(serde_json::json!({"routers": []}));
        });
        server.mock(|when, then| {
            when.method(httpmock::Method::GET).path("/v2.0/ports");
            then.status(200).json_body(serde_json::json!({"ports": []}));
        });
        server.mock(|when, then| {
            when.method(httpmock::Method::GET).path("/v2.0/floatingips");
            then.status(200)
                .json_body(serde_json::json!({"floatingips": []}));
        });
        server.mock(|when, then| {
            when.method(httpmock::Method::GET)
                .path("/v2.0/security-groups");
            then.status(200)
                .json_body(serde_json::json!({"security_groups": []}));
        });
        server.mock(|when, then| {
            when.method(httpmock::Method::GET)
                .path("/v2.0/security-group-rules");
            then.status(200)
                .json_body(serde_json::json!({"security_group_rules": []}));
        });

        let cleanup = ProjectCleanupBuilder::new(&client)
            .with_provider(NetworkCleanupProvider)
            .build();

        // Evaluation function selects only the network by name; the
        // subnet must still be pulled in via CascadeGroup, and net-1 must
        // remain deletable (its only blocking child, subnet-1, is also
        // selected via cascade).
        let eval: std::sync::Arc<dyn Fn(&PlannedResource) -> bool + Send + Sync> =
            std::sync::Arc::new(|r: &PlannedResource| {
                r.kind == NETWORK && r.name.as_deref() == Some("private")
            });

        let plan = cleanup
            .discover(HashMap::new(), Some(eval))
            .await
            .expect("discover failed");

        let net = plan.nodes.iter().find(|n| n.id == "net-1").unwrap();
        let subnet = plan.nodes.iter().find(|n| n.id == "subnet-1").unwrap();
        assert!(net.selected);
        assert!(
            subnet.selected,
            "subnet must be pulled in by the cascade group"
        );

        server.mock(|when, then| {
            when.method(httpmock::Method::DELETE)
                .path("/v2.0/subnets/subnet-1");
            then.status(204);
        });
        server.mock(|when, then| {
            when.method(httpmock::Method::DELETE)
                .path("/v2.0/networks/net-1");
            then.status(204);
        });

        let result = cleanup.apply(plan).await.expect("apply failed");
        assert!(
            result.errors.is_empty(),
            "unexpected errors: {:?}",
            result.errors
        );
        assert!(result.deleted_ids.contains(&"subnet-1".to_string()));
        assert!(result.deleted_ids.contains(&"net-1".to_string()));
        let subnet_pos = result
            .deleted_ids
            .iter()
            .position(|id| id == "subnet-1")
            .unwrap();
        let net_pos = result
            .deleted_ids
            .iter()
            .position(|id| id == "net-1")
            .unwrap();
        assert!(
            subnet_pos < net_pos,
            "subnet must delete before its network"
        );
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn discover_marks_router_interface_blocked_until_floatingip_selected() {
        let server = MockServer::start_async().await;
        let client = mock_client(&server).await;

        server.mock(|when, then| {
            when.method(httpmock::Method::GET).path("/v2.0/networks");
            then.status(200).json_body(serde_json::json!({"networks": [
                {"id": "net-1", "name": "private"}
            ]}));
        });
        server.mock(|when, then| {
            when.method(httpmock::Method::GET).path("/v2.0/subnets");
            then.status(200)
                .json_body(serde_json::json!({"subnets": []}));
        });
        server.mock(|when, then| {
            when.method(httpmock::Method::GET).path("/v2.0/routers");
            then.status(200).json_body(serde_json::json!({"routers": [
                {"id": "router-1", "name": "private-router"}
            ]}));
        });
        server.mock(|when, then| {
            when.method(httpmock::Method::GET).path("/v2.0/ports");
            then.status(200).json_body(serde_json::json!({"ports": [
                {"id": "iface-1", "network_id": "net-1",
                 "device_owner": "network:router_interface",
                 "device_id": "router-1",
                 "fixed_ips": [{"subnet_id": "subnet-1"}]}
            ]}));
        });
        server.mock(|when, then| {
            when.method(httpmock::Method::GET).path("/v2.0/floatingips");
            then.status(200)
                .json_body(serde_json::json!({"floatingips": [
                    {"id": "fip-1", "floating_ip_address": "203.0.113.5",
                     "router_id": "router-1"}
                ]}));
        });
        server.mock(|when, then| {
            when.method(httpmock::Method::GET)
                .path("/v2.0/security-groups");
            then.status(200)
                .json_body(serde_json::json!({"security_groups": []}));
        });
        server.mock(|when, then| {
            when.method(httpmock::Method::GET)
                .path("/v2.0/security-group-rules");
            then.status(200)
                .json_body(serde_json::json!({"security_group_rules": []}));
        });

        let cleanup = ProjectCleanupBuilder::new(&client)
            .with_provider(NetworkCleanupProvider)
            .build();

        // There is no `CascadeGroup` for this pair (unlike e.g.
        // `NETWORK`/`ROUTER_INTERFACE`) - a real caller selects the
        // floating IP itself by its own marker, same as the full
        // functional scenario does. Select all three directly here to
        // isolate the ordering assertion below from cascade behavior.
        let eval: std::sync::Arc<dyn Fn(&PlannedResource) -> bool + Send + Sync> =
            std::sync::Arc::new(|r: &PlannedResource| {
                r.kind == ROUTER_INTERFACE || r.kind == ROUTER || r.kind == FLOATINGIP
            });

        let plan = cleanup
            .discover(HashMap::new(), Some(eval))
            .await
            .expect("discover failed");

        let fip = plan.nodes.iter().find(|n| n.id == "fip-1").unwrap();
        let iface = plan.nodes.iter().find(|n| n.id == "iface-1").unwrap();
        assert!(fip.selected);
        assert!(iface.selected);

        server.mock(|when, then| {
            when.method(httpmock::Method::DELETE)
                .path("/v2.0/floatingips/fip-1");
            then.status(204);
        });
        server.mock(|when, then| {
            when.method(httpmock::Method::PUT)
                .path("/v2.0/routers/router-1/remove_router_interface");
            then.status(200).json_body(serde_json::json!({}));
        });
        server.mock(|when, then| {
            when.method(httpmock::Method::DELETE)
                .path("/v2.0/routers/router-1");
            then.status(204);
        });

        let result = cleanup.apply(plan).await.expect("apply failed");
        assert!(
            result.errors.is_empty(),
            "unexpected errors: {:?}",
            result.errors
        );
        let pos = |id: &str| {
            result
                .deleted_ids
                .iter()
                .position(|d| d == id)
                .unwrap_or_else(|| panic!("{id} missing from deleted_ids"))
        };
        assert!(
            pos("fip-1") < pos("iface-1"),
            "floating ip must delete before the router interface it holds"
        );
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn discover_classifies_ports_correctly() {
        let server = MockServer::start_async().await;
        let client = mock_client(&server).await;

        server.mock(|when, then| {
            when.method(httpmock::Method::GET).path("/v2.0/networks");
            then.status(200).json_body(serde_json::json!({"networks": [
                {"id": "net-1", "name": "private"}
            ]}));
        });
        server.mock(|when, then| {
            when.method(httpmock::Method::GET).path("/v2.0/subnets");
            then.status(200)
                .json_body(serde_json::json!({"subnets": []}));
        });
        server.mock(|when, then| {
            when.method(httpmock::Method::GET).path("/v2.0/routers");
            then.status(200)
                .json_body(serde_json::json!({"routers": []}));
        });
        server.mock(|when, then| {
            when.method(httpmock::Method::GET).path("/v2.0/ports");
            then.status(200).json_body(serde_json::json!({"ports": [
                {"id": "port-dhcp", "network_id": "net-1", "device_owner": "network:dhcp"},
                {"id": "port-ha-router", "network_id": "net-1",
                 "device_owner": "network:ha_router_replicated_interface",
                 "device_id": "router-1",
                 "fixed_ips": [{"subnet_id": "subnet-1"}]},
                {"id": "port-tenant", "network_id": "net-1", "device_owner": "compute:nova"}
            ]}));
        });
        server.mock(|when, then| {
            when.method(httpmock::Method::GET).path("/v2.0/floatingips");
            then.status(200)
                .json_body(serde_json::json!({"floatingips": []}));
        });
        server.mock(|when, then| {
            when.method(httpmock::Method::GET)
                .path("/v2.0/security-groups");
            then.status(200)
                .json_body(serde_json::json!({"security_groups": []}));
        });
        server.mock(|when, then| {
            when.method(httpmock::Method::GET)
                .path("/v2.0/security-group-rules");
            then.status(200)
                .json_body(serde_json::json!({"security_group_rules": []}));
        });

        let cleanup = ProjectCleanupBuilder::new(&client)
            .with_provider(NetworkCleanupProvider)
            .build();

        let plan = cleanup
            .discover(HashMap::new(), None)
            .await
            .expect("discover failed");

        assert!(
            plan.nodes.iter().find(|n| n.id == "port-dhcp").is_none(),
            "dhcp port must not be a node"
        );

        let ha_router_node = plan
            .nodes
            .iter()
            .find(|n| n.id == "port-ha-router")
            .expect("ha-router port must be a node");
        assert_eq!(
            ha_router_node.kind, ROUTER_INTERFACE,
            "ha-router-replicated-interface port must classify as ROUTER_INTERFACE"
        );

        let tenant_node = plan
            .nodes
            .iter()
            .find(|n| n.id == "port-tenant")
            .expect("tenant port must be a node");
        assert_eq!(tenant_node.kind, PORT);

        let net_idx = plan
            .nodes
            .iter()
            .position(|n| n.id == "net-1")
            .expect("network node must exist");
        let port_idx = plan
            .nodes
            .iter()
            .position(|n| n.id == "port-tenant")
            .expect("tenant port node must exist");

        assert!(
            plan.edges
                .iter()
                .any(|e| matches!(e.effect, RelationEffect::Blocks)
                    && e.child == port_idx
                    && e.parent == net_idx),
            "expected a Blocks edge from port-tenant to net-1"
        );
        assert!(
            plan.edges
                .iter()
                .any(|e| matches!(e.effect, RelationEffect::CascadeGroup)
                    && e.child == port_idx
                    && e.parent == net_idx),
            "expected a CascadeGroup edge from port-tenant to net-1"
        );
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn discover_maps_floating_ips_to_planned_resources() {
        let server = MockServer::start_async().await;
        let client = mock_client(&server).await;

        server.mock(|when, then| {
            when.method(httpmock::Method::GET).path("/v2.0/networks");
            then.status(200)
                .json_body(serde_json::json!({"networks": []}));
        });
        server.mock(|when, then| {
            when.method(httpmock::Method::GET).path("/v2.0/subnets");
            then.status(200)
                .json_body(serde_json::json!({"subnets": []}));
        });
        server.mock(|when, then| {
            when.method(httpmock::Method::GET).path("/v2.0/routers");
            then.status(200)
                .json_body(serde_json::json!({"routers": []}));
        });
        server.mock(|when, then| {
            when.method(httpmock::Method::GET).path("/v2.0/ports");
            then.status(200).json_body(serde_json::json!({"ports": []}));
        });
        server.mock(|when, then| {
            when.method(httpmock::Method::GET).path("/v2.0/floatingips");
            then.status(200)
                .json_body(serde_json::json!({"floatingips": [
                    {"id": "fip-1", "floating_ip_address": "203.0.113.5", "name": null}
                ]}));
        });
        server.mock(|when, then| {
            when.method(httpmock::Method::GET)
                .path("/v2.0/security-groups");
            then.status(200)
                .json_body(serde_json::json!({"security_groups": []}));
        });
        server.mock(|when, then| {
            when.method(httpmock::Method::GET)
                .path("/v2.0/security-group-rules");
            then.status(200)
                .json_body(serde_json::json!({"security_group_rules": []}));
        });

        let cleanup = ProjectCleanupBuilder::new(&client)
            .with_provider(NetworkCleanupProvider)
            .build();

        let plan = cleanup
            .discover(HashMap::new(), None)
            .await
            .expect("discover failed");

        let node = plan.nodes.iter().find(|n| n.id == "fip-1").unwrap();
        assert_eq!(node.kind, FLOATINGIP);
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn delete_issues_floatingip_delete_request() {
        let server = MockServer::start_async().await;
        let client = mock_client(&server).await;

        server.mock(|when, then| {
            when.method(httpmock::Method::GET).path("/v2.0/networks");
            then.status(200)
                .json_body(serde_json::json!({"networks": []}));
        });
        server.mock(|when, then| {
            when.method(httpmock::Method::GET).path("/v2.0/subnets");
            then.status(200)
                .json_body(serde_json::json!({"subnets": []}));
        });
        server.mock(|when, then| {
            when.method(httpmock::Method::GET).path("/v2.0/routers");
            then.status(200)
                .json_body(serde_json::json!({"routers": []}));
        });
        server.mock(|when, then| {
            when.method(httpmock::Method::GET).path("/v2.0/ports");
            then.status(200).json_body(serde_json::json!({"ports": []}));
        });
        server.mock(|when, then| {
            when.method(httpmock::Method::GET).path("/v2.0/floatingips");
            then.status(200)
                .json_body(serde_json::json!({"floatingips": [
                    {"id": "fip-1", "floating_ip_address": "203.0.113.5", "name": null}
                ]}));
        });
        server.mock(|when, then| {
            when.method(httpmock::Method::GET)
                .path("/v2.0/security-groups");
            then.status(200)
                .json_body(serde_json::json!({"security_groups": []}));
        });
        server.mock(|when, then| {
            when.method(httpmock::Method::GET)
                .path("/v2.0/security-group-rules");
            then.status(200)
                .json_body(serde_json::json!({"security_group_rules": []}));
        });
        server.mock(|when, then| {
            when.method(httpmock::Method::DELETE)
                .path("/v2.0/floatingips/fip-1");
            then.status(204);
        });

        let cleanup = ProjectCleanupBuilder::new(&client)
            .with_provider(NetworkCleanupProvider)
            .build();

        let eval: crate::cleanup::provider::EvaluationFn =
            std::sync::Arc::new(|r: &PlannedResource| r.kind == FLOATINGIP);
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
        assert!(result.deleted_ids.contains(&"fip-1".to_string()));
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn discover_excludes_default_security_group_and_cascades_rules() {
        let server = MockServer::start_async().await;
        let client = mock_client(&server).await;

        server.mock(|when, then| {
            when.method(httpmock::Method::GET).path("/v2.0/networks");
            then.status(200)
                .json_body(serde_json::json!({"networks": []}));
        });
        server.mock(|when, then| {
            when.method(httpmock::Method::GET).path("/v2.0/subnets");
            then.status(200)
                .json_body(serde_json::json!({"subnets": []}));
        });
        server.mock(|when, then| {
            when.method(httpmock::Method::GET).path("/v2.0/routers");
            then.status(200)
                .json_body(serde_json::json!({"routers": []}));
        });
        server.mock(|when, then| {
            when.method(httpmock::Method::GET).path("/v2.0/ports");
            then.status(200).json_body(serde_json::json!({"ports": []}));
        });
        server.mock(|when, then| {
            when.method(httpmock::Method::GET).path("/v2.0/floatingips");
            then.status(200)
                .json_body(serde_json::json!({"floatingips": []}));
        });
        server.mock(|when, then| {
            when.method(httpmock::Method::GET)
                .path("/v2.0/security-groups");
            then.status(200)
                .json_body(serde_json::json!({"security_groups": [
                    {"id": "sg-default", "name": "default"},
                    {"id": "sg-custom", "name": "web"}
                ]}));
        });
        server.mock(|when, then| {
            when.method(httpmock::Method::GET)
                .path("/v2.0/security-group-rules");
            then.status(200)
                .json_body(serde_json::json!({"security_group_rules": [
                    {"id": "rule-1", "security_group_id": "sg-custom"}
                ]}));
        });

        let cleanup = ProjectCleanupBuilder::new(&client)
            .with_provider(NetworkCleanupProvider)
            .build();

        let eval: crate::cleanup::provider::EvaluationFn =
            std::sync::Arc::new(|r: &PlannedResource| {
                r.kind == SECURITY_GROUP && r.name.as_deref() == Some("web")
            });

        let plan = cleanup
            .discover(HashMap::new(), Some(eval))
            .await
            .expect("discover failed");

        assert!(
            plan.nodes.iter().all(|n| n.id != "sg-default"),
            "default security group must never be a node"
        );
        let sg = plan.nodes.iter().find(|n| n.id == "sg-custom").unwrap();
        let rule = plan.nodes.iter().find(|n| n.id == "rule-1").unwrap();
        assert!(sg.selected);
        assert!(rule.selected, "rule must be pulled in by the cascade group");
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn discover_excludes_default_security_group_rules_with_no_eval_fn() {
        let server = MockServer::start_async().await;
        let client = mock_client(&server).await;

        server.mock(|when, then| {
            when.method(httpmock::Method::GET).path("/v2.0/networks");
            then.status(200)
                .json_body(serde_json::json!({"networks": []}));
        });
        server.mock(|when, then| {
            when.method(httpmock::Method::GET).path("/v2.0/subnets");
            then.status(200)
                .json_body(serde_json::json!({"subnets": []}));
        });
        server.mock(|when, then| {
            when.method(httpmock::Method::GET).path("/v2.0/routers");
            then.status(200)
                .json_body(serde_json::json!({"routers": []}));
        });
        server.mock(|when, then| {
            when.method(httpmock::Method::GET).path("/v2.0/ports");
            then.status(200).json_body(serde_json::json!({"ports": []}));
        });
        server.mock(|when, then| {
            when.method(httpmock::Method::GET).path("/v2.0/floatingips");
            then.status(200)
                .json_body(serde_json::json!({"floatingips": []}));
        });
        server.mock(|when, then| {
            when.method(httpmock::Method::GET)
                .path("/v2.0/security-groups");
            then.status(200)
                .json_body(serde_json::json!({"security_groups": [
                    {"id": "sg-default", "name": "default"}
                ]}));
        });
        server.mock(|when, then| {
            when.method(httpmock::Method::GET)
                .path("/v2.0/security-group-rules");
            then.status(200)
                .json_body(serde_json::json!({"security_group_rules": [
                    {"id": "rule-default", "security_group_id": "sg-default"}
                ]}));
        });

        let cleanup = ProjectCleanupBuilder::new(&client)
            .with_provider(NetworkCleanupProvider)
            .build();

        let plan = cleanup
            .discover(HashMap::new(), None)
            .await
            .expect("discover failed");

        assert!(
            plan.nodes.iter().all(|n| n.id != "rule-default"),
            "default security group's rule must never become a node"
        );
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn delete_issues_security_group_and_rule_delete_requests() {
        let server = MockServer::start_async().await;
        let client = mock_client(&server).await;

        server.mock(|when, then| {
            when.method(httpmock::Method::GET).path("/v2.0/networks");
            then.status(200)
                .json_body(serde_json::json!({"networks": []}));
        });
        server.mock(|when, then| {
            when.method(httpmock::Method::GET).path("/v2.0/subnets");
            then.status(200)
                .json_body(serde_json::json!({"subnets": []}));
        });
        server.mock(|when, then| {
            when.method(httpmock::Method::GET).path("/v2.0/routers");
            then.status(200)
                .json_body(serde_json::json!({"routers": []}));
        });
        server.mock(|when, then| {
            when.method(httpmock::Method::GET).path("/v2.0/ports");
            then.status(200).json_body(serde_json::json!({"ports": []}));
        });
        server.mock(|when, then| {
            when.method(httpmock::Method::GET).path("/v2.0/floatingips");
            then.status(200)
                .json_body(serde_json::json!({"floatingips": []}));
        });
        server.mock(|when, then| {
            when.method(httpmock::Method::GET)
                .path("/v2.0/security-groups");
            then.status(200)
                .json_body(serde_json::json!({"security_groups": [
                    {"id": "sg-custom", "name": "web"}
                ]}));
        });
        server.mock(|when, then| {
            when.method(httpmock::Method::GET)
                .path("/v2.0/security-group-rules");
            then.status(200)
                .json_body(serde_json::json!({"security_group_rules": [
                    {"id": "rule-1", "security_group_id": "sg-custom"}
                ]}));
        });
        server.mock(|when, then| {
            when.method(httpmock::Method::DELETE)
                .path("/v2.0/security-group-rules/rule-1");
            then.status(204);
        });
        server.mock(|when, then| {
            when.method(httpmock::Method::DELETE)
                .path("/v2.0/security-groups/sg-custom");
            then.status(204);
        });

        let cleanup = ProjectCleanupBuilder::new(&client)
            .with_provider(NetworkCleanupProvider)
            .build();

        let eval: crate::cleanup::provider::EvaluationFn =
            std::sync::Arc::new(|r: &PlannedResource| r.kind == SECURITY_GROUP);
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
        let rule_pos = result
            .deleted_ids
            .iter()
            .position(|id| id == "rule-1")
            .unwrap();
        let sg_pos = result
            .deleted_ids
            .iter()
            .position(|id| id == "sg-custom")
            .unwrap();
        assert!(
            rule_pos < sg_pos,
            "rule must delete before its security group"
        );
    }
}
