use http::{HeaderMap, HeaderValue};
use std::convert::TryFrom;
use tracing::{debug, error, info};

use anyhow::Context;
use thiserror::Error;

use http::request::Builder as RequestBuilder;

use crate::api::{self, Query};

use crate::api::identity::v3::auth::token::create as token_v3_new;
use crate::config;
use crate::types::identity::v3::AuthResponse;
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
        source: token_v3_new::PasswordBuilderError,
    },
    #[error("Cannot construct token auth information from config: {}", source)]
    AuthTokenBuild {
        #[from]
        source: token_v3_new::TokenBuilderError,
    },
    #[error("Cannot construct identity auth information from config: {}", source)]
    AuthIdentityBuild {
        #[from]
        source: token_v3_new::IdentityBuilderError,
    },
    #[error("Cannot construct user auth information from config: {}", source)]
    AuthUserBuild {
        #[from]
        source: token_v3_new::UserBuilderError,
    },
    #[error(
        "Cannot construct user/project domain information from config: {}",
        source
    )]
    AuthDomainBuild {
        #[from]
        source: token_v3_new::DomainStructStructBuilderError,
    },
    #[error("Cannot construct project scope information from config: {}", source)]
    AuthProjectScopeBuild {
        #[from]
        source: token_v3_new::ProjectBuilderError,
    },
    #[error("Cannot construct auth scope information from config: {}", source)]
    AuthScopeBuild {
        #[from]
        source: token_v3_new::ScopeBuilderError,
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

/// Extract AuthData from CloudConfig
impl TryFrom<&config::CloudConfig> for token_v3_new::Identity<'_> {
    type Error = AuthError;

    fn try_from(config: &config::CloudConfig) -> Result<Self, Self::Error> {
        let auth = config.auth.clone().ok_or(AuthError::MissingAuthData)?;
        // Current OpenStackSDK defines auth_type as a
        // single value string
        let auth_type = config.auth_type.clone().unwrap_or("password".to_string());
        let mut identity = token_v3_new::IdentityBuilder::default();
        identity.methods(Vec::from([auth_type.clone().into()]));
        match auth_type.as_str() {
            "password" => {
                let mut password = token_v3_new::PasswordBuilder::default();
                let mut user = token_v3_new::UserBuilder::default();
                if let Some(val) = auth.user_id {
                    user.id(val);
                }
                if let Some(val) = auth.username {
                    user.name(val);
                }
                if let Some(val) = auth.password {
                    user.password(val);
                }
                let mut user_domain = token_v3_new::DomainStructStructBuilder::default();
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
                let token = token_v3_new::TokenBuilder::default()
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

/// Extract Scope from CloudConfig
impl TryFrom<&config::CloudConfig> for token_v3_new::Scope<'_> {
    type Error = AuthError;
    fn try_from(config: &config::CloudConfig) -> Result<Self, Self::Error> {
        let auth = config.auth.clone().ok_or(AuthError::MissingAuthData)?;
        let mut scope = token_v3_new::ScopeBuilder::default();
        if auth.project_id.is_some() || auth.project_name.is_some() {
            // Project scope
            let mut project_scope = token_v3_new::ProjectBuilder::default();
            if auth.project_domain_name.is_some() || auth.project_domain_id.is_some() {
                let mut project_domain = token_v3_new::DomainStructStructBuilder::default();
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
            let mut domain_scope = token_v3_new::DomainStructStructBuilder::default();
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

/// An OpenStack API token (X-Auth-Token)
#[derive(Clone, Default)]
pub struct Auth {
    /// A session access token, obtained after successful authorization to
    /// OpenStack
    token: Option<String>,
    /// Authorization data (auth validity)
    pub data: Option<AuthResponse>,
}

impl Auth {
    /// Adds X-Auth-Token header to a request headers.
    ///
    /// Returns an error if the token string cannot be parsed as a header value.
    pub fn set_header<'a>(
        &self,
        headers: &'a mut HeaderMap<HeaderValue>,
    ) -> AuthResult<&'a mut HeaderMap<HeaderValue>> {
        if let Some(token) = &self.token {
            let mut token_header_value = HeaderValue::from_str(&token.clone())?;
            token_header_value.set_sensitive(true);
            headers.insert("X-Auth-Token", token_header_value);
        }

        Ok(headers)
    }

    /// Set token for use as the authorization
    pub fn set_token(&mut self, token: &str) {
        self.token = Some(token.to_string());
    }
}

#[cfg(test)]
mod tests {
    use serde::Serialize;
    use serde_json::json;
    use std::collections::HashMap;

    use crate::api::identity::v3::auth::token::create as token_v3;
    use crate::config;
    use crate::types::NameOrId;

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
}
