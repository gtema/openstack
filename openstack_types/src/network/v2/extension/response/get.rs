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
//! Response type for the GET `extensions/{id}` operation

use serde::{Deserialize, Serialize};
use structable::{StructTable, StructTableOptions};

/// Extension response representation
#[derive(Clone, Deserialize, Serialize, StructTable)]
pub struct ExtensionResponse {
    /// The alias for the extension. For example “quotas” or “security-group”.
    #[serde(default)]
    #[structable(optional)]
    pub alias: Option<String>,

    /// The human-readable description for the resource.
    #[serde(default)]
    #[structable(optional)]
    pub description: Option<String>,

    /// Human-readable name of the resource.
    #[serde(default)]
    #[structable(optional)]
    pub name: Option<String>,

    /// A URL pointing to the namespace for this extension.
    #[serde(default)]
    #[structable(optional)]
    pub namespace: Option<String>,

    /// The date and timestamp when the extension was last updated.
    #[serde(default)]
    #[structable(optional)]
    pub updated: Option<String>,
}
