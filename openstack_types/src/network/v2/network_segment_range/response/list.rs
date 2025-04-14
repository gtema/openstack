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
//! Response type for the GET `network-segment-ranges` operation

use crate::common::deser_bool_str_opt;
use crate::common::deser_num_str_opt;
use serde::{Deserialize, Serialize};
use structable::{StructTable, StructTableOptions};

/// NetworkSegmentRange response representation
#[derive(Clone, Deserialize, Serialize, StructTable)]
pub struct NetworkSegmentRangeResponse {
    #[structable(optional, wide)]
    pub available: Option<String>,

    #[structable(optional)]
    pub created_at: Option<String>,

    #[serde(deserialize_with = "deser_bool_str_opt", rename = "default")]
    #[structable(optional, title = "default", wide)]
    pub _default: Option<bool>,

    #[structable(optional, wide)]
    pub description: Option<String>,

    #[structable(optional)]
    pub id: Option<String>,

    #[serde(deserialize_with = "deser_num_str_opt")]
    #[structable(optional, wide)]
    pub maximum: Option<i64>,

    #[serde(deserialize_with = "deser_num_str_opt")]
    #[structable(optional, wide)]
    pub minimum: Option<i64>,

    #[structable(optional)]
    pub name: Option<String>,

    #[structable(optional, serialize, wide)]
    pub network_type: Option<NetworkType>,

    #[structable(optional, wide)]
    pub physical_network: Option<String>,

    #[structable(optional, wide)]
    pub project_id: Option<String>,

    #[structable(optional, wide)]
    pub revision_number: Option<i32>,

    #[serde(deserialize_with = "deser_bool_str_opt")]
    #[structable(optional, wide)]
    pub shared: Option<bool>,

    #[structable(optional, serialize, wide)]
    pub tags: Option<Vec<String>>,

    #[structable(optional)]
    pub updated_at: Option<String>,

    #[structable(optional, wide)]
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
