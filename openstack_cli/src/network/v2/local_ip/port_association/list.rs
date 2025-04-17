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

//! List PortAssociations command
//!
//! Wraps invoking of the `v2.0/local_ips/{local_ip_id}/port_associations` with `GET` method

use clap::Args;
use tracing::info;

use openstack_sdk::AsyncOpenStack;

use crate::Cli;
use crate::OpenStackCliError;
use crate::output::OutputProcessor;

use openstack_sdk::api::QueryAsync;
use openstack_sdk::api::network::v2::local_ip::port_association::list;
use openstack_sdk::api::{Pagination, paged};
use openstack_types::network::v2::local_ip::port_association::response::list::PortAssociationResponse;

/// Lists Associations for the given Local IP.
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
/// Error response codes: 401
#[derive(Args)]
#[command(about = "List Local IP Associations")]
pub struct PortAssociationsCommand {
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
    /// fixed_ip query parameter for
    /// /v2.0/local_ips/{local_ip_id}/port_associations API
    #[arg(help_heading = "Query parameters", long)]
    fixed_ip: Option<String>,

    /// fixed_port_id query parameter for
    /// /v2.0/local_ips/{local_ip_id}/port_associations API
    #[arg(help_heading = "Query parameters", long)]
    fixed_port_id: Option<String>,

    /// host query parameter for
    /// /v2.0/local_ips/{local_ip_id}/port_associations API
    #[arg(help_heading = "Query parameters", long)]
    host: Option<String>,

    /// Requests a page size of items. Returns a number of items up to a limit
    /// value. Use the limit parameter to make an initial limited request and
    /// use the ID of the last-seen item from the response as the marker
    /// parameter value in a subsequent limited request.
    #[arg(help_heading = "Query parameters", long)]
    limit: Option<i32>,

    /// local_ip_address query parameter for
    /// /v2.0/local_ips/{local_ip_id}/port_associations API
    #[arg(help_heading = "Query parameters", long)]
    local_ip_address: Option<String>,

    /// The ID of the last-seen item. Use the limit parameter to make an
    /// initial limited request and use the ID of the last-seen item from the
    /// response as the marker parameter value in a subsequent limited request.
    #[arg(help_heading = "Query parameters", long)]
    marker: Option<String>,

    /// Reverse the page direction
    #[arg(action=clap::ArgAction::Set, help_heading = "Query parameters", long)]
    page_reverse: Option<bool>,

    /// Sort direction. This is an optional feature and may be silently ignored
    /// by the server.
    #[arg(action=clap::ArgAction::Append, help_heading = "Query parameters", long)]
    sort_dir: Option<Vec<String>>,

    /// Sort results by the attribute. This is an optional feature and may be
    /// silently ignored by the server.
    #[arg(action=clap::ArgAction::Append, help_heading = "Query parameters", long)]
    sort_key: Option<Vec<String>>,
}

/// Path parameters
#[derive(Args)]
struct PathParameters {
    /// local_ip_id parameter for
    /// /v2.0/local_ips/{local_ip_id}/port_associations/{id} API
    #[arg(
        help_heading = "Path parameters",
        id = "path_param_local_ip_id",
        value_name = "LOCAL_IP_ID"
    )]
    local_ip_id: String,
}

impl PortAssociationsCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("List PortAssociations");

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;

        let mut ep_builder = list::Request::builder();

        // Set path parameters
        ep_builder.local_ip_id(&self.path.local_ip_id);
        // Set query parameters
        if let Some(val) = &self.query.local_ip_address {
            ep_builder.local_ip_address(val);
        }
        if let Some(val) = &self.query.fixed_port_id {
            ep_builder.fixed_port_id(val);
        }
        if let Some(val) = &self.query.fixed_ip {
            ep_builder.fixed_ip(val);
        }
        if let Some(val) = &self.query.host {
            ep_builder.host(val);
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
        op.output_list::<PortAssociationResponse>(data)?;
        Ok(())
    }
}
