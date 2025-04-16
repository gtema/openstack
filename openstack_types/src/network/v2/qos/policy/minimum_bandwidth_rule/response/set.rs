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
//! Response type for the PUT `qos/policies/{policy_id}/minimum_bandwidth_rules/{id}` operation

use serde::{Deserialize, Serialize};
use structable::{StructTable, StructTableOptions};

/// MinimumBandwidthRule response representation
#[derive(Clone, Deserialize, Serialize, StructTable)]
pub struct MinimumBandwidthRuleResponse {
    /// The direction of the traffic to which the QoS rule is applied, as seen
    /// from the point of view of the `port`. Valid values are `egress` and
    /// `ingress`.
    #[serde(default)]
    #[structable(optional, serialize)]
    pub direction: Option<Direction>,

    /// The ID of the QoS minimum bandwidth rule.
    #[serde(default)]
    #[structable(optional)]
    pub id: Option<String>,

    /// The minimum KBPS (kilobits per second) value which should be available
    /// for port.
    #[serde(default, deserialize_with = "crate::common::deser_num_str_opt")]
    #[structable(optional)]
    pub min_kbps: Option<i64>,
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
