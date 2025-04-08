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
//! Response type for the get OS-FEDERATION/mappings/{mapping_id} operation

use serde::{Deserialize, Serialize};

/// Mapping response representation
#[derive(Clone, Deserialize, Serialize)]
struct MappingResponse {
    /// The Federation Mapping unique ID
    ///
    id: Option<String>,

    rules: Option<Vec<Rules>>,

    /// Mapping schema version
    ///
    schema_version: Option<String>,
}

/// `Domain` type
#[derive(Clone, Debug, Deserialize, Serialize)]
struct Domain {
    id: Option<String>,
    name: Option<String>,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
#[serde(untagged)]
pub enum Type {
    // Local
    #[serde(rename = "local")]
    Local,

    // Ephemeral
    #[serde(rename = "ephemeral")]
    Ephemeral,
}

/// `User` type
#[derive(Clone, Debug, Deserialize, Serialize)]
struct User {
    domain: Option<Domain>,
    email: Option<String>,
    id: Option<String>,
    name: Option<String>,
    _type: Option<Type>,
}

/// `Roles` type
#[derive(Clone, Debug, Deserialize, Serialize)]
struct Roles {
    name: String,
}

/// `Projects` type
#[derive(Clone, Debug, Deserialize, Serialize)]
struct Projects {
    domain: Option<Domain>,
    name: String,
    roles: Vec<Roles>,
}

/// `Group` type
#[derive(Clone, Debug, Deserialize, Serialize)]
struct Group {
    id: String,
}

/// `GroupStructResponse` type
#[derive(Clone, Debug, Deserialize, Serialize)]
struct GroupStructResponse {
    domain: Domain,
    name: String,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
#[serde(untagged)]
pub enum LocalGroup {
    // F1
    F1(Group),
    // F2
    F2(GroupStructResponse),
}

/// `Local` type
#[derive(Clone, Debug, Deserialize, Serialize)]
struct Local {
    domain: Option<Domain>,
    group: Option<LocalGroup>,
    group_ids: Option<String>,
    groups: Option<String>,
    projects: Option<Vec<Projects>>,
    user: Option<User>,
}

/// `RemoteType` type
#[derive(Clone, Debug, Deserialize, Serialize)]
struct RemoteType {
    _type: String,
}

/// `RemoteTypeAnyOneOfRegex` type
#[derive(Clone, Debug, Deserialize, Serialize)]
struct RemoteTypeAnyOneOfRegex {
    any_one_of: Vec<String>,
    regex: Option<bool>,
    _type: String,
}

/// `RemoteTypeNotAnyOfRegex` type
#[derive(Clone, Debug, Deserialize, Serialize)]
struct RemoteTypeNotAnyOfRegex {
    not_any_of: Vec<String>,
    regex: Option<bool>,
    _type: String,
}

/// `RemoteTypeBlacklistRegex` type
#[derive(Clone, Debug, Deserialize, Serialize)]
struct RemoteTypeBlacklistRegex {
    blacklist: Vec<String>,
    regex: Option<bool>,
    _type: String,
}

/// `RemoteTypeWhitelistRegex` type
#[derive(Clone, Debug, Deserialize, Serialize)]
struct RemoteTypeWhitelistRegex {
    regex: Option<bool>,
    _type: String,
    whitelist: Vec<String>,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
#[serde(untagged)]
pub enum RulesRemote {
    // F1
    F1(RemoteType),
    // F2
    F2(RemoteTypeAnyOneOfRegex),
    // F3
    F3(RemoteTypeNotAnyOfRegex),
    // F4
    F4(RemoteTypeBlacklistRegex),
    // F5
    F5(RemoteTypeWhitelistRegex),
}

/// `Rules` type
#[derive(Clone, Debug, Deserialize, Serialize)]
struct Rules {
    local: Vec<Local>,
    remote: Vec<RulesRemote>,
}
