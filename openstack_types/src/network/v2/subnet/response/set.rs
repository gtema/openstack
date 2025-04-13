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
//! Response type for the put subnets/{subnet_id} operation

use crate::common::deser_bool_str_opt;
use serde::{Deserialize, Serialize};
use structable::{StructTable, StructTableOptions};

/// Subnet response representation
#[derive(Clone, Deserialize, Serialize, StructTable)]
pub struct SubnetResponse {
    /// Allocation pools with `start` and `end` IP addresses for this subnet.
    ///
    #[structable(optional, serialize)]
    pub allocation_pools: Option<Vec<AllocationPools>>,

    /// The CIDR of the subnet.
    ///
    #[structable(optional, serialize)]
    pub cidr: Option<String>,

    /// Time at which the resource has been created (in UTC ISO8601 format).
    ///
    #[structable(optional)]
    pub created_at: Option<String>,

    /// A human-readable description for the resource.
    ///
    #[structable(optional)]
    pub description: Option<String>,

    /// List of dns name servers associated with the subnet.
    ///
    #[structable(optional, serialize)]
    pub dns_nameservers: Option<Vec<String>>,

    /// Whether to publish DNS records for IPs from this subnet.
    ///
    #[serde(deserialize_with = "deser_bool_str_opt")]
    #[structable(optional)]
    pub dns_publish_fixed_ip: Option<bool>,

    /// Indicates whether dhcp is enabled or disabled for the subnet.
    ///
    #[serde(deserialize_with = "deser_bool_str_opt")]
    #[structable(optional)]
    pub enable_dhcp: Option<bool>,

    /// Gateway IP of this subnet. If the value is `null` that implies no
    /// gateway is associated with the subnet.
    ///
    #[structable(optional)]
    pub gateway_ip: Option<String>,

    /// Additional routes for the subnet. A list of dictionaries with
    /// `destination` and `nexthop` parameters.
    ///
    #[structable(optional, serialize)]
    pub host_routes: Option<Vec<HostRoutes>>,

    /// The ID of the subnet.
    ///
    #[structable(optional)]
    pub id: Option<String>,

    /// The IP protocol version. Value is `4` or `6`.
    ///
    #[structable(optional)]
    pub ip_version: Option<i32>,

    /// The IPv6 address modes specifies mechanisms for assigning IP addresses.
    /// Value is `slaac`, `dhcpv6-stateful`, `dhcpv6-stateless` or `null`.
    ///
    #[structable(optional, serialize)]
    pub ipv6_address_mode: Option<Ipv6AddressMode>,

    /// The IPv6 router advertisement specifies whether the networking service
    /// should transmit ICMPv6 packets, for a subnet. Value is `slaac`,
    /// `dhcpv6-stateful`, `dhcpv6-stateless` or `null`.
    ///
    #[structable(optional, serialize)]
    pub ipv6_ra_mode: Option<Ipv6RaMode>,

    /// Human-readable name of the resource.
    ///
    #[structable(optional)]
    pub name: Option<String>,

    /// The ID of the network to which the subnet belongs.
    ///
    #[structable(optional)]
    pub network_id: Option<String>,

    /// The revision number of the resource.
    ///
    #[structable(optional)]
    pub revision_number: Option<i32>,

    #[serde(deserialize_with = "deser_bool_str_opt", rename = "router:external")]
    #[structable(optional, title = "router:external")]
    pub router_external: Option<bool>,

    /// The ID of a network segment the subnet is associated with. It is
    /// available when `segment` extension is enabled.
    ///
    #[structable(optional, serialize)]
    pub segment_id: Option<String>,

    /// The service types associated with the subnet.
    ///
    #[structable(optional, serialize)]
    pub service_types: Option<Vec<String>>,

    /// The ID of the subnet pool associated with the subnet.
    ///
    #[structable(optional, serialize)]
    pub subnetpool_id: Option<String>,

    /// The list of tags on the resource.
    ///
    #[structable(optional, serialize)]
    pub tags: Option<Vec<String>>,

    /// The ID of the project.
    ///
    #[structable(optional)]
    pub tenant_id: Option<String>,

    /// Time at which the resource has been updated (in UTC ISO8601 format).
    ///
    #[structable(optional)]
    pub updated_at: Option<String>,
}

/// `AllocationPools` type
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AllocationPools {
    pub end: Option<String>,
    pub start: Option<String>,
}

/// `HostRoutes` type
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct HostRoutes {
    pub destination: Option<String>,
    pub nexthop: Option<String>,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
#[serde(untagged)]
pub enum Ipv6RaMode {
    // Dhcpv6Stateful
    #[serde(rename = "dhcpv6-stateful")]
    Dhcpv6Stateful,

    // Dhcpv6Stateless
    #[serde(rename = "dhcpv6-stateless")]
    Dhcpv6Stateless,

    // Slaac
    #[serde(rename = "slaac")]
    Slaac,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
#[serde(untagged)]
pub enum Ipv6AddressMode {
    // Dhcpv6Stateful
    #[serde(rename = "dhcpv6-stateful")]
    Dhcpv6Stateful,

    // Dhcpv6Stateless
    #[serde(rename = "dhcpv6-stateless")]
    Dhcpv6Stateless,

    // Slaac
    #[serde(rename = "slaac")]
    Slaac,
}
