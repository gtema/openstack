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
//! Response type for the get network-segment-ranges operation

use crate::common::BoolString;
use crate::common::IntString;
use serde::{Deserialize, Serialize};

/// NetworkSegmentRange response representation
#[derive(Clone, Deserialize, Serialize)]
struct NetworkSegmentRangeResponse {
    available: Option<String>,

    created_at: Option<String>,

    #[serde(rename = "default")]
    _default: Option<BoolString>,

    description: Option<String>,

    id: Option<String>,

    maximum: Option<IntString>,

    minimum: Option<IntString>,

    name: Option<String>,

    network_type: Option<NetworkType>,

    physical_network: Option<String>,

    project_id: Option<String>,

    revision_number: Option<i32>,

    shared: Option<BoolString>,

    tags: Option<Vec<String>>,

    updated_at: Option<String>,

    used: Option<String>,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
#[serde(untagged)]
pub enum NetworkType {
    // Vxlan
    #[serde(rename = "vxlan")]
    Vxlan,

    // Gre
    #[serde(rename = "gre")]
    Gre,

    // Vlan
    #[serde(rename = "vlan")]
    Vlan,

    // Geneve
    #[serde(rename = "geneve")]
    Geneve,
}
