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
//! Response type for the POST `subnets` operation

use serde::{Deserialize, Serialize};
use structable::{StructTable, StructTableOptions};

/// Subnet response representation
#[derive(Clone, Deserialize, Serialize, StructTable)]
pub struct SubnetResponse {
    /// Allocation pools with `start` and `end` IP addresses for this subnet.
    #[serde(default)]
    #[structable(optional, serialize)]
    pub allocation_pools: Option<Vec<AllocationPools>>,

    /// The CIDR of the subnet.
    #[serde(default)]
    #[structable(optional)]
    pub cidr: Option<String>,

    /// Time at which the resource has been created (in UTC ISO8601 format).
    #[serde(default)]
    #[structable(optional)]
    pub created_at: Option<String>,

    /// A human-readable description for the resource.
    #[serde(default)]
    #[structable(optional)]
    pub description: Option<String>,

    /// List of dns name servers associated with the subnet.
    #[serde(default)]
    #[structable(optional, serialize)]
    pub dns_nameservers: Option<Vec<String>>,

    /// Whether to publish DNS records for IPs from this subnet.
    #[serde(default, deserialize_with = "crate::common::deser_bool_str_opt")]
    #[structable(optional)]
    pub dns_publish_fixed_ip: Option<bool>,

    /// Indicates whether dhcp is enabled or disabled for the subnet.
    #[serde(default, deserialize_with = "crate::common::deser_bool_str_opt")]
    #[structable(optional)]
    pub enable_dhcp: Option<bool>,

    /// Gateway IP of this subnet. If the value is `null` that implies no
    /// gateway is associated with the subnet.
    #[serde(default)]
    #[structable(optional)]
    pub gateway_ip: Option<String>,

    /// Additional routes for the subnet. A list of dictionaries with
    /// `destination` and `nexthop` parameters.
    #[serde(default)]
    #[structable(optional, serialize)]
    pub host_routes: Option<Vec<HostRoutes>>,

    /// The ID of the subnet.
    #[serde(default)]
    #[structable(optional)]
    pub id: Option<String>,

    /// The IP protocol version. Value is `4` or `6`.
    #[serde(default)]
    #[structable(optional)]
    pub ip_version: Option<i32>,

    /// The IPv6 address modes specifies mechanisms for assigning IP addresses.
    /// Value is `slaac`, `dhcpv6-stateful`, `dhcpv6-stateless` or `null`.
    #[serde(default)]
    #[structable(optional, serialize)]
    pub ipv6_address_mode: Option<Ipv6AddressMode>,

    /// The IPv6 router advertisement specifies whether the networking service
    /// should transmit ICMPv6 packets, for a subnet. Value is `slaac`,
    /// `dhcpv6-stateful`, `dhcpv6-stateless` or `null`.
    #[serde(default)]
    #[structable(optional, serialize)]
    pub ipv6_ra_mode: Option<Ipv6RaMode>,

    /// Human-readable name of the resource.
    #[serde(default)]
    #[structable(optional)]
    pub name: Option<String>,

    /// The ID of the network to which the subnet belongs.
    #[serde(default)]
    #[structable(optional)]
    pub network_id: Option<String>,

    /// The revision number of the resource.
    #[serde(default)]
    #[structable(optional)]
    pub revision_number: Option<i32>,

    #[serde(
        default,
        deserialize_with = "crate::common::deser_bool_str_opt",
        rename = "router:external"
    )]
    #[structable(optional, title = "router:external")]
    pub router_external: Option<bool>,

    /// The ID of a network segment the subnet is associated with. It is
    /// available when `segment` extension is enabled.
    #[serde(default)]
    #[structable(optional)]
    pub segment_id: Option<String>,

    /// The service types associated with the subnet.
    #[serde(default)]
    #[structable(optional, serialize)]
    pub service_types: Option<Vec<String>>,

    /// The ID of the subnet pool associated with the subnet.
    #[serde(default)]
    #[structable(optional)]
    pub subnetpool_id: Option<String>,

    /// The list of tags on the resource.
    #[serde(default)]
    #[structable(optional, serialize)]
    pub tags: Option<Vec<String>>,

    /// The ID of the project.
    #[serde(default)]
    #[structable(optional)]
    pub tenant_id: Option<String>,

    /// Time at which the resource has been updated (in UTC ISO8601 format).
    #[serde(default)]
    #[structable(optional)]
    pub updated_at: Option<String>,
}

/// `AllocationPools` type
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AllocationPools {
    #[serde(default)]
    pub end: Option<String>,
    #[serde(default)]
    pub start: Option<String>,
}

/// `HostRoutes` type
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct HostRoutes {
    #[serde(default)]
    pub destination: Option<String>,
    #[serde(default)]
    pub nexthop: Option<String>,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
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

impl std::str::FromStr for Ipv6AddressMode {
    type Err = ();
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "dhcpv6-stateful" => Ok(Self::Dhcpv6Stateful),
            "dhcpv6-stateless" => Ok(Self::Dhcpv6Stateless),
            "slaac" => Ok(Self::Slaac),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Deserialize, Clone, Serialize)]
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

impl std::str::FromStr for Ipv6RaMode {
    type Err = ();
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "dhcpv6-stateful" => Ok(Self::Dhcpv6Stateful),
            "dhcpv6-stateless" => Ok(Self::Dhcpv6Stateless),
            "slaac" => Ok(Self::Slaac),
            _ => Err(()),
        }
    }
}
