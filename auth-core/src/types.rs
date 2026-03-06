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
use serde::{Deserialize, Serialize};

/// A reference to a resource by its Name and ID.
#[derive(Deserialize, Debug, Clone, Serialize, Eq, PartialEq)]
pub struct IdAndName {
    /// The name of the entity.
    pub name: String,
    /// The UID for the entity.
    pub id: String,
}

/// A reference to a resource by either its Name or ID.
#[derive(Clone, Debug, Serialize, PartialEq, Eq, Hash)]
pub enum NameOrId {
    /// Resource ID.
    #[serde(rename = "id")]
    Id(String),
    /// Resource name.
    #[serde(rename = "name")]
    Name(String),
}

/// AuthResponse structure returned by token authentication calls
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
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

#[derive(Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
pub struct ServiceEndpoints {
    pub endpoints: Vec<CatalogEndpoint>,
    #[serde(rename = "type")]
    pub service_type: String,
    pub name: String,
}

#[derive(Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
pub struct CatalogEndpoint {
    pub id: String,
    pub interface: String,
    pub region: Option<String>,
    pub url: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize, Eq, PartialEq)]
pub struct User {
    pub domain: Option<Domain>,
    pub name: String,
    pub id: String,
    // Note(gtema): some clouds return empty string instead of null when
    // password doesnot expire. It is technically possible to use
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
#[derive(Clone, Deserialize, Eq, Hash, PartialEq, Serialize, Debug)]
pub struct Project {
    pub id: Option<String>,
    pub name: Option<String>,
    pub domain: Option<Domain>,
}

#[derive(Clone, Deserialize, Eq, Hash, PartialEq, Serialize, Debug)]
pub struct Domain {
    pub id: Option<String>,
    pub name: Option<String>,
}

/// System Scope
///
#[derive(Clone, Deserialize, Eq, Hash, PartialEq, Serialize, Debug)]
pub struct System {
    pub all: Option<bool>,
}

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq)]
pub struct AuthReceiptResponse {
    pub receipt: AuthReceipt,
    pub required_auth_methods: Vec<Vec<String>>,
}

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq)]
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

///// Build Auth [`Scope`](token_v3::Scope) data from existing [`AuthTokenScope`]
//impl TryFrom<&AuthTokenScope> for Scope<'_> {
//    type Error = crate::authtoken::AuthTokenError;
//    fn try_from(scope: &crate::authtoken::AuthTokenScope) -> Result<Self, Self::Error> {
//        let mut scope_builder = ScopeBuilder::default();
//        match scope {
//            AuthTokenScope::Project(project) => {
//                let mut project_builder = token_v3::ProjectBuilder::default();
//                if let Some(val) = &project.id {
//                    project_builder.id(val.clone());
//                }
//                if let Some(val) = &project.name {
//                    project_builder.name(val.clone());
//                }
//                if let Some(domain) = &project.domain {
//                    let mut domain_builder = token_v3::DomainBuilder::default();
//                    if let Some(val) = &domain.id {
//                        domain_builder.id(val.clone());
//                    }
//                    if let Some(val) = &domain.name {
//                        domain_builder.name(val.clone());
//                    }
//                    project_builder.domain(domain_builder.build()?);
//                }
//                scope_builder.project(project_builder.build()?);
//            }
//            AuthTokenScope::Domain(domain) => {
//                let mut domain_builder = token_v3::DomainBuilder::default();
//                if let Some(val) = &domain.id {
//                    domain_builder.id(val.clone());
//                }
//                if let Some(val) = &domain.name {
//                    domain_builder.name(val.clone());
//                }
//                scope_builder.domain(domain_builder.build()?);
//            }
//            AuthTokenScope::System(system) => {
//                let mut system_builder = token_v3::SystemBuilder::default();
//                if let Some(all) = system.all {
//                    system_builder.all(all);
//                }
//                scope_builder.system(system_builder.build()?);
//            }
//            AuthTokenScope::Unscoped => {}
//        }
//        Ok(scope_builder.build()?)
//    }
//}
