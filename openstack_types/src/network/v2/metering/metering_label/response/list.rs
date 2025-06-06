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
//! Response type for the GET `metering/metering-labels` operation

use serde::{Deserialize, Serialize};
use structable::{StructTable, StructTableOptions};

/// MeteringLabel response representation
#[derive(Clone, Deserialize, Serialize, StructTable)]
pub struct MeteringLabelResponse {
    /// A human-readable description for the resource.
    #[serde(default)]
    #[structable(optional, wide)]
    pub description: Option<String>,

    /// The ID of the metering label.
    #[serde(default)]
    #[structable(optional)]
    pub id: Option<String>,

    /// Human-readable name of the resource.
    #[serde(default)]
    #[structable(optional)]
    pub name: Option<String>,

    /// Indicates whether this metering label is shared across all projects.
    #[serde(default, deserialize_with = "crate::common::deser_bool_str_opt")]
    #[structable(optional, wide)]
    pub shared: Option<bool>,

    /// The ID of the project.
    #[serde(default)]
    #[structable(optional, wide)]
    pub tenant_id: Option<String>,
}
