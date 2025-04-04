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

//! Set Volume command [microversion = 3.53]
//!
//! Wraps invoking of the `v3/volumes/{id}` with `PUT` method

use clap::Args;
use serde::{Deserialize, Serialize};
use tracing::info;

use openstack_sdk::AsyncOpenStack;

use crate::Cli;
use crate::OpenStackCliError;
use crate::OutputConfig;
use crate::StructTable;
use crate::output::OutputProcessor;

use crate::common::parse_key_val;
use openstack_sdk::api::QueryAsync;
use openstack_sdk::api::block_storage::v3::volume::find;
use openstack_sdk::api::block_storage::v3::volume::set_353;
use openstack_sdk::api::find;
use serde_json::Value;
use structable_derive::StructTable;

/// Update a volume.
///
#[derive(Args)]
pub struct VolumeCommand {
    /// Request Query parameters
    #[command(flatten)]
    query: QueryParameters,

    /// Path parameters
    #[command(flatten)]
    path: PathParameters,

    #[command(flatten)]
    volume: Volume,
}

/// Query parameters
#[derive(Args)]
struct QueryParameters {}

/// Path parameters
#[derive(Args)]
struct PathParameters {
    /// id parameter for /v3/volumes/{id} API
    ///
    #[arg(
        help_heading = "Path parameters",
        id = "path_param_id",
        value_name = "ID"
    )]
    id: String,
}
/// Volume Body data
#[derive(Args, Clone)]
struct Volume {
    #[arg(help_heading = "Body parameters", long)]
    description: Option<String>,

    #[arg(help_heading = "Body parameters", long)]
    display_description: Option<String>,

    #[arg(help_heading = "Body parameters", long)]
    display_name: Option<String>,

    #[arg(help_heading = "Body parameters", long, value_name="key=value", value_parser=parse_key_val::<String, String>)]
    metadata: Option<Vec<(String, String)>>,

    #[arg(help_heading = "Body parameters", long)]
    name: Option<String>,
}

/// Volume response representation
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
    #[structable(pretty)]
    attachments: Value,

    /// The name of the availability zone.
    ///
    #[serde()]
    #[structable(optional)]
    availability_zone: Option<String>,

    /// Enables or disables the bootable attribute. You can boot an instance
    /// from a bootable volume.
    ///
    #[serde()]
    #[structable()]
    bootable: String,

    /// The cluster name of volume backend.
    ///
    /// **New in version 3.61**
    ///
    #[serde()]
    #[structable(optional)]
    cluster_name: Option<String>,

    /// The UUID of the consistency group.
    ///
    #[serde()]
    #[structable(optional)]
    consistencygroup_id: Option<String>,

    /// Whether this resource consumes quota or not. Resources that not counted
    /// for quota usage are usually temporary internal resources created to
    /// perform an operation.
    ///
    /// **New in version 3.65**
    ///
    #[serde()]
    #[structable(optional)]
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
    #[structable(optional)]
    description: Option<String>,

    /// If true, this volume is encrypted.
    ///
    #[serde()]
    #[structable()]
    encrypted: bool,

    /// The ID of the group.
    ///
    /// **New in version 3.13**
    ///
    #[serde()]
    #[structable(optional)]
    group_id: Option<String>,

    /// The UUID of the volume.
    ///
    #[serde()]
    #[structable()]
    id: String,

    /// The volume links.
    ///
    #[serde()]
    #[structable(optional, pretty)]
    links: Option<Value>,

    /// A `metadata` object. Contains one or more metadata key and value pairs
    /// that are associated with the volume.
    ///
    #[serde()]
    #[structable(optional, pretty)]
    metadata: Option<Value>,

    /// The volume migration status. Admin only.
    ///
    #[serde()]
    #[structable()]
    migration_status: String,

    /// If true, this volume can attach to more than one instance.
    ///
    #[serde()]
    #[structable(optional)]
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
    #[structable(optional)]
    provider_id: Option<String>,

    /// The volume replication status.
    ///
    #[serde()]
    #[structable()]
    replication_status: String,

    /// A unique identifier that’s used to indicate what node the
    /// volume-service for a particular volume is being serviced by.
    ///
    /// **New in version 3.48**
    ///
    #[serde()]
    #[structable(optional)]
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
    #[structable(optional)]
    shared_targets: Option<bool>,

    /// The size of the volume, in gibibytes (GiB).
    ///
    #[serde()]
    #[structable()]
    size: i64,

    /// To create a volume from an existing snapshot, specify the UUID of the
    /// volume snapshot. The volume is created in same availability zone and
    /// with same size as the snapshot.
    ///
    #[serde()]
    #[structable(optional)]
    snapshot_id: Option<String>,

    /// The UUID of the source volume. The API creates a new volume with the
    /// same size as the source volume unless a larger size is requested.
    ///
    #[serde()]
    #[structable(optional)]
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
    #[structable()]
    user_id: String,

    /// The associated volume type name for the volume.
    ///
    #[serde()]
    #[structable(optional)]
    volume_type: Option<String>,

    /// The associated volume type ID for the volume.
    ///
    /// **New in version 3.63**
    ///
    #[serde()]
    #[structable(optional)]
    volume_type_id: Option<String>,
}

impl VolumeCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Set Volume");

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;

        let mut find_builder = find::Request::builder();

        find_builder.id(&self.path.id);
        find_builder.header("OpenStack-API-Version", "volume 3.53");
        let find_ep = find_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;
        let find_data: serde_json::Value = find(find_ep).query_async(client).await?;

        let mut ep_builder = set_353::Request::builder();
        ep_builder.header("OpenStack-API-Version", "volume 3.53");

        // Set path parameters
        let resource_id = find_data["id"]
            .as_str()
            .expect("Resource ID is a string")
            .to_string();
        ep_builder.id(resource_id.clone());
        // Set query parameters
        // Set body parameters
        // Set Request.volume data
        let args = &self.volume;
        let mut volume_builder = set_353::VolumeBuilder::default();
        if let Some(val) = &args.name {
            volume_builder.name(Some(val.into()));
        }

        if let Some(val) = &args.description {
            volume_builder.description(Some(val.into()));
        }

        if let Some(val) = &args.display_name {
            volume_builder.display_name(Some(val.into()));
        }

        if let Some(val) = &args.display_description {
            volume_builder.display_description(Some(val.into()));
        }

        if let Some(val) = &args.metadata {
            volume_builder.metadata(val.iter().cloned());
        }

        ep_builder.volume(volume_builder.build().unwrap());

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;

        let data = ep.query_async(client).await?;
        op.output_single::<ResponseData>(data)?;
        Ok(())
    }
}
