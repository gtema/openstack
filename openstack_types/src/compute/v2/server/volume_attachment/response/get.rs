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
//! Response type for the get servers/{server_id}/os-volume_attachments/{id} operation

use serde::{Deserialize, Serialize};
use structable::{StructTable, StructTableOptions};

/// VolumeAttachment response representation
#[derive(Clone, Deserialize, Serialize, StructTable)]
pub struct VolumeAttachmentResponse {
    /// The UUID of the associated volume attachment in Cinder.
    ///
    /// **New in version 2.89**
    ///
    #[structable(optional)]
    pub attachment_id: Option<String>,

    /// The UUID of the block device mapping record in Nova for the attachment.
    ///
    /// **New in version 2.89**
    ///
    #[structable(optional)]
    pub bdm_uuid: Option<String>,

    /// A flag indicating if the attached volume will be deleted when the
    /// server is deleted.
    ///
    /// **New in version 2.79**
    ///
    #[structable(optional)]
    pub delete_on_termination: Option<bool>,

    /// Name of the device in the attachment object, such as, `/dev/vdb`.
    ///
    #[structable()]
    pub device: String,

    /// The volume ID of the attachment.
    ///
    /// **Available until version 2.88**
    ///
    #[structable()]
    pub id: String,

    /// The UUID of the server.
    ///
    #[serde(rename = "serverId")]
    #[structable(title = "serverId")]
    pub server_id: String,

    /// The device tag applied to the volume block device or `null`.
    ///
    /// **New in version 2.70**
    ///
    #[structable(optional, serialize)]
    pub tag: Option<String>,

    /// The UUID of the attached volume.
    ///
    #[serde(rename = "volumeId")]
    #[structable(title = "volumeId")]
    pub volume_id: String,
}
