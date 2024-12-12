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

use openstack_sdk::{api::QueryAsync, AsyncOpenStack};

use crate::action::Action;
use crate::cloud_worker::common::CloudWorkerError;
use crate::cloud_worker::identity::types::IdentityApiRequest;
use crate::cloud_worker::types::{ApiRequest, ExecuteApiRequest};

/// Application Credential API operations
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum IdentityApplicationCredentialApiRequest {
    /// List
    List(IdentityApplicationCredentialList),
}

impl From<IdentityApplicationCredentialApiRequest> for ApiRequest {
    fn from(item: IdentityApplicationCredentialApiRequest) -> Self {
        ApiRequest::Identity(IdentityApiRequest::from(item))
    }
}

impl ExecuteApiRequest for IdentityApplicationCredentialApiRequest {
    async fn execute_request(
        &self,
        session: &mut AsyncOpenStack,
        request: &ApiRequest,
        app_tx: &UnboundedSender<Action>,
    ) -> Result<(), CloudWorkerError> {
        match self {
            IdentityApplicationCredentialApiRequest::List(ref req) => {
                let ep = TryInto::<
                    openstack_sdk::api::identity::v3::user::application_credential::list::Request<
                        '_,
                    >,
                >::try_into(req)
                .wrap_err("Cannot prepare request")?;
                app_tx.send(Action::ApiResponsesData {
                    request: request.clone(),
                    data: ep.query_async(session).await?,
                })?;
            }
        }
        Ok(())
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IdentityApplicationCredentialList {
    /// User id (used by API)
    pub user_id: String,
    /// User name (Set by caller for display only)
    pub user_name: Option<String>,
}

impl fmt::Display for IdentityApplicationCredentialList {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "user: {}",
            self.user_name.as_ref().unwrap_or(&self.user_id)
        )
    }
}

impl TryFrom<&IdentityApplicationCredentialList>
    for openstack_sdk::api::identity::v3::user::application_credential::list::Request<'_>
{
    type Error =
        openstack_sdk::api::identity::v3::user::application_credential::list::RequestBuilderError;

    fn try_from(value: &IdentityApplicationCredentialList) -> Result<Self, Self::Error> {
        Self::builder().user_id(value.user_id.clone()).build()
    }
}
