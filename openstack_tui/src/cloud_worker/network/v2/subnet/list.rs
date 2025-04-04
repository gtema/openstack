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
//
// WARNING: This file is automatically generated from OpenAPI schema using
// `openstack-codegenerator`.
use derive_builder::Builder;
use eyre::{Report, Result, WrapErr};
use serde::{Deserialize, Serialize};
use std::fmt;
use tokio::sync::mpsc::UnboundedSender;

use crate::action::Action;
use crate::cloud_worker::common::CloudWorkerError;
use crate::cloud_worker::types::{ApiRequest, ExecuteApiRequest};

use crate::utils::OutputConfig;
use crate::utils::StructTable;
use openstack_sdk::api::network::v2::subnet::list::RequestBuilder;
use openstack_sdk::api::{Pagination, paged};
use openstack_sdk::types::BoolString;
use openstack_sdk::{AsyncOpenStack, api::QueryAsync};
use serde_json::Value;
use structable_derive::StructTable;

#[derive(Builder, Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[builder(setter(strip_option))]
pub struct NetworkSubnetList {
    #[builder(default)]
    pub cidr: Option<String>,
    #[builder(default)]
    pub description: Option<String>,
    #[builder(default)]
    pub enable_dhcp: Option<bool>,
    #[builder(default)]
    pub gateway_ip: Option<String>,
    #[builder(default)]
    pub id: Option<String>,
    #[builder(default)]
    pub ip_version: Option<i32>,
    #[builder(default)]
    pub ipv6_address_mode: Option<String>,
    #[builder(default)]
    pub ipv6_ra_mode: Option<String>,
    #[builder(default)]
    pub limit: Option<i32>,
    #[builder(default)]
    pub marker: Option<String>,
    #[builder(default)]
    pub name: Option<String>,
    #[builder(default)]
    pub network_id: Option<String>,
    #[builder(default)]
    pub network_name: Option<String>,
    #[builder(default)]
    pub not_tags: Option<Vec<String>>,
    #[builder(default)]
    pub not_tags_any: Option<Vec<String>>,
    #[builder(default)]
    pub page_reverse: Option<bool>,
    #[builder(default)]
    pub revision_number: Option<String>,
    #[builder(default)]
    pub router_external: Option<bool>,
    #[builder(default)]
    pub segment_id: Option<String>,
    #[builder(default)]
    pub segment_name: Option<String>,
    #[builder(default)]
    pub shared: Option<bool>,
    #[builder(default)]
    pub sort_dir: Option<Vec<String>>,
    #[builder(default)]
    pub sort_key: Option<Vec<String>>,
    #[builder(default)]
    pub subnetpool_id: Option<String>,
    #[builder(default)]
    pub subnetpool_name: Option<String>,
    #[builder(default)]
    pub tags: Option<Vec<String>>,
    #[builder(default)]
    pub tags_any: Option<Vec<String>>,
    #[builder(default)]
    pub tenant_id: Option<String>,
    #[builder(default)]
    pub tenant_name: Option<String>,
}

impl fmt::Display for NetworkSubnetList {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut parts: Vec<String> = Vec::new();
        if self.id.is_some() || self.name.is_some() {
            parts.push(format!(
                "name/id: {}",
                self.name
                    .as_ref()
                    .or(self.id.as_ref())
                    .unwrap_or(&String::default())
            ));
        }

        if self.network_id.is_some() || self.network_name.is_some() {
            parts.push(format!(
                "network: {}",
                self.network_name
                    .as_ref()
                    .or(self.network_id.as_ref())
                    .unwrap_or(&String::default())
            ));
        }

        if self.segment_id.is_some() || self.segment_name.is_some() {
            parts.push(format!(
                "segment: {}",
                self.segment_name
                    .as_ref()
                    .or(self.segment_id.as_ref())
                    .unwrap_or(&String::default())
            ));
        }

        if self.subnetpool_id.is_some() || self.subnetpool_name.is_some() {
            parts.push(format!(
                "subnetpool: {}",
                self.subnetpool_name
                    .as_ref()
                    .or(self.subnetpool_id.as_ref())
                    .unwrap_or(&String::default())
            ));
        }

        if self.tenant_id.is_some() || self.tenant_name.is_some() {
            parts.push(format!(
                "tenant: {}",
                self.tenant_name
                    .as_ref()
                    .or(self.tenant_id.as_ref())
                    .unwrap_or(&String::default())
            ));
        }

        write!(f, "{}", parts.join(","))
    }
}

impl TryFrom<&NetworkSubnetList> for RequestBuilder<'_> {
    type Error = Report;
    fn try_from(value: &NetworkSubnetList) -> Result<Self, Self::Error> {
        let mut ep_builder = Self::default();
        if let Some(val) = &value.id {
            ep_builder.id(val.clone());
        }
        if let Some(val) = &value.name {
            ep_builder.name(val.clone());
        }
        if let Some(val) = &value.ip_version {
            ep_builder.ip_version(*val);
        }
        if let Some(val) = &value.network_id {
            ep_builder.network_id(val.clone());
        }
        if let Some(val) = &value.subnetpool_id {
            ep_builder.subnetpool_id(val.clone());
        }
        if let Some(val) = &value.cidr {
            ep_builder.cidr(val.clone());
        }
        if let Some(val) = &value.gateway_ip {
            ep_builder.gateway_ip(val.clone());
        }
        if let Some(val) = &value.tenant_id {
            ep_builder.tenant_id(val.clone());
        }
        if let Some(val) = &value.enable_dhcp {
            ep_builder.enable_dhcp(*val);
        }
        if let Some(val) = &value.ipv6_ra_mode {
            ep_builder.ipv6_ra_mode(val.clone());
        }
        if let Some(val) = &value.ipv6_address_mode {
            ep_builder.ipv6_address_mode(val.clone());
        }
        if let Some(val) = &value.shared {
            ep_builder.shared(*val);
        }
        if let Some(val) = &value.revision_number {
            ep_builder.revision_number(val.clone());
        }
        if let Some(val) = &value.router_external {
            ep_builder.router_external(*val);
        }
        if let Some(val) = &value.tags {
            ep_builder.tags(val.iter().cloned());
        }
        if let Some(val) = &value.tags_any {
            ep_builder.tags_any(val.iter().cloned());
        }
        if let Some(val) = &value.not_tags {
            ep_builder.not_tags(val.iter().cloned());
        }
        if let Some(val) = &value.not_tags_any {
            ep_builder.not_tags_any(val.iter().cloned());
        }
        if let Some(val) = &value.description {
            ep_builder.description(val.clone());
        }
        if let Some(val) = &value.segment_id {
            ep_builder.segment_id(val.clone());
        }
        if let Some(val) = &value.sort_key {
            ep_builder.sort_key(val.iter().cloned());
        }
        if let Some(val) = &value.sort_dir {
            ep_builder.sort_dir(val.iter().cloned());
        }
        if let Some(val) = &value.limit {
            ep_builder.limit(*val);
        }
        if let Some(val) = &value.marker {
            ep_builder.marker(val.clone());
        }
        if let Some(val) = &value.page_reverse {
            ep_builder.page_reverse(*val);
        }

        Ok(ep_builder)
    }
}

impl ExecuteApiRequest for NetworkSubnetList {
    async fn execute_request(
        &self,
        session: &mut AsyncOpenStack,
        request: &ApiRequest,
        app_tx: &UnboundedSender<Action>,
    ) -> Result<(), CloudWorkerError> {
        let ep = TryInto::<RequestBuilder>::try_into(self)?
            .build()
            .wrap_err("Cannot prepare request")?;
        app_tx.send(Action::ApiResponsesData {
            request: request.clone(),
            data: paged(ep, Pagination::All).query_async(session).await?,
        })?;
        Ok(())
    }
}
/// NetworkSubnet response representation
#[derive(Deserialize, Serialize, Clone, StructTable)]
pub struct NetworkSubnet {
    /// Allocation pools with `start` and `end` IP addresses for this subnet.
    ///
    #[serde(default)]
    #[structable(optional, title = "ALLOCATION_POOLS", wide)]
    pub allocation_pools: Option<Value>,

    /// The CIDR of the subnet.
    ///
    #[serde(default)]
    #[structable(optional, title = "CIDR", wide)]
    pub cidr: Option<String>,

    /// Time at which the resource has been created (in UTC ISO8601 format).
    ///
    #[serde(default)]
    #[structable(optional, title = "CREATED_AT")]
    pub created_at: Option<String>,

    /// A human-readable description for the resource.
    ///
    #[serde(default)]
    #[structable(optional, title = "DESCRIPTION", wide)]
    pub description: Option<String>,

    /// List of dns name servers associated with the subnet.
    ///
    #[serde(default)]
    #[structable(optional, title = "DNS_NAMESERVERS", wide)]
    pub dns_nameservers: Option<Value>,

    /// Whether to publish DNS records for IPs from this subnet.
    ///
    #[serde(default)]
    #[structable(optional, title = "DNS_PUBLISH_FIXED_IP", wide)]
    pub dns_publish_fixed_ip: Option<BoolString>,

    /// Indicates whether dhcp is enabled or disabled for the subnet.
    ///
    #[serde(default)]
    #[structable(optional, title = "ENABLE_DHCP", wide)]
    pub enable_dhcp: Option<BoolString>,

    /// Gateway IP of this subnet. If the value is `null` that implies no
    /// gateway is associated with the subnet.
    ///
    #[serde(default)]
    #[structable(optional, title = "GATEWAY_IP", wide)]
    pub gateway_ip: Option<String>,

    /// Additional routes for the subnet. A list of dictionaries with
    /// `destination` and `nexthop` parameters.
    ///
    #[serde(default)]
    #[structable(optional, title = "HOST_ROUTES", wide)]
    pub host_routes: Option<Value>,

    /// The ID of the subnet.
    ///
    #[serde(default)]
    #[structable(optional, title = "ID", wide)]
    pub id: Option<String>,

    /// The IP protocol version. Value is `4` or `6`.
    ///
    #[serde(default)]
    #[structable(optional, title = "IP_VERSION", wide)]
    pub ip_version: Option<i32>,

    /// The IPv6 address modes specifies mechanisms for assigning IP addresses.
    /// Value is `slaac`, `dhcpv6-stateful`, `dhcpv6-stateless` or `null`.
    ///
    #[serde(default)]
    #[structable(optional, title = "IPV6_ADDRESS_MODE", wide)]
    pub ipv6_address_mode: Option<Value>,

    /// The IPv6 router advertisement specifies whether the networking service
    /// should transmit ICMPv6 packets, for a subnet. Value is `slaac`,
    /// `dhcpv6-stateful`, `dhcpv6-stateless` or `null`.
    ///
    #[serde(default)]
    #[structable(optional, title = "IPV6_RA_MODE", wide)]
    pub ipv6_ra_mode: Option<Value>,

    /// Human-readable name of the resource.
    ///
    #[serde(default)]
    #[structable(optional, title = "NAME")]
    pub name: Option<String>,

    /// The ID of the network to which the subnet belongs.
    ///
    #[serde(default)]
    #[structable(optional, title = "NETWORK_ID", wide)]
    pub network_id: Option<String>,

    /// The revision number of the resource.
    ///
    #[serde(default)]
    #[structable(optional, title = "REVISION_NUMBER", wide)]
    pub revision_number: Option<i32>,

    #[serde(default, rename = "router:external")]
    #[structable(optional, title = "ROUTER:EXTERNAL", wide)]
    pub router_external: Option<BoolString>,

    /// The ID of a network segment the subnet is associated with. It is
    /// available when `segment` extension is enabled.
    ///
    #[serde(default)]
    #[structable(optional, title = "SEGMENT_ID", wide)]
    pub segment_id: Option<String>,

    /// The service types associated with the subnet.
    ///
    #[serde(default)]
    #[structable(optional, title = "SERVICE_TYPES", wide)]
    pub service_types: Option<Value>,

    /// The ID of the subnet pool associated with the subnet.
    ///
    #[serde(default)]
    #[structable(optional, title = "SUBNETPOOL_ID", wide)]
    pub subnetpool_id: Option<String>,

    /// The list of tags on the resource.
    ///
    #[serde(default)]
    #[structable(optional, title = "TAGS", wide)]
    pub tags: Option<Value>,

    /// The ID of the project.
    ///
    #[serde(default)]
    #[structable(optional, title = "TENANT_ID", wide)]
    pub tenant_id: Option<String>,

    /// Time at which the resource has been updated (in UTC ISO8601 format).
    ///
    #[serde(default)]
    #[structable(optional, title = "UPDATED_AT")]
    pub updated_at: Option<String>,
}
