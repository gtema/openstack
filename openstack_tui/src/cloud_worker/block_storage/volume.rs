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

use eyre::{Result, WrapErr};
use serde::{Deserialize, Serialize};
use std::fmt;
use tokio::sync::mpsc::UnboundedSender;

use openstack_sdk::{
    api::{paged, Pagination, QueryAsync},
    AsyncOpenStack,
};

use crate::action::Action;
use crate::cloud_worker::block_storage::types::BlockStorageApiRequest;
use crate::cloud_worker::common::CloudWorkerError;
use crate::cloud_worker::types::{ApiRequest, ExecuteApiRequest};
use crate::cloud_worker::ConfirmableRequest;

/// Volume API operations
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum BlockStorageVolumeApiRequest {
    /// Delete
    Delete(BlockStorageVolumeDelete),
    /// List
    List(BlockStorageVolumeList),
}

impl From<BlockStorageVolumeApiRequest> for ApiRequest {
    fn from(item: BlockStorageVolumeApiRequest) -> Self {
        ApiRequest::BlockStorage(BlockStorageApiRequest::from(item))
    }
}

impl ConfirmableRequest for BlockStorageVolumeApiRequest {
    fn get_confirm_message(&self) -> Option<String> {
        match &self {
            BlockStorageVolumeApiRequest::Delete(req) => req.get_confirm_message(),
            _ => None,
        }
    }
}

impl ExecuteApiRequest for BlockStorageVolumeApiRequest {
    async fn execute_request(
        &self,
        session: &mut AsyncOpenStack,
        request: &ApiRequest,
        app_tx: &UnboundedSender<Action>,
    ) -> Result<(), CloudWorkerError> {
        match self {
            BlockStorageVolumeApiRequest::Delete(ref filters) => {
                let ep = TryInto::<
                    openstack_sdk::api::block_storage::v3::volume::delete::Request<'_>,
                >::try_into(filters)
                .wrap_err("Cannot prepare request")?;
                openstack_sdk::api::ignore(ep).query_async(session).await?;
                app_tx.send(Action::Refresh)?;
            }
            BlockStorageVolumeApiRequest::List(ref filters) => {
                let ep = TryInto::<
                    openstack_sdk::api::block_storage::v3::volume::list_detailed::Request<'_>,
                >::try_into(filters)
                .wrap_err("Cannot prepare request")?;
                app_tx.send(Action::ApiResponsesData {
                    request: request.clone(),
                    data: paged(ep, Pagination::All).query_async(session).await?,
                })?;
            }
        }
        Ok(())
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BlockStorageVolumeList {}

impl fmt::Display for BlockStorageVolumeList {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "")
    }
}

impl TryFrom<&BlockStorageVolumeList>
    for openstack_sdk::api::block_storage::v3::volume::list_detailed::Request<'_>
{
    type Error = openstack_sdk::api::block_storage::v3::volume::list_detailed::RequestBuilderError;

    fn try_from(_value: &BlockStorageVolumeList) -> Result<Self, Self::Error> {
        let mut ep_builder = Self::builder();

        // TODO(gtema) cinder rejects "name" in few clouds
        ep_builder.sort_key("created_at");

        ep_builder.build()
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

impl TryFrom<&BlockStorageVolumeDelete>
    for openstack_sdk::api::block_storage::v3::volume::delete::Request<'_>
{
    type Error = openstack_sdk::api::block_storage::v3::volume::delete::RequestBuilderError;

    fn try_from(value: &BlockStorageVolumeDelete) -> Result<Self, Self::Error> {
        Self::builder().id(value.volume_id.clone()).build()
    }
}
