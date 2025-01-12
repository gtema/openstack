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
//
// WARNING: This file is automatically generated from OpenAPI schema using
// `openstack-codegenerator`.

//! `Loadbalancer` Service bindings

use eyre::Result;
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc::UnboundedSender;

use openstack_sdk::AsyncOpenStack;

use crate::action::Action;
use crate::cloud_worker::common::CloudWorkerError;
use crate::cloud_worker::types::{ApiRequest, ExecuteApiRequest};

pub mod healthmonitor;
pub mod listener;
pub mod loadbalancer;
pub mod pool;
pub mod quota;

pub use healthmonitor::*;
pub use listener::*;
pub use loadbalancer::*;
pub use pool::*;
pub use quota::*;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum LoadBalancerApiRequest {
    /// Healthmonitor
    Healthmonitor(Box<LoadBalancerHealthmonitorApiRequest>),
    /// Listener
    Listener(Box<LoadBalancerListenerApiRequest>),
    /// Loadbalancer
    Loadbalancer(Box<LoadBalancerLoadbalancerApiRequest>),
    /// Pool
    Pool(Box<LoadBalancerPoolApiRequest>),
    /// Quota
    Quota(Box<LoadBalancerQuotaApiRequest>),
}

impl From<LoadBalancerApiRequest> for ApiRequest {
    fn from(item: LoadBalancerApiRequest) -> Self {
        ApiRequest::LoadBalancer(item)
    }
}

impl From<LoadBalancerHealthmonitorApiRequest> for LoadBalancerApiRequest {
    fn from(item: LoadBalancerHealthmonitorApiRequest) -> Self {
        LoadBalancerApiRequest::Healthmonitor(Box::new(item))
    }
}

impl From<LoadBalancerListenerApiRequest> for LoadBalancerApiRequest {
    fn from(item: LoadBalancerListenerApiRequest) -> Self {
        LoadBalancerApiRequest::Listener(Box::new(item))
    }
}

impl From<LoadBalancerLoadbalancerApiRequest> for LoadBalancerApiRequest {
    fn from(item: LoadBalancerLoadbalancerApiRequest) -> Self {
        LoadBalancerApiRequest::Loadbalancer(Box::new(item))
    }
}

impl From<LoadBalancerPoolApiRequest> for LoadBalancerApiRequest {
    fn from(item: LoadBalancerPoolApiRequest) -> Self {
        LoadBalancerApiRequest::Pool(Box::new(item))
    }
}

impl From<LoadBalancerQuotaApiRequest> for LoadBalancerApiRequest {
    fn from(item: LoadBalancerQuotaApiRequest) -> Self {
        LoadBalancerApiRequest::Quota(Box::new(item))
    }
}

impl ExecuteApiRequest for LoadBalancerApiRequest {
    async fn execute_request(
        &self,
        session: &mut AsyncOpenStack,
        request: &ApiRequest,
        app_tx: &UnboundedSender<Action>,
    ) -> Result<(), CloudWorkerError> {
        match self {
            LoadBalancerApiRequest::Healthmonitor(ref req) => {
                req.execute_request(session, request, app_tx).await?;
            }
            LoadBalancerApiRequest::Listener(ref req) => {
                req.execute_request(session, request, app_tx).await?;
            }
            LoadBalancerApiRequest::Loadbalancer(ref req) => {
                req.execute_request(session, request, app_tx).await?;
            }
            LoadBalancerApiRequest::Pool(ref req) => {
                req.execute_request(session, request, app_tx).await?;
            }
            LoadBalancerApiRequest::Quota(ref req) => {
                req.execute_request(session, request, app_tx).await?;
            }
        }
        Ok(())
    }
}