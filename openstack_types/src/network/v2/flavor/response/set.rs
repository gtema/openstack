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
//! Response type for the put flavors/{id} operation

use serde::{Deserialize, Serialize};
use structable::{StructTable, StructTableOptions};

/// Flavor response representation
#[derive(Clone, Deserialize, Serialize, StructTable)]
pub struct FlavorResponse {
    /// The human-readable description for the flavor.
    ///
    #[structable(optional, serialize)]
    pub description: Option<String>,

    /// Indicates whether the flavor is enabled or not. Default is true.
    ///
    #[structable(optional, serialize)]
    pub enabled: Option<bool>,

    /// The ID of the flavor.
    ///
    #[structable(optional)]
    pub id: Option<String>,

    /// Name of the flavor.
    ///
    #[structable(optional)]
    pub name: Option<String>,

    /// Service profile UUIDs associated with this flavor.
    ///
    #[structable(optional, serialize)]
    pub service_profiles: Option<Vec<String>>,

    /// Service type for the flavor. Example: FIREWALL.
    ///
    #[structable(optional)]
    pub service_type: Option<String>,
}
