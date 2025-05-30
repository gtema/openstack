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
//! Response type for the GET `network-segment-ranges/{id}` operation

use serde::{Deserialize, Serialize};
use structable::{StructTable, StructTableOptions};

/// NetworkSegmentRange response representation
#[derive(Clone, Deserialize, Serialize, StructTable)]
pub struct NetworkSegmentRangeResponse {
    #[serde(default)]
    #[structable(optional)]
    pub available: Option<String>,

    #[serde(default)]
    #[structable(optional)]
    pub created_at: Option<String>,

    #[serde(
        default,
        deserialize_with = "crate::common::deser_bool_str_opt",
        rename = "default"
    )]
    #[structable(optional, title = "default")]
    pub _default: Option<bool>,

    #[serde(default)]
    #[structable(optional)]
    pub description: Option<String>,

    #[serde(default)]
    #[structable(optional)]
    pub id: Option<String>,

    #[serde(default, deserialize_with = "crate::common::deser_num_str_opt")]
    #[structable(optional)]
    pub maximum: Option<i64>,

    #[serde(default, deserialize_with = "crate::common::deser_num_str_opt")]
    #[structable(optional)]
    pub minimum: Option<i64>,

    #[serde(default)]
    #[structable(optional)]
    pub name: Option<String>,

    #[serde(default)]
    #[structable(optional, serialize)]
    pub network_type: Option<NetworkType>,

    #[serde(default)]
    #[structable(optional)]
    pub physical_network: Option<String>,

    #[serde(default)]
    #[structable(optional)]
    pub project_id: Option<String>,

    #[serde(default)]
    #[structable(optional)]
    pub revision_number: Option<i32>,

    #[serde(default, deserialize_with = "crate::common::deser_bool_str_opt")]
    #[structable(optional)]
    pub shared: Option<bool>,

    #[serde(default)]
    #[structable(optional, serialize)]
    pub tags: Option<Vec<String>>,

    #[serde(default)]
    #[structable(optional)]
    pub updated_at: Option<String>,

    #[serde(default)]
    #[structable(optional)]
    pub used: Option<String>,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub enum NetworkType {
    // Geneve
    #[serde(rename = "geneve")]
    Geneve,

    // Gre
    #[serde(rename = "gre")]
    Gre,

    // Vlan
    #[serde(rename = "vlan")]
    Vlan,

    // Vxlan
    #[serde(rename = "vxlan")]
    Vxlan,
}

impl std::str::FromStr for NetworkType {
    type Err = ();
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "geneve" => Ok(Self::Geneve),
            "gre" => Ok(Self::Gre),
            "vlan" => Ok(Self::Vlan),
            "vxlan" => Ok(Self::Vxlan),
            _ => Err(()),
        }
    }
}
