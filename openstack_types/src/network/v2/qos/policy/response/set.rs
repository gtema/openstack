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
//! Response type for the put qos/policies/{id} operation

use crate::common::deser_bool_str_opt;
use serde::{Deserialize, Serialize};
use structable::{StructTable, StructTableOptions};

/// Policy response representation
#[derive(Clone, Deserialize, Serialize, StructTable)]
pub struct PolicyResponse {
    /// Time at which the resource has been created (in UTC ISO8601 format).
    ///
    #[structable(optional)]
    pub created_at: Option<String>,

    /// A human-readable description for the resource.
    ///
    #[structable(optional)]
    pub description: Option<String>,

    /// The ID of the QoS policy.
    ///
    #[structable(optional)]
    pub id: Option<String>,

    /// If `true`, the QoS `policy` is the default policy.
    ///
    #[serde(deserialize_with = "deser_bool_str_opt")]
    #[structable(optional)]
    pub is_default: Option<bool>,

    /// Human-readable name of the resource.
    ///
    #[structable(optional)]
    pub name: Option<String>,

    /// The revision number of the resource.
    ///
    #[structable(optional)]
    pub revision_number: Option<i32>,

    /// A set of zero or more policy rules.
    ///
    #[structable(optional)]
    pub rules: Option<String>,

    /// Indicates whether this policy is shared across all projects.
    ///
    #[serde(deserialize_with = "deser_bool_str_opt")]
    #[structable(optional)]
    pub shared: Option<bool>,

    #[structable(optional, serialize)]
    pub tags: Option<Vec<String>>,

    /// The ID of the project.
    ///
    #[structable(optional)]
    pub tenant_id: Option<String>,

    /// Time at which the resource has been updated (in UTC ISO8601 format).
    ///
    #[structable(optional)]
    pub updated_at: Option<String>,
}
