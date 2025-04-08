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
//! Response type for the get images/{image_id}/tasks operation

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

/// Task response representation
#[derive(Clone, Deserialize, Serialize)]
struct TaskResponse {
    /// Datetime when this resource was created
    ///
    created_at: Option<String>,

    /// Datetime when this resource would be subject to removal
    ///
    expires_at: Option<String>,

    /// An identifier for the task
    ///
    id: Option<String>,

    /// Image associated with the task
    ///
    image_id: Option<String>,

    /// The parameters required by task, JSON blob
    ///
    input: Option<HashMap<String, Value>>,

    /// Human-readable informative message only included when appropriate
    /// (usually on failure)
    ///
    message: Option<String>,

    /// An identifier for the owner of this task
    ///
    owner: Option<String>,

    /// Human-readable informative request-id
    ///
    request_id: Option<String>,

    /// The result of current task, JSON blob
    ///
    result: Option<HashMap<String, Value>>,

    schema: Option<String>,

    #[serde(rename = "self")]
    _self: Option<String>,

    /// The current status of this task
    ///
    status: Option<Status>,

    /// The type of task represented by this content
    ///
    #[serde(rename = "type")]
    _type: Option<Type>,

    /// Datetime when this resource was updated
    ///
    updated_at: Option<String>,

    /// User associated with the task
    ///
    user_id: Option<String>,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
#[serde(untagged)]
pub enum Type {
    // Import
    #[serde(rename = "import")]
    Import,

    // LocationImport
    #[serde(rename = "location_import")]
    LocationImport,

    // ApiImageImport
    #[serde(rename = "api_image_import")]
    ApiImageImport,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
#[serde(untagged)]
pub enum Status {
    // Pending
    #[serde(rename = "pending")]
    Pending,

    // Failure
    #[serde(rename = "failure")]
    Failure,

    // Success
    #[serde(rename = "success")]
    Success,

    // Processing
    #[serde(rename = "processing")]
    Processing,
}
