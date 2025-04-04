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

//! List Volumes command
//!
//! Wraps invoking of the `v3/volumes/detail` with `GET` method

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
use openstack_sdk::api::block_storage::v3::volume::list_detailed;
use openstack_sdk::api::{Pagination, paged};
use serde_json::Value;
use structable_derive::StructTable;

/// Returns a detailed list of volumes.
///
#[derive(Args)]
pub struct VolumesCommand {
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

    /// Filters results by consumes_quota field. Resources that don’t use
    /// quotas are usually temporary internal resources created to perform an
    /// operation. Default is to not filter by it. Filtering by this option may
    /// not be always possible in a cloud, see List Resource Filters to
    /// determine whether this filter is available in your cloud.
    ///
    #[arg(action=clap::ArgAction::Set, help_heading = "Query parameters", long)]
    consumes_quota: Option<bool>,

    /// Filters reuslts by a time that resources are created at with time
    /// comparison operators: gt/gte/eq/neq/lt/lte.
    ///
    #[arg(help_heading = "Query parameters", long)]
    created_at: Option<String>,

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
    #[arg(help_heading = "Query parameters", long, value_parser = ["asc","desc"])]
    sort_dir: Option<String>,

    /// Sorts by an attribute. A valid value is name, status, container_format,
    /// disk_format, size, id, created_at, or updated_at. Default is
    /// created_at. The API uses the natural sorting direction of the sort_key
    /// attribute value. Deprecated in favour of the combined sort parameter.
    ///
    #[arg(help_heading = "Query parameters", long)]
    sort_key: Option<String>,

    /// Filters reuslts by a time that resources are updated at with time
    /// comparison operators: gt/gte/eq/neq/lt/lte.
    ///
    #[arg(help_heading = "Query parameters", long)]
    updated_at: Option<String>,

    /// Whether to show count in API response or not, default is False.
    ///
    #[arg(action=clap::ArgAction::Set, help_heading = "Query parameters", long)]
    with_count: Option<bool>,
}

/// Path parameters
#[derive(Args)]
struct PathParameters {}
/// Volumes response representation
#[derive(Deserialize, Serialize, Clone, StructTable)]
struct ResponseData {
    /// Instance attachment information. If this volume is attached to a server
    /// instance, the attachments list includes the UUID of the attached
    /// server, an attachment UUID, the name of the attached host, if any, the
    /// volume UUID, the device, and the device UUID. Otherwise, this list is
    /// empty. For example:
    ///
    /// ```text
    /// [
    ///   {
    ///     'server_id': '6c8cf6e0-4c8f-442f-9196-9679737feec6',
    ///     'attachment_id': '3dafcac4-1cb9-4b60-a227-d729baa10cf6',
    ///     'attached_at': '2019-09-30T19:30:34.000000',
    ///     'host_name': null,
    ///     'volume_id': '5d95d5ee-4bdd-4452-b9d7-d44ca10d3d53',
    ///     'device': '/dev/vda',
    ///     'id': '5d95d5ee-4bdd-4452-b9d7-d44ca10d3d53'
    ///   }
    /// ]
    ///
    /// ```
    ///
    #[serde()]
    #[structable(pretty, wide)]
    attachments: Value,

    /// The name of the availability zone.
    ///
    #[serde()]
    #[structable(optional, wide)]
    availability_zone: Option<String>,

    /// Enables or disables the bootable attribute. You can boot an instance
    /// from a bootable volume.
    ///
    #[serde()]
    #[structable(wide)]
    bootable: String,

    /// The cluster name of volume backend.
    ///
    /// **New in version 3.61**
    ///
    #[serde()]
    #[structable(optional, wide)]
    cluster_name: Option<String>,

    /// The UUID of the consistency group.
    ///
    #[serde()]
    #[structable(optional, wide)]
    consistencygroup_id: Option<String>,

    /// Whether this resource consumes quota or not. Resources that not counted
    /// for quota usage are usually temporary internal resources created to
    /// perform an operation.
    ///
    /// **New in version 3.65**
    ///
    #[serde()]
    #[structable(optional, wide)]
    consumes_quota: Option<bool>,

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

    /// The volume description.
    ///
    #[serde()]
    #[structable(optional, wide)]
    description: Option<String>,

    /// If true, this volume is encrypted.
    ///
    #[serde()]
    #[structable(wide)]
    encrypted: bool,

    /// The ID of the group.
    ///
    /// **New in version 3.13**
    ///
    #[serde()]
    #[structable(optional, wide)]
    group_id: Option<String>,

    /// The UUID of the volume.
    ///
    #[serde()]
    #[structable()]
    id: String,

    /// A `metadata` object. Contains one or more metadata key and value pairs
    /// that are associated with the volume.
    ///
    #[serde()]
    #[structable(optional, pretty, wide)]
    metadata: Option<Value>,

    /// The volume migration status. Admin only.
    ///
    #[serde()]
    #[structable(wide)]
    migration_status: String,

    /// If true, this volume can attach to more than one instance.
    ///
    #[serde()]
    #[structable(optional, wide)]
    multiattach: Option<bool>,

    /// The volume name.
    ///
    #[serde()]
    #[structable(optional)]
    name: Option<String>,

    /// The provider ID for the volume. The value is either a string set by the
    /// driver or `null` if the driver doesn’t use the field or if it hasn’t
    /// created it yet. Only returned for administrators.
    ///
    /// **New in version 3.21**
    ///
    #[serde()]
    #[structable(optional, wide)]
    provider_id: Option<String>,

    /// The volume replication status.
    ///
    #[serde()]
    #[structable(wide)]
    replication_status: String,

    /// A unique identifier that’s used to indicate what node the
    /// volume-service for a particular volume is being serviced by.
    ///
    /// **New in version 3.48**
    ///
    #[serde()]
    #[structable(optional, wide)]
    service_uuid: Option<String>,

    /// An indicator whether the host connecting the volume should lock for the
    /// whole attach/detach process or not. `true` means only is iSCSI
    /// initiator running on host doesn’t support manual scans, `false` means
    /// never use locks, and `null` means to always use locks. Look at
    /// os-brick’s `guard_connection` context manager. Default=True.
    ///
    /// **New in version 3.69**
    ///
    #[serde()]
    #[structable(optional, wide)]
    shared_targets: Option<bool>,

    /// The size of the volume, in gibibytes (GiB).
    ///
    #[serde()]
    #[structable(wide)]
    size: i64,

    /// To create a volume from an existing snapshot, specify the UUID of the
    /// volume snapshot. The volume is created in same availability zone and
    /// with same size as the snapshot.
    ///
    #[serde()]
    #[structable(optional, wide)]
    snapshot_id: Option<String>,

    /// The UUID of the source volume. The API creates a new volume with the
    /// same size as the source volume unless a larger size is requested.
    ///
    #[serde()]
    #[structable(optional, wide)]
    source_volid: Option<String>,

    /// The volume status.
    ///
    #[serde()]
    #[structable()]
    status: String,

    /// The date and time when the resource was updated.
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
    /// UTC. In the previous example, the offset value is `-05:00`.
    ///
    /// If the `updated_at` date and time stamp is not set, its value is
    /// `null`.
    ///
    #[serde()]
    #[structable(optional)]
    updated_at: Option<String>,

    /// The UUID of the user.
    ///
    #[serde()]
    #[structable(wide)]
    user_id: String,

    /// The associated volume type name for the volume.
    ///
    #[serde()]
    #[structable(optional, wide)]
    volume_type: Option<String>,

    /// The associated volume type ID for the volume.
    ///
    /// **New in version 3.63**
    ///
    #[serde()]
    #[structable(optional, wide)]
    volume_type_id: Option<String>,
}

impl VolumesCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("List Volumes");

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
        if let Some(val) = &self.query.offset {
            ep_builder.offset(*val);
        }
        if let Some(val) = &self.query.marker {
            ep_builder.marker(val);
        }
        if let Some(val) = &self.query.with_count {
            ep_builder.with_count(*val);
        }
        if let Some(val) = &self.query.created_at {
            ep_builder.created_at(val);
        }
        if let Some(val) = &self.query.updated_at {
            ep_builder.updated_at(val);
        }
        if let Some(val) = &self.query.consumes_quota {
            ep_builder.consumes_quota(*val);
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
