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
//! Response type for the patch domains/{domain_id} operation

use serde::{Deserialize, Serialize};
use structable::{StructTable, StructTableOptions};

/// Domain response representation
#[derive(Clone, Deserialize, Serialize, StructTable)]
pub struct DomainResponse {
    /// The description of the domain.
    ///
    #[structable(optional, serialize)]
    pub description: Option<String>,

    /// If set to `true`, domain is enabled. If set to `false`, domain is
    /// disabled.
    ///
    #[structable(optional)]
    pub enabled: Option<bool>,

    /// The ID of the domain.
    ///
    #[structable(optional)]
    pub id: Option<String>,

    /// The link to the resources in question.
    ///
    #[structable(optional, serialize)]
    pub links: Option<Links>,

    /// The name of the project.
    ///
    #[structable(optional)]
    pub name: Option<String>,

    /// The resource options for the role. Available resource options are
    /// `immutable`.
    ///
    #[structable(optional, serialize)]
    pub options: Option<Options>,

    /// A list of simple strings assigned to a project.
    ///
    #[structable(optional, serialize)]
    pub tags: Option<Vec<String>>,
}

/// The link to the resources in question.
///
/// `Links` type
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Links {
    pub _self: Option<String>,
}

/// The resource options for the role. Available resource options are
/// `immutable`.
///
/// `Options` type
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Options {
    pub immutable: Option<bool>,
}
