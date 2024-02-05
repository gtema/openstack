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

//! Create Volume command [microversion = 3.47]
//!
//! Wraps invoking of the `v3/volumes` with `POST` method

use clap::Args;
use serde::{Deserialize, Serialize};
use tracing::info;

use anyhow::Result;

use openstack_sdk::AsyncOpenStack;

use crate::output::OutputProcessor;
use crate::Cli;
use crate::OpenStackCliError;
use crate::OutputConfig;
use crate::StructTable;

use crate::common::parse_key_val;
use openstack_sdk::api::block_storage::v3::volume::create_347;
use openstack_sdk::api::QueryAsync;
use serde_json::Value;

use std::collections::HashMap;
use std::fmt;
use structable_derive::StructTable;

/// Creates a new volume.
///
/// :param req: the request
/// :param body: the request body
/// :returns: dict -- the new volume dictionary
/// :raises HTTPNotFound, HTTPBadRequest:
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
    #[arg(long, value_name="key=value", value_parser=parse_key_val::<String, Value>)]
    os_sch_hnt_scheduler_hints: Option<Vec<(String, Value)>>,
}

/// Query parameters
#[derive(Args)]
pub struct QueryParameters {}

/// Path parameters
#[derive(Args)]
pub struct PathParameters {}
/// Volume Body data
#[derive(Args)]
struct Volume {
    /// The volume name.
    #[arg(long)]
    name: Option<String>,

    /// The volume description.
    #[arg(long)]
    description: Option<String>,

    #[arg(long)]
    display_name: Option<String>,

    #[arg(long)]
    display_description: Option<String>,

    /// The volume type (either name or ID). To create an environment with
    /// multiple-storage back ends, you must specify a volume type. Block
    /// Storage volume back ends are spawned as children to `cinder-
    /// volume`, and they are keyed from a unique queue. They are named
    /// `cinder- volume.HOST.BACKEND`. For example, `cinder-
    /// volume.ubuntu.lvmdriver`. When a volume is created, the scheduler
    /// chooses an appropriate back end to handle the request based on the
    /// volume type. Default is `None`. For information about how to
    /// use volume types to create multiple- storage back ends, see
    /// [Configure multiple-storage back
    /// ends](https://docs.openstack.org/cinder/latest/admin/blockstorage-
    /// multi-backend.html).
    #[arg(long)]
    volume_type: Option<String>,

    /// One or more metadata key and value pairs to be associated
    /// with the new volume.
    #[arg(long, value_name="key=value", value_parser=parse_key_val::<String, String>)]
    metadata: Option<Vec<(String, String)>>,

    /// The UUID of the consistency group.
    #[arg(long)]
    snapshot_id: Option<String>,

    /// The UUID of the consistency group.
    #[arg(long)]
    source_volid: Option<String>,

    /// The UUID of the consistency group.
    #[arg(long)]
    consistencygroup_id: Option<String>,

    /// The size of the volume, in gibibytes (GiB).
    #[arg(long)]
    size: Option<Option<i32>>,

    /// The name of the availability zone.
    #[arg(long)]
    availability_zone: Option<String>,

    /// To enable this volume to attach to more than one
    /// server, set this value to `true`. Default is `false`.
    /// Note that support for multiattach volumes depends on the volume
    /// type being used. See [valid boolean values](#valid-boolean-values)
    #[arg(action=clap::ArgAction::Set, long)]
    multiattach: Option<Option<bool>>,

    #[arg(long)]
    image_id: Option<String>,

    /// The UUID of the image from which you want to
    /// create the volume. Required to create a bootable volume.
    #[arg(long)]
    image_ref: Option<String>,

    #[arg(long)]
    group_id: Option<String>,

    /// The UUID of the backup.
    ///
    ///
    /// **New in version 3.47**
    #[arg(long)]
    backup_id: Option<String>,
}

/// Volume response representation
#[derive(Deserialize, Serialize, Clone, StructTable)]
pub struct ResponseData {
    /// The volume name.
    #[serde()]
    #[structable(optional)]
    name: Option<String>,

    /// The volume description.
    #[serde()]
    #[structable(optional)]
    description: Option<String>,

    /// The associated volume type name for the volume.
    #[serde()]
    #[structable(optional)]
    volume_type: Option<String>,

    /// A `metadata` object. Contains one or more
    /// metadata key and value pairs that are associated with the volume.
    #[serde()]
    #[structable(optional)]
    metadata: Option<HashMapStringString>,

    /// To create a volume from an existing snapshot,
    /// specify the UUID of the volume snapshot. The volume is created in
    /// same availability zone and with same size as the snapshot.
    #[serde()]
    #[structable(optional)]
    snapshot_id: Option<String>,

    /// The UUID of the source volume. The API creates a new volume with the
    /// same
    /// size as the source volume unless a larger size is requested.
    #[serde()]
    #[structable(optional)]
    source_volid: Option<String>,

    /// The UUID of the consistency group.
    #[serde()]
    #[structable(optional)]
    consistencygroup_id: Option<String>,

    /// The size of the volume, in gibibytes (GiB).
    #[serde()]
    #[structable(optional)]
    size: Option<i64>,

    /// The name of the availability zone.
    #[serde()]
    #[structable(optional)]
    availability_zone: Option<String>,

    /// If true, this volume can attach to more than one
    /// instance.
    #[serde()]
    #[structable(optional)]
    multiattach: Option<bool>,

    /// The volume status.
    #[serde()]
    #[structable(optional)]
    status: Option<String>,

    /// The volume migration status. Admin only.
    #[serde()]
    #[structable(optional)]
    migration_status: Option<String>,

    /// Instance attachment information. If this volume
    /// is attached to a server instance, the attachments list includes
    /// the UUID of the attached server, an attachment UUID, the name of
    /// the attached host, if any, the volume UUID, the device, and the
    /// device UUID. Otherwise, this list is empty. For example:
    ///
    ///
    ///
    /// ```text
    /// [
    ///   {
    ///     'server\_id': '6c8cf6e0-4c8f-442f-9196-9679737feec6',
    ///     'attachment\_id': '3dafcac4-1cb9-4b60-a227-d729baa10cf6',
    ///     'attached\_at': '2019-09-30T19:30:34.000000',
    ///     'host\_name': null,
    ///     'volume\_id': '5d95d5ee-4bdd-4452-b9d7-d44ca10d3d53',
    ///     'device': '/dev/vda',
    ///     'id': '5d95d5ee-4bdd-4452-b9d7-d44ca10d3d53'
    ///   }
    /// ]
    ///
    /// ```
    #[serde()]
    #[structable(optional)]
    attachments: Option<VecResponseAttachments>,

    /// The volume links.
    #[serde()]
    #[structable(optional)]
    links: Option<Value>,

    /// If true, this volume is encrypted.
    #[serde()]
    #[structable(optional)]
    encrypted: Option<bool>,

    /// The date and time when the resource was created.
    ///
    ///
    /// The date and time stamp format is [ISO
    /// 8601](https://en.wikipedia.org/wiki/ISO_8601):
    ///
    ///
    ///
    /// ```text
    /// CCYY-MM-DDThh:mm:ss±hh:mm
    ///
    /// ```
    ///
    ///
    /// For example, `2015-08-27T09:49:58-05:00`.
    ///
    ///
    /// The `±hh:mm` value, if included, is the time zone as an offset
    /// from UTC.
    #[serde()]
    #[structable(optional)]
    created_at: Option<String>,

    /// The date and time when the resource was updated.
    ///
    ///
    /// The date and time stamp format is [ISO
    /// 8601](https://en.wikipedia.org/wiki/ISO_8601):
    ///
    ///
    ///
    /// ```text
    /// CCYY-MM-DDThh:mm:ss±hh:mm
    ///
    /// ```
    ///
    ///
    /// For example, `2015-08-27T09:49:58-05:00`.
    ///
    ///
    /// The `±hh:mm` value, if included, is the time zone as an offset
    /// from UTC. In the previous example, the offset value is `-05:00`.
    ///
    ///
    /// If the `updated\_at` date and time stamp is not set, its value is
    /// `null`.
    #[serde()]
    #[structable(optional)]
    updated_at: Option<String>,

    /// The volume replication status.
    #[serde()]
    #[structable(optional)]
    replication_status: Option<String>,

    /// The UUID of the volume.
    #[serde()]
    #[structable(optional)]
    id: Option<String>,

    /// The UUID of the user.
    #[serde()]
    #[structable(optional)]
    user_id: Option<String>,

    /// The associated volume type ID for the volume.
    #[serde()]
    #[structable(optional)]
    volume_type_id: Option<String>,

    /// The ID of the group.
    #[serde()]
    #[structable(optional)]
    group_id: Option<String>,

    /// The provider ID for the volume. The value is either a string set by the
    /// driver or null if the driver doesn’t use the field or if it hasn’t
    /// created it yet. Only returned for administrators.
    #[serde()]
    #[structable(optional)]
    provider_id: Option<String>,

    /// A unique identifier that’s used to indicate what node the volume-
    /// service for a particular volume is being serviced by.
    #[serde()]
    #[structable(optional)]
    service_uuid: Option<String>,

    /// An indicator whether the host connecting the volume should lock for the
    /// whole attach/detach process or not. true means only is iSCSI initiator
    /// running on host doesn’t support manual scans, false means never use
    /// locks, and null means to always use locks. Look at os-brick’s
    /// guard_connection context manager. Default=True.
    #[serde()]
    #[structable(optional)]
    shared_targets: Option<bool>,

    /// The cluster name of volume backend.
    #[serde()]
    #[structable(optional)]
    cluster_name: Option<String>,

    /// Whether this resource consumes quota or not. Resources that not counted
    /// for quota usage are usually temporary internal resources created to
    /// perform an operation.
    #[serde()]
    #[structable(optional)]
    consumes_quota: Option<bool>,
}
/// HashMap of String response type
#[derive(Default, Clone, Deserialize, Serialize)]
pub struct HashMapStringString(HashMap<String, String>);
impl fmt::Display for HashMapStringString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{{{}}}",
            self.0
                .iter()
                .map(|v| format!("{}={}", v.0, v.1))
                .collect::<Vec<String>>()
                .join("\n")
        )
    }
}
/// struct response type
#[derive(Default, Clone, Deserialize, Serialize)]
struct ResponseAttachments {
    server_id: Option<String>,
    attachment_id: Option<String>,
    attached_at: Option<String>,
    host_name: Option<String>,
    volume_id: Option<String>,
    device: Option<String>,
    id: Option<String>,
}

impl fmt::Display for ResponseAttachments {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let data = Vec::from([
            format!(
                "server_id={}",
                self.server_id
                    .clone()
                    .map(|v| v.to_string())
                    .unwrap_or("".to_string())
            ),
            format!(
                "attachment_id={}",
                self.attachment_id
                    .clone()
                    .map(|v| v.to_string())
                    .unwrap_or("".to_string())
            ),
            format!(
                "attached_at={}",
                self.attached_at
                    .clone()
                    .map(|v| v.to_string())
                    .unwrap_or("".to_string())
            ),
            format!(
                "host_name={}",
                self.host_name
                    .clone()
                    .map(|v| v.to_string())
                    .unwrap_or("".to_string())
            ),
            format!(
                "volume_id={}",
                self.volume_id
                    .clone()
                    .map(|v| v.to_string())
                    .unwrap_or("".to_string())
            ),
            format!(
                "device={}",
                self.device
                    .clone()
                    .map(|v| v.to_string())
                    .unwrap_or("".to_string())
            ),
            format!(
                "id={}",
                self.id
                    .clone()
                    .map(|v| v.to_string())
                    .unwrap_or("".to_string())
            ),
        ]);
        write!(f, "{}", data.join(";"))
    }
}
/// Vector of ResponseAttachments response type
#[derive(Default, Clone, Deserialize, Serialize)]
pub struct VecResponseAttachments(Vec<ResponseAttachments>);
impl fmt::Display for VecResponseAttachments {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[{}]",
            self.0
                .iter()
                .map(|v| v.to_string())
                .collect::<Vec<String>>()
                .join(",")
        )
    }
}
/// struct response type
#[derive(Default, Clone, Deserialize, Serialize)]
struct ResponseLinks {
    href: Option<String>,
    rel: Option<String>,
}

impl fmt::Display for ResponseLinks {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let data = Vec::from([
            format!(
                "href={}",
                self.href
                    .clone()
                    .map(|v| v.to_string())
                    .unwrap_or("".to_string())
            ),
            format!(
                "rel={}",
                self.rel
                    .clone()
                    .map(|v| v.to_string())
                    .unwrap_or("".to_string())
            ),
        ]);
        write!(f, "{}", data.join(";"))
    }
}

impl VolumeCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Create Volume");

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;

        let mut ep_builder = create_347::Request::builder();
        ep_builder.header("OpenStack-API-Version", "volume 3.47");

        // Set path parameters
        // Set query parameters
        // Set body parameters
        // Set Request.volume data
        let args = &self.volume;
        let mut volume_builder = create_347::VolumeBuilder::default();
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

        if let Some(val) = &args.volume_type {
            volume_builder.volume_type(Some(val.into()));
        }

        if let Some(val) = &args.metadata {
            volume_builder.metadata(val.iter().cloned());
        }

        if let Some(val) = &args.snapshot_id {
            volume_builder.snapshot_id(Some(val.into()));
        }

        if let Some(val) = &args.source_volid {
            volume_builder.source_volid(Some(val.into()));
        }

        if let Some(val) = &args.consistencygroup_id {
            volume_builder.consistencygroup_id(Some(val.into()));
        }

        if let Some(val) = &args.size {
            volume_builder.size(*val);
        }

        if let Some(val) = &args.availability_zone {
            volume_builder.availability_zone(Some(val.into()));
        }

        if let Some(val) = &args.multiattach {
            volume_builder.multiattach(*val);
        }

        if let Some(val) = &args.image_id {
            volume_builder.image_id(Some(val.into()));
        }

        if let Some(val) = &args.image_ref {
            volume_builder.image_ref(Some(val.into()));
        }

        if let Some(val) = &args.group_id {
            volume_builder.group_id(Some(val.into()));
        }

        if let Some(val) = &args.backup_id {
            volume_builder.backup_id(Some(val.into()));
        }

        ep_builder.volume(volume_builder.build().unwrap());

        // Set Request.os_sch_hnt_scheduler_hints data
        if let Some(args) = &self.os_sch_hnt_scheduler_hints {
            ep_builder.os_sch_hnt_scheduler_hints(args.iter().cloned());
        }

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;

        let data = ep.query_async(client).await?;
        op.output_single::<ResponseData>(data)?;
        Ok(())
    }
}
