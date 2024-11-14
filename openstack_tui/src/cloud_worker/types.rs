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

use openstack_sdk::types::ServiceType;

pub use crate::cloud_worker::compute::types::*;
pub use crate::cloud_worker::identity::types::*;
pub use crate::cloud_worker::image::types::*;
pub use crate::cloud_worker::network::types::*;

/// OpenStack "resource"
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Display, Deserialize)]
pub enum Resource {
    ComputeFlavors(ComputeFlavorFilters),
    ComputeServers(ComputeServerFilters),
    ComputeServerConsoleOutput(String),
    ComputeAggregates(ComputeAggregateFilters),
    ComputeHypervisors(ComputeHypervisorFilters),
    ComputeQuota,
    IdentityAuthProjects(IdentityAuthProjectFilters),
    IdentityProjects(IdentityProjectFilters),
    IdentityUsers(IdentityUserFilters),
    ImageImages(ImageFilters),
    NetworkNetworks(NetworkNetworkFilters),
    NetworkSubnets(NetworkSubnetFilters),
    NetworkQuota,
}

impl From<Resource> for ServiceType {
    fn from(item: Resource) -> Self {
        match item {
            Resource::ComputeServers(_)
            | Resource::ComputeServerConsoleOutput(_)
            | Resource::ComputeFlavors(_)
            | Resource::ComputeQuota
            | Resource::ComputeAggregates(_)
            | Resource::ComputeHypervisors(_) => Self::Compute,
            Resource::IdentityAuthProjects(_)
            | Resource::IdentityProjects(_)
            | Resource::IdentityUsers(_) => Self::Identity,
            Resource::ImageImages(_) => Self::Image,
            Resource::NetworkNetworks(_) | Resource::NetworkSubnets(_) | Resource::NetworkQuota => {
                Self::Network
            }
        }
    }
}
