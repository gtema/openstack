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
//! Response type for the GET `attachments/{id}` operation

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use structable::{StructTable, StructTableOptions};

/// Attachment response representation
#[derive(Clone, Deserialize, Serialize, StructTable)]
pub struct AttachmentResponse {
    /// The attach mode of attachment, read-only (‘ro’) or read-and-write
    /// (‘rw’), default is ‘rw’.
    ///
    #[structable(optional, serialize)]
    pub attach_mode: Option<AttachMode>,

    /// The time when attachment is attached.
    ///
    #[structable(optional)]
    pub attached_at: Option<String>,

    /// The connection info used for server to connect the volume.
    ///
    #[structable(optional, serialize)]
    pub connection_info: Option<HashMap<String, Value>>,

    /// The time when attachment is detached.
    ///
    #[structable(optional)]
    pub detached_at: Option<String>,

    /// The ID of attachment.
    ///
    #[structable()]
    pub id: String,

    /// The UUID of the attaching instance.
    ///
    #[structable(optional)]
    pub instance: Option<String>,

    /// The status of the attachment.
    ///
    #[structable(serialize)]
    pub status: Status,

    /// The UUID of the volume which the attachment belongs to.
    ///
    #[structable()]
    pub volume_id: String,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub enum AttachMode {
    // Ro
    #[serde(rename = "ro")]
    Ro,

    // Rw
    #[serde(rename = "rw")]
    Rw,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub enum Status {
    // Attached
    #[serde(rename = "attached")]
    Attached,

    // Attaching
    #[serde(rename = "attaching")]
    Attaching,

    // Deleted
    #[serde(rename = "deleted")]
    Deleted,

    // Detached
    #[serde(rename = "detached")]
    Detached,

    // ErrorAttaching
    #[serde(rename = "error_attaching")]
    ErrorAttaching,

    // ErrorDetaching
    #[serde(rename = "error_detaching")]
    ErrorDetaching,

    // Reserved
    #[serde(rename = "reserved")]
    Reserved,
}
