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
//! Response type for the GET `roles/{prior_role_id}/implies` operation

use serde::{Deserialize, Serialize};
use structable::{StructTable, StructTableOptions};

/// Imply response representation
#[derive(Clone, Deserialize, Serialize, StructTable)]
pub struct ImplyResponse {
    /// An array of implied role objects.
    #[serde(default)]
    #[structable(optional, serialize)]
    pub implies: Option<Vec<Implies>>,

    /// A prior role object.
    #[serde(default)]
    #[structable(optional, serialize)]
    pub prior_role: Option<PriorRole>,
}

/// The link to the resources in question.
/// `Links` type
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Links {
    #[serde(default, rename = "self")]
    pub _self: Option<String>,
}

/// A prior role object.
/// `Implies` type
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Implies {
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub links: Option<Links>,
    #[serde(default)]
    pub name: Option<String>,
}

/// A prior role object.
/// `PriorRole` type
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PriorRole {
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub links: Option<Links>,
    #[serde(default)]
    pub name: Option<String>,
}
