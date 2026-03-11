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

//! Types of the SDK authentication methods
use std::hash::{Hash, Hasher};

use chrono::prelude::*;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

use crate::BuilderError;

/// A reference to a resource by its Name and ID.
#[derive(Deserialize, Debug, Clone, Eq, PartialEq, Serialize)]
pub struct IdAndName {
    /// The name of the entity.
    pub name: String,
    /// The UID for the entity.
    pub id: String,
}

/// A reference to a resource by either its Name or ID.
#[derive(Clone, Debug, Hash, PartialEq, Serialize)]
pub enum NameOrId {
    /// Resource ID.
    #[serde(rename = "id")]
    Id(String),
    /// Resource name.
    #[serde(rename = "name")]
    Name(String),
}

/// AuthResponse structure returned by token authentication calls
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct AuthResponse {
    pub token: AuthToken,
}

/// AuthToken response information
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct AuthToken {
    /// Application credential information
    pub application_credential: Option<ApplicationCredential>,
    pub catalog: Option<Vec<ServiceEndpoints>>,
    pub roles: Option<Vec<IdAndName>>,
    pub user: User,
    pub project: Option<Project>,
    pub domain: Option<Domain>,
    pub system: Option<System>,
    pub issued_at: Option<DateTime<Utc>>,
    pub expires_at: DateTime<Utc>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ServiceEndpoints {
    pub endpoints: Vec<CatalogEndpoint>,
    #[serde(rename = "type")]
    pub service_type: String,
    pub name: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct CatalogEndpoint {
    pub id: String,
    pub interface: String,
    pub region: Option<String>,
    pub url: String,
}

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct User {
    pub domain: Option<Domain>,
    pub name: String,
    pub id: String,
    // Note(gtema): some clouds return empty string instead of null when
    // password does not expire. It is technically possible to use
    // deserialize_with to capture errors, but that leads bincode to fail
    // when deserializing. For now just leave it as optional string instead
    // of DateTime
    // #[serde(deserialize_with = "deser_ok_or_default")]
    pub password_expires_at: Option<String>,
}

/// Authorization project details.
///
/// While in the response `id` and `name` and mandatorily set this type is
/// also reused to manage authentications where at least one of them must be
/// present
#[derive(Builder, Clone, Debug, Default, Deserialize, Eq, Serialize)]
#[builder(build_fn(error = "BuilderError"))]
#[builder(setter(strip_option))]
pub struct Project {
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<Domain>,
}

impl PartialEq for Project {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
            || (self.name.is_some()
                && other.name.is_some()
                && self.name == other.name
                && self.domain == other.domain)
    }
}

impl Hash for Project {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state)
    }
}

#[derive(Builder, Clone, Debug, Default, Deserialize, Eq, Serialize)]
#[builder(build_fn(error = "BuilderError"))]
#[builder(setter(strip_option))]
pub struct Domain {
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl PartialEq for Domain {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
            || (self.name.is_some() && other.name.is_some() && self.name == other.name)
    }
}

impl Hash for Domain {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state)
    }
}

/// System Scope.
#[derive(Builder, Clone, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[builder(build_fn(error = "BuilderError"))]
#[builder(setter(strip_option))]
pub struct System {
    #[builder(default)]
    pub all: Option<bool>,
}

// Trust scope.
#[derive(Builder, Clone, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[builder(build_fn(error = "BuilderError"))]
#[builder(setter(strip_option))]
pub struct OsTrustTrust {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub id: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AuthReceiptResponse {
    pub receipt: AuthReceipt,
    pub required_auth_methods: Vec<Vec<String>>,
    pub token: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AuthReceipt {
    pub catalog: Option<Vec<ServiceEndpoints>>,
    pub roles: Option<Vec<IdAndName>>,
    pub methods: Vec<String>,
    pub user: User,
    pub issued_at: Option<DateTime<Local>>,
    pub expires_at: DateTime<Local>,
}

/// Application Credential information from the token
#[derive(Clone, Debug, Default, Deserialize, Serialize, Eq, PartialEq)]
pub struct ApplicationCredential {
    /// The ID of the application credential.
    pub id: Option<String>,
    /// The name of the application credential.
    pub name: Option<String>,
    /// A flag indicating whether the application credential may be used for creation or destruction
    /// of other application credentials or trusts.
    pub restricted: Option<bool>,
}

/// Build [`AuthTokenScope`] from [`AuthResponse`]
impl From<&AuthResponse> for crate::authtoken_scope::AuthTokenScope {
    fn from(auth: &AuthResponse) -> Self {
        if let Some(project) = &auth.token.project {
            Self::Project(project.clone())
        } else if let Some(domain) = &auth.token.domain {
            Self::Domain(domain.clone())
        } else if let Some(system) = &auth.token.system {
            Self::System(system.clone())
        } else {
            Self::Unscoped
        }
    }
}

/// Authentication error response.
#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct AuthErrorResponse {
    /// Error object.
    pub error: IdentityError,
}

/// Error object.
#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct IdentityError {
    /// Error code.
    pub code: u32,
    /// Error message.
    pub message: String,
}
