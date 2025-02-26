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

//! List Groups command
//!
//! Wraps invoking of the `v3/groups/detail` with `GET` method

use clap::Args;
use serde::{Deserialize, Serialize};
use tracing::info;

use openstack_sdk::AsyncOpenStack;

use crate::output::OutputProcessor;
use crate::Cli;
use crate::OpenStackCliError;
use crate::OutputConfig;
use crate::StructTable;

use openstack_sdk::api::block_storage::v3::group::list_detailed;
use openstack_sdk::api::QueryAsync;
use openstack_sdk::api::{paged, Pagination};
use serde_json::Value;
use structable_derive::StructTable;

/// Returns a detailed list of groups.
///
#[derive(Args)]
pub struct GroupsCommand {
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
/// Groups response representation
#[derive(Deserialize, Serialize, Clone, StructTable)]
struct ResponseData {
    /// The name of the availability zone.
    ///
    #[serde()]
    #[structable(optional, wide)]
    availability_zone: Option<String>,

    /// The date and time when the resource was created.
    ///
    /// The date and time stamp format is
    /// [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601):
    ///
    /// ```text
    /// CCYY-MM-DDThh:mm:ss±hh:mm
    ///
    /// ```
    ///
    /// For example, `2015-08-27T09:49:58-05:00`.
    ///
    /// The `±hh:mm` value, if included, is the time zone as an offset from
    /// UTC.
    ///
    #[serde()]
    #[structable(optional)]
    created_at: Option<String>,

    /// The group description.
    ///
    #[serde()]
    #[structable(optional, wide)]
    description: Option<String>,

    /// The ID of the group snapshot.
    ///
    #[serde()]
    #[structable(optional, wide)]
    group_snapshot_id: Option<String>,

    /// The group type ID.
    ///
    #[serde()]
    #[structable(optional, wide)]
    group_type: Option<String>,

    /// The UUID of the group.
    ///
    #[serde()]
    #[structable()]
    id: String,

    /// The name of the object.
    ///
    #[serde()]
    #[structable(optional)]
    name: Option<String>,

    /// The UUID of the volume group project.
    ///
    /// **New in version 3.58**
    ///
    #[serde()]
    #[structable(optional, wide)]
    project_id: Option<String>,

    /// The group replication status.
    ///
    /// **New in version 3.38**
    ///
    #[serde()]
    #[structable(optional, wide)]
    replication_status: Option<String>,

    /// The UUID of the source group.
    ///
    #[serde()]
    #[structable(optional, wide)]
    source_group_id: Option<String>,

    /// The status of the generic group.
    ///
    #[serde()]
    #[structable(optional)]
    status: Option<String>,

    /// The list of volume types. In an environment with multiple-storage back
    /// ends, the scheduler determines where to send the volume based on the
    /// volume type. For information about how to use volume types to create
    /// multiple- storage back ends, see
    /// [Configure multiple-storage back ends](https://docs.openstack.org/cinder/latest/admin/blockstorage-multi-backend.html).
    ///
    #[serde()]
    #[structable(optional, pretty, wide)]
    volume_types: Option<Value>,

    /// A list of `volume` ids, available only when `list_volume` set true.
    ///
    /// **New in version 3.25**
    ///
    #[serde()]
    #[structable(optional, pretty, wide)]
    volumes: Option<Value>,
}

impl GroupsCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("List Groups");

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;

        let mut ep_builder = list_detailed::Request::builder();

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
