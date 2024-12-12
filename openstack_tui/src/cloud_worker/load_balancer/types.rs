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

use serde::{Deserialize, Serialize};

pub use crate::cloud_worker::load_balancer::healthmonitor::*;
pub use crate::cloud_worker::load_balancer::listener::*;
pub use crate::cloud_worker::load_balancer::loadbalancer::*;
pub use crate::cloud_worker::load_balancer::pool::*;

/// LoadBalancer operations
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum LoadBalancerApiRequest {
    /// Health monitors
    Healthmonitor(LoadBalancerHealthmonitorApiRequest),
    /// Loadbalancers
    Loadbalancer(LoadBalancerLoadbalancerApiRequest),
    /// Listeners
    Listener(LoadBalancerListenerApiRequest),
    /// Pools
    Pool(LoadBalancerPoolApiRequest),
}

impl From<LoadBalancerHealthmonitorApiRequest> for LoadBalancerApiRequest {
    fn from(item: LoadBalancerHealthmonitorApiRequest) -> Self {
        LoadBalancerApiRequest::Healthmonitor(item)
    }
}

impl From<LoadBalancerLoadbalancerApiRequest> for LoadBalancerApiRequest {
    fn from(item: LoadBalancerLoadbalancerApiRequest) -> Self {
        LoadBalancerApiRequest::Loadbalancer(item)
    }
}

impl From<LoadBalancerListenerApiRequest> for LoadBalancerApiRequest {
    fn from(item: LoadBalancerListenerApiRequest) -> Self {
        LoadBalancerApiRequest::Listener(item)
    }
}

impl From<LoadBalancerPoolApiRequest> for LoadBalancerApiRequest {
    fn from(item: LoadBalancerPoolApiRequest) -> Self {
        LoadBalancerApiRequest::Pool(item)
    }
}
