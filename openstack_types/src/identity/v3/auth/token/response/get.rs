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
//! Response type for the get auth/tokens operation

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

/// Token response representation
#[derive(Clone, Deserialize, Serialize)]
struct TokenResponse {
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
    audit_ids: Option<Vec<String>>,

    /// A `catalog` object.
    ///
    catalog: Option<Vec<Catalog>>,

    /// A domain object including the id and name representing the domain the
    /// token is scoped to. This is only included in tokens that are scoped to
    /// a domain.
    ///
    domain: Option<DomainStructResponse>,

    /// The date and time when the token expires.
    ///
    /// The date and time stamp format is
    /// [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601):
    ///
    /// ```text
    /// CCYY-MM-DDThh:mm:ss.sssZ
    ///
    /// ```
    ///
    /// For example, `2015-08-27T09:49:58.000000Z`.
    ///
    /// A `null` value indicates that the token never expires.
    ///
    expires_at: Option<String>,

    is_domain: Option<bool>,

    /// The date and time when the token was issued.
    ///
    issues_at: Option<String>,

    /// The authentication methods, which are commonly `password`, `token`, or
    /// other methods. Indicates the accumulated set of authentication methods
    /// that were used to obtain the token. For example, if the token was
    /// obtained by password authentication, it contains `password`. Later, if
    /// the token is exchanged by using the token authentication method one or
    /// more times, the subsequently created tokens contain both `password` and
    /// `token` in their `methods` attribute. Unlike multi-factor
    /// authentication, the `methods` attribute merely indicates the methods
    /// that were used to authenticate the user in exchange for a token. The
    /// client is responsible for determining the total number of
    /// authentication factors.
    ///
    methods: Option<Vec<String>>,

    /// A `project` object including the `id`, `name` and `domain` object
    /// representing the project the token is scoped to. This is only included
    /// in tokens that are scoped to a project.
    ///
    project: Option<Project>,

    /// A list of `role` objects
    ///
    roles: Option<Vec<Roles>>,

    /// A `system` object containing information about which parts of the
    /// system the token is scoped to. If the token is scoped to the entire
    /// deployment system, the `system` object will consist of `{"all": true}`.
    /// This is only included in tokens that are scoped to the system.
    ///
    system: Option<HashMap<String, bool>>,

    /// A `user` object.
    ///
    user: Option<User>,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
#[serde(untagged)]
pub enum Interface {
    // Public
    #[serde(rename = "public")]
    Public,

    // Internal
    #[serde(rename = "internal")]
    Internal,

    // Admin
    #[serde(rename = "admin")]
    Admin,
}

/// `Endpoints` type
#[derive(Clone, Debug, Deserialize, Serialize)]
struct Endpoints {
    id: Option<String>,
    interface: Option<Interface>,
    region: Option<String>,
    url: Option<String>,
}

/// `Catalog` type
#[derive(Clone, Debug, Deserialize, Serialize)]
struct Catalog {
    endpoints: Option<Vec<Endpoints>>,
    id: Option<String>,
    name: Option<String>,
    _type: Option<String>,
}

/// A `domain` object including the `id` and `name` representing the domain the
/// token is scoped to. This is only included in tokens that are scoped to a
/// domain.
///
/// `Domain` type
#[derive(Clone, Debug, Deserialize, Serialize)]
struct Domain {
    id: Option<String>,
    name: Option<String>,
}

/// A `user` object.
///
/// `User` type
#[derive(Clone, Debug, Deserialize, Serialize)]
struct User {
    domain: Option<Domain>,
    id: Option<String>,
    name: Option<String>,
    os_federation: Option<HashMap<String, Value>>,
    password_expires_at: Option<String>,
}

/// A domain object including the id and name representing the domain the token
/// is scoped to. This is only included in tokens that are scoped to a domain.
///
/// `DomainStructResponse` type
#[derive(Clone, Debug, Deserialize, Serialize)]
struct DomainStructResponse {
    id: Option<String>,
    name: Option<String>,
}

/// A `project` object including the `id`, `name` and `domain` object
/// representing the project the token is scoped to. This is only included in
/// tokens that are scoped to a project.
///
/// `Project` type
#[derive(Clone, Debug, Deserialize, Serialize)]
struct Project {
    id: Option<String>,
    name: Option<String>,
}

/// `Roles` type
#[derive(Clone, Debug, Deserialize, Serialize)]
struct Roles {
    id: Option<String>,
    name: Option<String>,
}
