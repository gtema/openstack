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
//! Response type for the patch OS-FEDERATION/identity_providers/{idp_id} operation

use serde::{Deserialize, Serialize};
use structable::{StructTable, StructTableOptions};

/// IdentityProvider response representation
#[derive(Clone, Deserialize, Serialize, StructTable)]
pub struct IdentityProviderResponse {
    /// The length of validity in minutes for group memberships carried over
    /// through mapping and persisted in the database.
    ///
    #[structable(optional)]
    pub authorization_ttl: Option<i32>,

    /// The Identity Provider description
    ///
    #[structable(optional)]
    pub description: Option<String>,

    /// The ID of a domain that is associated with the Identity Provider.
    ///
    #[structable(optional)]
    pub domain_id: Option<String>,

    /// Whether the Identity Provider is enabled or not
    ///
    #[structable(optional)]
    pub enabled: Option<bool>,

    /// The Identity Provider unique ID
    ///
    #[structable(optional)]
    pub id: Option<String>,

    /// List of the unique Identity Provider’s remote IDs
    ///
    #[structable(optional, serialize)]
    pub remote_ids: Option<Vec<String>>,
}
