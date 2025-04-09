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
//! Response type for the get {account}/{container} operation

use serde::{Deserialize, Serialize};

/// Container response representation
#[derive(Clone, Deserialize, Serialize)]
pub struct ContainerResponse {
    /// The total number of bytes that are stored in Object Storage for the
    /// container.
    ///
    pub bytes: Option<i64>,

    /// The content type of the object.
    ///
    pub content_type: Option<String>,

    /// The MD5 checksum value of the object content.
    ///
    pub hash: Option<String>,

    /// The date and time when the object was last modified. The date and time
    /// stamp format is ISO 8601
    ///
    pub last_modified: Option<String>,

    /// The name of the container.
    ///
    pub name: Option<String>,

    /// This field exists only when the object is symlink. This is the target
    /// path of the symlink object.
    ///
    pub symlink_path: Option<String>,
}
