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
use strum::Display;

use crate::action::Action;
use openstack_sdk::AsyncOpenStack;
use openstack_sdk::types::ServiceType;
use tokio::sync::mpsc::UnboundedSender;

pub use crate::cloud_worker::block_storage::*;
pub use crate::cloud_worker::common::{CloudWorkerError, ConfirmableRequest};
pub use crate::cloud_worker::compute::*;
pub use crate::cloud_worker::dns::*;
pub use crate::cloud_worker::identity::*;
pub use crate::cloud_worker::image::*;
pub use crate::cloud_worker::load_balancer::*;
pub use crate::cloud_worker::network::*;

/// OpenStack "resource"
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Display, Deserialize)]
pub enum ApiRequest {
    // Block storage resources
    /// Block Storage api requests
    BlockStorage(BlockStorageApiRequest),

    // Compute resources
    /// Compute api requests
    Compute(ComputeApiRequest),

    /// DNS resources
    Dns(DnsApiRequest),

    /// Identity (Keystone)
    Identity(IdentityApiRequest),

    /// Image (Glance)
    Image(ImageApiRequest),

    /// Load Balancer
    LoadBalancer(LoadBalancerApiRequest),

    // Network (Neutron)
    Network(NetworkApiRequest),
}

impl From<ApiRequest> for ServiceType {
    fn from(item: ApiRequest) -> Self {
        match item {
            ApiRequest::BlockStorage(_) => Self::BlockStorage,
            ApiRequest::Compute(_) => Self::Compute,
            ApiRequest::Dns(_) => Self::Dns,
            ApiRequest::Identity(_) => Self::Identity,
            ApiRequest::Image(_) => Self::Image,
            ApiRequest::LoadBalancer(_) => Self::LoadBalancer,
            ApiRequest::Network(_) => Self::Network,
        }
    }
}

impl ConfirmableRequest for ApiRequest {
    fn get_confirm_message(&self) -> Option<String> {
        match &self {
            ApiRequest::BlockStorage(x) => x.get_confirm_message(),
            ApiRequest::Compute(x) => x.get_confirm_message(),
            ApiRequest::Dns(x) => x.get_confirm_message(),
            ApiRequest::Image(x) => x.get_confirm_message(),
            _ => None,
        }
    }
}

pub trait ExecuteApiRequest {
    async fn execute_request(
        &self,
        session: &mut AsyncOpenStack,
        request: &ApiRequest,
        app_tx: &UnboundedSender<Action>,
    ) -> Result<(), CloudWorkerError>;
}
