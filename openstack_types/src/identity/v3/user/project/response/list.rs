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
//! Response type for the get users/{user_id}/projects operation

use serde::{Deserialize, Serialize};
use structable::{StructTable, StructTableOptions};

/// Project response representation
#[derive(Clone, Deserialize, Serialize, StructTable)]
pub struct ProjectResponse {
    /// The description of the project.
    ///
    #[structable(optional, wide)]
    pub description: Option<String>,

    /// The ID of the domain of the project.
    ///
    #[structable(optional, wide)]
    pub domain_id: Option<String>,

    /// The ID of the project.
    ///
    #[structable(optional)]
    pub id: Option<String>,

    /// The name of the project.
    ///
    #[structable(optional)]
    pub name: Option<String>,

    /// The parent id of the project.
    ///
    #[structable(optional, wide)]
    pub parent_id: Option<String>,
}
