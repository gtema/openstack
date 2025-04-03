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
//! Wraps invoking of the `v2/recordsets` with `GET` method

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
use openstack_sdk::api::dns::v2::recordset::list;
use openstack_sdk::api::{Pagination, paged};
use serde_json::Value;
use structable_derive::StructTable;

/// This lists all recordsets owned by a project in Designate
///
#[derive(Args)]
#[command(about = "List all Recordsets owned by project")]
pub struct RecordsetsCommand {
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
    /// Filter results to only show zones that have a type matching the filter
    ///
    #[arg(help_heading = "Query parameters", long, value_parser = ["CATALOG","PRIMARY","SECONDARY"])]
    _type: Option<String>,

    /// Filter results to only show recordsets that have a record with data
    /// matching the filter
    ///
    #[arg(help_heading = "Query parameters", long)]
    data: Option<String>,

    /// Filter results to only show zones that have a description matching the
    /// filter
    ///
    #[arg(help_heading = "Query parameters", long)]
    description: Option<String>,

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
    market: Option<String>,

    /// Filter results to only show zones that have a name matching the filter
    ///
    #[arg(help_heading = "Query parameters", long)]
    name: Option<String>,

    /// Sorts the response by the requested sort direction. A valid value is
    /// asc (ascending) or desc (descending). Default is asc. You can specify
    /// multiple pairs of sort key and sort direction query parameters. If you
    /// omit the sort direction in a pair, the API uses the natural sorting
    /// direction of the server attribute that is provided as the sort_key.
    ///
    #[arg(help_heading = "Query parameters", long, value_parser = ["asc","desc"])]
    sort_dir: Option<String>,

    /// Sorts the response by the this attribute value. Default is id. You can
    /// specify multiple pairs of sort key and sort direction query parameters.
    /// If you omit the sort direction in a pair, the API uses the natural
    /// sorting direction of the server attribute that is provided as the
    /// sort_key.
    ///
    #[arg(help_heading = "Query parameters", long, value_parser = ["created_at","id","name","serial","status","tenant_id","ttl","updated_at"])]
    sort_key: Option<String>,

    /// Filter results to only show zones that have a status matching the
    /// filter
    ///
    #[arg(help_heading = "Query parameters", long, value_parser = ["ACTIVE","DELETED","ERROR","PENDING","SUCCESS","ZONE"])]
    status: Option<String>,

    /// Filter results to only show zones that have a ttl matching the filter
    ///
    #[arg(help_heading = "Query parameters", long)]
    ttl: Option<i32>,

    /// ID for the zone
    ///
    #[arg(help_heading = "Query parameters", long)]
    zone_id: Option<String>,
}

/// Path parameters
#[derive(Args)]
struct PathParameters {}
/// Recordsets response representation
#[derive(Deserialize, Serialize, Clone, StructTable)]
struct ResponseData {
    /// current action in progress on the resource
    ///
    #[serde()]
    #[structable(optional, wide)]
    action: Option<String>,

    /// Date / Time when resource was created.
    ///
    #[serde()]
    #[structable(optional)]
    created_at: Option<String>,

    /// Description for this recordset
    ///
    #[serde()]
    #[structable(optional, wide)]
    description: Option<String>,

    /// ID for the resource
    ///
    #[serde()]
    #[structable(optional)]
    id: Option<String>,

    /// DNS Name for the recordset
    ///
    #[serde()]
    #[structable(optional)]
    name: Option<String>,

    /// ID for the project that owns the resource
    ///
    #[serde()]
    #[structable(optional, wide)]
    project_id: Option<String>,

    /// A list of data for this recordset. Each item will be a separate record
    /// in Designate These items should conform to the DNS spec for the record
    /// type - e.g. A records must be IPv4 addresses, CNAME records must be a
    /// hostname.
    ///
    #[serde()]
    #[structable(optional, pretty, wide)]
    records: Option<Value>,

    /// The status of the resource.
    ///
    #[serde()]
    #[structable(optional)]
    status: Option<String>,

    /// TTL (Time to Live) for the recordset.
    ///
    #[serde()]
    #[structable(optional, wide)]
    ttl: Option<i32>,

    /// They RRTYPE of the recordset.
    ///
    #[serde(rename = "type")]
    #[structable(optional, title = "type", wide)]
    _type: Option<String>,

    /// Date / Time when resource last updated.
    ///
    #[serde()]
    #[structable(optional)]
    updated_at: Option<String>,

    /// Version of the resource
    ///
    #[serde()]
    #[structable(optional, wide)]
    version: Option<i32>,

    /// ID for the zone that contains this recordset
    ///
    #[serde()]
    #[structable(optional, wide)]
    zone_id: Option<String>,

    /// The name of the zone that contains this recordset
    ///
    #[serde()]
    #[structable(optional, wide)]
    zone_name: Option<String>,
}

impl RecordsetsCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("List Recordsets");

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;

        let mut ep_builder = list::Request::builder();

        // Set path parameters
        // Set query parameters
        if let Some(val) = &self.query.zone_id {
            ep_builder.zone_id(val);
        }
        if let Some(val) = &self.query.limit {
            ep_builder.limit(*val);
        }
        if let Some(val) = &self.query.market {
            ep_builder.market(val);
        }
        if let Some(val) = &self.query.sort_dir {
            ep_builder.sort_dir(val);
        }
        if let Some(val) = &self.query.sort_key {
            ep_builder.sort_key(val);
        }
        if let Some(val) = &self.query.name {
            ep_builder.name(val);
        }
        if let Some(val) = &self.query.description {
            ep_builder.description(val);
        }
        if let Some(val) = &self.query._type {
            ep_builder._type(val);
        }
        if let Some(val) = &self.query.status {
            ep_builder.status(val);
        }
        if let Some(val) = &self.query.ttl {
            ep_builder.ttl(*val);
        }
        if let Some(val) = &self.query.data {
            ep_builder.data(val);
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
