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
//! Response type for the post log/logs operation

use crate::common::BoolString;
use serde::{Deserialize, Serialize};

/// Log response representation
#[derive(Clone, Deserialize, Serialize)]
struct LogResponse {
    /// Time at which the resource has been created (in UTC ISO8601 format).
    ///
    created_at: Option<String>,

    /// A human-readable description for the resource.
    ///
    description: Option<String>,

    /// Indicates whether this log object is enabled or disabled.
    ///
    enabled: Option<BoolString>,

    /// Type of security events to log. `ACCEPT`, `DROP`, or `ALL`.
    ///
    event: Option<Event>,

    /// The ID of the log object.
    ///
    id: Option<String>,

    /// Human-readable name of the resource.
    ///
    name: Option<String>,

    /// The ID of the project.
    ///
    project_id: Option<String>,

    /// The ID of resource log (e.g security group ID).
    ///
    resource_id: Option<String>,

    /// The resource log type such as ‘security_group’.
    ///
    resource_type: Option<String>,

    /// The revision number of the resource.
    ///
    revision_number: Option<i32>,

    /// The ID of resource target log such as port ID.
    ///
    target_id: Option<String>,

    /// Time at which the resource has been updated (in UTC ISO8601 format).
    ///
    updated_at: Option<String>,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
#[serde(untagged)]
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
