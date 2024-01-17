//! Identity v3 data types
use http::HeaderMap;
use serde::{Deserialize, Serialize};
use std::fmt;

use crate::types::IdAndName;

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
    pub issued_at: String,
    pub expires_at: String,
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
    pub password_expires_at: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Project {
    pub id: String,
    pub name: String,
    pub domain: Domain,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Domain(IdAndName);
