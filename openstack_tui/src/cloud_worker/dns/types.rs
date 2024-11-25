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

use crate::cloud_worker::common::ConfirmableRequest;

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DnsRecordsetFilters {
    /// Zone Id
    pub zone_id: Option<String>,
    /// Optional name (for info purposes
    pub zone_name: Option<String>,
}
impl fmt::Display for DnsRecordsetFilters {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut parts: Vec<String> = Vec::new();
        if self.zone_id.is_some() || self.zone_name.is_some() {
            parts.push(format!(
                "zone: {}",
                self.zone_name
                    .as_ref()
                    .or(self.zone_name.as_ref())
                    .unwrap_or(&String::new())
            ));
        }
        write!(f, "{}", parts.join(","))
    }
}

impl TryFrom<&DnsRecordsetFilters>
    for openstack_sdk::api::dns::v2::zone::recordset::list::RequestBuilder<'_>
{
    type Error = eyre::Report;

    fn try_from(value: &DnsRecordsetFilters) -> Result<Self, Self::Error> {
        let mut ep_builder = openstack_sdk::api::dns::v2::zone::recordset::list::Request::builder();
        ep_builder.sort_key("name");

        if let Some(zone_id) = &value.zone_id {
            ep_builder.zone_id(zone_id.clone());
        }

        Ok(ep_builder)
    }
}

impl TryFrom<&DnsRecordsetFilters>
    for openstack_sdk::api::dns::v2::recordset::list::RequestBuilder<'_>
{
    type Error = eyre::Report;

    fn try_from(_value: &DnsRecordsetFilters) -> Result<Self, Self::Error> {
        let mut ep_builder = openstack_sdk::api::dns::v2::recordset::list::Request::builder();

        ep_builder.sort_key("name");
        Ok(ep_builder)
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DnsZoneFilters {}
impl fmt::Display for DnsZoneFilters {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "")
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DnsZoneDelete {
    pub zone_id: String,
    pub zone_name: Option<String>,
}

impl fmt::Display for DnsZoneDelete {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "")
    }
}

impl ConfirmableRequest for DnsZoneDelete {
    fn get_confirm_message(&self) -> Option<String> {
        Some(format!(
            "Delete DNS Zone {} ?",
            self.zone_name.clone().unwrap_or(self.zone_id.clone())
        ))
    }
}
