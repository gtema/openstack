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

/// Group API operations
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum IdentityGroupApiRequest {
    /// List
    List(IdentityGroupList),
    /// Users
    UserList(IdentityGroupUserList),
}

impl From<IdentityGroupApiRequest> for ApiRequest {
    fn from(item: IdentityGroupApiRequest) -> Self {
        ApiRequest::Identity(IdentityApiRequest::from(item))
    }
}

impl ExecuteApiRequest for IdentityGroupApiRequest {
    async fn execute_request(
        &self,
        session: &mut AsyncOpenStack,
        request: &ApiRequest,
        app_tx: &UnboundedSender<Action>,
    ) -> Result<(), CloudWorkerError> {
        match self {
            IdentityGroupApiRequest::List(ref req) => {
                let ep =
                    TryInto::<openstack_sdk::api::identity::v3::group::list::Request<'_>>::try_into(
                        req,
                    )
                    .wrap_err("Cannot prepare request")?;
                app_tx.send(Action::ApiResponsesData {
                    request: request.clone(),
                    data: ep.query_async(session).await?,
                })?;
            }
            IdentityGroupApiRequest::UserList(ref req) => {
                let ep =
                    TryInto::<openstack_sdk::api::identity::v3::group::user::list::Request<'_>>::try_into(
                        req,
                    )
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
pub struct IdentityGroupList {}
impl fmt::Display for IdentityGroupList {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "")
    }
}

impl TryFrom<&IdentityGroupList> for openstack_sdk::api::identity::v3::group::list::Request<'_> {
    type Error = openstack_sdk::api::identity::v3::group::list::RequestBuilderError;

    fn try_from(_value: &IdentityGroupList) -> Result<Self, Self::Error> {
        Self::builder().build()
    }
}

/// Group Users filter
#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IdentityGroupUserList {
    /// Group id (used by API)
    pub group_id: String,
    /// Group name (Set by caller for display only)
    pub group_name: Option<String>,
}

impl fmt::Display for IdentityGroupUserList {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "group: {}",
            self.group_name.as_ref().unwrap_or(&self.group_id)
        )
    }
}

impl TryFrom<&IdentityGroupUserList>
    for openstack_sdk::api::identity::v3::group::user::list::Request<'_>
{
    type Error = openstack_sdk::api::identity::v3::group::user::list::RequestBuilderError;

    fn try_from(value: &IdentityGroupUserList) -> Result<Self, Self::Error> {
        Self::builder().group_id(value.group_id.clone()).build()
    }
}
