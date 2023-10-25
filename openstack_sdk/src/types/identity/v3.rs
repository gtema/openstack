//! Identity v3 data types
use http::HeaderMap;
use serde::{Deserialize, Serialize};
use std::fmt;

use crate::types::IdAndName;

#[derive(Deserialize, Clone)]
pub struct AuthResponse {
    pub token: AuthToken,
    //pub(crate) x_auth_token: String,
}

impl fmt::Debug for AuthResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AuthResponse")
            .field("token", &self.token)
            .finish()
    }
}

pub trait ResourceWithHeaders {
    fn consume_headers(&mut self, headers: HeaderMap);
}

impl ResourceWithHeaders for AuthResponse {
    fn consume_headers(&mut self, headers: HeaderMap) {}
}

#[derive(Deserialize, Debug, Clone)]
pub struct AuthToken {
    pub catalog: Option<Vec<ServiceEndpoints>>,
    pub roles: Vec<IdAndName>,
    pub user: User,
    pub project: Option<Project>,
    pub issued_at: String,
    pub expires_at: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct TokenData {}

#[derive(Deserialize, Debug, Clone, Serialize)]
pub struct ServiceEndpoints {
    pub endpoints: Vec<CatalogEndpoint>,
    #[serde(rename = "type")]
    pub service_type: String,
    pub name: String,
}

#[derive(Deserialize, Debug, Clone, Serialize)]
pub struct CatalogEndpoint {
    pub id: String,
    pub interface: String,
    pub region: String,
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    pub domain: Option<IdAndName>,
    pub name: String,
    pub id: String,
    pub password_expires_at: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Project {
    pub id: String,
    pub name: String,
    pub domain: IdAndName,
}
