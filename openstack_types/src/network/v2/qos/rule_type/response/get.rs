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
//! Response type for the get qos/rule-types/{id} operation

use serde::{Deserialize, Serialize};
use structable::{StructTable, StructTableOptions};

/// RuleType response representation
#[derive(Clone, Deserialize, Serialize, StructTable)]
pub struct RuleTypeResponse {
    /// List of loaded QoS drivers with supported rule type parameters with
    /// possible values for each. Each driver is represented by a dict with the
    /// keys `name` and `supported_parameters`. Field `name` contains the name
    /// of a backend driver. Field `supported_parameters` contains a list of
    /// dicts with `parameter_name`, `parameter_type` and `parameter_values`
    /// fields. The valid values for `parameter_type` are `choices` or `range`.
    /// If `parameter_type` is `choices` then `parameter_values` contains a
    /// list of acceptable values, otherwise it contains a dict with keys of
    /// `start` and `end` which define the range of acceptable values.
    ///
    #[structable(optional)]
    pub drivers: Option<String>,

    /// The type of QoS rule.
    ///
    #[serde(rename = "type")]
    #[structable(optional, title = "type")]
    pub _type: Option<String>,
}
