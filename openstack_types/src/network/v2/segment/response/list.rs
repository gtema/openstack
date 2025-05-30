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
//! Response type for the GET `segments` operation

use serde::{Deserialize, Serialize};
use structable::{StructTable, StructTableOptions};

/// Segment response representation
#[derive(Clone, Deserialize, Serialize, StructTable)]
pub struct SegmentResponse {
    /// Time at which the resource has been created (in UTC ISO8601 format).
    #[serde(default)]
    #[structable(optional)]
    pub created_at: Option<String>,

    /// A human-readable description for the resource.
    #[serde(default)]
    #[structable(optional, wide)]
    pub description: Option<String>,

    /// The UUID of the segment.
    #[serde(default)]
    #[structable(optional)]
    pub id: Option<String>,

    /// Human-readable name of the resource.
    #[serde(default)]
    #[structable(optional)]
    pub name: Option<String>,

    /// The ID of the attached network.
    #[serde(default)]
    #[structable(optional, wide)]
    pub network_id: Option<String>,

    /// The type of physical network that maps to this network resource. For
    /// example, `flat`, `vlan`, `vxlan`, or `gre`.
    #[serde(default)]
    #[structable(optional, wide)]
    pub network_type: Option<String>,

    /// The physical network where this network/segment is implemented.
    #[serde(default)]
    #[structable(optional, wide)]
    pub physical_network: Option<String>,

    /// The revision number of the resource.
    #[serde(default)]
    #[structable(optional, wide)]
    pub revision_number: Option<i32>,

    /// The ID of the isolated segment on the physical network. The
    /// `network_type` attribute defines the segmentation model. For example,
    /// if the `network_type` value is vlan, this ID is a vlan identifier. If
    /// the `network_type` value is gre, this ID is a gre key. `Note` that only
    /// the segmentation-id of VLAN type networks can be changed!
    #[serde(default, deserialize_with = "crate::common::deser_num_str_opt")]
    #[structable(optional, wide)]
    pub segmentation_id: Option<i64>,

    /// Time at which the resource has been updated (in UTC ISO8601 format).
    #[serde(default)]
    #[structable(optional)]
    pub updated_at: Option<String>,
}
