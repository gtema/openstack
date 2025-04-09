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
//! Response type for the patch OS-FEDERATION/mappings/{mapping_id} operation

use serde::{Deserialize, Serialize};

/// Mapping response representation
#[derive(Clone, Deserialize, Serialize)]
pub struct MappingResponse {
    /// The Federation Mapping unique ID
    ///
    pub id: Option<String>,

    pub rules: Option<Vec<Rules>>,

    /// Mapping schema version
    ///
    pub schema_version: Option<String>,
}

/// `Domain` type
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Domain {
    pub id: Option<String>,
    pub name: Option<String>,
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
pub struct User {
    pub domain: Option<Domain>,
    pub email: Option<String>,
    pub id: Option<String>,
    pub name: Option<String>,
    pub _type: Option<Type>,
}

/// `Roles` type
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Roles {
    pub name: String,
}

/// `Projects` type
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Projects {
    pub domain: Option<Domain>,
    pub name: String,
    pub roles: Vec<Roles>,
}

/// `Group` type
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Group {
    pub id: String,
}

/// `GroupStructResponse` type
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GroupStructResponse {
    pub domain: Domain,
    pub name: String,
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
pub struct Local {
    pub domain: Option<Domain>,
    pub group: Option<LocalGroup>,
    pub group_ids: Option<String>,
    pub groups: Option<String>,
    pub projects: Option<Vec<Projects>>,
    pub user: Option<User>,
}

/// `RemoteType` type
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteType {
    pub _type: String,
}

/// `RemoteTypeAnyOneOfRegex` type
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteTypeAnyOneOfRegex {
    pub any_one_of: Vec<String>,
    pub regex: Option<bool>,
    pub _type: String,
}

/// `RemoteTypeNotAnyOfRegex` type
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteTypeNotAnyOfRegex {
    pub not_any_of: Vec<String>,
    pub regex: Option<bool>,
    pub _type: String,
}

/// `RemoteTypeBlacklistRegex` type
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteTypeBlacklistRegex {
    pub blacklist: Vec<String>,
    pub regex: Option<bool>,
    pub _type: String,
}

/// `RemoteTypeWhitelistRegex` type
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RemoteTypeWhitelistRegex {
    pub regex: Option<bool>,
    pub _type: String,
    pub whitelist: Vec<String>,
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
pub struct Rules {
    pub local: Vec<Local>,
    pub remote: Vec<RulesRemote>,
}
