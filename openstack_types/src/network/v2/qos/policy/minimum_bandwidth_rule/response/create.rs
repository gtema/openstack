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
//! Response type for the post qos/policies/{policy_id}/minimum_bandwidth_rules operation

use crate::common::IntString;
use serde::{Deserialize, Serialize};

/// MinimumBandwidthRule response representation
#[derive(Clone, Deserialize, Serialize)]
struct MinimumBandwidthRuleResponse {
    /// The direction of the traffic to which the QoS rule is applied, as seen
    /// from the point of view of the `port`. Valid values are `egress` and
    /// `ingress`. Default value is `egress`.
    ///
    direction: Option<Direction>,

    /// The ID of the QoS minimum bandwidth rule.
    ///
    id: Option<String>,

    /// The minimum KBPS (kilobits per second) value which should be available
    /// for port.
    ///
    min_kbps: Option<IntString>,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
#[serde(untagged)]
pub enum Direction {
    // Ingress
    #[serde(rename = "ingress")]
    Ingress,

    // Egress
    #[serde(rename = "egress")]
    Egress,
}
