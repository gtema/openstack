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
use derive_builder::Builder;
use eyre::{Report, Result, WrapErr};
use serde::{Deserialize, Serialize};
use std::fmt;
use tokio::sync::mpsc::UnboundedSender;

use crate::action::Action;
use crate::cloud_worker::common::CloudWorkerError;
use crate::cloud_worker::types::{ApiRequest, ExecuteApiRequest};

use crate::utils::OutputConfig;
use crate::utils::StructTable;
use openstack_sdk::api::block_storage::v3::volume::list_detailed::RequestBuilder;
use openstack_sdk::api::{paged, Pagination};
use openstack_sdk::{api::QueryAsync, AsyncOpenStack};
use serde_json::Value;
use structable_derive::StructTable;

#[derive(Builder, Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[builder(setter(strip_option))]
pub struct BlockStorageVolumeList {
    #[builder(default)]
    pub all_tenants: Option<bool>,
    #[builder(default)]
    pub consumes_quota: Option<bool>,
    #[builder(default)]
    pub created_at: Option<String>,
    #[builder(default)]
    pub limit: Option<i32>,
    #[builder(default)]
    pub marker: Option<String>,
    #[builder(default)]
    pub offset: Option<i32>,
    #[builder(default)]
    pub sort: Option<String>,
    #[builder(default)]
    pub sort_dir: Option<String>,
    #[builder(default)]
    pub sort_key: Option<String>,
    #[builder(default)]
    pub updated_at: Option<String>,
    #[builder(default)]
    pub with_count: Option<bool>,
}

impl fmt::Display for BlockStorageVolumeList {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let parts: Vec<String> = Vec::new();
        write!(f, "{}", parts.join(","))
    }
}

impl TryFrom<&BlockStorageVolumeList> for RequestBuilder<'_> {
    type Error = Report;
    fn try_from(value: &BlockStorageVolumeList) -> Result<Self, Self::Error> {
        let mut ep_builder = Self::default();
        if let Some(val) = &value.all_tenants {
            ep_builder.all_tenants(*val);
        }
        if let Some(val) = &value.sort {
            ep_builder.sort(val.clone());
        }
        if let Some(val) = &value.sort_key {
            ep_builder.sort_key(val.clone());
        }
        if let Some(val) = &value.sort_dir {
            ep_builder.sort_dir(val.clone());
        }
        if let Some(val) = &value.limit {
            ep_builder.limit(*val);
        }
        if let Some(val) = &value.offset {
            ep_builder.offset(*val);
        }
        if let Some(val) = &value.marker {
            ep_builder.marker(val.clone());
        }
        if let Some(val) = &value.with_count {
            ep_builder.with_count(*val);
        }
        if let Some(val) = &value.created_at {
            ep_builder.created_at(val.clone());
        }
        if let Some(val) = &value.updated_at {
            ep_builder.updated_at(val.clone());
        }
        if let Some(val) = &value.consumes_quota {
            ep_builder.consumes_quota(*val);
        }

        Ok(ep_builder)
    }
}

impl ExecuteApiRequest for BlockStorageVolumeList {
    async fn execute_request(
        &self,
        session: &mut AsyncOpenStack,
        request: &ApiRequest,
        app_tx: &UnboundedSender<Action>,
    ) -> Result<(), CloudWorkerError> {
        let ep = TryInto::<RequestBuilder>::try_into(self)?
            .build()
            .wrap_err("Cannot prepare request")?;
        app_tx.send(Action::ApiResponsesData {
            request: request.clone(),
            data: paged(ep, Pagination::All).query_async(session).await?,
        })?;
        Ok(())
    }
}
/// BlockStorageVolume response representation
#[derive(Deserialize, Serialize, Clone, StructTable)]
struct BlockStorageVolume {
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
    #[serde(default)]
    #[structable(optional, title = "ATTACHMENTS", wide)]
    attachments: Option<Value>,

    /// The name of the availability zone.
    ///
    #[serde(default)]
    #[structable(optional, title = "AVAILABILITY_ZONE", wide)]
    availability_zone: Option<String>,

    /// The cluster name of volume backend.
    ///
    /// **New in version 3.61**
    ///
    #[serde(default)]
    #[structable(optional, title = "CLUSTER_NAME", wide)]
    cluster_name: Option<String>,

    /// The UUID of the consistency group.
    ///
    #[serde(default)]
    #[structable(optional, title = "CONSISTENCYGROUP_ID", wide)]
    consistencygroup_id: Option<String>,

    /// Whether this resource consumes quota or not. Resources that not counted
    /// for quota usage are usually temporary internal resources created to
    /// perform an operation.
    ///
    /// **New in version 3.65**
    ///
    #[serde(default)]
    #[structable(optional, title = "CONSUMES_QUOTA", wide)]
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
    #[serde(default)]
    #[structable(optional, title = "CREATED_AT")]
    created_at: Option<String>,

    /// The volume description.
    ///
    #[serde(default)]
    #[structable(optional, title = "DESCRIPTION", wide)]
    description: Option<String>,

    /// If true, this volume is encrypted.
    ///
    #[serde(default)]
    #[structable(optional, title = "ENCRYPTED", wide)]
    encrypted: Option<bool>,

    /// The ID of the group.
    ///
    /// **New in version 3.13**
    ///
    #[serde(default)]
    #[structable(optional, title = "GROUP_ID", wide)]
    group_id: Option<String>,

    /// The UUID of the volume.
    ///
    #[serde(default)]
    #[structable(optional, title = "ID", wide)]
    id: Option<String>,

    /// A `metadata` object. Contains one or more metadata key and value pairs
    /// that are associated with the volume.
    ///
    #[serde(default)]
    #[structable(optional, title = "METADATA", wide)]
    metadata: Option<Value>,

    /// The volume migration status. Admin only.
    ///
    #[serde(default)]
    #[structable(optional, title = "MIGRATION_STATUS", wide)]
    migration_status: Option<String>,

    /// If true, this volume can attach to more than one instance.
    ///
    #[serde(default)]
    #[structable(optional, title = "MULTIATTACH", wide)]
    multiattach: Option<bool>,

    /// The volume name.
    ///
    #[serde(default)]
    #[structable(optional, title = "NAME")]
    name: Option<String>,

    /// The provider ID for the volume. The value is either a string set by the
    /// driver or `null` if the driver doesn’t use the field or if it hasn’t
    /// created it yet. Only returned for administrators.
    ///
    /// **New in version 3.21**
    ///
    #[serde(default)]
    #[structable(optional, title = "PROVIDER_ID", wide)]
    provider_id: Option<String>,

    /// The volume replication status.
    ///
    #[serde(default)]
    #[structable(optional, title = "REPLICATION_STATUS", wide)]
    replication_status: Option<String>,

    /// A unique identifier that’s used to indicate what node the
    /// volume-service for a particular volume is being serviced by.
    ///
    /// **New in version 3.48**
    ///
    #[serde(default)]
    #[structable(optional, title = "SERVICE_UUID", wide)]
    service_uuid: Option<String>,

    /// An indicator whether the host connecting the volume should lock for the
    /// whole attach/detach process or not. `true` means only is iSCSI
    /// initiator running on host doesn’t support manual scans, `false` means
    /// never use locks, and `null` means to always use locks. Look at
    /// os-brick’s `guard_connection` context manager. Default=True.
    ///
    /// **New in version 3.69**
    ///
    #[serde(default)]
    #[structable(optional, title = "SHARED_TARGETS", wide)]
    shared_targets: Option<bool>,

    /// The size of the volume, in gibibytes (GiB).
    ///
    #[serde(default)]
    #[structable(optional, title = "SIZE", wide)]
    size: Option<i64>,

    /// To create a volume from an existing snapshot, specify the UUID of the
    /// volume snapshot. The volume is created in same availability zone and
    /// with same size as the snapshot.
    ///
    #[serde(default)]
    #[structable(optional, title = "SNAPSHOT_ID", wide)]
    snapshot_id: Option<String>,

    /// The UUID of the source volume. The API creates a new volume with the
    /// same size as the source volume unless a larger size is requested.
    ///
    #[serde(default)]
    #[structable(optional, title = "SOURCE_VOLID", wide)]
    source_volid: Option<String>,

    /// The volume status.
    ///
    #[serde(default)]
    #[structable(optional, title = "STATUS")]
    status: Option<String>,

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
    #[serde(default)]
    #[structable(optional, title = "UPDATED_AT")]
    updated_at: Option<String>,

    /// The UUID of the user.
    ///
    #[serde(default)]
    #[structable(optional, title = "USER_ID", wide)]
    user_id: Option<String>,

    /// The associated volume type name for the volume.
    ///
    #[serde(default)]
    #[structable(optional, title = "VOLUME_TYPE", wide)]
    volume_type: Option<String>,

    /// The associated volume type ID for the volume.
    ///
    /// **New in version 3.63**
    ///
    #[serde(default)]
    #[structable(optional, title = "VOLUME_TYPE_ID", wide)]
    volume_type_id: Option<String>,
}
