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
//! Response type for the get security-group-rules operation

use serde::{Deserialize, Serialize};
use structable::{StructTable, StructTableOptions};

/// SecurityGroupRule response representation
#[derive(Clone, Deserialize, Serialize, StructTable)]
pub struct SecurityGroupRuleResponse {
    /// Indicates if the security group rule belongs to the default security
    /// group of the project or not.
    ///
    #[structable(optional, serialize, wide)]
    pub belongs_to_default_sg: Option<bool>,

    /// Time at which the resource has been created (in UTC ISO8601 format).
    ///
    #[structable(optional)]
    pub created_at: Option<String>,

    /// A human-readable description for the resource.
    ///
    #[structable(optional, wide)]
    pub description: Option<String>,

    /// Ingress or egress, which is the direction in which the security group
    /// rule is applied.
    ///
    #[structable(optional, serialize, wide)]
    pub direction: Option<Direction>,

    /// Must be IPv4 or IPv6, and addresses represented in CIDR must match the
    /// ingress or egress rules.
    ///
    #[structable(optional, serialize, wide)]
    pub ethertype: Option<Ethertype>,

    /// The ID of the security group rule.
    ///
    #[structable(optional)]
    pub id: Option<String>,

    #[structable(optional, serialize, wide)]
    pub normalized_cidr: Option<String>,

    /// The maximum port number in the range that is matched by the security
    /// group rule. If the protocol is TCP, UDP, DCCP, SCTP or UDP-Lite this
    /// value must be greater than or equal to the `port_range_min` attribute
    /// value. If the protocol is ICMP, this value must be an ICMP code.
    ///
    #[structable(optional, serialize, wide)]
    pub port_range_max: Option<i32>,

    /// The minimum port number in the range that is matched by the security
    /// group rule. If the protocol is TCP, UDP, DCCP, SCTP or UDP-Lite this
    /// value must be less than or equal to the `port_range_max` attribute
    /// value. If the protocol is ICMP, this value must be an ICMP type.
    ///
    #[structable(optional, serialize, wide)]
    pub port_range_min: Option<i32>,

    /// The IP protocol can be represented by a string, an integer, or `null`.
    /// Valid string or integer values are `any` or `0`, `ah` or `51`, `dccp`
    /// or `33`, `egp` or `8`, `esp` or `50`, `gre` or `47`, `icmp` or `1`,
    /// `icmpv6` or `58`, `igmp` or `2`, `ipip` or `4`, `ipv6-encap` or `41`,
    /// `ipv6-frag` or `44`, `ipv6-icmp` or `58`, `ipv6-nonxt` or `59`,
    /// `ipv6-opts` or `60`, `ipv6-route` or `43`, `ospf` or `89`, `pgm` or
    /// `113`, `rsvp` or `46`, `sctp` or `132`, `tcp` or `6`, `udp` or `17`,
    /// `udplite` or `136`, `vrrp` or `112`. Additionally, any integer value
    /// between [0-255] is also valid. The string `any` (or integer `0`) means
    /// `all` IP protocols. See the constants in `neutron_lib.constants` for
    /// the most up-to-date list of supported strings.
    ///
    #[structable(optional, wide)]
    pub protocol: Option<String>,

    /// The remote address group UUID that is associated with this security
    /// group rule.
    ///
    #[structable(optional, wide)]
    pub remote_address_group_id: Option<String>,

    /// The remote group UUID to associate with this security group rule. You
    /// can specify either the `remote_group_id` or `remote_ip_prefix`
    /// attribute in the request body.
    ///
    #[structable(optional, wide)]
    pub remote_group_id: Option<String>,

    /// The remote IP prefix that is matched by this security group rule.
    ///
    #[structable(optional, wide)]
    pub remote_ip_prefix: Option<String>,

    /// The revision number of the resource.
    ///
    #[structable(optional, wide)]
    pub revision_number: Option<i32>,

    /// The security group ID to associate with this security group rule.
    ///
    #[structable(optional, wide)]
    pub security_group_id: Option<String>,

    /// The ID of the project.
    ///
    #[structable(optional, wide)]
    pub tenant_id: Option<String>,

    /// Time at which the resource has been updated (in UTC ISO8601 format).
    ///
    #[structable(optional)]
    pub updated_at: Option<String>,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
#[serde(untagged)]
pub enum Direction {
    // Egress
    #[serde(rename = "egress")]
    Egress,

    // Ingress
    #[serde(rename = "ingress")]
    Ingress,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
#[serde(untagged)]
pub enum Ethertype {
    // Ipv4
    #[serde(rename = "IPv4")]
    Ipv4,

    // Ipv6
    #[serde(rename = "IPv6")]
    Ipv6,
}
