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
//! Response type for the GET `group_types/{id}` operation

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use structable::{StructTable, StructTableOptions};

/// GroupType response representation
#[derive(Clone, Deserialize, Serialize, StructTable)]
pub struct GroupTypeResponse {
    /// The group type description.
    ///
    #[structable(optional)]
    pub description: Option<String>,

    /// A set of key and value pairs that contains the specifications for a
    /// group type.
    ///
    #[structable(optional, serialize)]
    pub group_specs: Option<HashMap<String, String>>,

    /// The group type ID.
    ///
    #[structable()]
    pub id: String,

    /// Whether the group type is publicly visible.
    ///
    #[structable(optional)]
    pub is_public: Option<bool>,

    /// The group type name.
    ///
    #[structable(optional)]
    pub name: Option<String>,
}
