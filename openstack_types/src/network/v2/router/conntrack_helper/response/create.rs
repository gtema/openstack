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
//! Response type for the post routers/{router_id}/conntrack_helpers operation

use crate::common::IntString;
use serde::{Deserialize, Serialize};

/// ConntrackHelper response representation
#[derive(Clone, Deserialize, Serialize)]
pub struct ConntrackHelperResponse {
    /// The netfilter conntrack helper module.
    ///
    pub helper: Option<String>,

    /// The ID of the conntrack helper.
    ///
    pub id: Option<String>,

    /// The network port for the netfilter conntrack target rule.
    ///
    pub port: Option<IntString>,

    /// The network protocol for the netfilter conntrack target rule.
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
