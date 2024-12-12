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
use crate::cloud_worker::types::ExecuteApiRequest;
use crate::cloud_worker::{ApiRequest, CloudWorkerError};

pub mod auth;
pub mod group;
pub mod project;
pub mod types;
pub mod user;
use types::*;

impl ExecuteApiRequest for IdentityApiRequest {
    async fn execute_request(
        &self,
        session: &mut AsyncOpenStack,
        request: &ApiRequest,
        app_tx: &UnboundedSender<Action>,
    ) -> Result<(), CloudWorkerError> {
        match self {
            IdentityApiRequest::Auth(data) => data.execute_request(session, request, app_tx).await,
            IdentityApiRequest::Group(data) => data.execute_request(session, request, app_tx).await,
            IdentityApiRequest::Project(data) => {
                data.execute_request(session, request, app_tx).await
            }
            IdentityApiRequest::User(data) => data.execute_request(session, request, app_tx).await,
        }
    }
}
