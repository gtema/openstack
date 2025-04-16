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
//! Response type for the PUT `default-security-group-rules/{id}` operation

use serde::{Deserialize, Serialize};
use structable::{StructTable, StructTableOptions};

/// DefaultSecurityGroupRule response representation
#[derive(Clone, Deserialize, Serialize, StructTable)]
pub struct DefaultSecurityGroupRuleResponse {
    #[serde(default)]
    #[structable(optional)]
    pub description: Option<String>,

    #[serde(default)]
    #[structable(optional, serialize)]
    pub direction: Option<Direction>,

    #[serde(default)]
    #[structable(optional, serialize)]
    pub ethertype: Option<Ethertype>,

    #[serde(default)]
    #[structable(optional)]
    pub id: Option<String>,

    #[serde(default)]
    #[structable(optional)]
    pub port_range_max: Option<i32>,

    #[serde(default)]
    #[structable(optional)]
    pub port_range_min: Option<i32>,

    #[serde(default)]
    #[structable(optional)]
    pub protocol: Option<String>,

    #[serde(default)]
    #[structable(optional)]
    pub remote_address_group_id: Option<String>,

    #[serde(default)]
    #[structable(optional)]
    pub remote_group_id: Option<String>,

    #[serde(default)]
    #[structable(optional)]
    pub remote_ip_prefix: Option<String>,

    #[serde(default, deserialize_with = "crate::common::deser_bool_str_opt")]
    #[structable(optional)]
    pub used_in_default_sg: Option<bool>,

    #[serde(default, deserialize_with = "crate::common::deser_bool_str_opt")]
    #[structable(optional)]
    pub used_in_non_default_sg: Option<bool>,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub enum Direction {
    // Egress
    #[serde(rename = "egress")]
    Egress,

    // Ingress
    #[serde(rename = "ingress")]
    Ingress,
}

impl std::str::FromStr for Direction {
    type Err = ();
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "egress" => Ok(Self::Egress),
            "ingress" => Ok(Self::Ingress),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub enum Ethertype {
    // Ipv4
    #[serde(rename = "IPv4")]
    Ipv4,

    // Ipv6
    #[serde(rename = "IPv6")]
    Ipv6,
}

impl std::str::FromStr for Ethertype {
    type Err = ();
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "IPv4" => Ok(Self::Ipv4),
            "IPv6" => Ok(Self::Ipv6),
            _ => Err(()),
        }
    }
}
