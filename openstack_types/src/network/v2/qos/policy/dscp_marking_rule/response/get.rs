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
//! Response type for the GET `qos/policies/{policy_id}/dscp_marking_rules/{id}` operation

use serde::{Deserialize, Serialize};
use structable::{StructTable, StructTableOptions};

/// DscpMarkingRule response representation
#[derive(Clone, Deserialize, Serialize, StructTable)]
pub struct DscpMarkingRuleResponse {
    /// The DSCP mark value.
    ///
    #[structable(optional)]
    pub dscp_mark: Option<i32>,

    /// The ID of the QoS DSCP marking rule.
    ///
    #[structable(optional)]
    pub id: Option<String>,
}
