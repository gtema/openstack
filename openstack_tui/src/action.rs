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
    Mode(crate::mode::Mode),
    Error(String),
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

    // Compute (neutron)
    SetComputeServerFilters(cloud_types::ComputeServerFilters),
    SetComputeServerInstanceActionFilters(cloud_types::ComputeServerInstanceActionFilters),
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
    SetIdentityGroupUserFilters(cloud_types::IdentityGroupUserFilters),
    /// Add user into the group
    IdentityGroupUserAdd,
    /// Remove user from the group
    IdentityGroupUserRemove,
    //  Users
    // Set ApplicationCredentials filters
    ShowIdentityUserApplicationCredentials,
    SetIdentityApplicationCredentialFilters(cloud_types::IdentityApplicationCredentialFilters),
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
    SetImageFilters(cloud_types::ImageFilters),

    // Network (neutron)
    /// Set Security group filters
    SetNetworkSecurityGroupFilters(cloud_types::NetworkSecurityGroupFilters),
    /// Switch to NetworkSecurityGroupRules
    ShowNetworkSecurityGroupRules,
    /// Switch to routers view
    ShowNetworkRouters,
    /// Set Security group rule filters
    SetNetworkSecurityGroupRuleFilters(cloud_types::NetworkSecurityGroupRuleFilters),
    SetNetworkSubnetFilters(cloud_types::NetworkSubnetFilters),
    /// Show Subnetworks of a network
    ShowNetworkSubnets,
}
