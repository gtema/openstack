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

use eyre::Result;
use serde::{Deserialize, Serialize};
use std::fmt;

use crate::cloud_worker::common::ConfirmableRequest;

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BlockStorageBackupFilters {}

impl fmt::Display for BlockStorageBackupFilters {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "")
    }
}

impl TryFrom<&BlockStorageBackupFilters>
    for openstack_sdk::api::block_storage::v3::backup::list_detailed::RequestBuilder<'_>
{
    type Error = eyre::Report;

    fn try_from(_value: &BlockStorageBackupFilters) -> Result<Self, Self::Error> {
        let mut ep_builder =
            openstack_sdk::api::block_storage::v3::backup::list_detailed::Request::builder();

        // TODO(gtema) cinder rejects "name" in few clouds
        ep_builder.sort_key("created_at");
        Ok(ep_builder)
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BlockStorageSnapshotFilters {}

impl fmt::Display for BlockStorageSnapshotFilters {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "")
    }
}

impl TryFrom<&BlockStorageSnapshotFilters>
    for openstack_sdk::api::block_storage::v3::snapshot::list_detailed::RequestBuilder<'_>
{
    type Error = eyre::Report;

    fn try_from(_value: &BlockStorageSnapshotFilters) -> Result<Self, Self::Error> {
        let mut ep_builder =
            openstack_sdk::api::block_storage::v3::snapshot::list_detailed::Request::builder();

        // TODO(gtema) cinder rejects "name" in few clouds
        ep_builder.sort_key("created_at");
        Ok(ep_builder)
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BlockStorageVolumeFilters {}

impl fmt::Display for BlockStorageVolumeFilters {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "")
    }
}

impl TryFrom<&BlockStorageVolumeFilters>
    for openstack_sdk::api::block_storage::v3::volume::list_detailed::RequestBuilder<'_>
{
    type Error = eyre::Report;

    fn try_from(_value: &BlockStorageVolumeFilters) -> Result<Self, Self::Error> {
        let mut ep_builder =
            openstack_sdk::api::block_storage::v3::volume::list_detailed::Request::builder();

        ep_builder.sort_key("name");
        Ok(ep_builder)
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BlockStorageVolumeDelete {
    pub volume_id: String,
    pub volume_name: Option<String>,
}

impl fmt::Display for BlockStorageVolumeDelete {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "")
    }
}

impl ConfirmableRequest for BlockStorageVolumeDelete {
    fn get_confirm_message(&self) -> Option<String> {
        Some(format!(
            "Delete volume {} ?",
            self.volume_name.clone().unwrap_or(self.volume_id.clone())
        ))
    }
}
