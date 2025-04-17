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
//! Response type for the GET `vpn/endpoint-groups` operation

use serde::{Deserialize, Serialize};
use structable::{StructTable, StructTableOptions};

/// EndpointGroup response representation
#[derive(Clone, Deserialize, Serialize, StructTable)]
pub struct EndpointGroupResponse {
    /// A human-readable description for the resource. Default is an empty
    /// string.
    #[serde(default)]
    #[structable(optional, wide)]
    pub description: Option<String>,

    /// List of endpoints of the same type, for the endpoint group. The values
    /// will depend on type.
    #[serde(default)]
    #[structable(optional, serialize, wide)]
    pub endpoints: Option<Vec<String>>,

    /// The ID of the VPN endpoint group.
    #[serde(default)]
    #[structable(optional)]
    pub id: Option<String>,

    /// Human-readable name of the resource. Default is an empty string.
    #[serde(default)]
    #[structable(optional)]
    pub name: Option<String>,

    /// The ID of the project.
    #[serde(default)]
    #[structable(optional, wide)]
    pub tenant_id: Option<String>,

    /// The type of the endpoints in the group. A valid value is `subnet`,
    /// `cidr`, `network`, `router`, or `vlan`. Only `subnet` and `cidr` are
    /// supported at this moment.
    #[serde(default, rename = "type")]
    #[structable(optional, serialize, title = "type", wide)]
    pub _type: Option<Type>,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub enum Type {
    // Cidr
    #[serde(rename = "cidr")]
    Cidr,

    // Network
    #[serde(rename = "network")]
    Network,

    // Router
    #[serde(rename = "router")]
    Router,

    // Subnet
    #[serde(rename = "subnet")]
    Subnet,

    // Vlan
    #[serde(rename = "vlan")]
    Vlan,
}

impl std::str::FromStr for Type {
    type Err = ();
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "cidr" => Ok(Self::Cidr),
            "network" => Ok(Self::Network),
            "router" => Ok(Self::Router),
            "subnet" => Ok(Self::Subnet),
            "vlan" => Ok(Self::Vlan),
            _ => Err(()),
        }
    }
}
