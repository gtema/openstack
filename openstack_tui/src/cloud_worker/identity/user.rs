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

mod application_credential;

pub use application_credential::*;

/// User API operations
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum IdentityUserApiRequest {
    /// Application Credentials
    ApplicationCredential(IdentityApplicationCredentialApiRequest),
    /// List
    List(IdentityUserList),
    /// Update
    Update(IdentityUserUpdate),
}

impl From<IdentityUserApiRequest> for ApiRequest {
    fn from(item: IdentityUserApiRequest) -> Self {
        ApiRequest::Identity(IdentityApiRequest::from(item))
    }
}

impl From<IdentityApplicationCredentialApiRequest> for IdentityUserApiRequest {
    fn from(item: IdentityApplicationCredentialApiRequest) -> Self {
        IdentityUserApiRequest::ApplicationCredential(item)
    }
}

impl From<IdentityApplicationCredentialApiRequest> for IdentityApiRequest {
    fn from(item: IdentityApplicationCredentialApiRequest) -> Self {
        IdentityApiRequest::User(IdentityUserApiRequest::from(item))
    }
}

impl ExecuteApiRequest for IdentityUserApiRequest {
    async fn execute_request(
        &self,
        session: &mut AsyncOpenStack,
        request: &ApiRequest,
        app_tx: &UnboundedSender<Action>,
    ) -> Result<(), CloudWorkerError> {
        match self {
            IdentityUserApiRequest::ApplicationCredential(ref req) => {
                req.execute_request(session, request, app_tx).await?
            }
            IdentityUserApiRequest::List(ref req) => {
                let ep =
                    TryInto::<openstack_sdk::api::identity::v3::user::list::Request<'_>>::try_into(
                        req,
                    )
                    .wrap_err("Cannot prepare request")?;
                app_tx.send(Action::ApiResponsesData {
                    request: request.clone(),
                    data: ep.query_async(session).await?,
                })?;
            }
            IdentityUserApiRequest::Update(ref req) => {
                let ep =
                    TryInto::<openstack_sdk::api::identity::v3::user::set::Request<'_>>::try_into(
                        req,
                    )
                    .wrap_err("Cannot prepare request")?;
                app_tx.send(Action::ApiResponseData {
                    request: request.clone(),
                    data: ep.query_async(session).await?,
                })?;
            }
        }
        Ok(())
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IdentityUserList {}
impl fmt::Display for IdentityUserList {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "")
    }
}

impl TryFrom<&IdentityUserList> for openstack_sdk::api::identity::v3::user::list::Request<'_> {
    type Error = openstack_sdk::api::identity::v3::user::list::RequestBuilderError;

    fn try_from(_value: &IdentityUserList) -> Result<Self, Self::Error> {
        Self::builder().build()
    }
}

/// Update user properties
#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IdentityUserUpdate {
    /// User ID
    pub id: String,
    /// New user name
    pub name: Option<String>,
    /// Enabled
    pub enabled: Option<bool>,
}

impl fmt::Display for IdentityUserUpdate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "user: {}", self.name.as_ref().unwrap_or(&self.id))
    }
}

impl TryFrom<&IdentityUserUpdate> for openstack_sdk::api::identity::v3::user::set::Request<'_> {
    type Error = eyre::Report;

    fn try_from(value: &IdentityUserUpdate) -> Result<Self, Self::Error> {
        let mut ep_builder = Self::builder();
        let mut user_builder = openstack_sdk::api::identity::v3::user::set::UserBuilder::default();
        ep_builder.id(value.id.clone());
        if let Some(name) = &value.name {
            user_builder.name(name.clone());
        }
        if let Some(enabled) = &value.enabled {
            user_builder.enabled(*enabled);
        }
        ep_builder.user(user_builder.build()?);
        Ok(ep_builder.build()?)
    }
}
