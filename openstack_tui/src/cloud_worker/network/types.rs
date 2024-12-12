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

pub use crate::cloud_worker::network::network::*;
pub use crate::cloud_worker::network::quota::*;
pub use crate::cloud_worker::network::router::*;
pub use crate::cloud_worker::network::security_group::*;
pub use crate::cloud_worker::network::security_group_rule::*;
pub use crate::cloud_worker::network::subnet::*;

/// Network operations
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum NetworkApiRequest {
    /// Networks
    Network(NetworkNetworkApiRequest),
    /// Quota
    Quota(NetworkQuotaApiRequest),
    /// Routers
    Router(NetworkRouterApiRequest),
    /// Security groups
    SecurityGroup(NetworkSecurityGroupApiRequest),
    /// Security group rules
    SecurityGroupRule(NetworkSecurityGroupRuleApiRequest),
    /// Subnets
    Subnet(NetworkSubnetApiRequest),
}

impl From<NetworkNetworkApiRequest> for NetworkApiRequest {
    fn from(item: NetworkNetworkApiRequest) -> Self {
        NetworkApiRequest::Network(item)
    }
}

impl From<NetworkQuotaApiRequest> for NetworkApiRequest {
    fn from(item: NetworkQuotaApiRequest) -> Self {
        NetworkApiRequest::Quota(item)
    }
}

impl From<NetworkRouterApiRequest> for NetworkApiRequest {
    fn from(item: NetworkRouterApiRequest) -> Self {
        NetworkApiRequest::Router(item)
    }
}

impl From<NetworkSecurityGroupApiRequest> for NetworkApiRequest {
    fn from(item: NetworkSecurityGroupApiRequest) -> Self {
        NetworkApiRequest::SecurityGroup(item)
    }
}

impl From<NetworkSecurityGroupRuleApiRequest> for NetworkApiRequest {
    fn from(item: NetworkSecurityGroupRuleApiRequest) -> Self {
        NetworkApiRequest::SecurityGroupRule(item)
    }
}

impl From<NetworkSubnetApiRequest> for NetworkApiRequest {
    fn from(item: NetworkSubnetApiRequest) -> Self {
        NetworkApiRequest::Subnet(item)
    }
}
