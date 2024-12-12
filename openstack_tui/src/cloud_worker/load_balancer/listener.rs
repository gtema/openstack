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

/// Listener API operations
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum LoadBalancerListenerApiRequest {
    /// List
    List(LoadBalancerListenerList),
}

impl From<LoadBalancerListenerApiRequest> for ApiRequest {
    fn from(item: LoadBalancerListenerApiRequest) -> Self {
        ApiRequest::LoadBalancer(LoadBalancerApiRequest::from(item))
    }
}

impl ExecuteApiRequest for LoadBalancerListenerApiRequest {
    async fn execute_request(
        &self,
        session: &mut AsyncOpenStack,
        request: &ApiRequest,
        app_tx: &UnboundedSender<Action>,
    ) -> Result<(), CloudWorkerError> {
        match self {
            LoadBalancerListenerApiRequest::List(ref req) => {
                let ep = TryInto::<
                    openstack_sdk::api::load_balancer::v2::listener::list::Request<'_>,
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
pub struct LoadBalancerListenerList {
    pub loadbalancer_id: Option<String>,
    pub loadbalancer_name: Option<String>,
}

impl fmt::Display for LoadBalancerListenerList {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut parts: Vec<String> = Vec::new();
        if self.loadbalancer_id.is_some() || self.loadbalancer_name.is_some() {
            parts.push(format!(
                "lb: {}",
                self.loadbalancer_name
                    .as_ref()
                    .or(self.loadbalancer_id.as_ref())
                    .unwrap_or(&String::new())
            ));
        }
        write!(f, "{}", parts.join(","))
    }
}

impl TryFrom<&LoadBalancerListenerList>
    for openstack_sdk::api::load_balancer::v2::listener::list::Request<'_>
{
    type Error = openstack_sdk::api::load_balancer::v2::listener::list::RequestBuilderError;

    fn try_from(value: &LoadBalancerListenerList) -> Result<Self, Self::Error> {
        let mut ep_builder = Self::builder();
        if let Some(lb_id) = &value.loadbalancer_id {
            ep_builder.load_balancer_id(lb_id.clone());
        }

        ep_builder.build()
    }
}
