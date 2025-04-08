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
//! Response type for the get types/{type_id}/encryption operation

use serde::{Deserialize, Serialize};

/// Encryption response representation
#[derive(Clone, Deserialize, Serialize)]
pub struct EncryptionResponse {
    /// The encryption algorithm or mode. For example, aes-xts-plain64. The
    /// default value is None.
    ///
    pub cipher: Option<String>,

    /// Notional service where encryption is performed. Valid values are
    /// “front-end” or “back-end”. The default value is “front-end”.
    ///
    pub control_location: Option<ControlLocation>,

    /// The date and time when the resource was created.
    ///
    pub created_at: Option<String>,

    /// The resource is deleted or not.
    ///
    pub deleted: Option<bool>,

    /// The date and time when the resource was deleted.
    ///
    pub deleted_at: Option<String>,

    /// The UUID of the encryption.
    ///
    pub encryption_id: Option<String>,

    /// Size of encryption key, in bits. This is usually 256. The default value
    /// is None.
    ///
    pub key_size: Option<i32>,

    /// The class that provides encryption support.
    ///
    pub provider: Option<String>,

    /// The date and time when the resource was updated.
    ///
    pub updated_at: Option<String>,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
#[serde(untagged)]
pub enum ControlLocation {
    // FrontEnd
    #[serde(rename = "front-end")]
    FrontEnd,

    // BackEnd
    #[serde(rename = "back-end")]
    BackEnd,
}
