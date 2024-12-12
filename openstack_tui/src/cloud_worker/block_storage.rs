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
use tokio::sync::mpsc::UnboundedSender;

use openstack_sdk::AsyncOpenStack;

use crate::action::Action;
use crate::cloud_worker::block_storage::types::BlockStorageApiRequest;
use crate::cloud_worker::common::CloudWorkerError;
use crate::cloud_worker::types::ExecuteApiRequest;
use crate::cloud_worker::ApiRequest;

pub mod backup;
pub mod snapshot;
pub mod types;
pub mod volume;

impl ExecuteApiRequest for BlockStorageApiRequest {
    async fn execute_request(
        &self,
        session: &mut AsyncOpenStack,
        request: &ApiRequest,
        app_tx: &UnboundedSender<Action>,
    ) -> Result<(), CloudWorkerError> {
        match self {
            BlockStorageApiRequest::Backup(data) => {
                data.execute_request(session, request, app_tx).await
            }
            BlockStorageApiRequest::Snapshot(data) => {
                data.execute_request(session, request, app_tx).await
            }
            BlockStorageApiRequest::Volume(data) => {
                data.execute_request(session, request, app_tx).await
            }
        }
    }
}
