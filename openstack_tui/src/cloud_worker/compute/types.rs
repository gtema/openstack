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

use eyre::{OptionExt, Result};
use serde::{Deserialize, Serialize};
use std::fmt;

use crate::cloud_worker::common::ConfirmableRequest;

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ComputeFlavorFilters {}

impl fmt::Display for ComputeFlavorFilters {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "")
    }
}

impl TryFrom<&ComputeFlavorFilters>
    for openstack_sdk::api::compute::v2::flavor::list_detailed::RequestBuilder<'_>
{
    type Error = eyre::Report;

    fn try_from(_value: &ComputeFlavorFilters) -> Result<Self, Self::Error> {
        let mut ep_builder =
            openstack_sdk::api::compute::v2::flavor::list_detailed::Request::builder();

        ep_builder.sort_key("name");
        Ok(ep_builder)
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ComputeServerFilters {
    /// All tenants (admin only)
    pub all_tenants: Option<bool>,
    /// List servers with specific flavor
    pub flavor_id: Option<String>,
    /// Flavor name (used only for display)
    pub flavor_name: Option<String>,
}

impl fmt::Display for ComputeServerFilters {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut parts: Vec<String> = Vec::new();
        if self.all_tenants.is_some() {
            parts.push(String::from("all"));
        }
        if self.flavor_id.is_some() || self.flavor_name.is_some() {
            parts.push(format!(
                "flavor: {}",
                self.flavor_name
                    .as_ref()
                    .or(self.flavor_name.as_ref())
                    .unwrap_or(&String::new())
            ));
        }
        write!(f, "{}", parts.join(","))
    }
}

impl TryFrom<&ComputeServerFilters>
    for openstack_sdk::api::compute::v2::server::list_detailed::RequestBuilder<'_>
{
    type Error = eyre::Report;

    fn try_from(value: &ComputeServerFilters) -> Result<Self, Self::Error> {
        let mut ep_builder =
            openstack_sdk::api::compute::v2::server::list_detailed::Request::builder();

        ep_builder.sort_key("display_name");
        ep_builder.sort_dir("asc");

        if let Some(true) = &value.all_tenants {
            ep_builder.all_tenants("true");
        }

        if let Some(flavor_id) = &value.flavor_id {
            ep_builder.flavor(flavor_id.clone());
        }

        Ok(ep_builder)
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ComputeServerDelete {
    pub server_id: String,
    pub server_name: Option<String>,
}

impl fmt::Display for ComputeServerDelete {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "")
    }
}

impl ConfirmableRequest for ComputeServerDelete {
    fn get_confirm_message(&self) -> Option<String> {
        Some(format!(
            "Delete server {} ?",
            self.server_name.clone().unwrap_or(self.server_id.clone())
        ))
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ComputeServerInstanceActionFilters {
    /// Server ID
    pub server_id: Option<String>,
    /// Server name
    pub server_name: Option<String>,
    /// Request id
    pub request_id: Option<String>,
}

impl fmt::Display for ComputeServerInstanceActionFilters {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.server_id.is_some() || self.server_name.is_some() {
            write!(
                f,
                "server: {}",
                self.server_name
                    .as_ref()
                    .or(self.server_name.as_ref())
                    .unwrap_or(&String::new())
            )?;
        }

        write!(f, "")
    }
}

impl TryFrom<&ComputeServerInstanceActionFilters>
    for openstack_sdk::api::compute::v2::server::instance_action::list::RequestBuilder<'_>
{
    type Error = eyre::Report;

    fn try_from(filters: &ComputeServerInstanceActionFilters) -> Result<Self, Self::Error> {
        let mut ep_builder =
            openstack_sdk::api::compute::v2::server::instance_action::list::Request::builder();

        ep_builder.server_id(
            filters
                .server_id
                .clone()
                .ok_or_eyre("Server ID must be set")?,
        );

        Ok(ep_builder)
    }
}

impl TryFrom<&ComputeServerInstanceActionFilters>
    for openstack_sdk::api::compute::v2::server::instance_action::get::RequestBuilder<'_>
{
    type Error = eyre::Report;

    fn try_from(filters: &ComputeServerInstanceActionFilters) -> Result<Self, Self::Error> {
        let mut ep_builder =
            openstack_sdk::api::compute::v2::server::instance_action::get::Request::builder();

        ep_builder.server_id(
            filters
                .server_id
                .clone()
                .ok_or_eyre("Server ID must be set")?,
        );
        ep_builder.id(filters
            .request_id
            .clone()
            .ok_or_eyre("InstanceAction ID must be set")?);

        Ok(ep_builder)
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ComputeHypervisorFilters {}

impl fmt::Display for ComputeHypervisorFilters {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "")
    }
}

impl TryFrom<&ComputeHypervisorFilters>
    for openstack_sdk::api::compute::v2::hypervisor::list_detailed::RequestBuilder<'_>
{
    type Error = eyre::Report;

    fn try_from(_value: &ComputeHypervisorFilters) -> Result<Self, Self::Error> {
        Ok(openstack_sdk::api::compute::v2::hypervisor::list_detailed::Request::builder())
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ComputeAggregateFilters {}

impl fmt::Display for ComputeAggregateFilters {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "")
    }
}
impl TryFrom<&ComputeAggregateFilters>
    for openstack_sdk::api::compute::v2::aggregate::list::RequestBuilder
{
    type Error = eyre::Report;

    fn try_from(_value: &ComputeAggregateFilters) -> Result<Self, Self::Error> {
        Ok(openstack_sdk::api::compute::v2::aggregate::list::Request::builder())
    }
}
