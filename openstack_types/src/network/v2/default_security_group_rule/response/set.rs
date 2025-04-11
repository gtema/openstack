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
//! Response type for the put default-security-group-rules/{id} operation

use crate::common::BoolString;
use serde::{Deserialize, Serialize};

/// DefaultSecurityGroupRule response representation
#[derive(Clone, Deserialize, Serialize)]
pub struct DefaultSecurityGroupRuleResponse {
    pub description: Option<String>,

    pub direction: Option<Direction>,

    pub ethertype: Option<Ethertype>,

    pub id: Option<String>,

    pub port_range_max: Option<i32>,

    pub port_range_min: Option<i32>,

    pub protocol: Option<String>,

    pub remote_address_group_id: Option<String>,

    pub remote_group_id: Option<String>,

    pub remote_ip_prefix: Option<String>,

    pub used_in_default_sg: Option<BoolString>,

    pub used_in_non_default_sg: Option<BoolString>,
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
