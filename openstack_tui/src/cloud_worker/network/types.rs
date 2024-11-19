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
use std::fmt;

/// Network filters
#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NetworkNetworkFilters {}
impl fmt::Display for NetworkNetworkFilters {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "")
    }
}

impl TryFrom<&NetworkNetworkFilters>
    for openstack_sdk::api::network::v2::network::list::RequestBuilder<'_>
{
    type Error = eyre::Report;

    fn try_from(_value: &NetworkNetworkFilters) -> Result<Self, Self::Error> {
        let mut ep_builder = openstack_sdk::api::network::v2::network::list::Request::builder();

        ep_builder.sort_key(["name"].into_iter());
        ep_builder.sort_dir(["asc"].into_iter());

        Ok(ep_builder)
    }
}

/// Router filters
#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NetworkRouterFilters {}
impl fmt::Display for NetworkRouterFilters {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "")
    }
}

impl TryFrom<&NetworkRouterFilters>
    for openstack_sdk::api::network::v2::router::list::RequestBuilder<'_>
{
    type Error = eyre::Report;

    fn try_from(_value: &NetworkRouterFilters) -> Result<Self, Self::Error> {
        let mut ep_builder = openstack_sdk::api::network::v2::router::list::Request::builder();

        ep_builder.sort_key(["name"].into_iter());
        ep_builder.sort_dir(["asc"].into_iter());

        Ok(ep_builder)
    }
}

/// Subnet filters
#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NetworkSubnetFilters {
    pub network_id: Option<String>,
    /// Name of the parent network
    pub network_name: Option<String>,
}
impl fmt::Display for NetworkSubnetFilters {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.network_id.is_some() || self.network_name.is_some() {
            write!(
                f,
                "network: {}",
                self.network_name
                    .as_ref()
                    .or(self.network_id.as_ref())
                    .unwrap_or(&String::new())
            )?;
        }
        Ok(())
    }
}

impl TryFrom<&NetworkSubnetFilters>
    for openstack_sdk::api::network::v2::subnet::list::RequestBuilder<'_>
{
    type Error = eyre::Report;

    fn try_from(value: &NetworkSubnetFilters) -> Result<Self, Self::Error> {
        let mut ep_builder = openstack_sdk::api::network::v2::subnet::list::Request::builder();

        ep_builder.sort_key(["name"].into_iter());
        ep_builder.sort_dir(["asc"].into_iter());

        if let Some(network_id) = &value.network_id {
            ep_builder.network_id(network_id.clone());
        }

        Ok(ep_builder)
    }
}

/// Security groups
#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NetworkSecurityGroupFilters {}
impl fmt::Display for NetworkSecurityGroupFilters {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "")
    }
}

impl TryFrom<&NetworkSecurityGroupFilters>
    for openstack_sdk::api::network::v2::security_group::list::RequestBuilder<'_>
{
    type Error = eyre::Report;

    fn try_from(_value: &NetworkSecurityGroupFilters) -> Result<Self, Self::Error> {
        let mut ep_builder =
            openstack_sdk::api::network::v2::security_group::list::Request::builder();

        ep_builder.sort_key(["name"].into_iter());
        ep_builder.sort_dir(["asc"].into_iter());

        Ok(ep_builder)
    }
}

/// Security group rules
#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NetworkSecurityGroupRuleFilters {
    pub security_group_id: Option<String>,
    pub security_group_name: Option<String>,
}

impl fmt::Display for NetworkSecurityGroupRuleFilters {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.security_group_id.is_some() || self.security_group_name.is_some() {
            write!(
                f,
                "security_group: {}",
                self.security_group_name
                    .as_ref()
                    .or(self.security_group_id.as_ref())
                    .unwrap_or(&String::new())
            )?;
        }
        Ok(())
    }
}

impl TryFrom<&NetworkSecurityGroupRuleFilters>
    for openstack_sdk::api::network::v2::security_group_rule::list::RequestBuilder<'_>
{
    type Error = eyre::Report;

    fn try_from(value: &NetworkSecurityGroupRuleFilters) -> Result<Self, Self::Error> {
        let mut ep_builder =
            openstack_sdk::api::network::v2::security_group_rule::list::Request::builder();

        ep_builder.sort_key(["ethertype", "direction", "protocol", "port_range_min"].into_iter());
        ep_builder.sort_dir(["asc", "asc", "asc", "asc"].into_iter());

        if let Some(security_group_id) = &value.security_group_id {
            ep_builder.security_group_id(security_group_id.clone());
        }

        if let Some(security_group_id) = &value.security_group_id {
            ep_builder.security_group_id(security_group_id.clone());
        }

        Ok(ep_builder)
    }
}
