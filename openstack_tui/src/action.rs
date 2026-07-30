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

/// Deserialize a `ViewKey` field from an owned string, resolving it to the
/// canonical `'static` constant. Needed because `#[derive(Deserialize)]`
/// cannot produce a `&'static str` directly (see `crate::mode::resolve_view_key`).
fn deserialize_view_key<'de, D>(deserializer: D) -> Result<crate::mode::ViewKey, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    crate::mode::resolve_view_key(&s)
        .ok_or_else(|| serde::de::Error::custom(format!("unknown resource view key: {s}")))
}

/// Identity-only operation on a resource, addressed by `ViewKey`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ResourceOp {
    Create,
    Delete,
}

/// Operations on Identity Users, addressed by `ViewKey`.
/// Kept separate from `ResourceOp` because:
/// 1. `FlipEnable` is a toggle with no Create/Delete analogue.
/// 2. `FlipEnable` dispatches via `action_to_request` (immediate, no confirmation),
///    while `Delete` dispatches via `confirm_request` (requires confirmation).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum IdentityUserOp {
    Delete,
    FlipEnable,
}

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
    CloudChangeScope(Box<openstack_sdk::auth::authtoken::AuthTokenScope>),
    /// New cloud connection established
    ConnectedToCloud(Box<openstack_sdk::types::identity::v3::TokenInfo>),
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
    /// Switch current region to the selected region
    SwitchToRegion(String),
    /// Open region selection popup
    SelectRegion,
    /// Query list of available regions
    ListRegions,
    /// List of available regions
    Regions(Vec<String>),
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

    /// Edit. Open the default editor to get the user input for the operation.
    Edit {
        template: String,
        original_action: Box<Action>,
    },
    EditResult {
        result: serde_json::Value,
        original_action: Box<Action>,
    },

    // Compute (Nova)
    // BEGIN GENERATED rust-tui-view action compute.server
    /// Set compute.server filters
    SetComputeServerListFilters(Box<cloud_types::ComputeServerList>),
    // END GENERATED rust-tui-view action compute.server
    // BEGIN GENERATED rust-tui-view action compute.server/instance_action
    /// Set compute.server/instance_action filters
    SetComputeServerInstanceActionListFilters(Box<cloud_types::ComputeServerInstanceActionList>),
    // END GENERATED rust-tui-view action compute.server/instance_action
    // BEGIN GENERATED rust-tui-view action compute.server/instance_action/event
    /// Set compute.server/instance_action/event filters
    SetComputeServerInstanceActionShowFilters(Box<cloud_types::ComputeServerInstanceActionShow>),
    // END GENERATED rust-tui-view action compute.server/instance_action/event
    /// Show console output of the selected entry
    ShowServerConsoleOutput,

    // DNS (Designate)
    // BEGIN GENERATED rust-tui-view action dns.zone
    /// Set dns.zone filters
    SetDnsZoneListFilters(cloud_types::DnsZoneList),
    // END GENERATED rust-tui-view action dns.zone
    // BEGIN GENERATED rust-tui-view action dns.recordset
    /// Set dns.recordset filters
    SetDnsRecordsetListFilters(cloud_types::DnsRecordsetList),
    // END GENERATED rust-tui-view action dns.recordset

    // Identity (keystone)
    //  Groups
    /// Create new identity group
    IdentityGroupCreate,
    //  Group users
    // BEGIN GENERATED rust-tui-view action identity.group_user
    /// Set identity.group_user filters
    SetIdentityGroupUserListFilters(cloud_types::IdentityGroupUserList),
    // END GENERATED rust-tui-view action identity.group_user
    /// Add user into the group
    IdentityGroupUserAdd,
    /// Remove user from the group
    IdentityGroupUserRemove,
    //  Users
    // BEGIN GENERATED rust-tui-view action identity.user/application_credential
    /// Set identity.user/application_credential filters
    SetIdentityApplicationCredentialListFilters(cloud_types::IdentityUserApplicationCredentialList),
    // END GENERATED rust-tui-view action identity.user/application_credential
    /// Create new user
    IdentityUserCreate,
    /// Update user password
    IdentityUserSetPassword,
    /// Switch current project scope to the selected project
    SwitchToProject,

    // Image (glance)
    // BEGIN GENERATED rust-tui-view action image.image
    /// Set image.image filters
    SetImageListFilters(cloud_types::ImageImageList),
    // END GENERATED rust-tui-view action image.image

    // LB
    // BEGIN GENERATED rust-tui-view action load-balancer.loadbalancer
    /// Set load-balancer.loadbalancer filters
    SetLoadBalancerListFilters(cloud_types::LoadBalancerLoadbalancerList),
    // END GENERATED rust-tui-view action load-balancer.loadbalancer
    // BEGIN GENERATED rust-tui-view action load-balancer.listener
    /// Set load-balancer.listener filters
    SetLoadBalancerListenerListFilters(cloud_types::LoadBalancerListenerList),
    // END GENERATED rust-tui-view action load-balancer.listener
    // BEGIN GENERATED rust-tui-view action load-balancer.pool
    /// Set load-balancer.pool filters
    SetLoadBalancerPoolListFilters(cloud_types::LoadBalancerPoolList),
    // END GENERATED rust-tui-view action load-balancer.pool
    /// Show LB Listener Pools
    ShowLoadBalancerListenerPools,
    // BEGIN GENERATED rust-tui-view action load-balancer.pool/member
    /// Set load-balancer.pool/member filters
    SetLoadBalancerPoolMemberListFilters(cloud_types::LoadBalancerPoolMemberList),
    // END GENERATED rust-tui-view action load-balancer.pool/member
    // BEGIN GENERATED rust-tui-view action load-balancer.healthmonitor
    /// Set load-balancer.healthmonitor filters
    SetLoadBalancerHealthMonitorListFilters(cloud_types::LoadBalancerHealthmonitorList),
    // END GENERATED rust-tui-view action load-balancer.healthmonitor

    // Network (neutron)
    // BEGIN GENERATED rust-tui-view action network.security_group
    /// Set network.security_group filters
    SetNetworkSecurityGroupListFilters(cloud_types::NetworkSecurityGroupList),
    // END GENERATED rust-tui-view action network.security_group
    /// Show a resource view, addressed by `ViewKey`.
    ShowResource(#[serde(deserialize_with = "deserialize_view_key")] crate::mode::ViewKey),
    /// Create/Delete a resource, addressed by `ViewKey`.
    ResourceOp {
        #[serde(deserialize_with = "deserialize_view_key")]
        key: crate::mode::ViewKey,
        op: ResourceOp,
    },
    /// Identity-user-specific operation, addressed by `ViewKey`.
    IdentityUserOp {
        #[serde(deserialize_with = "deserialize_view_key")]
        key: crate::mode::ViewKey,
        op: IdentityUserOp,
    },
    /// Switch to routers view
    ShowNetworkRouters,
    // BEGIN GENERATED rust-tui-view action network.security_group_rule
    /// Set network.security_group_rule filters
    SetNetworkSecurityGroupRuleListFilters(cloud_types::NetworkSecurityGroupRuleList),
    // END GENERATED rust-tui-view action network.security_group_rule
    // GENERATED-ANCHOR: action variants
    // BEGIN GENERATED rust-tui-view action network.router
    /// Set network.router filters
    SetNetworkRouterListFilters(cloud_types::NetworkRouterList),
    // END GENERATED rust-tui-view action network.router
    // BEGIN GENERATED rust-tui-view action network.network
    /// Set network.network filters
    SetNetworkNetworkListFilters(cloud_types::NetworkNetworkList),
    // END GENERATED rust-tui-view action network.network
    // BEGIN GENERATED rust-tui-view action network.subnet
    /// Set network.subnet filters
    SetNetworkSubnetListFilters(cloud_types::NetworkSubnetList),
    // END GENERATED rust-tui-view action network.subnet
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mode::NETWORK_SECURITY_GROUP_RULE;

    /// serde_yaml 0.9 serializes/deserializes non-unit enum variants using a
    /// YAML tag (`!VariantName`) by default, not the `{VariantName: ...}`
    /// mapping form that `serde_json`/`toml` use for externally tagged
    /// enums. Parsing that mapping form (which is what our config files and
    /// these tests use) requires opting into `serde_yaml`'s
    /// `singleton_map_recursive` adapter, which walks the whole value tree
    /// applying the "single-key map" convention to every nested enum too
    /// (`Action`'s content contains further enums like `ResourceOp`). See
    /// https://docs.rs/serde_yaml/latest/serde_yaml/with/singleton_map_recursive/index.html.
    fn action_from_yaml_str(yaml: &str) -> Result<Action, serde_yaml::Error> {
        let deserializer = serde_yaml::Deserializer::from_str(yaml);
        serde_yaml::with::singleton_map_recursive::deserialize(deserializer)
    }

    #[test]
    fn show_resource_round_trips_through_yaml() {
        let yaml = "ShowResource: \"network.security_group_rule\"";
        let action: Action = action_from_yaml_str(yaml).unwrap();
        assert_eq!(action, Action::ShowResource(NETWORK_SECURITY_GROUP_RULE));
    }

    #[test]
    fn resource_op_round_trips_through_yaml() {
        let yaml = "ResourceOp:\n  key: \"network.security_group_rule\"\n  op: Delete";
        let action: Action = action_from_yaml_str(yaml).unwrap();
        assert_eq!(
            action,
            Action::ResourceOp {
                key: NETWORK_SECURITY_GROUP_RULE,
                op: ResourceOp::Delete,
            }
        );
    }

    #[test]
    fn resource_op_variants_are_distinct() {
        assert_ne!(
            Action::ResourceOp {
                key: NETWORK_SECURITY_GROUP_RULE,
                op: ResourceOp::Create,
            },
            Action::ResourceOp {
                key: NETWORK_SECURITY_GROUP_RULE,
                op: ResourceOp::Delete,
            }
        );
    }

    #[test]
    fn unit_variant_action_deserializes_from_bare_string() {
        let action: Action = serde_yaml::from_str("Quit").unwrap();
        assert_eq!(action, Action::Quit);
    }
}
