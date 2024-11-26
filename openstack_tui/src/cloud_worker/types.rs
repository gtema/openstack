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
pub use crate::cloud_worker::dns::types::*;
pub use crate::cloud_worker::identity::types::*;
pub use crate::cloud_worker::image::types::*;
pub use crate::cloud_worker::load_balancer::types::*;
pub use crate::cloud_worker::network::types::*;

/// OpenStack "resource"
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Display, Deserialize)]
pub enum ApiRequest {
    // Block storage resources
    /// Cinder backup
    BlockStorageBackups(BlockStorageBackupFilters),
    /// Cinder snapshot
    BlockStorageSnapshots(BlockStorageSnapshotFilters),
    /// Cinder volume
    BlockStorageVolumes(BlockStorageVolumeFilters),
    /// Delete Cinder volume
    BlockStorageVolumeDelete(BlockStorageVolumeDelete),
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

    // DNS
    /// DNS Recordsets
    DnsRecordsets(DnsRecordsetFilters),
    /// DNS Zones
    DnsZones(DnsZoneFilters),
    /// Delete DNS zone
    DnsZoneDelete(DnsZoneDelete),

    // Identity (Keystone)
    IdentityAuthProjects(IdentityAuthProjectFilters),
    IdentityApplicationCredentials(IdentityApplicationCredentialFilters),
    IdentityGroups(IdentityGroupFilters),
    IdentityGroupUsers(IdentityGroupUserFilters),
    IdentityProjects(IdentityProjectFilters),
    IdentityUsers(IdentityUserFilters),
    IdentityUserUpdate(IdentityUserUpdate),

    // Image (Glance)
    ImageImages(ImageFilters),
    /// Delete image
    ImageImageDelete(ImageImageDelete),

    // Load Balancer
    LoadBalancers(LoadBalancerFilters),
    LoadBalancerListeners(LoadBalancerListenerFilters),
    LoadBalancerPools(LoadBalancerPoolFilters),
    LoadBalancerPoolMembers(LoadBalancerPoolMemberFilters),
    LoadBalancerHealthMonitors(LoadBalancerHealthMonitorFilters),

    // Network (Neutron)
    NetworkNetworks(NetworkNetworkFilters),
    NetworkRouters(NetworkRouterFilters),
    NetworkSubnets(NetworkSubnetFilters),
    NetworkSecurityGroups(NetworkSecurityGroupFilters),
    NetworkSecurityGroupRules(NetworkSecurityGroupRuleFilters),
    NetworkQuota,
}

impl From<ApiRequest> for ServiceType {
    fn from(item: ApiRequest) -> Self {
        match item {
            ApiRequest::BlockStorageBackups(_)
            | ApiRequest::BlockStorageSnapshots(_)
            | ApiRequest::BlockStorageVolumes(_)
            | ApiRequest::BlockStorageVolumeDelete(_) => Self::BlockStorage,
            ApiRequest::ComputeServers(_)
            | ApiRequest::ComputeServerDelete(_)
            | ApiRequest::ComputeServerConsoleOutput(_)
            | ApiRequest::ComputeServerInstanceAction(_)
            | ApiRequest::ComputeServerInstanceActions(_)
            | ApiRequest::ComputeFlavors(_)
            | ApiRequest::ComputeQuota
            | ApiRequest::ComputeAggregates(_)
            | ApiRequest::ComputeHypervisors(_) => Self::Compute,
            ApiRequest::DnsRecordsets(_)
            | ApiRequest::DnsZones(_)
            | ApiRequest::DnsZoneDelete(_) => Self::Dns,
            ApiRequest::IdentityAuthProjects(_)
            | ApiRequest::IdentityApplicationCredentials(_)
            | ApiRequest::IdentityGroups(_)
            | ApiRequest::IdentityGroupUsers(_)
            | ApiRequest::IdentityProjects(_)
            | ApiRequest::IdentityUserUpdate(_)
            | ApiRequest::IdentityUsers(_) => Self::Identity,
            ApiRequest::ImageImages(_) | ApiRequest::ImageImageDelete(_) => Self::Image,
            ApiRequest::LoadBalancers(_)
            | ApiRequest::LoadBalancerListeners(_)
            | ApiRequest::LoadBalancerHealthMonitors(_)
            | ApiRequest::LoadBalancerPools(_)
            | ApiRequest::LoadBalancerPoolMembers(_) => Self::LoadBalancer,
            ApiRequest::NetworkNetworks(_)
            | ApiRequest::NetworkRouters(_)
            | ApiRequest::NetworkQuota
            | ApiRequest::NetworkSecurityGroups(_)
            | ApiRequest::NetworkSecurityGroupRules(_)
            | ApiRequest::NetworkSubnets(_) => Self::Network,
        }
    }
}

impl ConfirmableRequest for ApiRequest {
    fn get_confirm_message(&self) -> Option<String> {
        match &self {
            ApiRequest::BlockStorageVolumeDelete(x) => x.get_confirm_message(),
            ApiRequest::ComputeServerDelete(x) => x.get_confirm_message(),
            ApiRequest::DnsZoneDelete(x) => x.get_confirm_message(),
            ApiRequest::ImageImageDelete(x) => x.get_confirm_message(),
            _ => None,
        }
    }
}
