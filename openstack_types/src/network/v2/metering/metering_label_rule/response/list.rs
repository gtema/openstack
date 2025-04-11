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
//! Response type for the get metering/metering-label-rules operation

use crate::common::BoolString;
use serde::{Deserialize, Serialize};

/// MeteringLabelRule response representation
#[derive(Clone, Deserialize, Serialize)]
pub struct MeteringLabelRuleResponse {
    pub destination_ip_prefix: Option<String>,

    /// Ingress or egress, which is the direction in which the metering rule is
    /// applied.
    ///
    pub direction: Option<Direction>,

    /// Indicates whether to count the traffic of a specific IP address with
    /// the `remote_ip_prefix`, `source_ip_prefix`, or `destination_ip_prefix`
    /// values.
    ///
    pub excluded: Option<BoolString>,

    /// The ID of the metering label rule.
    ///
    pub id: Option<String>,

    /// The metering label ID associated with this metering rule.
    ///
    pub metering_label_id: Option<String>,

    /// (deprecated) The source IP prefix that is matched by this metering
    /// rule. By source IP prefix, one should read the internal/private IPs
    /// used in OpenStack.
    ///
    pub remote_ip_prefix: Option<String>,

    pub source_ip_prefix: Option<String>,

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
