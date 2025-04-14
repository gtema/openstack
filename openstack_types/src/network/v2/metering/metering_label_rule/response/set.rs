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
//! Response type for the PUT `metering/metering-label-rules/{id}` operation

use crate::common::deser_bool_str_opt;
use serde::{Deserialize, Serialize};
use structable::{StructTable, StructTableOptions};

/// MeteringLabelRule response representation
#[derive(Clone, Deserialize, Serialize, StructTable)]
pub struct MeteringLabelRuleResponse {
    #[structable(optional)]
    pub destination_ip_prefix: Option<String>,

    #[structable(optional, serialize)]
    pub direction: Option<Direction>,

    #[serde(deserialize_with = "deser_bool_str_opt")]
    #[structable(optional)]
    pub excluded: Option<bool>,

    #[structable(optional)]
    pub id: Option<String>,

    #[structable(optional)]
    pub metering_label_id: Option<String>,

    #[structable(optional)]
    pub remote_ip_prefix: Option<String>,

    #[structable(optional)]
    pub source_ip_prefix: Option<String>,

    #[structable(optional)]
    pub tenant_id: Option<String>,
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
