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
use chrono::prelude::*;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

use crate::BuilderError;

/// A reference to a resource by both its name and ID.
#[derive(Deserialize, Debug, Clone, Eq, PartialEq, Serialize)]
pub struct IdAndName {
    /// The UID for the entity.
    pub id: String,
    /// The name of the entity.
    pub name: String,
}

/// A reference to a resource by either its Name or ID.
#[derive(Clone, Debug, Hash, PartialEq, Serialize, Deserialize)]
pub enum NameOrId {
    /// Resource ID.
    #[serde(rename = "id")]
    Id(String),
    /// Resource name.
    #[serde(rename = "name")]
    Name(String),
}

/// Authentication response structure returned by token calls.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct AuthResponse {
    /// Token information.
    pub token: TokenInfo,
}

/// AuthToken response information.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct TokenInfo {
    /// Application credential information.
    pub application_credential: Option<ApplicationCredential>,
    /// Catalog of available services.
    pub catalog: Option<Vec<ServiceEndpoints>>,
    /// Domain in which the token was issued.
    pub domain: Option<Domain>,
    /// Token expiration time.
    pub expires_at: DateTime<Utc>,
    /// Token issue time.
    pub issued_at: Option<DateTime<Utc>>,
    /// Project in which the token was issued.
    pub project: Option<Project>,
    /// Roles assigned to the token.
    pub roles: Option<Vec<IdAndName>>,
    /// System scope of the token.
    pub system: Option<System>,
    /// User who obtained the token.
    pub user: User,
}

/// Service endpoint catalog entries.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ServiceEndpoints {
    /// List of available endpoints for this service.
    pub endpoints: Vec<CatalogEndpoint>,
    /// Human-readable service name.
    pub name: String,
    #[serde(rename = "type")]
    /// Service type identifier (e.g., "compute", "network").
    pub service_type: String,
}

/// Service catalog endpoint information.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct CatalogEndpoint {
    /// Endpoint unique identifier.
    pub id: String,
    /// Interface type (public, internal, admin).
    pub interface: String,
    /// Region identifier.
    pub region: Option<String>,
    /// Endpoint URL.
    pub url: String,
}

/// User identity information.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct User {
    /// Domain the user belongs to.
    pub domain: Option<Domain>,
    /// User unique identifier.
    pub id: String,
    /// User name.
    pub name: String,
    // Note(gtema): some clouds return empty string instead of null when
    // password does not expire. It is technically possible to use
    // deserialize_with to capture errors, but that leads bincode to fail
    // when deserializing. For now just leave it as optional string instead
    // of DateTime
    // #[serde(deserialize_with = "deser_ok_or_default")]
    /// Optional password expiration date.
    pub password_expires_at: Option<String>,
}

/// Authorization project details.
///
/// While in the response `id` and `name` and mandatorily set this type is
/// also reused to manage authentications where at least one of them must be
/// present.
#[derive(Builder, Clone, Debug, Default, Deserialize, Eq, Hash, PartialEq)]
#[builder(build_fn(error = "BuilderError"))]
#[builder(setter(strip_option))]
pub struct Project {
    /// Associated domain for the project.
    #[builder(default)]
    pub domain: Option<Domain>,

    /// Project unique identifier.
    #[builder(default)]
    pub id: Option<String>,

    /// Project name.
    #[builder(default)]
    pub name: Option<String>,
}

impl Serialize for Project {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        if serializer.is_human_readable() {
            #[derive(Serialize)]
            struct ProjectJson<'a> {
                #[serde(skip_serializing_if = "Option::is_none")]
                domain: Option<&'a Domain>,
                #[serde(skip_serializing_if = "Option::is_none")]
                id: Option<&'a str>,
                #[serde(skip_serializing_if = "Option::is_none")]
                name: Option<&'a str>,
            }
            let helper = ProjectJson {
                domain: self.domain.as_ref(),
                id: self.id.as_deref(),
                name: self.name.as_deref(),
            };
            helper.serialize(serializer)
        } else {
            #[derive(Serialize)]
            struct ProjectRaw<'a> {
                domain: &'a Option<Domain>,
                id: &'a Option<String>,
                name: &'a Option<String>,
            }
            let helper = ProjectRaw {
                domain: &self.domain,
                id: &self.id,
                name: &self.name,
            };
            helper.serialize(serializer)
        }
    }
}

/// Domain identity information.
#[derive(Builder, Clone, Debug, Default, Deserialize, Eq, Hash, PartialEq)]
#[builder(build_fn(error = "BuilderError"))]
#[builder(setter(strip_option))]
#[serde(default)]
pub struct Domain {
    /// Domain unique identifier.
    #[builder(default)]
    pub id: Option<String>,

    /// Domain name.
    #[builder(default)]
    pub name: Option<String>,
}

impl Serialize for Domain {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        if serializer.is_human_readable() {
            #[derive(Serialize)]
            struct DomainJson<'a> {
                #[serde(skip_serializing_if = "Option::is_none")]
                id: Option<&'a str>,
                #[serde(skip_serializing_if = "Option::is_none")]
                name: Option<&'a str>,
            }
            let helper = DomainJson {
                id: self.id.as_deref(),
                name: self.name.as_deref(),
            };
            helper.serialize(serializer)
        } else {
            #[derive(Serialize)]
            struct DomainRaw<'a> {
                id: &'a Option<String>,
                name: &'a Option<String>,
            }
            let helper = DomainRaw {
                id: &self.id,
                name: &self.name,
            };
            helper.serialize(serializer)
        }
    }
}

/// System Scope.
#[derive(Builder, Clone, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[builder(build_fn(error = "BuilderError"))]
#[builder(setter(strip_option))]
pub struct System {
    /// Flag indicating if the system scope is all.
    #[builder(default)]
    pub all: Option<bool>,
}

/// Trust scope information.
#[derive(Builder, Clone, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[builder(build_fn(error = "BuilderError"))]
#[builder(setter(strip_option))]
pub struct OsTrustTrust {
    /// Trust unique identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub id: Option<String>,
}

/// Multimodal authentication receipt response from the identity service.
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AuthReceiptResponse {
    /// The actual auth receipt data.
    pub receipt: AuthReceipt,
    /// Required authentication methods for the receipt.
    pub required_auth_methods: Vec<Vec<String>>,
    /// Token associated with this receipt.
    pub token: Option<String>,
}

/// Authentication receipt data returned when additional authentication methods are required.
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AuthReceipt {
    /// Catalog of available services.
    pub catalog: Option<Vec<ServiceEndpoints>>,
    /// Receipt expiration time.
    pub expires_at: DateTime<Local>,
    /// Receipt issue time.
    pub issued_at: Option<DateTime<Local>>,
    /// Authentication methods already completed.
    pub methods: Vec<String>,
    /// Roles assigned to the receipt.
    pub roles: Option<Vec<IdAndName>>,
    /// User information.
    pub user: User,
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
