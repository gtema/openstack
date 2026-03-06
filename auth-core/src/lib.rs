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
//! # Core trait for implementing OpenStack authentication plugins to [`openstack_sdk`]

use std::collections::HashMap;
use std::fmt::{self, Debug};

use async_trait::async_trait;
use http::{HeaderMap, HeaderValue};
use secrecy::SecretString;
use thiserror::Error;

pub mod authtoken;
pub mod authtoken_scope;
pub mod types;

pub use authtoken::{AuthToken, AuthTokenError};
pub use types::*;

/// Authentication error.
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum AuthError {
    /// AuthToken error.
    #[error("AuthToken error: {}", source)]
    AuthToken {
        /// The source of the error.
        #[from]
        source: AuthTokenError,
    },

    /// Token is missing in the authentication response.
    #[error("token missing in the response")]
    AuthTokenNotInResponse,

    /// X-Subject-Token cannot be converted to string.
    #[error("token missing cannot be converted to string")]
    AuthTokenNotString,

    /// Necessary data was not supplied to the auth method.
    #[error("value necessary for the chosen auth method was not supplied to the auth method")]
    AuthValueNotSupplied(String),

    // TODO: Move out
    /// Necessary data was not supplied to the auth method.
    #[error("plugin specified malformed requirements")]
    PluginMalformedRequirement,

    /// (De)Serialization error.
    #[error("failed to deserialize response body: {}", source)]
    Serde {
        /// The source of the error.
        #[from]
        source: serde_json::Error,
    },

    /// Header error.
    #[error("header value error: {}", source)]
    HeaderValue {
        /// The source of the error.
        #[from]
        source: http::header::InvalidHeaderValue,
    },

    /// AuthPlugin error.
    #[error("plugin error: {}", source)]
    Plugin {
        /// The source of the error.
        #[source]
        source: Box<dyn std::error::Error + Send + Sync + 'static>,
    },

    /// Reqwest error.
    #[error(transparent)]
    Reqwest {
        /// The error source
        #[from]
        source: reqwest::Error,
    },
}

impl AuthError {
    pub fn plugin<E>(error: E) -> Self
    where
        E: std::error::Error + Send + Sync + 'static,
    {
        Self::Plugin {
            source: Box::new(error),
        }
    }
}

/// The trait for authentication "plugins" for enabling OpenStack authentication.
#[async_trait]
pub trait OpenStackAuthType: Send + Sync {
    /// Return list of supported authentication methods.
    ///
    /// Example:
    ///
    /// ```rust
    /// fn get_supported_auth_methods(&self) -> Vec<&'static str> {
    ///     vec!["v4federation", "federation"]
    /// }
    /// ```
    fn get_supported_auth_methods(&self) -> Vec<&'static str>;

    /// Get the json schema of the data the plugin requires to complete the authentication.
    fn requirements(&self) -> serde_json::Value;

    /// Get the API version of the Identity Service the plugin need to communicate to.
    fn api_version(&self) -> (u8, u8);

    /// Authenticate the client with the configuration.
    async fn auth(
        &self,
        http_client: &reqwest::Client,
        identity_url: &url::Url,
        values: HashMap<String, SecretString>,
    ) -> Result<Auth, AuthError>;
}

// This struct "wraps" the trait object so inventory can track it
pub struct AuthPluginRegistration {
    pub method: &'static dyn OpenStackAuthType,
}

// Essential: This creates the global registry for this specific struct
inventory::collect!(AuthPluginRegistration);

/// An OpenStack Authentication type
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
    ) -> Result<&'a mut HeaderMap<HeaderValue>, AuthError> {
        if let Auth::AuthToken(token) = self {
            let _ = token.set_header(headers);
        }

        Ok(headers)
    }
}

impl fmt::Debug for Auth {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Auth {}",
            match self {
                Auth::AuthToken(_) => "Token",
                Auth::None => "unauthed",
            }
        )
    }
}

impl TryFrom<http::Response<bytes::Bytes>> for Auth {
    type Error = AuthError;
    fn try_from(value: http::Response<bytes::Bytes>) -> Result<Self, Self::Error> {
        Ok(Self::AuthToken(Box::new(AuthToken::try_from(value)?)))
    }
}

/// Authentication state enum
#[derive(Debug, Eq, PartialEq)]
pub enum AuthState {
    /// Auth is valid
    Valid,
    /// Expired
    Expired,
    /// About to expire
    AboutToExpire,
    /// Authentication is missing
    Unset,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{AuthResponse, AuthToken};

    #[test]
    fn test_auth_validity_unset() {
        let auth = super::AuthToken::default();
        assert!(matches!(auth.get_state(None), AuthState::Unset));
    }

    #[test]
    fn test_auth_validity_expired() {
        let auth = super::AuthToken::new(
            String::new(),
            Some(AuthResponse {
                token: AuthToken {
                    expires_at: chrono::Utc::now() - chrono::TimeDelta::days(1),
                    ..Default::default()
                },
            }),
        );
        assert!(matches!(auth.get_state(None), AuthState::Expired));
    }

    #[test]
    fn test_auth_validity_expire_soon() {
        let auth = super::AuthToken::new(
            String::new(),
            Some(AuthResponse {
                token: AuthToken {
                    expires_at: chrono::Utc::now() + chrono::TimeDelta::minutes(10),
                    ..Default::default()
                },
            }),
        );
        assert!(matches!(
            auth.get_state(Some(chrono::TimeDelta::minutes(15))),
            AuthState::AboutToExpire
        ));
    }

    #[test]
    fn test_auth_validity_valid() {
        let auth = super::AuthToken::new(
            String::new(),
            Some(AuthResponse {
                token: AuthToken {
                    expires_at: chrono::Utc::now() + chrono::TimeDelta::days(1),
                    ..Default::default()
                },
            }),
        );
        assert!(matches!(auth.get_state(None), AuthState::Valid));
    }
}
