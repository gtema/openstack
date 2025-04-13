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
//! Response type for the get auth/domains operation

use serde::{Deserialize, Serialize};
use structable::{StructTable, StructTableOptions};

/// Domain response representation
#[derive(Clone, Deserialize, Serialize, StructTable)]
pub struct DomainResponse {
    /// The description of the domain.
    ///
    #[structable(optional, wide)]
    pub description: Option<String>,

    /// If set to `true`, domain is enabled. If set to `false`, domain is
    /// disabled.
    ///
    #[structable(optional, wide)]
    pub enabled: Option<bool>,

    /// The ID of the domain.
    ///
    #[structable(optional)]
    pub id: Option<String>,

    /// The name of the domain.
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
