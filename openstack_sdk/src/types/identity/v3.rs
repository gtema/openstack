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

//! Identity v3 data types
use chrono::prelude::*;
use serde::{Deserialize, Serialize};

use crate::types::IdAndName;

pub mod version;

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
    pub issued_at: Option<DateTime<Local>>,
    pub expires_at: DateTime<Local>,
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
    pub region: String,
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

/// Build Domain type if id or name are given
#[inline]
pub(crate) fn get_domain(id: Option<String>, name: Option<String>) -> Option<Domain> {
    if id.is_some() || name.is_some() {
        Some(Domain {
            id: id.clone(),
            name: name.clone(),
        })
    } else {
        None
    }
}
