use http::{HeaderMap, HeaderValue};
use serde::{Deserialize, Serialize};
use std::collections::hash_map::DefaultHasher;
use std::convert::TryFrom;
use std::fmt::{self, Debug};
use std::hash::{Hash, Hasher};
use tracing::error;

use thiserror::Error;
use url::Url;

use crate::api::identity::v3::auth::token::create as token_v3;
use crate::config;
use crate::types::identity::v3::{self as types_v3, AuthResponse, Domain, Project};
// TODO: complete adding error context through anyhow

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum AuthError {
    #[error("header value error: {}", source)]
    HeaderValue {
        #[from]
        source: http::header::InvalidHeaderValue,
    },

    #[error("Invalid auth_url: {}", source)]
    InvalidAuthUrl {
        #[from]
        source: url::ParseError,
    },

    #[error("Cannot construct auth from config: {}", msg)]
    Config { msg: String },

    #[error("Unsupported auth_type: {}", auth_type)]
    IdentityMethod { auth_type: String },

    #[error("Cannot determine scope from config")]
    MissingScope,
    #[error("Auth data is missing")]
    MissingAuthData,
    #[error("Auth URL is missing")]
    MissingAuthUrl,
    #[error("User password is missing")]
    MissingPassword,
    #[error("User id/name is missing")]
    MissingUserId,
    #[error("Auth token is missing")]
    MissingToken,

    #[error("Cannot construct password auth information from config: {}", source)]
    AuthPasswordBuild {
        #[from]
        source: token_v3::PasswordBuilderError,
    },
    #[error("Cannot construct token auth information from config: {}", source)]
    AuthTokenBuild {
        #[from]
        source: token_v3::TokenBuilderError,
    },
    #[error("Cannot construct identity auth information from config: {}", source)]
    AuthIdentityBuild {
        #[from]
        source: token_v3::IdentityBuilderError,
    },
    #[error("Cannot construct user auth information from config: {}", source)]
    AuthUserBuild {
        #[from]
        source: token_v3::UserBuilderError,
    },
    #[error(
        "Cannot construct user/project domain information from config: {}",
        source
    )]
    AuthDomainBuild {
        #[from]
        source: token_v3::DomainStructStructBuilderError,
    },
    #[error("Cannot construct project scope information from config: {}", source)]
    AuthProjectScopeBuild {
        #[from]
        source: token_v3::ProjectBuilderError,
    },
    #[error("Cannot construct auth scope information from config: {}", source)]
    AuthScopeBuild {
        #[from]
        source: token_v3::ScopeBuilderError,
    },
    #[error("error preparing auth data: {}", source)]
    AuthBuilderError {
        #[from]
        source: token_v3::AuthBuilderError,
    },
    #[error("error preparing auth request: {}", source)]
    AuthTokenRequestBuilderError {
        #[from]
        source: token_v3::RequestBuilderError,
    },

    #[error(transparent)]
    Other(#[from] anyhow::Error), // source and Display delegate to anyhow::Error
}

impl AuthError {
    pub fn config(msg: String) -> Self {
        AuthError::Config { msg }
    }

    pub fn auth_type(auth_type: &str) -> Self {
        AuthError::IdentityMethod {
            auth_type: auth_type.to_string(),
        }
    }
}

pub(crate) type AuthResult<T> = Result<T, AuthError>;

/// Build Auth `Identity` request data from `CloudConfig`
impl TryFrom<&config::CloudConfig> for token_v3::Identity<'_> {
    type Error = AuthError;

    fn try_from(config: &config::CloudConfig) -> Result<Self, Self::Error> {
        let auth = config.auth.clone().ok_or(AuthError::MissingAuthData)?;
        // Current OpenStackSDK defines auth_type as a
        // single value string
        let auth_type = config.auth_type.clone().unwrap_or("password".to_string());
        let mut identity = token_v3::IdentityBuilder::default();
        match auth_type.as_str() {
            "password" => {
                identity.methods(Vec::from([token_v3::Methods::Password]));
                let mut password = token_v3::PasswordBuilder::default();
                let mut user = token_v3::UserBuilder::default();
                if let Some(val) = auth.user_id {
                    user.id(val);
                }
                if let Some(val) = auth.username {
                    user.name(val);
                }
                if let Some(val) = auth.password {
                    user.password(val);
                }
                let mut user_domain = token_v3::DomainStructStructBuilder::default();
                let mut user_domain_info_present: bool = false;
                if let Some(val) = auth.user_domain_id {
                    user_domain.id(val);
                    user_domain_info_present = true;
                }
                if let Some(val) = auth.user_domain_name {
                    user_domain.name(val);
                    user_domain_info_present = true;
                }
                if user_domain_info_present {
                    user.domain(user_domain.build()?);
                }
                password.user(user.build()?);
                identity.password(password.build()?);
            }
            "token" => {
                identity.methods(Vec::from([token_v3::Methods::Token]));
                let token = token_v3::TokenBuilder::default()
                    .id(auth.token.ok_or(AuthError::MissingToken)?)
                    .build()?;
                identity.token(token);
            }
            other => {
                return Err(AuthError::auth_type(other));
            }
        };
        Ok(identity.build()?)
    }
}

/// Build Auth `Identity` from existing `Auth` (use token)
impl TryFrom<&AuthToken> for token_v3::Identity<'_> {
    type Error = AuthError;

    fn try_from(auth: &AuthToken) -> Result<Self, Self::Error> {
        Ok(token_v3::IdentityBuilder::default()
            .methods(Vec::from([token_v3::Methods::Token]))
            .token(
                token_v3::TokenBuilder::default()
                    .id(auth.token.clone())
                    .build()?,
            )
            .build()?)
    }
}

/// Build Auth `Scope` data from `CloudConfig`
impl TryFrom<&config::CloudConfig> for token_v3::Scope<'_> {
    type Error = AuthError;
    fn try_from(config: &config::CloudConfig) -> Result<Self, Self::Error> {
        let auth = config.auth.clone().ok_or(AuthError::MissingAuthData)?;
        let mut scope = token_v3::ScopeBuilder::default();
        if auth.project_id.is_some() || auth.project_name.is_some() {
            // Project scope
            let mut project_scope = token_v3::ProjectBuilder::default();
            if auth.project_domain_name.is_some() || auth.project_domain_id.is_some() {
                let mut project_domain = token_v3::DomainStructStructBuilder::default();
                if let Some(val) = auth.project_domain_id {
                    project_domain.id(val);
                }
                if let Some(val) = auth.project_domain_name {
                    project_domain.name(val);
                }
                project_scope.domain(project_domain.build()?);
            };
            if let Some(val) = auth.project_id {
                project_scope.id(val);
            }
            if let Some(val) = auth.project_name {
                project_scope.name(val);
            }
            scope.project(project_scope.build()?);
        } else if auth.domain_id.is_some() || auth.domain_name.is_some() {
            // Domain scope
            let mut domain_scope = token_v3::DomainStructStructBuilder::default();
            if let Some(val) = auth.domain_id {
                domain_scope.id(val);
            }
            if let Some(val) = auth.domain_name {
                domain_scope.name(val);
            }
            scope.domain(domain_scope.build()?);
        } else {
            return Err(AuthError::MissingScope);
        }

        Ok(scope.build()?)
    }
}

/// Build Auth `Scope` data from existing `AuthorizationScope`
impl TryFrom<&AuthorizationScope> for token_v3::Scope<'_> {
    type Error = AuthError;
    fn try_from(scope: &AuthorizationScope) -> Result<Self, Self::Error> {
        let mut scope_builder = token_v3::ScopeBuilder::default();
        match scope {
            AuthorizationScope::Project(project) => {
                let mut project_builder = token_v3::ProjectBuilder::default();
                if let Some(val) = &project.id {
                    project_builder.id(val.clone());
                }
                if let Some(val) = &project.name {
                    project_builder.name(val.clone());
                }
                if let Some(domain) = &project.domain {
                    let mut domain_builder = token_v3::DomainStructStructBuilder::default();
                    if let Some(val) = &domain.id {
                        domain_builder.id(val.clone());
                    }
                    if let Some(val) = &domain.name {
                        domain_builder.name(val.clone());
                    }
                    project_builder.domain(domain_builder.build()?);
                }
                scope_builder.project(project_builder.build()?);
            }
            AuthorizationScope::Domain(domain) => {
                let mut domain_builder = token_v3::DomainStructStructBuilder::default();
                if let Some(val) = &domain.id {
                    domain_builder.id(val.clone());
                }
                if let Some(val) = &domain.name {
                    domain_builder.name(val.clone());
                }
                scope_builder.domain(domain_builder.build()?);
            }
            AuthorizationScope::Unscoped => {}
        }
        Ok(scope_builder.build()?)
    }
}

/// Build `AuthorizationScope` data from `CloudConfig`
impl TryFrom<&config::CloudConfig> for AuthorizationScope {
    type Error = AuthError;
    fn try_from(config: &config::CloudConfig) -> Result<Self, Self::Error> {
        let auth = config.auth.clone().ok_or(AuthError::MissingAuthData)?;
        if auth.project_id.is_some() || auth.project_name.is_some() {
            // Project scope
            Ok(AuthorizationScope::Project(Project {
                id: auth.project_id.clone(),
                name: auth.project_name.clone(),
                domain: types_v3::get_domain(auth.project_domain_id, auth.project_domain_name),
            }))
        } else if auth.domain_id.is_some() || auth.domain_name.is_some() {
            // Domain scope
            Ok(AuthorizationScope::Domain(Domain {
                id: auth.domain_id.clone(),
                name: auth.domain_name.clone(),
            }))
        } else {
            Ok(AuthorizationScope::Unscoped)
        }
    }
}

/// Build Auth request from `CloudConfig`
impl TryFrom<&config::CloudConfig> for token_v3::Request<'_> {
    type Error = AuthError;
    fn try_from(config: &config::CloudConfig) -> Result<Self, Self::Error> {
        let identity_data = token_v3::Identity::try_from(config)?;
        let mut auth_request_data = token_v3::AuthBuilder::default();
        auth_request_data.identity(identity_data);
        if let Ok(scope_data) = token_v3::Scope::try_from(config) {
            auth_request_data.scope(scope_data);
        }

        Ok(token_v3::RequestBuilder::default()
            .auth(auth_request_data.build()?)
            .build()?)
    }
}

/// Build Auth request from `Identity` and `AuthScope`
pub(crate) fn build_auth_request_with_identity_and_scope<'a>(
    auth: &token_v3::Identity<'a>,
    scope: &AuthorizationScope,
) -> Result<token_v3::Request<'a>, AuthError> {
    let mut auth_request_data = token_v3::AuthBuilder::default();
    auth_request_data.identity(auth.clone());
    if let Ok(scope_data) = token_v3::Scope::try_from(scope) {
        auth_request_data.scope(scope_data);
    }

    Ok(token_v3::RequestBuilder::default()
        .auth(auth_request_data.build()?)
        .build()?)
}

/// Build Auth request from `AuthToken` and `AuthScope
pub(crate) fn build_reauth_request<'a>(
    auth: &AuthToken,
    scope: &AuthorizationScope,
) -> Result<token_v3::Request<'a>, AuthError> {
    let identity_data = token_v3::Identity::try_from(auth)?;
    let mut auth_request_data = token_v3::AuthBuilder::default();
    auth_request_data.identity(identity_data);
    if let Ok(scope_data) = token_v3::Scope::try_from(scope) {
        auth_request_data.scope(scope_data);
    }

    Ok(token_v3::RequestBuilder::default()
        .auth(auth_request_data.build()?)
        .build()?)
}

impl Hash for token_v3::Identity<'_> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        if let Ok(x) = serde_json::to_string(&self) {
            state.write(x.as_bytes());
        }
    }
}

impl Hash for token_v3::Request<'_> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.auth.identity.hash(state);
    }
}

/// Get an authentication hash
pub fn get_auth_hash<E>(auth_url: Url, id_data: E) -> u64
where
    E: std::hash::Hash,
{
    // Calculate hash of the auth information
    let mut s = DefaultHasher::new();
    s.write(auth_url.as_str().as_bytes());
    id_data.hash(&mut s);
    s.finish()
}

/// Authentication state enum
#[derive(Debug, Eq, PartialEq)]
pub enum AuthState {
    /// Auth is valid
    Valid,
    /// Expired
    Expired,
    /// Authentication is missing
    Unset,
}

/// An OpenStackAuthentication type
#[derive(Clone)]
#[non_exhaustive]
pub enum Auth {
    /// An X-Auth-Token
    AuthToken(Box<AuthToken>),
    /// Unauthenticated access
    None,
}

impl Auth {
    /// Adds X-Auth-Token header to a request headers.
    ///
    /// Returns an error if the token string cannot be parsed as a header value.
    pub fn set_header<'a>(
        &self,
        headers: &'a mut HeaderMap<HeaderValue>,
    ) -> AuthResult<&'a mut HeaderMap<HeaderValue>> {
        if let Auth::AuthToken(token) = self {
            let _ = token.set_header(headers);
        }

        Ok(headers)
    }
}

/// OpenStack AuthToken authorization structure
#[derive(Clone, Default, Deserialize, Serialize)]
pub struct AuthToken {
    pub(crate) token: String,
    pub(crate) auth_info: Option<AuthResponse>,
}

impl Debug for AuthToken {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Auth")
            .field("data", &self.auth_info)
            .finish()
    }
}

impl AuthToken {
    /// Adds X-Auth-Token header to a request headers.
    ///
    /// Returns an error if the token string cannot be parsed as a header value.
    pub fn set_header<'a>(
        &self,
        headers: &'a mut HeaderMap<HeaderValue>,
    ) -> AuthResult<&'a mut HeaderMap<HeaderValue>> {
        let mut token_header_value = HeaderValue::from_str(&self.token.clone())?;
        token_header_value.set_sensitive(true);
        headers.insert("X-Auth-Token", token_header_value);

        Ok(headers)
    }

    /// Detect authentication validity (valid/expired/unset)
    pub fn get_state(&self) -> AuthState {
        match &self.auth_info {
            Some(data) => {
                if data.token.expires_at <= chrono::offset::Local::now() {
                    AuthState::Expired
                } else {
                    AuthState::Valid
                }
            }
            None => AuthState::Unset,
        }
    }
}

/// Represent authorization scope
#[derive(Clone, Deserialize, Eq, Hash, PartialEq, Serialize, Debug)]
pub enum AuthorizationScope {
    Project(Project),
    Domain(Domain),
    Unscoped,
}

impl From<&AuthResponse> for AuthorizationScope {
    fn from(auth: &AuthResponse) -> Self {
        if let Some(project) = &auth.token.project {
            Self::Project(project.clone())
        } else if let Some(domain) = &auth.token.domain {
            Self::Domain(domain.clone())
        } else {
            Self::Unscoped
        }
    }
}

#[cfg(test)]
mod tests {
    use chrono::prelude::*;
    use serde::Serialize;
    use serde_json::json;
    use std::collections::HashMap;

    use super::AuthState;
    use crate::api::identity::v3::auth::token::create as token_v3;
    use crate::config;
    use crate::types::identity::v3::{self as types_v3, AuthResponse, AuthToken};

    #[test]
    fn test_config_into_auth_password() -> Result<(), &'static str> {
        let config = config::CloudConfig {
            auth: Some(config::Auth {
                password: Some("pwd".into()),
                username: Some("un".into()),
                user_id: Some("ui".into()),
                user_domain_name: Some("udn".into()),
                user_domain_id: Some("udi".into()),
                ..Default::default()
            }),
            auth_type: Some("password".into()),
            ..Default::default()
        };

        let auth_data = token_v3::Identity::try_from(&config).unwrap();
        assert_eq!(
            json!({
              "methods": ["password"],
              "password": {
                "user": {
                  "name": "un",
                  "id": "ui",
                  "password": "pwd",
                  "domain": {
                    "id": "udi",
                    "name": "udn"
                  }
                }
              }
            }),
            serde_json::to_value(auth_data).unwrap()
        );
        Ok(())
    }

    #[test]
    fn test_config_into_auth_token() -> Result<(), &'static str> {
        let config = config::CloudConfig {
            auth: Some(config::Auth {
                token: Some("token".into()),
                user_domain_name: Some("udn".into()),
                user_domain_id: Some("udi".into()),
                ..Default::default()
            }),
            auth_type: Some("token".into()),
            ..Default::default()
        };

        let auth_data = token_v3::Identity::try_from(&config).unwrap();
        assert_eq!(
            json!({
              "methods": ["token"],
                "token": {
                  "id": "token",
              }
            }),
            serde_json::to_value(auth_data).unwrap()
        );
        Ok(())
    }

    #[test]
    fn test_auth_validity_unset() {
        let auth = super::AuthToken::default();
        assert!(matches!(auth.get_state(), AuthState::Unset));
    }

    #[test]
    fn test_auth_validity_expired() {
        let auth = super::AuthToken {
            token: "".to_string(),
            auth_info: Some(AuthResponse {
                token: AuthToken {
                    expires_at: chrono::offset::Local::now() - chrono::Duration::days(1),
                    ..Default::default()
                },
            }),
        };
        assert!(matches!(auth.get_state(), AuthState::Expired));
    }

    #[test]
    fn test_auth_validity_valid() {
        let auth = super::AuthToken {
            token: "".to_string(),
            auth_info: Some(AuthResponse {
                token: AuthToken {
                    expires_at: chrono::offset::Local::now() + chrono::Duration::days(1),
                    ..Default::default()
                },
            }),
        };
        assert!(matches!(auth.get_state(), AuthState::Valid));
    }
}
