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
//! Response type for the POST `tasks` operation

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::BTreeMap;
use structable::{StructTable, StructTableOptions};

/// Task response representation
#[derive(Clone, Deserialize, Serialize, StructTable)]
pub struct TaskResponse {
    /// The date and time when the task was created.
    ///
    /// The date and time stamp format is
    /// [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601).
    #[serde(default)]
    #[structable(optional)]
    pub created_at: Option<String>,

    /// Datetime when this resource would be subject to removal
    #[serde(default)]
    #[structable(optional)]
    pub expires_at: Option<String>,

    /// The UUID of the task.
    #[serde(default)]
    #[structable(optional)]
    pub id: Option<String>,

    /// Image associated with the task
    #[serde(default)]
    #[structable(optional)]
    pub image_id: Option<String>,

    /// A JSON object specifying the input parameters to the task. Consult your
    /// cloud provider’s documentation for details.
    #[serde(default)]
    #[structable(optional, serialize)]
    pub input: Option<BTreeMap<String, Value>>,

    /// Human-readable text, possibly an empty string, usually displayed in an
    /// error situation to provide more information about what has occurred.
    #[serde(default)]
    #[structable(optional)]
    pub message: Option<String>,

    /// An identifier for the owner of the task, usually the tenant ID.
    #[serde(default)]
    #[structable(optional)]
    pub owner: Option<String>,

    /// Human-readable informative request-id
    #[serde(default)]
    #[structable(optional)]
    pub request_id: Option<String>,

    /// A JSON object specifying information about the ultimate outcome of the
    /// task. Consult your cloud provider’s documentation for details.
    #[serde(default)]
    #[structable(optional, serialize)]
    pub result: Option<BTreeMap<String, Value>>,

    /// The URI for the schema describing an image task.
    #[serde(default)]
    #[structable(optional)]
    pub schema: Option<String>,

    /// A URI for this task.
    #[serde(default, rename = "self")]
    #[structable(optional, title = "self")]
    pub _self: Option<String>,

    /// The current status of this task. The value can be `pending`,
    /// `processing`, `success` or `failure`.
    #[serde(default)]
    #[structable(optional, serialize)]
    pub status: Option<Status>,

    /// The type of task represented by this content.
    #[serde(default, rename = "type")]
    #[structable(optional, serialize, title = "type")]
    pub _type: Option<Type>,

    /// The date and time when the task was updated.
    ///
    /// The date and time stamp format is
    /// [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601).
    ///
    /// If the `updated_at` date and time stamp is not set, its value is
    /// `null`.
    #[serde(default)]
    #[structable(optional)]
    pub updated_at: Option<String>,

    /// User associated with the task
    #[serde(default)]
    #[structable(optional)]
    pub user_id: Option<String>,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
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

impl std::str::FromStr for Type {
    type Err = ();
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "api_image_import" => Ok(Self::ApiImageImport),
            "import" => Ok(Self::Import),
            "location_import" => Ok(Self::LocationImport),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Deserialize, Clone, Serialize)]
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

impl std::str::FromStr for Status {
    type Err = ();
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "failure" => Ok(Self::Failure),
            "pending" => Ok(Self::Pending),
            "processing" => Ok(Self::Processing),
            "success" => Ok(Self::Success),
            _ => Err(()),
        }
    }
}
