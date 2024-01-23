//! Identity v3 data types
use chrono::prelude::*;
use serde::{Deserialize, Serialize};

use crate::types::IdAndName;
use crate::utils::deser_ok_or_default;

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct AuthResponse {
    pub token: AuthToken,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct AuthToken {
    pub catalog: Option<Vec<ServiceEndpoints>>,
    pub roles: Option<Vec<IdAndName>>,
    pub user: User,
    pub project: Option<Project>,
    pub domain: Option<Domain>,
    pub issued_at: Option<DateTime<Local>>,
    pub expires_at: DateTime<Local>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ServiceEndpoints {
    pub endpoints: Vec<CatalogEndpoint>,
    #[serde(rename = "type")]
    pub service_type: String,
    pub name: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CatalogEndpoint {
    pub id: String,
    pub interface: String,
    pub region: String,
    pub url: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
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

#[derive(Clone, Debug, Default, Deserialize)]
pub struct AuthReceiptResponse {
    pub receipt: AuthReceipt,
    pub required_auth_methods: Vec<Vec<String>>,
}

#[derive(Clone, Debug, Default, Deserialize)]
pub struct AuthReceipt {
    pub catalog: Option<Vec<ServiceEndpoints>>,
    pub roles: Option<Vec<IdAndName>>,
    pub methods: Vec<String>,
    pub user: User,
    pub issued_at: Option<DateTime<Local>>,
    pub expires_at: DateTime<Local>,
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
