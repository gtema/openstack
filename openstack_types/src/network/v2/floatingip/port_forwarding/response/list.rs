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
//! Response type for the GET `floatingips/{floatingip_id}/port_forwardings` operation

use serde::{Deserialize, Serialize};
use structable::{StructTable, StructTableOptions};

/// PortForwarding response representation
#[derive(Clone, Deserialize, Serialize, StructTable)]
pub struct PortForwardingResponse {
    /// A text describing the rule, which helps users to manage/find easily
    /// theirs rules.
    #[serde(default)]
    #[structable(optional, wide)]
    pub description: Option<String>,

    /// The TCP/UDP/other protocol port number of the port forwarding’s
    /// floating IP address.
    #[serde(default)]
    #[structable(optional, wide)]
    pub external_port: Option<f32>,

    /// The TCP/UDP/other protocol port range of the port forwarding’s floating
    /// IP address.
    #[serde(default)]
    #[structable(optional, wide)]
    pub external_port_range: Option<f32>,

    /// The ID of the floating IP port forwarding.
    #[serde(default)]
    #[structable(optional)]
    pub id: Option<String>,

    /// The fixed IPv4 address of the Neutron port associated to the floating
    /// IP port forwarding.
    #[serde(default)]
    #[structable(optional, wide)]
    pub internal_ip_address: Option<String>,

    /// The TCP/UDP/other protocol port number of the Neutron port fixed IP
    /// address associated to the floating ip port forwarding.
    #[serde(default)]
    #[structable(optional, wide)]
    pub internal_port: Option<f32>,

    /// The ID of the Neutron port associated to the floating IP port
    /// forwarding.
    #[serde(default)]
    #[structable(optional, wide)]
    pub internal_port_id: Option<String>,

    /// The TCP/UDP/other protocol port range of the Neutron port fixed IP
    /// address associated to the floating ip port forwarding.
    #[serde(default)]
    #[structable(optional, wide)]
    pub internal_port_range: Option<f32>,

    /// The IP protocol used in the floating IP port forwarding.
    #[serde(default)]
    #[structable(optional, serialize, wide)]
    pub protocol: Option<Protocol>,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub enum Protocol {
    // Dccp
    #[serde(rename = "dccp")]
    Dccp,

    // Icmp
    #[serde(rename = "icmp")]
    Icmp,

    // Ipv6Icmp
    #[serde(rename = "ipv6-icmp")]
    Ipv6Icmp,

    // Sctp
    #[serde(rename = "sctp")]
    Sctp,

    // Tcp
    #[serde(rename = "tcp")]
    Tcp,

    // Udp
    #[serde(rename = "udp")]
    Udp,
}

impl std::str::FromStr for Protocol {
    type Err = ();
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "dccp" => Ok(Self::Dccp),
            "icmp" => Ok(Self::Icmp),
            "ipv6-icmp" => Ok(Self::Ipv6Icmp),
            "sctp" => Ok(Self::Sctp),
            "tcp" => Ok(Self::Tcp),
            "udp" => Ok(Self::Udp),
            _ => Err(()),
        }
    }
}
