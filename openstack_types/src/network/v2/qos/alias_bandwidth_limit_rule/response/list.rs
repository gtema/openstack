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
//! Response type for the get qos/alias-bandwidth-limit-rules operation

use crate::common::IntString;
use serde::{Deserialize, Serialize};

/// AliasBandwidthLimitRule response representation
#[derive(Clone, Deserialize, Serialize)]
pub struct AliasBandwidthLimitRuleResponse {
    pub direction: Option<Direction>,

    pub id: Option<String>,

    pub max_burst_kbps: Option<IntString>,

    pub max_kbps: Option<IntString>,

    pub tenant_id: Option<String>,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
#[serde(untagged)]
pub enum Direction {
    // Egress
    #[serde(rename = "egress")]
    Egress,

    // Ingress
    #[serde(rename = "ingress")]
    Ingress,
}
