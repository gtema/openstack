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
//! Response type for the GET `attachments/detail` operation

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::BTreeMap;
use structable::{StructTable, StructTableOptions};

/// Attachment response representation
#[derive(Clone, Deserialize, Serialize, StructTable)]
pub struct AttachmentResponse {
    /// The attach mode of attachment, read-only (‘ro’) or read-and-write
    /// (‘rw’), default is ‘rw’.
    #[serde(default)]
    #[structable(optional, serialize, wide)]
    pub attach_mode: Option<AttachMode>,

    /// The time when attachment is attached.
    #[serde(default)]
    #[structable(optional, wide)]
    pub attached_at: Option<String>,

    /// The connection info used for server to connect the volume.
    #[serde(default)]
    #[structable(optional, serialize, wide)]
    pub connection_info: Option<BTreeMap<String, Value>>,

    /// The time when attachment is detached.
    #[serde(default)]
    #[structable(optional, wide)]
    pub detached_at: Option<String>,

    /// The ID of attachment.
    #[structable()]
    pub id: String,

    /// The UUID of the attaching instance.
    #[structable(optional, wide)]
    pub instance: Option<String>,

    /// The status of the attachment.
    #[structable(serialize)]
    pub status: Status,

    /// The UUID of the volume which the attachment belongs to.
    #[structable(wide)]
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

impl std::str::FromStr for AttachMode {
    type Err = ();
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "ro" => Ok(Self::Ro),
            "rw" => Ok(Self::Rw),
            _ => Err(()),
        }
    }
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

impl std::str::FromStr for Status {
    type Err = ();
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "attached" => Ok(Self::Attached),
            "attaching" => Ok(Self::Attaching),
            "deleted" => Ok(Self::Deleted),
            "detached" => Ok(Self::Detached),
            "error_attaching" => Ok(Self::ErrorAttaching),
            "error_detaching" => Ok(Self::ErrorDetaching),
            "reserved" => Ok(Self::Reserved),
            _ => Err(()),
        }
    }
}
