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
use openstack_sdk::api::block_storage::v3::backup::list_detailed::RequestBuilder;
use openstack_sdk::api::{paged, Pagination};
use openstack_sdk::{api::QueryAsync, AsyncOpenStack};
use serde_json::Value;
use structable_derive::StructTable;

#[derive(Builder, Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[builder(setter(strip_option))]
pub struct BlockStorageBackupList {
    #[builder(default)]
    pub all_tenants: Option<bool>,
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
    pub with_count: Option<bool>,
}

impl fmt::Display for BlockStorageBackupList {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let parts: Vec<String> = Vec::new();
        write!(f, "{}", parts.join(","))
    }
}

impl TryFrom<&BlockStorageBackupList> for RequestBuilder<'_> {
    type Error = Report;
    fn try_from(value: &BlockStorageBackupList) -> Result<Self, Self::Error> {
        let mut ep_builder = Self::default();
        if let Some(val) = &value.all_tenants {
            ep_builder.all_tenants(*val);
        }
        if let Some(val) = &value.with_count {
            ep_builder.with_count(*val);
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
        if let Some(val) = &value.marker {
            ep_builder.marker(val.clone());
        }
        if let Some(val) = &value.offset {
            ep_builder.offset(*val);
        }

        Ok(ep_builder)
    }
}

impl ExecuteApiRequest for BlockStorageBackupList {
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
/// BlockStorageBackup response representation
#[derive(Deserialize, Serialize, Clone, StructTable)]
pub struct BlockStorageBackup {
    /// The name of the availability zone.
    ///
    #[serde(default)]
    #[structable(optional, title = "AVAILABILITY_ZONE", wide)]
    pub availability_zone: Option<String>,

    /// The container name or null.
    ///
    #[serde(default)]
    #[structable(optional, title = "CONTAINER", wide)]
    pub container: Option<String>,

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
    pub created_at: Option<String>,

    /// The time when the data on the volume was first saved. If it is a backup
    /// from volume, it will be the same as `created_at` for a backup. If it is
    /// a backup from a snapshot, it will be the same as `created_at` for the
    /// snapshot.
    ///
    #[serde(default)]
    #[structable(optional, title = "DATA_TIMESTAMP", wide)]
    pub data_timestamp: Option<String>,

    /// The backup description or null.
    ///
    #[serde(default)]
    #[structable(optional, title = "DESCRIPTION", wide)]
    pub description: Option<String>,

    /// If the backup failed, the reason for the failure. Otherwise, null.
    ///
    #[serde(default)]
    #[structable(optional, title = "FAIL_REASON", wide)]
    pub fail_reason: Option<String>,

    /// If this value is `true`, there are other backups depending on this
    /// backup.
    ///
    #[serde(default)]
    #[structable(optional, title = "HAS_DEPENDENT_BACKUPS", wide)]
    pub has_dependent_backups: Option<bool>,

    /// The UUID of the backup.
    ///
    #[serde()]
    #[structable(title = "ID", wide)]
    pub id: String,

    /// Indicates whether the backup mode is incremental. If this value is
    /// `true`, the backup mode is incremental. If this value is `false`, the
    /// backup mode is full.
    ///
    #[serde(default)]
    #[structable(optional, title = "IS_INCREMENTAL", wide)]
    pub is_incremental: Option<bool>,

    /// The backup metadata key value pairs.
    ///
    /// **New in version 3.43**
    ///
    #[serde(default)]
    #[structable(optional, title = "METADATA", wide)]
    pub metadata: Option<Value>,

    /// The backup name.
    ///
    #[serde(default)]
    #[structable(optional, title = "NAME")]
    pub name: Option<String>,

    /// The number of objects in the backup.
    ///
    #[serde(default)]
    #[structable(optional, title = "OBJECT_COUNT", wide)]
    pub object_count: Option<i32>,

    /// The size of the volume, in gibibytes (GiB).
    ///
    #[serde()]
    #[structable(title = "SIZE", wide)]
    pub size: i64,

    /// The UUID of the source volume snapshot.
    ///
    #[serde(default)]
    #[structable(optional, title = "SNAPSHOT_ID", wide)]
    pub snapshot_id: Option<String>,

    /// The backup status. Refer to Backup statuses table for the possible
    /// status value.
    ///
    #[serde()]
    #[structable(title = "STATUS")]
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
    #[serde(default)]
    #[structable(optional, title = "UPDATED_AT")]
    pub updated_at: Option<String>,

    /// The UUID of the volume.
    ///
    #[serde()]
    #[structable(title = "VOLUME_ID", wide)]
    pub volume_id: String,
}
