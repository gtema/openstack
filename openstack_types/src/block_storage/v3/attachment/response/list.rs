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
//! Response type for the get attachments operation

use serde::{Deserialize, Serialize};
use structable_derive::StructTable;

use crate::common::{OutputConfig, StructTable};

/// Attachment response representation
#[derive(Clone, Deserialize, Serialize, StructTable)]
pub struct AttachmentResponse {
    /// The ID of attachment.
    ///
    #[structable(optional)]
    pub id: Option<String>,

    /// The UUID of the attaching instance.
    ///
    #[structable(optional, serialize, wide)]
    pub instance: Option<String>,

    /// The status of the attachment.
    ///
    #[structable(optional, serialize)]
    pub status: Option<Status>,

    /// The UUID of the volume which the attachment belongs to.
    ///
    #[structable(optional, wide)]
    pub volume_id: Option<String>,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
#[serde(untagged)]
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
