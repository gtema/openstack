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
//! Response type for the POST `OS-FEDERATION/identity_providers/{idp_id}/protocols/{protocol_id}/auth` operation

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use structable::{StructTable, StructTableOptions};

/// Auth response representation
#[derive(Clone, Deserialize, Serialize, StructTable)]
pub struct AuthResponse {
    /// A list of one or two audit IDs. An audit ID is a unique, randomly
    /// generated, URL-safe string that you can use to track a token. The first
    /// audit ID is the current audit ID for the token. The second audit ID is
    /// present for only re-scoped tokens and is the audit ID from the token
    /// before it was re-scoped. A re- scoped token is one that was exchanged
    /// for another token of the same or different scope. You can use these
    /// audit IDs to track the use of a token or chain of tokens across
    /// multiple requests and endpoints without exposing the token ID to
    /// non-privileged users.
    ///
    #[structable(optional, serialize)]
    pub audit_ids: Option<Vec<String>>,

    /// A catalog object.
    ///
    #[structable(optional, serialize)]
    pub catalog: Option<Vec<Catalog>>,

    /// The date and time when the token expires.
    ///
    #[structable(optional)]
    pub expires_at: Option<String>,

    /// The date and time when the token was issued.
    ///
    #[structable(optional)]
    pub issues_at: Option<String>,

    /// The authentication methods, which are commonly password, token, or
    /// other methods. Indicates the accumulated set of authentication methods
    /// that were used to obtain the token. For example, if the token was
    /// obtained by password authentication, it contains password. Later, if
    /// the token is exchanged by using the token authentication method one or
    /// more times, the subsequently created tokens contain both password and
    /// token in their methods attribute. Unlike multi-factor authentication,
    /// the methods attribute merely indicates the methods that were used to
    /// authenticate the user in exchange for a token. The client is
    /// responsible for determining the total number of authentication factors.
    ///
    #[structable(optional, serialize)]
    pub methods: Option<Vec<String>>,

    /// A user object
    ///
    #[structable(optional, serialize)]
    pub user: Option<User>,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub enum Interface {
    // Admin
    #[serde(rename = "admin")]
    Admin,

    // Internal
    #[serde(rename = "internal")]
    Internal,

    // Public
    #[serde(rename = "public")]
    Public,
}

/// `Endpoints` type
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Endpoints {
    pub id: Option<String>,
    pub interface: Option<Interface>,
    pub region: Option<String>,
    pub url: Option<String>,
}

/// `Catalog` type
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Catalog {
    pub endpoints: Option<Vec<Endpoints>>,
    pub id: Option<String>,
    pub name: Option<String>,
    pub _type: Option<String>,
}

/// `Domain` type
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Domain {
    pub id: Option<String>,
    pub name: Option<String>,
}

/// A user object
///
/// `User` type
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct User {
    pub domain: Option<Domain>,
    pub id: Option<String>,
    pub name: Option<String>,
    pub os_federation: Option<HashMap<String, Value>>,
    pub password_expires_at: Option<String>,
}
