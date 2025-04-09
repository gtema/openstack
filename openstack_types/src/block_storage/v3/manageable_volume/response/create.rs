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
//! Response type for the post manageable_volumes operation

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// ManageableVolume response representation
#[derive(Clone, Deserialize, Serialize)]
pub struct ManageableVolumeResponse {
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
    pub attachments: Vec<Attachments>,

    /// The name of the availability zone.
    ///
    pub availability_zone: Option<String>,

    /// Enables or disables the bootable attribute. You can boot an instance
    /// from a bootable volume.
    ///
    pub bootable: String,

    /// The cluster name of volume backend.
    ///
    pub cluster_name: Option<String>,

    /// The UUID of the consistency group.
    ///
    pub consistencygroup_id: Option<String>,

    /// Whether this resource consumes quota or not. Resources that not counted
    /// for quota usage are usually temporary internal resources created to
    /// perform an operation.
    ///
    pub consumes_quota: Option<bool>,

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
    pub created_at: Option<String>,

    /// The volume description.
    ///
    pub description: Option<String>,

    /// If true, this volume is encrypted.
    ///
    pub encrypted: bool,

    /// The ID of the group.
    ///
    pub group_id: Option<String>,

    /// The UUID of the volume.
    ///
    pub id: String,

    /// The volume links.
    ///
    pub links: Option<Vec<Links>>,

    /// A `metadata` object. Contains one or more metadata key and value pairs
    /// that are associated with the volume.
    ///
    pub metadata: Option<HashMap<String, String>>,

    /// The volume migration status. Admin only.
    ///
    pub migration_status: String,

    /// If true, this volume can attach to more than one instance.
    ///
    pub multiattach: Option<bool>,

    /// The volume name.
    ///
    pub name: Option<String>,

    /// The provider ID for the volume. The value is either a string set by the
    /// driver or null if the driver doesn’t use the field or if it hasn’t
    /// created it yet. Only returned for administrators.
    ///
    pub provider_id: Option<String>,

    /// The volume replication status.
    ///
    pub replication_status: String,

    /// A unique identifier that’s used to indicate what node the
    /// volume-service for a particular volume is being serviced by.
    ///
    pub service_uuid: Option<String>,

    /// An indicator whether the host connecting the volume should lock for the
    /// whole attach/detach process or not. true means only is iSCSI initiator
    /// running on host doesn’t support manual scans, false means never use
    /// locks, and null means to always use locks. Look at os-brick’s
    /// guard_connection context manager. Default=True.
    ///
    pub shared_targets: Option<bool>,

    /// The size of the volume, in gibibytes (GiB).
    ///
    pub size: i64,

    /// To create a volume from an existing snapshot, specify the UUID of the
    /// volume snapshot. The volume is created in same availability zone and
    /// with same size as the snapshot.
    ///
    pub snapshot_id: Option<String>,

    /// The UUID of the source volume. The API creates a new volume with the
    /// same size as the source volume unless a larger size is requested.
    ///
    pub source_volid: Option<String>,

    /// The volume status.
    ///
    pub status: String,

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
    pub updated_at: Option<String>,

    /// The UUID of the user.
    ///
    pub user_id: String,

    /// A `volume_type` object.
    ///
    pub volume_type: Option<String>,

    /// The associated volume type ID for the volume.
    ///
    pub volume_type_id: Option<String>,
}

/// `Attachments` type
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Attachments {
    pub attached_at: Option<String>,
    pub attachment_id: String,
    pub device: Option<String>,
    pub host_name: Option<String>,
    pub id: String,
    pub server_id: Option<String>,
    pub volume_id: String,
}

/// Links to the resources in question. See
/// [API Guide / Links and References](https://docs.openstack.org/api-guide/compute/links_and_references.html)
/// for more info.
///
/// `Links` type
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Links {
    pub href: Option<String>,
    pub rel: Option<String>,
}
