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
//! Response type for the POST `log/logs` operation

use serde::{Deserialize, Serialize};
use structable::{StructTable, StructTableOptions};

/// Log response representation
#[derive(Clone, Deserialize, Serialize, StructTable)]
pub struct LogResponse {
    /// Time at which the resource has been created (in UTC ISO8601 format).
    #[serde(default)]
    #[structable(optional)]
    pub created_at: Option<String>,

    /// A human-readable description for the resource.
    #[serde(default)]
    #[structable(optional)]
    pub description: Option<String>,

    /// Indicates whether this log object is enabled or disabled.
    #[serde(default, deserialize_with = "crate::common::deser_bool_str_opt")]
    #[structable(optional)]
    pub enabled: Option<bool>,

    /// Type of security events to log. `ACCEPT`, `DROP`, or `ALL`.
    #[serde(default)]
    #[structable(optional, serialize)]
    pub event: Option<Event>,

    /// The ID of the log object.
    #[serde(default)]
    #[structable(optional)]
    pub id: Option<String>,

    /// Human-readable name of the resource.
    #[serde(default)]
    #[structable(optional)]
    pub name: Option<String>,

    /// The ID of the project.
    #[serde(default)]
    #[structable(optional)]
    pub project_id: Option<String>,

    /// The ID of resource log (e.g security group ID).
    #[serde(default)]
    #[structable(optional)]
    pub resource_id: Option<String>,

    /// The resource log type such as ‘security_group’.
    #[serde(default)]
    #[structable(optional)]
    pub resource_type: Option<String>,

    /// The revision number of the resource.
    #[serde(default)]
    #[structable(optional)]
    pub revision_number: Option<i32>,

    /// The ID of resource target log such as port ID.
    #[serde(default)]
    #[structable(optional)]
    pub target_id: Option<String>,

    /// Time at which the resource has been updated (in UTC ISO8601 format).
    #[serde(default)]
    #[structable(optional)]
    pub updated_at: Option<String>,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub enum Event {
    // Accept
    #[serde(rename = "ACCEPT")]
    Accept,

    // All
    #[serde(rename = "ALL")]
    All,

    // Drop
    #[serde(rename = "DROP")]
    Drop,
}

impl std::str::FromStr for Event {
    type Err = ();
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "ACCEPT" => Ok(Self::Accept),
            "ALL" => Ok(Self::All),
            "DROP" => Ok(Self::Drop),
            _ => Err(()),
        }
    }
}
