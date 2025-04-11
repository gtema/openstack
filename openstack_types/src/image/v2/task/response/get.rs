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
//! Response type for the get tasks/{task_id} operation

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

/// Task response representation
#[derive(Clone, Deserialize, Serialize)]
pub struct TaskResponse {
    /// The date and time when the task was created.
    ///
    /// The date and time stamp format is
    /// [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601).
    ///
    pub created_at: Option<String>,

    /// The date and time when the task is subject to removal. While the *task
    /// object*, that is, the record describing the task is subject to
    /// deletion, the result of the task (for example, an imported image) still
    /// exists.
    ///
    /// The date and time stamp format is
    /// [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601).
    ///
    /// This value is only set when the task reaches status `success` or
    /// `failure`. Otherwise its value is `null`. It may not appear in the
    /// response when its value is `null`.
    ///
    pub expires_at: Option<String>,

    /// The UUID of the task.
    ///
    pub id: Option<String>,

    /// Image associated with the task
    ///
    pub image_id: Option<String>,

    /// A JSON object specifying the input parameters to the task. Consult your
    /// cloud provider’s documentation for details.
    ///
    pub input: Option<HashMap<String, Value>>,

    /// Human-readable text, possibly an empty string, usually displayed in an
    /// error situation to provide more information about what has occurred.
    ///
    pub message: Option<String>,

    /// An identifier for the owner of the task, usually the tenant ID.
    ///
    pub owner: Option<String>,

    /// Human-readable informative request-id
    ///
    pub request_id: Option<String>,

    /// A JSON object specifying information about the ultimate outcome of the
    /// task. Consult your cloud provider’s documentation for details.
    ///
    pub result: Option<HashMap<String, Value>>,

    /// The URI for the schema describing an image task.
    ///
    pub schema: Option<String>,

    /// A URI for this task.
    ///
    #[serde(rename = "self")]
    pub _self: Option<String>,

    /// The current status of this task. The value can be `pending`,
    /// `processing`, `success` or `failure`.
    ///
    pub status: Option<Status>,

    /// The type of task represented by this content.
    ///
    #[serde(rename = "type")]
    pub _type: Option<Type>,

    /// The date and time when the task was updated.
    ///
    /// The date and time stamp format is
    /// [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601).
    ///
    /// If the `updated_at` date and time stamp is not set, its value is
    /// `null`.
    ///
    pub updated_at: Option<String>,

    /// User associated with the task
    ///
    pub user_id: Option<String>,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
#[serde(untagged)]
pub enum Type {
    // ApiImageImport
    #[serde(rename = "api_image_import")]
    ApiImageImport,

    // Import
    #[serde(rename = "import")]
    Import,

    // LocationImport
    #[serde(rename = "location_import")]
    LocationImport,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
#[serde(untagged)]
pub enum Status {
    // Failure
    #[serde(rename = "failure")]
    Failure,

    // Pending
    #[serde(rename = "pending")]
    Pending,

    // Processing
    #[serde(rename = "processing")]
    Processing,

    // Success
    #[serde(rename = "success")]
    Success,
}
