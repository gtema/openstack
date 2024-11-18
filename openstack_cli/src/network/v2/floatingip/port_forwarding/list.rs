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

//! List PortForwardings command
//!
//! Wraps invoking of the `v2.0/floatingips/{floatingip_id}/port_forwardings` with `GET` method

use clap::Args;
use serde::{Deserialize, Serialize};
use tracing::info;

use openstack_sdk::AsyncOpenStack;

use crate::output::OutputProcessor;
use crate::Cli;
use crate::OpenStackCliError;
use crate::OutputConfig;
use crate::StructTable;

use openstack_sdk::api::network::v2::floatingip::port_forwarding::list;
use openstack_sdk::api::QueryAsync;
use openstack_sdk::api::{paged, Pagination};
use structable_derive::StructTable;

/// Lists floating IP port forwardings that the project has access to.
///
/// Default policy settings return only the port forwardings associated to
/// floating IPs owned by the project of the user submitting the request,
/// unless the user has administrative role.
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
/// Normal response codes: 200
///
/// Error response codes: 400, 404
///
#[derive(Args)]
#[command(about = "List floating IP port forwardings")]
pub struct PortForwardingsCommand {
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
    /// description query parameter for
    /// /v2.0/floatingips/{floatingip_id}/port_forwardings API
    ///
    #[arg(help_heading = "Query parameters", long)]
    description: Option<String>,

    /// external_port query parameter for
    /// /v2.0/floatingips/{floatingip_id}/port_forwardings API
    ///
    #[arg(help_heading = "Query parameters", long)]
    external_port: Option<f32>,

    /// external_port_range query parameter for
    /// /v2.0/floatingips/{floatingip_id}/port_forwardings API
    ///
    #[arg(help_heading = "Query parameters", long)]
    external_port_range: Option<f32>,

    /// id query parameter for
    /// /v2.0/floatingips/{floatingip_id}/port_forwardings API
    ///
    #[arg(help_heading = "Query parameters", long)]
    id: Option<String>,

    /// internal_port_id query parameter for
    /// /v2.0/floatingips/{floatingip_id}/port_forwardings API
    ///
    #[arg(help_heading = "Query parameters", long)]
    internal_port_id: Option<String>,

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

    /// Reverse the page direction
    ///
    #[arg(action=clap::ArgAction::Set, help_heading = "Query parameters", long)]
    page_reverse: Option<bool>,

    /// protocol query parameter for
    /// /v2.0/floatingips/{floatingip_id}/port_forwardings API
    ///
    #[arg(help_heading = "Query parameters", long, value_parser = ["dccp","icmp","ipv6-icmp","sctp","tcp","udp"])]
    protocol: Option<String>,

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
}

/// Path parameters
#[derive(Args)]
struct PathParameters {
    /// floatingip_id parameter for
    /// /v2.0/floatingips/{floatingip_id}/port_forwardings/{id} API
    ///
    #[arg(
        help_heading = "Path parameters",
        id = "path_param_floatingip_id",
        value_name = "FLOATINGIP_ID"
    )]
    floatingip_id: String,
}
/// PortForwardings response representation
#[derive(Deserialize, Serialize, Clone, StructTable)]
struct ResponseData {
    /// A text describing the rule, which helps users to manage/find easily
    /// theirs rules.
    ///
    #[serde()]
    #[structable(optional, wide)]
    description: Option<String>,

    /// The TCP/UDP/other protocol port number of the port forwarding’s
    /// floating IP address.
    ///
    #[serde()]
    #[structable(optional, wide)]
    external_port: Option<f32>,

    /// The TCP/UDP/other protocol port range of the port forwarding’s floating
    /// IP address.
    ///
    #[serde()]
    #[structable(optional, wide)]
    external_port_range: Option<f32>,

    /// The ID of the floating IP port forwarding.
    ///
    #[serde()]
    #[structable(optional)]
    id: Option<String>,

    /// The fixed IPv4 address of the Neutron port associated to the floating
    /// IP port forwarding.
    ///
    #[serde()]
    #[structable(optional, wide)]
    internal_ip_address: Option<String>,

    /// The TCP/UDP/other protocol port number of the Neutron port fixed IP
    /// address associated to the floating ip port forwarding.
    ///
    #[serde()]
    #[structable(optional, wide)]
    internal_port: Option<f32>,

    /// The ID of the Neutron port associated to the floating IP port
    /// forwarding.
    ///
    #[serde()]
    #[structable(optional, wide)]
    internal_port_id: Option<String>,

    /// The TCP/UDP/other protocol port range of the Neutron port fixed IP
    /// address associated to the floating ip port forwarding.
    ///
    #[serde()]
    #[structable(optional, wide)]
    internal_port_range: Option<f32>,

    /// The IP protocol used in the floating IP port forwarding.
    ///
    #[serde()]
    #[structable(optional, wide)]
    protocol: Option<String>,
}

impl PortForwardingsCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("List PortForwardings");

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;

        let mut ep_builder = list::Request::builder();

        // Set path parameters
        ep_builder.floatingip_id(&self.path.floatingip_id);
        // Set query parameters
        if let Some(val) = &self.query.id {
            ep_builder.id(val);
        }
        if let Some(val) = &self.query.external_port {
            ep_builder.external_port(*val);
        }
        if let Some(val) = &self.query.protocol {
            ep_builder.protocol(val);
        }
        if let Some(val) = &self.query.internal_port_id {
            ep_builder.internal_port_id(val);
        }
        if let Some(val) = &self.query.description {
            ep_builder.description(val);
        }
        if let Some(val) = &self.query.external_port_range {
            ep_builder.external_port_range(*val);
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
