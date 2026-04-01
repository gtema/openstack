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
use std::time::SystemTime;

use async_trait::async_trait;
use http::{HeaderMap, HeaderValue};
use reqwest::{Client, Request, Response};
use secrecy::SecretString;
use thiserror::Error;
use tracing::{Level, event, info, instrument};

pub mod authtoken;
pub mod authtoken_scope;
pub mod types;

pub use authtoken::{AuthToken, AuthTokenError};
pub use authtoken_scope::AuthTokenScope;
pub use types::*;

/// Authentication error.
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum AuthError {
    /// Authentication rejected with a receipt.
    #[error("authentication rejected")]
    AuthReceipt(AuthReceiptResponse),

    /// openstack-auth-receipt cannot be converted to string.
    #[error("authentication receipt cannot be converted to string")]
    AuthReceiptNotString,

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
    #[error("token cannot be converted to string")]
    AuthTokenNotString,

    /// Necessary data was not supplied to the auth method.
    #[error("value necessary for the chosen auth method was not supplied to the auth method")]
    AuthValueNotSupplied(String),

    /// Keystone error.
    #[error("authentication method error: {}", .0.message)]
    Identity(IdentityError),

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

    /// Identity error not supported.
    #[error("identity service error")]
    UnknownAuth {
        /// The error code.
        code: u16,
        /// The error body.
        message: Option<String>,
    },

    /// Url parse error.
    #[error(transparent)]
    Url {
        /// The error source
        #[from]
        source: url::ParseError,
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
    fn get_supported_auth_methods(&self) -> Vec<&'static str>;

    /// Get the json schema of the data the plugin requires to complete the authentication.
    fn requirements(
        &self,
        hints: Option<&serde_json::Value>,
    ) -> Result<serde_json::Value, AuthError>;

    /// Get the API version of the Identity Service the plugin need to communicate to.
    fn api_version(&self) -> (u8, u8);

    /// Authenticate the client with the configuration.
    async fn auth(
        &self,
        http_client: &reqwest::Client,
        identity_url: &url::Url,
        values: HashMap<String, SecretString>,
        scope: Option<&AuthTokenScope>,
        hints: Option<&serde_json::Value>,
    ) -> Result<Auth, AuthError>;
}

// This struct "wraps" the trait object so inventory can track it
pub struct AuthPluginRegistration {
    pub method: &'static dyn OpenStackAuthType,
}

// Essential: This creates the global registry for this specific struct
inventory::collect!(AuthPluginRegistration);

/// The trait for multifactor-capable authentication methods.
pub trait OpenStackMultifactorAuthMethod: Send + Sync {
    /// Return list of supported authentication methods.
    fn get_supported_auth_methods(&self) -> Vec<&'static str>;

    /// Get the json schema of the data the plugin requires to complete the authentication.
    fn requirements(
        &self,
        hints: Option<&serde_json::Value>,
    ) -> Result<serde_json::Value, AuthError>;

    /// Authenticate the client with the configuration.
    fn get_auth_data(
        &self,
        values: &HashMap<String, SecretString>,
    ) -> Result<(&'static str, serde_json::Value), AuthError>;
}

// This struct "wraps" the trait object so inventory can track it
pub struct AuthMethodPluginRegistration {
    pub method: &'static dyn OpenStackMultifactorAuthMethod,
}
inventory::collect!(AuthMethodPluginRegistration);

#[instrument(name="request", skip_all, fields(http.uri = request.url().as_str(), http.method = request.method().as_str(), openstack.ver=request.headers().get("openstack-api-version").map(|v| v.to_str().unwrap_or(""))))]
pub async fn execute_auth_request(
    client: &Client,
    request: Request,
) -> Result<Response, reqwest::Error> {
    info!("Sending request {:?}", request);
    let url = request.url().clone();
    let method = request.method().clone();
    let start = SystemTime::now();
    let rsp = client.execute(request).await?;
    let elapsed = SystemTime::now().duration_since(start).unwrap_or_default();
    event!(
        name: "http_request",
        Level::INFO,
        url=url.as_str(),
        duration_ms=elapsed.as_millis(),
        status=rsp.status().as_u16(),
        method=method.as_str(),
        request_id=rsp.headers().get("x-openstack-request-id").map(|v| v.to_str().unwrap_or("")),
        "Request completed with status {}",
        rsp.status(),
    );
    Ok(rsp)
}

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

/// Builder error.
///
/// A wrapper error that is used instead of the error generated by the
/// `derive_builder`.
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum BuilderError {
    /// Uninitialized field.
    #[error("{0}")]
    UninitializedField(String),
    /// Custom validation error.
    #[error("{0}")]
    Validation(String),
}

impl From<String> for BuilderError {
    fn from(s: String) -> Self {
        Self::Validation(s)
    }
}

impl From<derive_builder::UninitializedFieldError> for BuilderError {
    fn from(ufe: derive_builder::UninitializedFieldError) -> Self {
        Self::UninitializedField(ufe.to_string())
    }
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
