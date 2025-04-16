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
use tracing::info;

use openstack_sdk::AsyncOpenStack;

use crate::Cli;
use crate::OpenStackCliError;
use crate::output::OutputProcessor;

use openstack_sdk::api::QueryAsync;
use openstack_sdk::api::network::v2::network::list;
use openstack_sdk::api::{Pagination, paged};
use openstack_types::network::v2::network::response::list::NetworkResponse;

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
    #[arg(action=clap::ArgAction::Set, help_heading = "Query parameters", long)]
    admin_state_up: Option<bool>,

    /// description query parameter for /v2.0/networks API
    #[arg(help_heading = "Query parameters", long)]
    description: Option<String>,

    /// id query parameter for /v2.0/networks API
    #[arg(help_heading = "Query parameters", long)]
    id: Option<String>,

    /// is_default query parameter for /v2.0/networks API
    #[arg(action=clap::ArgAction::Set, help_heading = "Query parameters", long)]
    is_default: Option<bool>,

    /// Requests a page size of items. Returns a number of items up to a limit
    /// value. Use the limit parameter to make an initial limited request and
    /// use the ID of the last-seen item from the response as the marker
    /// parameter value in a subsequent limited request.
    #[arg(help_heading = "Query parameters", long)]
    limit: Option<i32>,

    /// The ID of the last-seen item. Use the limit parameter to make an
    /// initial limited request and use the ID of the last-seen item from the
    /// response as the marker parameter value in a subsequent limited request.
    #[arg(help_heading = "Query parameters", long)]
    marker: Option<String>,

    /// mtu query parameter for /v2.0/networks API
    #[arg(help_heading = "Query parameters", long)]
    mtu: Option<i32>,

    /// name query parameter for /v2.0/networks API
    #[arg(help_heading = "Query parameters", long)]
    name: Option<String>,

    /// not-tags query parameter for /v2.0/networks API
    #[arg(action=clap::ArgAction::Append, help_heading = "Query parameters", long)]
    not_tags: Option<Vec<String>>,

    /// not-tags-any query parameter for /v2.0/networks API
    #[arg(action=clap::ArgAction::Append, help_heading = "Query parameters", long)]
    not_tags_any: Option<Vec<String>>,

    /// Reverse the page direction
    #[arg(action=clap::ArgAction::Set, help_heading = "Query parameters", long)]
    page_reverse: Option<bool>,

    /// provider:network_type query parameter for /v2.0/networks API
    #[arg(help_heading = "Query parameters", long)]
    provider_network_type: Option<String>,

    /// provider:physical_network query parameter for /v2.0/networks API
    #[arg(help_heading = "Query parameters", long)]
    provider_physical_network: Option<String>,

    /// provider:segmentation_id query parameter for /v2.0/networks API
    #[arg(help_heading = "Query parameters", long)]
    provider_segmentation_id: Option<i32>,

    /// revision_number query parameter for /v2.0/networks API
    #[arg(help_heading = "Query parameters", long)]
    revision_number: Option<String>,

    /// router:external query parameter for /v2.0/networks API
    #[arg(action=clap::ArgAction::Set, help_heading = "Query parameters", long)]
    router_external: Option<bool>,

    /// shared query parameter for /v2.0/networks API
    #[arg(action=clap::ArgAction::Set, help_heading = "Query parameters", long)]
    shared: Option<bool>,

    /// Sort direction. This is an optional feature and may be silently ignored
    /// by the server.
    #[arg(action=clap::ArgAction::Append, help_heading = "Query parameters", long)]
    sort_dir: Option<Vec<String>>,

    /// Sort results by the attribute. This is an optional feature and may be
    /// silently ignored by the server.
    #[arg(action=clap::ArgAction::Append, help_heading = "Query parameters", long)]
    sort_key: Option<Vec<String>>,

    /// status query parameter for /v2.0/networks API
    #[arg(help_heading = "Query parameters", long)]
    status: Option<String>,

    /// tags query parameter for /v2.0/networks API
    #[arg(action=clap::ArgAction::Append, help_heading = "Query parameters", long)]
    tags: Option<Vec<String>>,

    /// tags-any query parameter for /v2.0/networks API
    #[arg(action=clap::ArgAction::Append, help_heading = "Query parameters", long)]
    tags_any: Option<Vec<String>>,

    /// tenant_id query parameter for /v2.0/networks API
    #[arg(help_heading = "Query parameters", long)]
    tenant_id: Option<String>,
}

/// Path parameters
#[derive(Args)]
struct PathParameters {}

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
        op.output_list::<NetworkResponse>(data)?;
        Ok(())
    }
}
