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
#![allow(clippy::module_inception)]

use eyre::Result;
use tokio::sync::mpsc::UnboundedSender;

use openstack_sdk::AsyncOpenStack;

use crate::action::Action;
use crate::cloud_worker::ExecuteApiRequest;
use crate::cloud_worker::{ApiRequest, CloudWorkerError};

pub mod network;
pub mod quota;
pub mod router;
pub mod security_group;
pub mod security_group_rule;
pub mod subnet;
pub mod types;
pub mod v2;

use types::*;
impl ExecuteApiRequest for NetworkApiRequest {
    async fn execute_request(
        &self,
        session: &mut AsyncOpenStack,
        request: &ApiRequest,
        app_tx: &UnboundedSender<Action>,
    ) -> Result<(), CloudWorkerError> {
        match self {
            NetworkApiRequest::Network(data) => {
                data.execute_request(session, request, app_tx).await
            }
            NetworkApiRequest::Quota(data) => data.execute_request(session, request, app_tx).await,
            NetworkApiRequest::Router(data) => data.execute_request(session, request, app_tx).await,
            NetworkApiRequest::SecurityGroup(data) => {
                data.execute_request(session, request, app_tx).await
            }
            NetworkApiRequest::SecurityGroupRule(data) => {
                data.execute_request(session, request, app_tx).await
            }
            NetworkApiRequest::Subnet(data) => data.execute_request(session, request, app_tx).await,
        }
    }
}
