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
//! Response type for the get types/{id} operation

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Type response representation
#[derive(Clone, Deserialize, Serialize)]
pub struct TypeResponse {
    /// The volume type description.
    ///
    description: Option<String>,

    /// A key and value pair that contains additional specifications that are
    /// associated with the volume type. Examples include capabilities,
    /// capacity, compression, and so on, depending on the storage driver in
    /// use.
    ///
    extra_specs: Option<HashMap<String, Option<String>>>,

    /// The UUID of the volume type.
    ///
    id: String,

    /// Whether the volume type is publicly visible.
    ///
    is_public: Option<bool>,

    /// The name of the volume type.
    ///
    name: String,

    /// Whether the volume type is publicly visible.
    ///
    #[serde(rename = "os-volume-type-access:is_public")]
    os_volume_type_access_is_public: Option<bool>,

    /// The QoS specifications ID.
    ///
    qos_specs_id: Option<String>,
}
