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
//! Response type for the get floatingips/{floatingip_id}/port_forwardings/{id} operation

use serde::{Deserialize, Serialize};

/// PortForwarding response representation
#[derive(Clone, Deserialize, Serialize)]
pub struct PortForwardingResponse {
    /// A text describing the rule, which helps users to manage/find easily
    /// theirs rules.
    ///
    pub description: Option<String>,

    /// The TCP/UDP/other protocol port number of the port forwarding’s
    /// floating IP address.
    ///
    pub external_port: Option<f32>,

    /// The TCP/UDP/other protocol port range of the port forwarding’s floating
    /// IP address.
    ///
    pub external_port_range: Option<f32>,

    /// The ID of the floating IP port forwarding.
    ///
    pub id: Option<String>,

    /// The fixed IPv4 address of the Neutron port associated to the floating
    /// IP port forwarding.
    ///
    pub internal_ip_address: Option<String>,

    /// The TCP/UDP/other protocol port number of the Neutron port fixed IP
    /// address associated to the floating ip port forwarding.
    ///
    pub internal_port: Option<f32>,

    /// The ID of the Neutron port associated to the floating IP port
    /// forwarding.
    ///
    pub internal_port_id: Option<String>,

    /// The TCP/UDP/other protocol port range of the Neutron port fixed IP
    /// address associated to the floating ip port forwarding.
    ///
    pub internal_port_range: Option<f32>,

    /// The IP protocol used in the floating IP port forwarding.
    ///
    pub protocol: Option<Protocol>,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
#[serde(untagged)]
pub enum Protocol {
    // Ipv6Icmp
    #[serde(rename = "ipv6-icmp")]
    Ipv6Icmp,

    // Udp
    #[serde(rename = "udp")]
    Udp,

    // Tcp
    #[serde(rename = "tcp")]
    Tcp,

    // Dccp
    #[serde(rename = "dccp")]
    Dccp,

    // Sctp
    #[serde(rename = "sctp")]
    Sctp,

    // Icmp
    #[serde(rename = "icmp")]
    Icmp,
}
