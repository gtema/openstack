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
//! Response type for the get qos/policies/{policy_id}/bandwidth_limit_rules operation

use crate::common::IntString;
use serde::{Deserialize, Serialize};

/// BandwidthLimitRule response representation
#[derive(Clone, Deserialize, Serialize)]
pub struct BandwidthLimitRuleResponse {
    /// The direction of the traffic to which the QoS rule is applied, as seen
    /// from the point of view of the `port`. Valid values are `egress` and
    /// `ingress`. Default value is `egress`.
    ///
    pub direction: Option<Direction>,

    /// The ID of the QoS Bandwidth limit rule.
    ///
    pub id: Option<String>,

    /// The maximum burst size (in kilobits).
    ///
    pub max_burst_kbps: Option<IntString>,

    /// The maximum KBPS (kilobits per second) value. If you specify this
    /// value, must be greater than 0 otherwise max_kbps will have no value.
    ///
    pub max_kbps: Option<IntString>,
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
