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

//! List Logs command
//!
//! Wraps invoking of the `v2.0/log/logs` with `GET` method

use clap::Args;
use serde::{Deserialize, Serialize};
use tracing::info;

use openstack_sdk::AsyncOpenStack;

use crate::output::OutputProcessor;
use crate::Cli;
use crate::OpenStackCliError;
use crate::OutputConfig;
use crate::StructTable;

use crate::common::BoolString;
use openstack_sdk::api::network::v2::log::log::list;
use openstack_sdk::api::QueryAsync;
use openstack_sdk::api::{paged, Pagination};
use structable_derive::StructTable;

/// Lists all log resources associated with your project.
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
/// The list might be empty.
///
/// Normal response codes: 200
///
/// Error response codes: 401
///
#[derive(Args)]
#[command(about = "List logs")]
pub struct LogsCommand {
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
    /// description query parameter for /v2.0/log/logs API
    ///
    #[arg(help_heading = "Query parameters", long)]
    description: Option<String>,

    /// enabled query parameter for /v2.0/log/logs API
    ///
    #[arg(action=clap::ArgAction::Set, help_heading = "Query parameters", long)]
    enabled: Option<bool>,

    /// event query parameter for /v2.0/log/logs API
    ///
    #[arg(help_heading = "Query parameters", long, value_parser = ["ACCEPT","ALL","DROP"])]
    event: Option<String>,

    /// id query parameter for /v2.0/log/logs API
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

    /// name query parameter for /v2.0/log/logs API
    ///
    #[arg(help_heading = "Query parameters", long)]
    name: Option<String>,

    /// Reverse the page direction
    ///
    #[arg(action=clap::ArgAction::Set, help_heading = "Query parameters", long)]
    page_reverse: Option<bool>,

    /// project_id query parameter for /v2.0/log/logs API
    ///
    #[arg(help_heading = "Query parameters", long)]
    project_id: Option<String>,

    /// resource_id query parameter for /v2.0/log/logs API
    ///
    #[arg(help_heading = "Query parameters", long)]
    resource_id: Option<String>,

    /// resource_type query parameter for /v2.0/log/logs API
    ///
    #[arg(help_heading = "Query parameters", long)]
    resource_type: Option<String>,

    /// revision_number query parameter for /v2.0/log/logs API
    ///
    #[arg(help_heading = "Query parameters", long)]
    revision_number: Option<String>,

    /// Sort direction. This is an optional feature and may be silently ignored
    /// by the server.
    ///
    #[arg(help_heading = "Query parameters", long, value_parser = ["asc","desc"])]
    sort_dir: Option<String>,

    /// Sort results by the attribute. This is an optional feature and may be
    /// silently ignored by the server.
    ///
    #[arg(help_heading = "Query parameters", long)]
    sort_key: Option<String>,

    /// target_id query parameter for /v2.0/log/logs API
    ///
    #[arg(help_heading = "Query parameters", long)]
    target_id: Option<String>,
}

/// Path parameters
#[derive(Args)]
struct PathParameters {}
/// Logs response representation
#[derive(Deserialize, Serialize, Clone, StructTable)]
struct ResponseData {
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

    /// Indicates whether this log object is enabled or disabled.
    ///
    #[serde()]
    #[structable(optional, wide)]
    enabled: Option<BoolString>,

    /// Type of security events to log. `ACCEPT`, `DROP`, or `ALL`.
    ///
    #[serde()]
    #[structable(optional, wide)]
    event: Option<String>,

    /// The ID of the log object.
    ///
    #[serde()]
    #[structable(optional)]
    id: Option<String>,

    /// Human-readable name of the resource.
    ///
    #[serde()]
    #[structable(optional)]
    name: Option<String>,

    /// The ID of the project.
    ///
    #[serde()]
    #[structable(optional, wide)]
    project_id: Option<String>,

    /// The ID of resource log (e.g security group ID).
    ///
    #[serde()]
    #[structable(optional, wide)]
    resource_id: Option<String>,

    /// The resource log type such as ‘security_group’.
    ///
    #[serde()]
    #[structable(optional, wide)]
    resource_type: Option<String>,

    /// The revision number of the resource.
    ///
    #[serde()]
    #[structable(optional, wide)]
    revision_number: Option<i32>,

    /// The ID of resource target log such as port ID.
    ///
    #[serde()]
    #[structable(optional, wide)]
    target_id: Option<String>,

    /// Time at which the resource has been updated (in UTC ISO8601 format).
    ///
    #[serde()]
    #[structable(optional)]
    updated_at: Option<String>,
}

impl LogsCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("List Logs");

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;

        let mut ep_builder = list::Request::builder();

        // Set path parameters
        // Set query parameters
        if let Some(val) = &self.query.id {
            ep_builder.id(val);
        }
        if let Some(val) = &self.query.project_id {
            ep_builder.project_id(val);
        }
        if let Some(val) = &self.query.name {
            ep_builder.name(val);
        }
        if let Some(val) = &self.query.resource_type {
            ep_builder.resource_type(val);
        }
        if let Some(val) = &self.query.resource_id {
            ep_builder.resource_id(val);
        }
        if let Some(val) = &self.query.event {
            ep_builder.event(val);
        }
        if let Some(val) = &self.query.target_id {
            ep_builder.target_id(val);
        }
        if let Some(val) = &self.query.enabled {
            ep_builder.enabled(*val);
        }
        if let Some(val) = &self.query.revision_number {
            ep_builder.revision_number(val);
        }
        if let Some(val) = &self.query.description {
            ep_builder.description(val);
        }
        if let Some(val) = &self.query.sort_key {
            ep_builder.sort_key(val);
        }
        if let Some(val) = &self.query.sort_dir {
            ep_builder.sort_dir(val);
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
