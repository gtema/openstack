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

use crate::cloud_worker::types as cloud_types;

/// TUI action
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Display, Deserialize)]
pub enum Action {
    Tick,
    Render,
    Resize(u16, u16),
    Info,
    Suspend,
    Resume,
    Quit,
    ClearScreen,
    Mode {
        mode: crate::mode::Mode,
        stack: bool,
    },
    /// Switched to previous mode
    PrevMode,
    Error {
        msg: String,
        action: Option<Box<Action>>,
    },
    Help,
    /// Trigger connection to the cloud
    ConnectToCloud(String),
    /// Request rescoping current connection
    CloudChangeScope(openstack_sdk::auth::authtoken::AuthTokenScope),
    /// New cloud connection established
    ConnectedToCloud(Box<openstack_sdk::types::identity::v3::AuthToken>),
    /// Perform API request
    PerformApiRequest(cloud_types::ApiRequest),
    /// Propagate single resource data to components
    ApiResponseData {
        request: cloud_types::ApiRequest,
        data: serde_json::Value,
    },
    /// Propagate resources list to components
    ApiResponsesData {
        request: cloud_types::ApiRequest,
        data: Vec<serde_json::Value>,
    },
    /// Open resource(mode) select popup
    ApiRequestSelect,
    /// Refresh data
    Refresh,

    /// AuthHelper
    AuthDataRequired {
        prompt: String,
        connection_name: Option<String>,
        is_sensitive: bool,
    },
    AuthHelperCompleted,

    /// Open describe view with the details
    SetDescribeApiResponseData(serde_json::Value),
    /// Describe resource under cursor
    DescribeApiResponse,
    /// Set describe mode loading
    SetDescribeLoading(bool),
    /// Open cloud connection popup
    CloudSelect,
    /// Open project selection popup
    SelectProject,
    /// Query list of configured cloud connections
    ListClouds,
    /// List of configured cloud connections
    Clouds(Vec<String>),

    /// Confirm operation
    Confirm(cloud_types::ApiRequest),
    /// Reject (close) Confirmation prompt
    ConfirmRejected(cloud_types::ApiRequest),
    /// Close confirmation prompt
    ConfirmAccepted(cloud_types::ApiRequest),

    // Block Storage (Cinder)
    /// Delete volume
    DeleteBlockStorageVolume,

    // Compute (Nova)
    SetComputeServerListFilters(Box<cloud_types::ComputeServerList>),
    SetComputeServerInstanceActionListFilters(Box<cloud_types::ComputeServerInstanceActionList>),
    SetComputeServerInstanceActionShowFilters(cloud_types::ComputeServerInstanceActionShow),
    /// Show servers provisioned with selected flavor
    ShowComputeServersWithFlavor,
    /// Delete selected server
    DeleteComputeServer,
    /// Show console output of the selected entry
    ShowServerConsoleOutput,
    /// Show selected server instance actions
    ShowComputeServerInstanceActions,
    /// Show selected server instance action events
    ShowComputeServerInstanceActionEvents,

    // DNS (Designate)
    /// Set DNS Zone filters
    SetDnsZoneListFilters(cloud_types::DnsZoneList),
    /// Delete DNS zone
    DeleteDnsZone,
    /// Set DNS Recordset filters
    SetDnsRecordsetListFilters(cloud_types::DnsRecordsetList),
    /// Zone recordsets
    ShowDnsZoneRecordsets,

    // Identity (keystone)
    //  Groups
    /// Create new identity group
    IdentityGroupCreate,
    /// Delete identity group
    IdentityGroupDelete,
    //  Group users
    /// Action user invokes to switch mode for selected entity
    ShowIdentityGroupUsers,
    /// Set GroupUser filters
    SetIdentityGroupUserListFilters(cloud_types::IdentityGroupUserList),
    /// Add user into the group
    IdentityGroupUserAdd,
    /// Remove user from the group
    IdentityGroupUserRemove,
    //  Users
    // Set ApplicationCredentials filters
    ShowIdentityUserApplicationCredentials,
    SetIdentityApplicationCredentialListFilters(cloud_types::IdentityUserApplicationCredentialList),
    /// Toggle user enabled property
    IdentityUserFlipEnable,
    /// Remove user
    IdentityUserDelete,
    /// Create new user
    IdentityUserCreate,
    /// Update user password
    IdentityUserSetPassword,
    /// Switch current project scope to the selected project
    SwitchToProject,

    // Image (glance)
    SetImageListFilters(cloud_types::ImageImageList),
    /// Delete image
    DeleteImage,

    // LB
    /// Set LB filters
    SetLoadBalancerListFilters(cloud_types::LoadBalancerLoadbalancerList),
    /// Set LB Listener filters
    SetLoadBalancerListenerListFilters(cloud_types::LoadBalancerListenerList),
    /// Show LB Listeners
    ShowLoadBalancerListeners,
    /// Show LB Pools
    ShowLoadBalancerPools,
    /// Set LB Pool filters
    SetLoadBalancerPoolListFilters(cloud_types::LoadBalancerPoolList),
    /// Show LB Listener Pools
    ShowLoadBalancerListenerPools,
    /// Set LB Member filters
    SetLoadBalancerPoolMemberListFilters(cloud_types::LoadBalancerPoolMemberList),
    /// Show LB Pool members
    ShowLoadBalancerPoolMembers,
    /// Set LB Healthmonitor filters
    SetLoadBalancerHealthMonitorListFilters(cloud_types::LoadBalancerHealthmonitorList),
    /// Show LB Listener Pools
    ShowLoadBalancerPoolHealthMonitors,

    // Network (neutron)
    /// Set Security group filters
    SetNetworkSecurityGroupListFilters(cloud_types::NetworkSecurityGroupList),
    /// Switch to NetworkSecurityGroupRules
    ShowNetworkSecurityGroupRules,
    /// Switch to routers view
    ShowNetworkRouters,
    /// Set Security group rule filters
    SetNetworkSecurityGroupRuleListFilters(cloud_types::NetworkSecurityGroupRuleList),
    SetNetworkSubnetListFilters(cloud_types::NetworkSubnetList),
    /// Show Subnetworks of a network
    ShowNetworkSubnets,
}
