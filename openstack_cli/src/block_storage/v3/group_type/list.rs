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

//! List GroupTypes command
//!
//! Wraps invoking of the `v3/group_types` with `GET` method

use clap::Args;
use serde::{Deserialize, Serialize};
use tracing::info;

use openstack_sdk::AsyncOpenStack;

use crate::output::OutputProcessor;
use crate::Cli;
use crate::OpenStackCliError;
use crate::OutputConfig;
use crate::StructTable;

use openstack_sdk::api::block_storage::v3::group_type::list;
use openstack_sdk::api::QueryAsync;
use openstack_sdk::api::{paged, Pagination};
use serde_json::Value;
use structable_derive::StructTable;

/// Returns the list of group types.
///
#[derive(Args)]
pub struct GroupTypesCommand {
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
    /// Shows details for all project. Admin only.
    ///
    #[arg(action=clap::ArgAction::Set, help_heading = "Query parameters", long)]
    all_tenants: Option<bool>,

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

    /// Used in conjunction with limit to return a slice of items. offset is
    /// where to start in the list.
    ///
    #[arg(help_heading = "Query parameters", long)]
    offset: Option<i32>,

    /// Comma-separated list of sort keys and optional sort directions in the
    /// form of < key > [: < direction > ]. A valid direction is asc
    /// (ascending) or desc (descending).
    ///
    #[arg(help_heading = "Query parameters", long)]
    sort: Option<String>,

    /// Sorts by one or more sets of attribute and sort direction combinations.
    /// If you omit the sort direction in a set, default is desc. Deprecated in
    /// favour of the combined sort parameter.
    ///
    #[arg(help_heading = "Query parameters", long)]
    sort_dir: Option<String>,

    /// Sorts by an attribute. A valid value is name, status, container_format,
    /// disk_format, size, id, created_at, or updated_at. Default is
    /// created_at. The API uses the natural sorting direction of the sort_key
    /// attribute value. Deprecated in favour of the combined sort parameter.
    ///
    #[arg(help_heading = "Query parameters", long)]
    sort_key: Option<String>,
}

/// Path parameters
#[derive(Args)]
struct PathParameters {}
/// GroupTypes response representation
#[derive(Deserialize, Serialize, Clone, StructTable)]
struct ResponseData {
    /// The group type description.
    ///
    #[serde()]
    #[structable(optional, wide)]
    description: Option<String>,

    /// A set of key and value pairs that contains the specifications for a
    /// group type.
    ///
    #[serde()]
    #[structable(optional, pretty, wide)]
    group_specs: Option<Value>,

    /// The group type ID.
    ///
    #[serde()]
    #[structable()]
    id: String,

    /// Whether the group type is publicly visible.
    ///
    #[serde()]
    #[structable(optional, wide)]
    is_public: Option<bool>,

    /// The group type name.
    ///
    #[serde()]
    #[structable(optional)]
    name: Option<String>,
}

impl GroupTypesCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("List GroupTypes");

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;

        let mut ep_builder = list::Request::builder();

        // Set path parameters
        // Set query parameters
        if let Some(val) = &self.query.all_tenants {
            ep_builder.all_tenants(*val);
        }
        if let Some(val) = &self.query.sort {
            ep_builder.sort(val);
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
        if let Some(val) = &self.query.offset {
            ep_builder.offset(*val);
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
