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
//! Response type for the get routers/{router_id}/conntrack_helpers operation

use crate::common::deser_num_str_opt;
use serde::{Deserialize, Serialize};
use structable::{StructTable, StructTableOptions};

/// ConntrackHelper response representation
#[derive(Clone, Deserialize, Serialize, StructTable)]
pub struct ConntrackHelperResponse {
    /// The netfilter conntrack helper module.
    ///
    #[structable(optional, wide)]
    pub helper: Option<String>,

    /// The ID of the conntrack helper.
    ///
    #[structable(optional)]
    pub id: Option<String>,

    /// The network port for the netfilter conntrack target rule.
    ///
    #[serde(deserialize_with = "deser_num_str_opt")]
    #[structable(optional, wide)]
    pub port: Option<i64>,

    /// The network protocol for the netfilter conntrack target rule.
    ///
    #[structable(optional, serialize, wide)]
    pub protocol: Option<Protocol>,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
#[serde(untagged)]
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
