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
//
// WARNING: This file is automatically generated from OpenAPI schema using
// `openstack-codegenerator`.

//! List Networks command
//!
//! Wraps invoking of the `v2.0/networks` with `GET` method

use clap::Args;
use serde::{Deserialize, Serialize};
use tracing::info;

use openstack_sdk::AsyncOpenStack;

use crate::Cli;
use crate::OpenStackCliError;
use crate::OutputConfig;
use crate::StructTable;
use crate::output::OutputProcessor;

use openstack_sdk::api::QueryAsync;
use openstack_sdk::api::network::v2::network::list;
use openstack_sdk::api::{Pagination, paged};
use openstack_sdk::types::BoolString;
use openstack_sdk::types::IntString;
use serde_json::Value;
use structable_derive::StructTable;

/// Lists networks to which the project has access.
///
/// Default policy settings return only networks that the project who submits
/// the request owns, unless an administrative user submits the request. In
/// addition, networks shared with the project who submits the request are also
/// returned.
///
/// Standard query parameters are supported on the URI. For more information,
/// see [Filtering and Column Selection](#filtering).
///
/// Use the `fields` query parameter to control which fields are returned in
/// the response body. For more information, see [Fields](#fields).
///
/// Pagination query parameters are supported if Neutron configuration supports
/// it by overriding `allow_pagination=false`. For more information, see
/// [Pagination](#pagination).
///
/// Sorting query parameters are supported if Neutron configuration supports it
/// with `allow_sorting=true`. For more information, see [Sorting](#sorting).
///
/// You can also use the `tags`, `tags-any`, `not-tags`, `not-tags-any` query
/// parameter to filter the response with tags. For information, see
/// [REST API Impact](http://specs.openstack.org/openstack/neutron-specs/specs/mitaka/add-tags-to-core-resources.html#rest-api-impact).
///
/// Normal response codes: 200
///
/// Error response codes: 401
///
#[derive(Args)]
#[command(about = "List networks")]
pub struct NetworksCommand {
    /// Request Query parameters
    #[command(flatten)]
    query: QueryParameters,

    /// Path parameters
    #[command(flatten)]
    path: PathParameters,

    /// Total limit of entities count to return. Use this when there are too many entries.
    #[arg(long, default_value_t = 10000)]
    max_items: usize,
}

/// Query parameters
#[derive(Args)]
struct QueryParameters {
    /// admin_state_up query parameter for /v2.0/networks API
    ///
    #[arg(action=clap::ArgAction::Set, help_heading = "Query parameters", long)]
    admin_state_up: Option<bool>,

    /// description query parameter for /v2.0/networks API
    ///
    #[arg(help_heading = "Query parameters", long)]
    description: Option<String>,

    /// id query parameter for /v2.0/networks API
    ///
    #[arg(help_heading = "Query parameters", long)]
    id: Option<String>,

    /// is_default query parameter for /v2.0/networks API
    ///
    #[arg(action=clap::ArgAction::Set, help_heading = "Query parameters", long)]
    is_default: Option<bool>,

    /// Requests a page size of items. Returns a number of items up to a limit
    /// value. Use the limit parameter to make an initial limited request and
    /// use the ID of the last-seen item from the response as the marker
    /// parameter value in a subsequent limited request.
    ///
    #[arg(help_heading = "Query parameters", long)]
    limit: Option<i32>,

    /// The ID of the last-seen item. Use the limit parameter to make an
    /// initial limited request and use the ID of the last-seen item from the
    /// response as the marker parameter value in a subsequent limited request.
    ///
    #[arg(help_heading = "Query parameters", long)]
    marker: Option<String>,

    /// mtu query parameter for /v2.0/networks API
    ///
    #[arg(help_heading = "Query parameters", long)]
    mtu: Option<i32>,

    /// name query parameter for /v2.0/networks API
    ///
    #[arg(help_heading = "Query parameters", long)]
    name: Option<String>,

    /// not-tags query parameter for /v2.0/networks API
    ///
    #[arg(action=clap::ArgAction::Append, help_heading = "Query parameters", long)]
    not_tags: Option<Vec<String>>,

    /// not-tags-any query parameter for /v2.0/networks API
    ///
    #[arg(action=clap::ArgAction::Append, help_heading = "Query parameters", long)]
    not_tags_any: Option<Vec<String>>,

    /// Reverse the page direction
    ///
    #[arg(action=clap::ArgAction::Set, help_heading = "Query parameters", long)]
    page_reverse: Option<bool>,

    /// provider:network_type query parameter for /v2.0/networks API
    ///
    #[arg(help_heading = "Query parameters", long)]
    provider_network_type: Option<String>,

    /// provider:physical_network query parameter for /v2.0/networks API
    ///
    #[arg(help_heading = "Query parameters", long)]
    provider_physical_network: Option<String>,

    /// provider:segmentation_id query parameter for /v2.0/networks API
    ///
    #[arg(help_heading = "Query parameters", long)]
    provider_segmentation_id: Option<i32>,

    /// revision_number query parameter for /v2.0/networks API
    ///
    #[arg(help_heading = "Query parameters", long)]
    revision_number: Option<String>,

    /// router:external query parameter for /v2.0/networks API
    ///
    #[arg(action=clap::ArgAction::Set, help_heading = "Query parameters", long)]
    router_external: Option<bool>,

    /// shared query parameter for /v2.0/networks API
    ///
    #[arg(action=clap::ArgAction::Set, help_heading = "Query parameters", long)]
    shared: Option<bool>,

    /// Sort direction. This is an optional feature and may be silently ignored
    /// by the server.
    ///
    #[arg(action=clap::ArgAction::Append, help_heading = "Query parameters", long)]
    sort_dir: Option<Vec<String>>,

    /// Sort results by the attribute. This is an optional feature and may be
    /// silently ignored by the server.
    ///
    #[arg(action=clap::ArgAction::Append, help_heading = "Query parameters", long)]
    sort_key: Option<Vec<String>>,

    /// status query parameter for /v2.0/networks API
    ///
    #[arg(help_heading = "Query parameters", long)]
    status: Option<String>,

    /// tags query parameter for /v2.0/networks API
    ///
    #[arg(action=clap::ArgAction::Append, help_heading = "Query parameters", long)]
    tags: Option<Vec<String>>,

    /// tags-any query parameter for /v2.0/networks API
    ///
    #[arg(action=clap::ArgAction::Append, help_heading = "Query parameters", long)]
    tags_any: Option<Vec<String>>,

    /// tenant_id query parameter for /v2.0/networks API
    ///
    #[arg(help_heading = "Query parameters", long)]
    tenant_id: Option<String>,
}

/// Path parameters
#[derive(Args)]
struct PathParameters {}
/// Networks response representation
#[derive(Deserialize, Serialize, Clone, StructTable)]
struct ResponseData {
    /// The administrative state of the network, which is up (`true`) or down
    /// (`false`).
    ///
    #[serde()]
    #[structable(optional, wide)]
    admin_state_up: Option<BoolString>,

    /// The availability zone candidate for the network.
    ///
    #[serde()]
    #[structable(optional, pretty, wide)]
    availability_zone_hints: Option<Value>,

    /// The availability zone for the network.
    ///
    #[serde()]
    #[structable(optional, pretty, wide)]
    availability_zones: Option<Value>,

    /// Time at which the resource has been created (in UTC ISO8601 format).
    ///
    #[serde()]
    #[structable(optional)]
    created_at: Option<String>,

    /// A human-readable description for the resource.
    ///
    #[serde()]
    #[structable(optional, wide)]
    description: Option<String>,

    /// A valid DNS domain.
    ///
    #[serde()]
    #[structable(optional, wide)]
    dns_domain: Option<String>,

    /// The ID of the network.
    ///
    #[serde()]
    #[structable(optional)]
    id: Option<String>,

    /// The ID of the IPv4 address scope that the network is associated with.
    ///
    #[serde()]
    #[structable(optional, wide)]
    ipv4_address_scope: Option<String>,

    /// The ID of the IPv6 address scope that the network is associated with.
    ///
    #[serde()]
    #[structable(optional, wide)]
    ipv6_address_scope: Option<String>,

    /// The network is default pool or not.
    ///
    #[serde()]
    #[structable(optional, wide)]
    is_default: Option<BoolString>,

    /// Indicates whether L2 connectivity is available throughout the
    /// `network`.
    ///
    #[serde()]
    #[structable(optional, wide)]
    l2_adjacency: Option<String>,

    /// The maximum transmission unit (MTU) value to address fragmentation.
    /// Minimum value is 68 for IPv4, and 1280 for IPv6.
    ///
    #[serde()]
    #[structable(optional, wide)]
    mtu: Option<IntString>,

    /// Human-readable name of the network.
    ///
    #[serde()]
    #[structable(optional)]
    name: Option<String>,

    /// The port security status of the network. Valid values are enabled
    /// (`true`) and disabled (`false`). This value is used as the default
    /// value of `port_security_enabled` field of a newly created port.
    ///
    #[serde()]
    #[structable(optional, wide)]
    port_security_enabled: Option<BoolString>,

    #[serde(rename = "provider:network_type")]
    #[structable(optional, title = "provider:network_type", wide)]
    provider_network_type: Option<String>,

    #[serde(rename = "provider:physical_network")]
    #[structable(optional, title = "provider:physical_network", wide)]
    provider_physical_network: Option<String>,

    #[serde(rename = "provider:segmentation_id")]
    #[structable(optional, title = "provider:segmentation_id", wide)]
    provider_segmentation_id: Option<IntString>,

    /// The ID of the QoS policy associated with the network.
    ///
    #[serde()]
    #[structable(optional, wide)]
    qos_policy_id: Option<String>,

    /// The revision number of the resource.
    ///
    #[serde()]
    #[structable(optional, wide)]
    revision_number: Option<i32>,

    /// Defines whether the network may be used for creation of floating IPs.
    /// Only networks with this flag may be an external gateway for routers.
    /// The network must have an external routing facility that is not managed
    /// by the networking service. If the network is updated from external to
    /// internal the unused floating IPs of this network are automatically
    /// deleted when extension `floatingip-autodelete-internal` is present.
    ///
    #[serde(rename = "router:external")]
    #[structable(optional, title = "router:external", wide)]
    router_external: Option<BoolString>,

    /// A list of provider `segment` objects.
    ///
    #[serde()]
    #[structable(optional, pretty, wide)]
    segments: Option<Value>,

    /// Indicates whether this network is shared across all tenants. By
    /// default, only administrative users can change this value.
    ///
    #[serde()]
    #[structable(optional, wide)]
    shared: Option<BoolString>,

    /// The network status. Values are `ACTIVE`, `DOWN`, `BUILD` or `ERROR`.
    ///
    #[serde()]
    #[structable(optional)]
    status: Option<String>,

    /// The associated subnets.
    ///
    #[serde()]
    #[structable(optional, pretty, wide)]
    subnets: Option<Value>,

    /// The list of tags on the resource.
    ///
    #[serde()]
    #[structable(optional, pretty, wide)]
    tags: Option<Value>,

    /// The ID of the project.
    ///
    #[serde()]
    #[structable(optional, wide)]
    tenant_id: Option<String>,

    /// Time at which the resource has been updated (in UTC ISO8601 format).
    ///
    #[serde()]
    #[structable(optional)]
    updated_at: Option<String>,
}

impl NetworksCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("List Networks");

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;

        let mut ep_builder = list::Request::builder();

        // Set path parameters
        // Set query parameters
        if let Some(val) = &self.query.id {
            ep_builder.id(val);
        }
        if let Some(val) = &self.query.name {
            ep_builder.name(val);
        }
        if let Some(val) = &self.query.admin_state_up {
            ep_builder.admin_state_up(*val);
        }
        if let Some(val) = &self.query.status {
            ep_builder.status(val);
        }
        if let Some(val) = &self.query.tenant_id {
            ep_builder.tenant_id(val);
        }
        if let Some(val) = &self.query.shared {
            ep_builder.shared(*val);
        }
        if let Some(val) = &self.query.router_external {
            ep_builder.router_external(*val);
        }
        if let Some(val) = &self.query.mtu {
            ep_builder.mtu(*val);
        }
        if let Some(val) = &self.query.provider_network_type {
            ep_builder.provider_network_type(val);
        }
        if let Some(val) = &self.query.provider_physical_network {
            ep_builder.provider_physical_network(val);
        }
        if let Some(val) = &self.query.provider_segmentation_id {
            ep_builder.provider_segmentation_id(*val);
        }
        if let Some(val) = &self.query.revision_number {
            ep_builder.revision_number(val);
        }
        if let Some(val) = &self.query.tags {
            ep_builder.tags(val.iter());
        }
        if let Some(val) = &self.query.tags_any {
            ep_builder.tags_any(val.iter());
        }
        if let Some(val) = &self.query.not_tags {
            ep_builder.not_tags(val.iter());
        }
        if let Some(val) = &self.query.not_tags_any {
            ep_builder.not_tags_any(val.iter());
        }
        if let Some(val) = &self.query.is_default {
            ep_builder.is_default(*val);
        }
        if let Some(val) = &self.query.description {
            ep_builder.description(val);
        }
        if let Some(val) = &self.query.sort_key {
            ep_builder.sort_key(val.iter());
        }
        if let Some(val) = &self.query.sort_dir {
            ep_builder.sort_dir(val.iter());
        }
        if let Some(val) = &self.query.limit {
            ep_builder.limit(*val);
        }
        if let Some(val) = &self.query.marker {
            ep_builder.marker(val);
        }
        if let Some(val) = &self.query.page_reverse {
            ep_builder.page_reverse(*val);
        }
        // Set body parameters

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;

        let data: Vec<serde_json::Value> = paged(ep, Pagination::Limit(self.max_items))
            .query_async(client)
            .await?;

        op.output_list::<ResponseData>(data)?;
        Ok(())
    }
}
