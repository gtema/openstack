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

//! List Recordsets command
//!
//! Wraps invoking of the `v2/zones/{zone_id}/recordsets` with `GET` method

use clap::Args;
use tracing::info;

use openstack_sdk::AsyncOpenStack;

use crate::Cli;
use crate::OpenStackCliError;
use crate::output::OutputProcessor;

use openstack_sdk::api::QueryAsync;
use openstack_sdk::api::dns::v2::zone::find as find_zone;
use openstack_sdk::api::dns::v2::zone::recordset::list;
use openstack_sdk::api::find_by_name;
use openstack_sdk::api::{Pagination, paged};
use openstack_types::dns::v2::zone::recordset::response::list::RecordsetResponse;
use tracing::warn;

/// This lists all recordsets in a zone
#[derive(Args)]
#[command(about = "List Recordsets in a Zone")]
pub struct RecordsetsCommand {
    /// Request Query parameters
    #[command(flatten)]
    query: QueryParameters,

    /// Request Headers parameters
    #[command(flatten)]
    headers: HeaderParameters,

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
    /// Filter results to only show zones that have a type matching the filter
    #[arg(help_heading = "Query parameters", long, value_parser = ["CATALOG","PRIMARY","SECONDARY"])]
    _type: Option<String>,

    /// Filter results to only show recordsets that have a record with data
    /// matching the filter
    #[arg(help_heading = "Query parameters", long)]
    data: Option<String>,

    /// Filter results to only show zones that have a description matching the
    /// filter
    #[arg(help_heading = "Query parameters", long)]
    description: Option<String>,

    /// Requests a page size of items. Returns a number of items up to a limit
    /// value. Use the limit parameter to make an initial limited request and
    /// use the ID of the last-seen item from the response as the marker
    /// parameter value in a subsequent limited request.
    #[arg(
        help_heading = "Query parameters",
        long("page-size"),
        visible_alias("limit")
    )]
    limit: Option<i32>,

    /// The ID of the last-seen item. Use the limit parameter to make an
    /// initial limited request and use the ID of the last-seen item from the
    /// response as the marker parameter value in a subsequent limited request.
    #[arg(help_heading = "Query parameters", long)]
    marker: Option<String>,

    /// Filter results to only show zones that have a name matching the filter
    #[arg(help_heading = "Query parameters", long)]
    name: Option<String>,

    /// Sorts the response by the requested sort direction. A valid value is
    /// asc (ascending) or desc (descending). Default is asc. You can specify
    /// multiple pairs of sort key and sort direction query parameters. If you
    /// omit the sort direction in a pair, the API uses the natural sorting
    /// direction of the server attribute that is provided as the sort_key.
    #[arg(help_heading = "Query parameters", long, value_parser = ["asc","desc"])]
    sort_dir: Option<String>,

    /// Sorts the response by the this attribute value. Default is id. You can
    /// specify multiple pairs of sort key and sort direction query parameters.
    /// If you omit the sort direction in a pair, the API uses the natural
    /// sorting direction of the server attribute that is provided as the
    /// sort_key.
    #[arg(help_heading = "Query parameters", long, value_parser = ["created_at","id","name","serial","status","tenant_id","ttl","updated_at"])]
    sort_key: Option<String>,

    /// Filter results to only show zones that have a status matching the
    /// filter
    #[arg(help_heading = "Query parameters", long, value_parser = ["ACTIVE","DELETED","ERROR","PENDING","SUCCESS","ZONE"])]
    status: Option<String>,

    /// Filter results to only show zones that have a ttl matching the filter
    #[arg(help_heading = "Query parameters", long)]
    ttl: Option<i32>,
}

/// Header parameters
#[derive(Args)]
struct HeaderParameters {
    /// If enabled this will show results from all projects in Designate
    #[arg(long)]
    x_auth_all_projects: Option<bool>,

    /// This allows a user to impersonate another project
    #[arg(long)]
    x_auth_sudo_project_id: Option<String>,
}

/// Path parameters
#[derive(Args)]
struct PathParameters {
    /// Zone resource for which the operation should be performed.
    #[command(flatten)]
    zone: ZoneInput,
}

/// Zone input select group
#[derive(Args)]
#[group(required = true, multiple = false)]
struct ZoneInput {
    /// Zone Name.
    #[arg(long, help_heading = "Path parameters", value_name = "ZONE_NAME")]
    zone_name: Option<String>,
    /// Zone ID.
    #[arg(long, help_heading = "Path parameters", value_name = "ZONE_ID")]
    zone_id: Option<String>,
}

impl RecordsetsCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("List Recordsets");

        let op = OutputProcessor::from_args(parsed_args, Some("dns.zone/recordset"), Some("list"));
        op.validate_args(parsed_args)?;

        let mut ep_builder = list::Request::builder();
        // Set path parameters

        // Process path parameter `zone_id`
        if let Some(id) = &self.path.zone.zone_id {
            // zone_id is passed. No need to lookup
            ep_builder.zone_id(id);
        } else if let Some(name) = &self.path.zone.zone_name {
            // zone_name is passed. Need to lookup resource
            let mut sub_find_builder = find_zone::Request::builder();
            warn!(
                "Querying zone by name (because of `--zone-name` parameter passed) may not be definite. This may fail in which case parameter `--zone-id` should be used instead."
            );

            sub_find_builder.id(name);
            let find_ep = sub_find_builder
                .build()
                .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;
            let find_data: serde_json::Value = find_by_name(find_ep).query_async(client).await?;
            // Try to extract resource id
            match find_data.get("id") {
                Some(val) => match val.as_str() {
                    Some(id_str) => {
                        ep_builder.zone_id(id_str.to_owned());
                    }
                    None => {
                        return Err(OpenStackCliError::ResourceAttributeNotString(
                            serde_json::to_string(&val)?,
                        ));
                    }
                },
                None => {
                    return Err(OpenStackCliError::ResourceAttributeMissing(
                        "id".to_string(),
                    ));
                }
            };
        }
        // Set query parameters
        if let Some(val) = &self.query.data {
            ep_builder.data(val);
        }
        if let Some(val) = &self.query.description {
            ep_builder.description(val);
        }
        if let Some(val) = &self.query.limit {
            ep_builder.limit(*val);
        }
        if let Some(val) = &self.query.marker {
            ep_builder.marker(val);
        }
        if let Some(val) = &self.query.name {
            ep_builder.name(val);
        }
        if let Some(val) = &self.query.sort_dir {
            ep_builder.sort_dir(val);
        }
        if let Some(val) = &self.query.sort_key {
            ep_builder.sort_key(val);
        }
        if let Some(val) = &self.query.status {
            ep_builder.status(val);
        }
        if let Some(val) = &self.query.ttl {
            ep_builder.ttl(*val);
        }
        if let Some(val) = &self.query._type {
            ep_builder._type(val);
        }
        // Set header parameters
        if let Some(val) = &self.headers.x_auth_all_projects {
            ep_builder.header(
                http::header::HeaderName::from_static("x-auth-all-projects"),
                http::header::HeaderValue::from_static(if *val { "true" } else { "false" }),
            );
        }
        if let Some(val) = &self.headers.x_auth_sudo_project_id {
            ep_builder.header(
                http::header::HeaderName::from_static("x-auth-sudo-project-id"),
                http::header::HeaderValue::from_str(val)?,
            );
        }

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;

        let data: Vec<serde_json::Value> = paged(ep, Pagination::Limit(self.max_items))
            .query_async(client)
            .await?;
        op.output_list::<RecordsetResponse>(data)?;
        Ok(())
    }
}
