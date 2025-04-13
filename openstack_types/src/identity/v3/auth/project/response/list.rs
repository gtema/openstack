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
//! Response type for the get auth/projects operation

use serde::{Deserialize, Serialize};
use structable::{StructTable, StructTableOptions};

/// Project response representation
#[derive(Clone, Deserialize, Serialize, StructTable)]
pub struct ProjectResponse {
    /// The ID of the domain for the project.
    ///
    #[structable(optional, wide)]
    pub domain_id: Option<String>,

    /// If set to `true`, project is enabled. If set to `false`, project is
    /// disabled.
    ///
    #[structable(optional, wide)]
    pub enabled: Option<bool>,

    /// The ID for the project.
    ///
    #[structable(optional)]
    pub id: Option<String>,

    /// The name of the project.
    ///
    #[structable(optional)]
    pub name: Option<String>,
}

/// Links to the resources in question. See
/// [API Guide / Links and References](https://docs.openstack.org/api-guide/compute/links_and_references.html)
/// for more info.
///
/// `Links` type
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Links {
    pub href: Option<String>,
    pub rel: Option<String>,
}
