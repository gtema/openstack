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
//! Response type for the GET `projects/{project_id}` operation

use serde::{Deserialize, Serialize};
use structable::{StructTable, StructTableOptions};

/// Project response representation
#[derive(Clone, Deserialize, Serialize, StructTable)]
pub struct ProjectResponse {
    /// The description of the project.
    #[serde(default)]
    #[structable(optional)]
    pub description: Option<String>,

    /// The ID of the domain for the project.
    #[serde(default)]
    #[structable(optional)]
    pub domain_id: Option<String>,

    /// If the user is enabled, this value is `true`. If the user is disabled,
    /// this value is `false`.
    #[serde(default)]
    #[structable(optional)]
    pub enabled: Option<bool>,

    /// The ID for the project.
    #[serde(default)]
    #[structable(optional)]
    pub id: Option<String>,

    /// If the user is enabled, this value is `true`. If the user is disabled,
    /// this value is `false`.
    #[serde(default)]
    #[structable(optional)]
    pub is_domain: Option<bool>,

    /// The link to the resources in question.
    #[serde(default)]
    #[structable(optional, serialize)]
    pub links: Option<Links>,

    /// The name of the project.
    #[serde(default)]
    #[structable(optional)]
    pub name: Option<String>,

    /// The resource options for the project. Available resource options are
    /// `immutable`.
    #[serde(default)]
    #[structable(optional, serialize)]
    pub options: Option<Options>,

    /// The ID of the parent for the project.
    ///
    /// **New in version 3.4**
    #[serde(default)]
    #[structable(optional)]
    pub parent_id: Option<String>,

    /// A list of simple strings assigned to a project.
    #[serde(default)]
    #[structable(optional, serialize)]
    pub tags: Option<Vec<String>>,
}

/// The link to the resources in question.
/// `Links` type
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Links {
    pub _self: Option<String>,
}

/// The resource options for the project. Available resource options are
/// `immutable`.
/// `Options` type
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Options {
    pub immutable: Option<bool>,
}
