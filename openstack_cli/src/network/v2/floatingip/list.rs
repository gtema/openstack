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

//! List Floatingips command
//!
//! Wraps invoking of the `v2.0/floatingips` with `GET` method

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
use openstack_sdk::api::network::v2::floatingip::list;
use openstack_sdk::api::{Pagination, paged};
use serde_json::Value;
use structable_derive::StructTable;

/// Lists floating IPs visible to the user.
///
/// Default policy settings return only the floating IPs owned by the user’s
/// project, unless the user has admin role.
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
/// This example request lists floating IPs in JSON format:
///
/// Normal response codes: 200
///
/// Error response codes: 401
///
#[derive(Args)]
#[command(about = "List floating IPs")]
pub struct FloatingipsCommand {
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
    /// description query parameter for /v2.0/floatingips API
    ///
    #[arg(help_heading = "Query parameters", long)]
    description: Option<String>,

    /// fixed_ip_address query parameter for /v2.0/floatingips API
    ///
    #[arg(help_heading = "Query parameters", long)]
    fixed_ip_address: Option<String>,

    /// floating_ip_address query parameter for /v2.0/floatingips API
    ///
    #[arg(help_heading = "Query parameters", long)]
    floating_ip_address: Option<String>,

    /// floating_network_id query parameter for /v2.0/floatingips API
    ///
    #[arg(help_heading = "Query parameters", long)]
    floating_network_id: Option<String>,

    /// id query parameter for /v2.0/floatingips API
    ///
    #[arg(help_heading = "Query parameters", long)]
    id: Option<String>,

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

    /// not-tags query parameter for /v2.0/floatingips API
    ///
    #[arg(action=clap::ArgAction::Append, help_heading = "Query parameters", long)]
    not_tags: Option<Vec<String>>,

    /// not-tags-any query parameter for /v2.0/floatingips API
    ///
    #[arg(action=clap::ArgAction::Append, help_heading = "Query parameters", long)]
    not_tags_any: Option<Vec<String>>,

    /// Reverse the page direction
    ///
    #[arg(action=clap::ArgAction::Set, help_heading = "Query parameters", long)]
    page_reverse: Option<bool>,

    /// port_id query parameter for /v2.0/floatingips API
    ///
    #[arg(help_heading = "Query parameters", long)]
    port_id: Option<String>,

    /// revision_number query parameter for /v2.0/floatingips API
    ///
    #[arg(help_heading = "Query parameters", long)]
    revision_number: Option<String>,

    /// router_id query parameter for /v2.0/floatingips API
    ///
    #[arg(help_heading = "Query parameters", long)]
    router_id: Option<String>,

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

    /// status query parameter for /v2.0/floatingips API
    ///
    #[arg(help_heading = "Query parameters", long)]
    status: Option<String>,

    /// tags query parameter for /v2.0/floatingips API
    ///
    #[arg(action=clap::ArgAction::Append, help_heading = "Query parameters", long)]
    tags: Option<Vec<String>>,

    /// tags-any query parameter for /v2.0/floatingips API
    ///
    #[arg(action=clap::ArgAction::Append, help_heading = "Query parameters", long)]
    tags_any: Option<Vec<String>>,

    /// tenant_id query parameter for /v2.0/floatingips API
    ///
    #[arg(help_heading = "Query parameters", long)]
    tenant_id: Option<String>,
}

/// Path parameters
#[derive(Args)]
struct PathParameters {}
/// Floatingips response representation
#[derive(Deserialize, Serialize, Clone, StructTable)]
struct ResponseData {
    /// Time at which the resource has been created (in UTC ISO8601 format).
    ///
    #[serde()]
    #[structable(optional)]
    created_at: Option<String>,

    #[serde()]
    #[structable(optional, wide)]
    description: Option<String>,

    /// A valid DNS domain.
    ///
    #[serde()]
    #[structable(optional, wide)]
    dns_domain: Option<String>,

    /// A valid DNS name.
    ///
    #[serde()]
    #[structable(optional, wide)]
    dns_name: Option<String>,

    /// The fixed IP address that is associated with the floating IP address.
    ///
    #[serde()]
    #[structable(optional, wide)]
    fixed_ip_address: Option<String>,

    /// The floating IP address.
    ///
    #[serde()]
    #[structable(optional)]
    floating_ip_address: Option<String>,

    /// The ID of the network associated with the floating IP.
    ///
    #[serde()]
    #[structable(optional, wide)]
    floating_network_id: Option<String>,

    /// The ID of the floating IP address.
    ///
    #[serde()]
    #[structable(optional)]
    id: Option<String>,

    /// The information of the port that this floating IP associates with. In
    /// particular, if the floating IP is associated with a port, this field
    /// contains some attributes of the associated port, including `name`,
    /// `network_id`, `mac_address`, `admin_state_up`, `status`, `device_id`
    /// and `device_owner`. If the floating IP is not associated with a port,
    /// this field is `null`.
    ///
    #[serde()]
    #[structable(optional, pretty, wide)]
    port_details: Option<Value>,

    /// The associated port forwarding resources for the floating IP. If the
    /// floating IP has multiple port forwarding resources, this field has
    /// multiple entries. Each entry consists of network IP protocol
    /// (`protocol`), the fixed IP address of internal neutron port
    /// (`internal_ip_address`), the TCP or UDP port or port range used by
    /// internal neutron port (`internal_port`) or (`internal_port_range`) and
    /// the TCP or UDP port or port range used by floating IP (`external_port`)
    /// or (`external_port_range`).
    ///
    #[serde()]
    #[structable(optional, pretty, wide)]
    port_forwardings: Option<Value>,

    /// The ID of a port associated with the floating IP.
    ///
    #[serde()]
    #[structable(optional, wide)]
    port_id: Option<String>,

    /// The ID of the QoS policy associated with the floating IP.
    ///
    #[serde()]
    #[structable(optional, wide)]
    qos_policy_id: Option<String>,

    /// The revision number of the resource.
    ///
    #[serde()]
    #[structable(optional, wide)]
    revision_number: Option<i32>,

    /// The ID of the router for the floating IP.
    ///
    #[serde()]
    #[structable(optional, wide)]
    router_id: Option<String>,

    /// The status of the floating IP. Values are `ACTIVE`, `DOWN` and `ERROR`.
    ///
    #[serde()]
    #[structable(optional)]
    status: Option<String>,

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

impl FloatingipsCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("List Floatingips");

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;

        let mut ep_builder = list::Request::builder();

        // Set path parameters
        // Set query parameters
        if let Some(val) = &self.query.id {
            ep_builder.id(val);
        }
        if let Some(val) = &self.query.floating_ip_address {
            ep_builder.floating_ip_address(val);
        }
        if let Some(val) = &self.query.floating_network_id {
            ep_builder.floating_network_id(val);
        }
        if let Some(val) = &self.query.router_id {
            ep_builder.router_id(val);
        }
        if let Some(val) = &self.query.port_id {
            ep_builder.port_id(val);
        }
        if let Some(val) = &self.query.fixed_ip_address {
            ep_builder.fixed_ip_address(val);
        }
        if let Some(val) = &self.query.tenant_id {
            ep_builder.tenant_id(val);
        }
        if let Some(val) = &self.query.status {
            ep_builder.status(val);
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
