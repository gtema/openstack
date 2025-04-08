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
//! Response type for the get segments operation

use crate::common::IntString;
use serde::{Deserialize, Serialize};

/// Segment response representation
#[derive(Clone, Deserialize, Serialize)]
struct SegmentResponse {
    /// Time at which the resource has been created (in UTC ISO8601 format).
    ///
    created_at: Option<String>,

    /// A human-readable description for the resource.
    ///
    description: Option<String>,

    /// The UUID of the segment.
    ///
    id: Option<String>,

    /// Human-readable name of the resource.
    ///
    name: Option<String>,

    /// The ID of the attached network.
    ///
    network_id: Option<String>,

    /// The type of physical network that maps to this network resource. For
    /// example, `flat`, `vlan`, `vxlan`, or `gre`.
    ///
    network_type: Option<String>,

    /// The physical network where this network/segment is implemented.
    ///
    physical_network: Option<String>,

    /// The revision number of the resource.
    ///
    revision_number: Option<i32>,

    /// The ID of the isolated segment on the physical network. The
    /// `network_type` attribute defines the segmentation model. For example,
    /// if the `network_type` value is vlan, this ID is a vlan identifier. If
    /// the `network_type` value is gre, this ID is a gre key. `Note` that only
    /// the segmentation-id of VLAN type networks can be changed!
    ///
    segmentation_id: Option<IntString>,

    /// Time at which the resource has been updated (in UTC ISO8601 format).
    ///
    updated_at: Option<String>,
}
