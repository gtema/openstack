use http::{HeaderMap, HeaderValue};
use std::convert::TryFrom;
use tracing::{debug, error, info};

use anyhow::Context;
use thiserror::Error;

use http::request::Builder as RequestBuilder;

use crate::api::{self, Query};

use crate::api::identity::v3::auth_tokens::create as token_v3;
use crate::config;
use crate::types::NameOrId;
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
    IdentityMethod { auth_type: &'static str },

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
    #[error(transparent)]
    Other(#[from] anyhow::Error), // source and Display delegate to anyhow::Error
}

impl AuthError {
    pub fn config(msg: String) -> Self {
        AuthError::Config { msg }
    }

    pub fn auth_type(auth_type: &'static str) -> Self {
        AuthError::IdentityMethod { auth_type }
    }
}

pub(crate) type AuthResult<T> = Result<T, AuthError>;

/// Extract AuthData from CloudConfig
impl TryFrom<&config::CloudConfig> for token_v3::AuthData<'_> {
    type Error = AuthError;

    fn try_from(config: &config::CloudConfig) -> Result<Self, Self::Error> {
        let auth = config.auth.clone().ok_or(AuthError::MissingAuthData)?;
        let auth_type = config.auth_type.clone().unwrap_or("password".to_string());
        let auth_method = token_v3::IdentityMethod::try_from(auth_type)?;
        let auth_data: token_v3::AuthData = match auth_method {
            token_v3::IdentityMethod::Password => {
                token_v3::AuthData::Password(token_v3::PasswordAuthData {
                    user: token_v3::UserWithPassword {
                        user: match (auth.username, auth.user_id) {
                            (_, Some(id)) => NameOrId::Id(id),
                            (Some(name), _) => NameOrId::Name(name),
                            _ => return Err(AuthError::MissingUserId),
                        },
                        domain: match (auth.user_domain_name, auth.user_domain_id) {
                            (_, Some(id)) => Some(NameOrId::Id(id)),
                            (Some(name), _) => Some(NameOrId::Name(name)),
                            _ => None,
                        },
                        password: auth.password.ok_or(AuthError::MissingPassword)?.into(),
                    },
                })
            }
            token_v3::IdentityMethod::Token => token_v3::AuthData::Token(token_v3::TokenAuthData {
                id: auth.token.ok_or(AuthError::MissingToken)?.into(),
            }),
        };
        Ok(auth_data)
    }
}

/// Extract Scope from CloudConfig
impl TryFrom<&config::CloudConfig> for token_v3::Scope {
    type Error = AuthError;
    fn try_from(config: &config::CloudConfig) -> Result<Self, Self::Error> {
        let auth = config.auth.clone().ok_or(AuthError::MissingAuthData)?;
        if auth.project_id.is_some() || auth.project_name.is_some() {
            // Project scope
            let domain = match (auth.project_domain_name, auth.project_domain_id) {
                (_, Some(id)) => Some(NameOrId::Id(id)),
                (Some(name), _) => Some(NameOrId::Name(name)),
                _ => None,
            };
            let project = match (auth.project_name, auth.project_id) {
                (_, Some(id)) => NameOrId::Id(id),
                (Some(name), _) => NameOrId::Name(name),
                _ => todo!(),
            };
            Ok(token_v3::Scope::Project(token_v3::Project {
                project,
                domain,
            }))
        } else if auth.domain_id.is_some() || auth.domain_name.is_some() {
            // Domain scope
            let domain = match (auth.domain_name, auth.domain_id) {
                (_, Some(id)) => NameOrId::Id(id),
                (Some(name), _) => NameOrId::Name(name),
                _ => todo!(),
            };
            Ok(token_v3::Scope::Domain(domain))
        } else {
            return Err(AuthError::MissingScope);
        }
    }
}

/// An OpenStack API token (X-Auth-Token)
///
#[derive(Clone)]
pub enum Auth {
    /// A session access token, obtained after sucessful authorization to OpenStack
    Token(String),
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
        match self {
            Auth::Token(token) => {
                let mut token_header_value = HeaderValue::from_str(token)?;
                token_header_value.set_sensitive(true);
                headers.insert("X-Auth-Token", token_header_value);
            }
            Auth::None => {}
        }

        Ok(headers)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::api::identity::v3::auth_tokens::create as token_v3;
    use crate::config;
    use crate::types::NameOrId;

    #[test]
    fn test_config_into_auth_password() -> Result<(), &'static str> {
        let config = config::CloudConfig {
            auth: Some(config::Auth {
                auth_url: None,
                domain_id: None,
                domain_name: None,
                endpoint: None,
                password: Some("pwd".into()),
                project_id: None,
                project_name: None,
                project_domain_id: None,
                project_domain_name: None,
                token: None,
                username: Some("un".into()),
                user_id: Some("ui".into()),
                user_domain_name: Some("udn".into()),
                user_domain_id: Some("udi".into()),
            }),
            auth_type: Some("password".into()),
            profile: None,
            interface: None,
            region_name: None,
            options: HashMap::new(),
        };

        let auth_data = token_v3::AuthData::try_from(&config).unwrap();
        match auth_data {
            token_v3::AuthData::Password(data) => {
                assert_eq!("pwd", data.user.password);
                assert_eq!(NameOrId::Id("ui".into()), data.user.user);
                assert_eq!(Some(NameOrId::Id("udi".into())), data.user.domain);
                Ok(())
            }
            _ => panic!("UserPassword expected"),
        }
    }

    #[test]
    fn test_config_into_auth_token() -> Result<(), &'static str> {
        let config = config::CloudConfig {
            auth: Some(config::Auth {
                auth_url: None,
                domain_id: None,
                domain_name: None,
                endpoint: None,
                password: None,
                project_id: None,
                project_name: None,
                project_domain_id: None,
                project_domain_name: None,
                token: Some("token".into()),
                username: None,
                user_id: None,
                user_domain_name: Some("udn".into()),
                user_domain_id: Some("udi".into()),
            }),
            auth_type: Some("token".into()),
            profile: None,
            interface: None,
            region_name: None,
            options: HashMap::new(),
        };

        let auth = token_v3::AuthData::try_from(&config).unwrap();
        match auth {
            token_v3::AuthData::Token(data) => {
                assert_eq!("token", data.id);
                Ok(())
            }
            _ => panic!("Token expected"),
        }
    }
}
