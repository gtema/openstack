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

pub use crate::cloud_worker::block_storage::types::*;
pub use crate::cloud_worker::common::ConfirmableRequest;
pub use crate::cloud_worker::compute::types::*;
pub use crate::cloud_worker::identity::types::*;
pub use crate::cloud_worker::image::types::*;
pub use crate::cloud_worker::network::types::*;

/// OpenStack "resource"
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Display, Deserialize)]
pub enum Resource {
    // Block storage resources
    /// Cinder backup
    BlockStorageBackups(BlockStorageBackupFilters),
    /// Cinder snapshot
    BlockStorageSnapshots(BlockStorageSnapshotFilters),
    /// Cinder volume
    BlockStorageVolumes(BlockStorageVolumeFilters),
    // Compute resources
    ComputeFlavors(ComputeFlavorFilters),
    ComputeServers(ComputeServerFilters),
    /// Delete server request
    ComputeServerDelete(ComputeServerDelete),
    ComputeServerInstanceActions(ComputeServerInstanceActionFilters),
    ComputeServerInstanceAction(ComputeServerInstanceActionFilters),
    ComputeServerConsoleOutput(String),
    ComputeAggregates(ComputeAggregateFilters),
    ComputeHypervisors(ComputeHypervisorFilters),
    ComputeQuota,
    IdentityAuthProjects(IdentityAuthProjectFilters),
    IdentityApplicationCredentials(IdentityApplicationCredentialFilters),
    IdentityGroups(IdentityGroupFilters),
    IdentityGroupUsers(IdentityGroupUserFilters),
    IdentityProjects(IdentityProjectFilters),
    IdentityUsers(IdentityUserFilters),
    IdentityUserUpdate(IdentityUserUpdate),
    ImageImages(ImageFilters),
    NetworkNetworks(NetworkNetworkFilters),
    NetworkRouters(NetworkRouterFilters),
    NetworkSubnets(NetworkSubnetFilters),
    NetworkSecurityGroups(NetworkSecurityGroupFilters),
    NetworkSecurityGroupRules(NetworkSecurityGroupRuleFilters),
    NetworkQuota,
}

impl From<Resource> for ServiceType {
    fn from(item: Resource) -> Self {
        match item {
            Resource::BlockStorageBackups(_)
            | Resource::BlockStorageSnapshots(_)
            | Resource::BlockStorageVolumes(_) => Self::BlockStorage,
            Resource::ComputeServers(_)
            | Resource::ComputeServerDelete(_)
            | Resource::ComputeServerConsoleOutput(_)
            | Resource::ComputeServerInstanceAction(_)
            | Resource::ComputeServerInstanceActions(_)
            | Resource::ComputeFlavors(_)
            | Resource::ComputeQuota
            | Resource::ComputeAggregates(_)
            | Resource::ComputeHypervisors(_) => Self::Compute,
            Resource::IdentityAuthProjects(_)
            | Resource::IdentityApplicationCredentials(_)
            | Resource::IdentityGroups(_)
            | Resource::IdentityGroupUsers(_)
            | Resource::IdentityProjects(_)
            | Resource::IdentityUserUpdate(_)
            | Resource::IdentityUsers(_) => Self::Identity,
            Resource::ImageImages(_) => Self::Image,
            Resource::NetworkNetworks(_)
            | Resource::NetworkRouters(_)
            | Resource::NetworkQuota
            | Resource::NetworkSecurityGroups(_)
            | Resource::NetworkSecurityGroupRules(_)
            | Resource::NetworkSubnets(_) => Self::Network,
        }
    }
}

impl ConfirmableRequest for Resource {
    fn get_confirm_message(&self) -> Option<String> {
        match &self {
            Resource::ComputeServerDelete(x) => x.get_confirm_message(),
            _ => None,
        }
    }
}
