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
//! Response type for the GET `projects/{project_id}/groups/{group_id}/roles` operation

use serde::{Deserialize, Serialize};
use structable::{StructTable, StructTableOptions};

/// Role response representation
#[derive(Clone, Deserialize, Serialize, StructTable)]
pub struct RoleResponse {
    /// The role description.
    #[serde(default)]
    #[structable(optional, wide)]
    pub description: Option<String>,

    /// The role ID.
    #[serde(default)]
    #[structable(optional)]
    pub id: Option<String>,

    /// The role name.
    #[serde(default)]
    #[structable(optional)]
    pub name: Option<String>,
}

/// The link to the resources in question.
/// `Links` type
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Links {
    pub _self: Option<String>,
}
