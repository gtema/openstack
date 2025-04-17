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
//! Response type for the GET `security-groups` operation

use serde::{Deserialize, Serialize};
use structable::{StructTable, StructTableOptions};

/// SecurityGroup response representation
#[derive(Clone, Deserialize, Serialize, StructTable)]
pub struct SecurityGroupResponse {
    #[serde(default)]
    #[structable(optional)]
    pub created_at: Option<String>,

    #[serde(default)]
    #[structable(optional, wide)]
    pub description: Option<String>,

    /// The ID of the security group.
    #[serde(default)]
    #[structable(optional)]
    pub id: Option<String>,

    /// Human-readable name of the resource.
    #[serde(default)]
    #[structable(optional)]
    pub name: Option<String>,

    #[serde(default)]
    #[structable(optional, wide)]
    pub revision_number: Option<i32>,

    /// A list of `security_group_rule` objects. Refer to
    /// [Security group rules](#security-group-rules) for details.
    #[serde(default)]
    #[structable(optional, serialize, wide)]
    pub security_group_rules: Option<Vec<SecurityGroupRules>>,

    /// Indicates whether this security group is shared to the requester’s
    /// project.
    #[serde(default, deserialize_with = "crate::common::deser_bool_str_opt")]
    #[structable(optional, wide)]
    pub shared: Option<bool>,

    /// Indicates if the security group is stateful or stateless.
    #[serde(default, deserialize_with = "crate::common::deser_bool_str_opt")]
    #[structable(optional, wide)]
    pub stateful: Option<bool>,

    /// The list of tags on the resource.
    #[serde(default)]
    #[structable(optional, serialize, wide)]
    pub tags: Option<Vec<String>>,

    /// The ID of the project.
    #[serde(default)]
    #[structable(optional, wide)]
    pub tenant_id: Option<String>,

    #[serde(default)]
    #[structable(optional)]
    pub updated_at: Option<String>,
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

/// `SecurityGroupRules` type
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SecurityGroupRules {
    pub belongs_to_default_sg: Option<bool>,
    pub created_at: Option<String>,
    pub description: Option<String>,
    pub direction: Option<Direction>,
    pub ethertype: Option<Ethertype>,
    pub id: Option<String>,
    pub normalized_cidr: Option<String>,
    pub port_range_max: Option<String>,
    pub port_range_min: Option<String>,
    pub protocol: Option<String>,
    pub remote_address_group_id: Option<String>,
    pub remote_group_id: Option<String>,
    pub remote_ip_prefix: Option<String>,
    pub revision_number: Option<i32>,
    pub security_group_id: Option<String>,
    pub tenant_id: Option<String>,
    pub updated_at: Option<String>,
}
