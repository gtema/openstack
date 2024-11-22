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
use serde_json::Value;
use tokio::sync::mpsc::UnboundedSender;

use openstack_sdk::{api::Pagination, api::QueryAsync};

use crate::action::Action;
use crate::cloud_worker::{ApiRequest, Cloud};

pub mod types;
use types::*;

pub trait BlockStorageExt {
    async fn perform_api_request(
        &mut self,
        app_tx: &UnboundedSender<Action>,
        request: ApiRequest,
    ) -> Result<()>;

    /// List Backups
    async fn get_backups(&mut self, filters: &BlockStorageBackupFilters) -> Result<Vec<Value>>;
    /// List Snapshots
    async fn get_snapshots(&mut self, filters: &BlockStorageSnapshotFilters) -> Result<Vec<Value>>;
    /// List Volumes
    async fn get_volumes(&mut self, filters: &BlockStorageVolumeFilters) -> Result<Vec<Value>>;
}

impl BlockStorageExt for Cloud {
    async fn perform_api_request(
        &mut self,
        app_tx: &UnboundedSender<Action>,
        request: ApiRequest,
    ) -> Result<()> {
        match request {
            ApiRequest::BlockStorageBackups(ref filters) => match self.get_backups(filters).await {
                Ok(data) => app_tx.send(Action::ApiResponsesData { request, data })?,
                Err(err) => app_tx.send(Action::Error(format!(
                    "Failed to fetch block-storage backups: {:?}",
                    err
                )))?,
            },
            ApiRequest::BlockStorageSnapshots(ref filters) => {
                match self.get_snapshots(filters).await {
                    Ok(data) => app_tx.send(Action::ApiResponsesData { request, data })?,
                    Err(err) => app_tx.send(Action::Error(format!(
                        "Failed to fetch block-storage snapshots: {:?}",
                        err
                    )))?,
                }
            }
            ApiRequest::BlockStorageVolumes(ref filters) => match self.get_volumes(filters).await {
                Ok(data) => app_tx.send(Action::ApiResponsesData { request, data })?,
                Err(err) => app_tx.send(Action::Error(format!(
                    "Failed to fetch block-storage volumes: {:?}",
                    err
                )))?,
            },
            _ => {
                todo!()
            }
        }
        Ok(())
    }

    async fn get_backups(&mut self, filters: &BlockStorageBackupFilters) -> Result<Vec<Value>> {
        if let Some(session) = &self.cloud {
            let ep =
                openstack_sdk::api::block_storage::v3::backup::list_detailed::RequestBuilder::try_from(
                    filters,
                )?
                .build()?;

            let res: Vec<Value> = openstack_sdk::api::paged(ep, Pagination::All)
                .query_async(session)
                .await?;
            return Ok(res);
        }
        Ok(Vec::new())
    }

    async fn get_snapshots(&mut self, filters: &BlockStorageSnapshotFilters) -> Result<Vec<Value>> {
        if let Some(session) = &self.cloud {
            let ep =
                openstack_sdk::api::block_storage::v3::snapshot::list_detailed::RequestBuilder::try_from(
                    filters,
                )?
                .build()?;

            let res: Vec<Value> = openstack_sdk::api::paged(ep, Pagination::All)
                .query_async(session)
                .await?;
            return Ok(res);
        }
        Ok(Vec::new())
    }

    async fn get_volumes(&mut self, filters: &BlockStorageVolumeFilters) -> Result<Vec<Value>> {
        if let Some(session) = &self.cloud {
            let ep =
                openstack_sdk::api::block_storage::v3::volume::list_detailed::RequestBuilder::try_from(
                    filters,
                )?
                .build()?;

            let res: Vec<Value> = openstack_sdk::api::paged(ep, Pagination::All)
                .query_async(session)
                .await?;
            return Ok(res);
        }
        Ok(Vec::new())
    }
}
