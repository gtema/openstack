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
use crate::cloud_worker::load_balancer::types::LoadBalancerApiRequest;
use crate::cloud_worker::types::{ApiRequest, ExecuteApiRequest};

/// Pool Member API operations
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum LoadBalancerPoolMemberApiRequest {
    /// List
    List(LoadBalancerPoolMemberList),
}

impl From<LoadBalancerPoolMemberApiRequest> for ApiRequest {
    fn from(item: LoadBalancerPoolMemberApiRequest) -> Self {
        ApiRequest::LoadBalancer(LoadBalancerApiRequest::from(item))
    }
}

impl ExecuteApiRequest for LoadBalancerPoolMemberApiRequest {
    async fn execute_request(
        &self,
        session: &mut AsyncOpenStack,
        request: &ApiRequest,
        app_tx: &UnboundedSender<Action>,
    ) -> Result<(), CloudWorkerError> {
        match self {
            LoadBalancerPoolMemberApiRequest::List(ref req) => {
                let ep = TryInto::<
                    openstack_sdk::api::load_balancer::v2::pool::member::list::Request<'_>,
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
pub struct LoadBalancerPoolMemberList {
    pub pool_id: String,
    pub pool_name: Option<String>,
}

impl fmt::Display for LoadBalancerPoolMemberList {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut parts: Vec<String> = Vec::new();
        parts.push(format!(
            "pool: {}",
            self.pool_name.as_ref().unwrap_or(&self.pool_id)
        ));
        write!(f, "{}", parts.join(","))
    }
}

impl TryFrom<&LoadBalancerPoolMemberList>
    for openstack_sdk::api::load_balancer::v2::pool::member::list::Request<'_>
{
    type Error = openstack_sdk::api::load_balancer::v2::pool::member::list::RequestBuilderError;

    fn try_from(value: &LoadBalancerPoolMemberList) -> Result<Self, Self::Error> {
        Self::builder().pool_id(value.pool_id.clone()).build()
    }
}
