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
//!
//! This crate provides the foundational types and traits required to authenticate
//! against the OpenStack Identity service (Keystone). It defines:
//!
//! - [`OpenStackAuthType`] — the primary trait that all authentication plugins must
//!   implement to enable login flows (password, token, JWT, WebSSO, etc.).
//! - [`OpenStackMultifactorAuthMethod`] — the trait for multifactor-capable methods
//!   that can be composed into multipass/multifactor authentication requests.
//! - [`AuthToken`] — the structure that represents a successful Keystone authentication
//!   result, including the bearer token and parsed server response.
//! - [`AuthTokenScope`] — represents the authorization scope attached to a token
//!   (project, domain, system, or unscoped).
//! - [`AuthError`] — the unified error type covering all authentication-related
//!   failures, including receipts for multipass flows.
//! - [`Auth`] — the enum that wraps the current authentication state (token or none).
//! - [`AuthState`] — describes token validity (valid, expired, about-to-expire, or unset).
//!
//! ## Plugin Registration
//!
//! Authentication plugins are registered at compile time using Rust's `inventory` crate.
//! Implement [`OpenStackAuthType`] and submit an [`AuthPluginRegistration`] via
//! `inventory::submit!{}`. For multipass support, additionally implement
//! [`OpenStackMultifactorAuthMethod`] and submit an [`AuthMethodPluginRegistration`].
//!
//! The [`execute_auth_request`] function provides a common, instrumented pathway
//! for sending authentication HTTP requests with timing and request-id logging.
//!
//! ## Examples
//!
//! ### Basic Authentication
//!
//! Authenticate by creating an `AuthToken` from an existing token string
//! (e.g., obtained from an environment variable or a previous login):
//!
//! ```no_run
//! use secrecy::{ExposeSecret, SecretString};
//! use openstack_sdk_auth_core::{AuthToken, AuthTokenScope};
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let token = AuthToken::new(SecretString::from("my-token"), None);
//! let scope = AuthTokenScope::Unscoped;
//!
//! println!("Scope: {:?}", token.get_scope());
//! println!("State: {:?}", token.get_state(None));
//! # Ok(())
//! # }
//! ```
//!
//! ### Checking Token State
//!
//! Tokens can be checked for validity with an optional expiration threshold:
//!
//! ```no_run
//! use chrono::TimeDelta;
//! use openstack_sdk_auth_core::AuthState;
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! # let auth = openstack_sdk_auth_core::AuthToken::default();
//! let offset = TimeDelta::minutes(5);
//! match auth.get_state(Some(offset)) {
//!     AuthState::Valid => println!("Token is valid"),
//!     AuthState::AboutToExpire => println!("Token will expire within 5 minutes"),
//!     AuthState::Expired => println!("Token has expired"),
//!     AuthState::Unset => println!("No token data available"),
//! }
//! # Ok(())
//! # }
//! ```
//!
//! ### Setting Request Headers
//!
//! The `Auth` type can inject the `X-Auth-Token` header into outgoing HTTP requests:
//!
//! ```no_run
//! use http::HeaderMap;
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! # let auth = openstack_sdk_auth_core::Auth::None;
//! let mut headers = HeaderMap::new();
//! auth.set_header(&mut headers)?;
//! // headers now contains the `X-Auth-Token` header if the auth type is a token.
//! # Ok(())
//! # }
//! ```
//!
//! ### Implementing a Custom Authenticator
//!
//! Creating a new authentication method requires implementing [`OpenStackAuthType`]:
//!
//! ```no_run
//! use std::collections::HashMap;
//! use async_trait::async_trait;
//! use secrecy::SecretString;
//! use serde_json::{Value, json};
//! use openstack_sdk_auth_core::{Auth, AuthError, AuthToken, AuthTokenScope, OpenStackAuthType};
//!
//! pub struct MyAuthenticator;
//!
//! static PLUGIN: MyAuthenticator = MyAuthenticator;
//! inventory::submit! {
//!     openstack_sdk_auth_core::AuthPluginRegistration { method: &PLUGIN }
//! }
//!
//! #[async_trait]
//! impl OpenStackAuthType for MyAuthenticator {
//!     fn get_supported_auth_methods(&self) -> Vec<&'static str> {
//!         vec!["v3myauth", "myauth"]
//!     }
//!
//!     fn requirements(&self, _hints: Option<&Value>) -> Result<Value, AuthError> {
//!         Ok(json!({
//!             "type": "object",
//!             "required": ["token_id"],
//!             "properties": {
//!                 "token_id": {
//!                     "type": "string",
//!                     "description": "The token identifier"
//!                 }
//!             }
//!         }))
//!     }
//!
//!     fn api_version(&self) -> (u8, u8) {
//!         (3, 0)
//!     }
//!
//!     async fn auth(
//!         &self,
//!         _http_client: &reqwest::Client,
//!         _identity_url: &url::Url,
//!         _values: &HashMap<String, SecretString>,
//!         _scope: Option<&AuthTokenScope>,
//!         _hints: Option<&Value>,
//!     ) -> Result<Auth, AuthError> {
//!         // Perform your authentication logic here and return an AuthToken
//!         let auth_token = AuthToken::new("example-token", None);
//!         Ok(Auth::AuthToken(Box::new(auth_token)))
//!     }
//! }
//! ```
//!
//! ### Error Handling Patterns
//!
//! The [`AuthError`] enum provides different error variants that you can match on
//! to handle specific failure scenarios:
//!
//! ```no_run
//! use openstack_sdk_auth_core::{AuthError, Auth, AuthToken, AuthTokenScope};
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! # fn simulate_auth_result() -> Result<Auth, AuthError> { Ok(Auth::None) }
//!
//! match simulate_auth_result() {
//!     Ok(Auth::AuthToken(token)) => {
//!         println!("Authentication successful");
//!         println!("Token expires at: {:?}", token.get_state(None));
//!     }
//!     Ok(Auth::None) => {
//!         println!("No authentication available");
//!     }
//!     Ok(auth) => {
//!         println!("Unknown auth type: {:?}", auth);
//!     }
//!     Err(AuthError::AuthReceipt(receipt)) => {
//!         println!("Multifactor authentication required");
//!         let methods: Vec<_> = receipt.required_auth_methods.iter()
//!             .flatten()
//!             .cloned()
//!             .collect();
//!         println!("Required methods: {:?}", methods);
//!     }
//!     Err(AuthError::Serde { .. }) => {
//!         println!("Failed to parse response (malformed JSON)");
//!     }
//!     Err(AuthError::UnknownAuth { code, message }) => {
//!         println!("Unknown authentication error (code: {})", code);
//!         if let Some(msg) = &message {
//!             println!("Message: {}", msg);
//!         }
//!     }
//!     Err(e) => {
//!         println!("Authentication failed: {}", e);
//!     }
//! }
//! # Ok(())
//! # }
//! ```
//!
//! ### Implementing a Multifactor-Authenticator
//!
//! To support multifactor authentication (e.g., TOTP or password + token),
//! implement both [`OpenStackAuthType`] and [`OpenStackMultifactorAuthMethod`]:
//!
//! ```no_run
//! use std::collections::HashMap;
//! use async_trait::async_trait;
//! use secrecy::{SecretString, ExposeSecret};
//! use serde_json::{Value, json};
//! use openstack_sdk_auth_core::{
//!     Auth, AuthError, AuthToken, AuthTokenScope,
//!     OpenStackAuthType, OpenStackMultifactorAuthMethod,
//! };
//!
//! pub struct MyMultifactorAuthenticator;
//!
//! static PLUGIN: MyMultifactorAuthenticator = MyMultifactorAuthenticator;
//! inventory::submit! {
//!     openstack_sdk_auth_core::AuthPluginRegistration { method: &PLUGIN }
//! }
//! inventory::submit! {
//!     openstack_sdk_auth_core::AuthMethodPluginRegistration { method: &PLUGIN }
//! }
//!
//! #[async_trait]
//! impl OpenStackMultifactorAuthMethod for MyMultifactorAuthenticator {
//!     fn get_supported_auth_methods(&self) -> Vec<&'static str> {
//!         vec!["v3myauth", "myauth"]
//!     }
//!
//!     fn requirements(&self, _hints: Option<&Value>) -> Result<Value, AuthError> {
//!         Ok(json!({
//!             "type": "object",
//!             "required": ["auth_code"],  // Additional auth method requirement
//!             "properties": {
//!                 "auth_code": {
//!                     "type": "string",
//!                     "format": "password",
//!                     "description": "One-time authentication code"
//!                 }
//!             }
//!         }))
//!     }
//!
//!     /// Extracts authentication data from the values map.
//!     fn get_auth_data(
//!         &self,
//!         values: &HashMap<String, SecretString>,
//!     ) -> Result<(&'static str, Value), AuthError> {
//!         let auth_code = values
//!             .get("auth_code")
//!             .ok_or_else(|| AuthError::AuthValueNotSupplied("auth_code".to_string()))?;
//!         Ok(("myauth", json!({
//!             "auth_code": auth_code.expose_secret()
//!         })))
//!     }
//! }
//!
//! #[async_trait]
//! impl OpenStackAuthType for MyMultifactorAuthenticator {
//!     fn get_supported_auth_methods(&self) -> Vec<&'static str> {
//!         vec!["v3myauth", "myauth"]
//!     }
//!
//!     fn requirements(&self, _hints: Option<&Value>) -> Result<Value, AuthError> {
//!         Ok(json!({
//!             "type": "object",
//!             "required": ["auth_code"],  
//!             "properties": {
//!                 "auth_code": {
//!                     "type": "string",
//!                     "format": "password",
//!                     "description": "One-time authentication code"
//!                 }
//!             }
//!         }))
//!     }
//!
//!     fn api_version(&self) -> (u8, u8) {
//!         (3, 0)
//!     }
//!
//!     async fn auth(
//!         &self,
//!         _http_client: &reqwest::Client,
//!         _identity_url: &url::Url,
//!         _values: &HashMap<String, SecretString>,
//!         _scope: Option<&AuthTokenScope>,
//!         _hints: Option<&Value>,
//!     ) -> Result<Auth, AuthError> {
//!         // Perform multifactor authentication and return an AuthToken:
//!         let auth_token = AuthToken::new("example-token", None);
//!         Ok(Auth::AuthToken(Box::new(auth_token)))
//!     }
//! }
//! ```
//!
//! ### Token Scoping
//!
//! Tokens can be scoped to specific projects or domains using [`AuthTokenScope`]:
//!
//! ```
//! use openstack_sdk_auth_core::{AuthTokenScope};
//! use openstack_sdk_auth_core::types::{Project, Domain};
//!
//! // Project-scope by ID
//! let project_scope = AuthTokenScope::Project(Project {
//!     id: Some("project-id-123".to_string()),
//!     name: None,
//!     domain: None,
//! });
//!
//! // Domain-scope with domain name
//! let domain_scope = AuthTokenScope::Domain(Domain {
//!     id: None,
//!     name: Some("Default".to_string()),
//! });
//!
//! // Unscoped (default)
//! let unscoped: AuthTokenScope = AuthTokenScope::default();
//! ```
//!
//! ### Handling Authentication Receipts
//!
//! When multifactor authentication is enabled, Keystone returns an authentication receipt
//! instead of a new token. The receipt contains information about what additional
//! authentication methods are required:
//!
//! ```no_run
//! use openstack_sdk_auth_core::{AuthTokenScope, AuthError, Auth, AuthReceiptResponse, AuthReceipt};
//! use chrono::Local;
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! # fn simulate_auth_result() -> Result<Auth, AuthError> {
//! #     Ok(Auth::None)
//! # }
//!
//! let scope = Some(&AuthTokenScope::Unscoped);
//! match simulate_auth_result() {
//!     Ok(auth) => {
//!         match auth {
//!             Auth::AuthToken(token) => {
//!                 println!("Authenticated with token");
//!                 println!("Token: {:?}", token.get_scope());
//!             }
//!             Auth::None => {
//!                 println!("Not authenticated");
//!             }
//!             _ => {
//!                 println!("Unknown auth type");
//!             }
//!         }
//!     }
//!     Err(AuthError::AuthReceipt(receipt)) => {
//!         println!("Additional authentication methods required");
//!         println!("Required methods: {:?}", receipt.required_auth_methods);
//!         let methods: Vec<_> = receipt.receipt.methods.iter().cloned().collect();
//!         println!("Already completed methods: {:?}", methods);
//!
//!         // Use the receipt token for subsequent authentication requests
//!         if let Some(receipt_token) = &receipt.token {
//!             println!("Receipt token: {}...", &receipt_token.chars().take(8).collect::<String>());
//!         }
//!     }
//!     Err(e) => {
//!         println!("Authentication error: {}", e);
//!     }
//! }
//! # Ok(())
//! # }
//! ```
//! ### Token State Management
//!
//! The [`AuthToken::get_state`] method can be used to determine if a token is valid and
//! needs to be refreshed. Using an expiration offset allows you to proactively refresh
//! tokens before they expire:
//!
//! ```
//! use chrono::TimeDelta;
//! use openstack_sdk_auth_core::{AuthToken, AuthState, types::{AuthResponse, TokenInfo, User, Project}};
//! use secrecy::SecretString;
//!
//! // Create a token with expiration info
//! let auth_info = AuthResponse {
//!     token: TokenInfo {
//!         expires_at: chrono::Utc::now() + TimeDelta::hours(24),
//!         user: User::default(),
//!         ..Default::default()
//!     },
//! };
//! let token = AuthToken::new("my-token", Some(auth_info));
//!
//! // Check with 5-minute buffer for proactive refresh
//! match token.get_state(Some(TimeDelta::minutes(5))) {
//!     AuthState::Valid => {
//!         println!("Token is valid");
//!     }
//!     AuthState::AboutToExpire => {
//!         println!("Token will expire soon, refreshing...");
//!         // Refresh the token before it expires
//!     }
//!     AuthState::Expired => {
//!         println!("Token has expired, re-authenticating...");
//!     }
//!     AuthState::Unset => {
//!         println!("No token data available");
//!     }
//! }
//! ```

use std::collections::HashMap;
use std::fmt::{self, Debug};
use std::time::SystemTime;

use async_trait::async_trait;
use http::{HeaderMap, HeaderValue};
use reqwest::{Client, Request, Response};
use secrecy::SecretString;
use thiserror::Error;
use tracing::{Level, event, instrument};

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
    AuthReceipt(Box<AuthReceiptResponse>),

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
        values: &HashMap<String, SecretString>,
        scope: Option<&AuthTokenScope>,
        hints: Option<&serde_json::Value>,
    ) -> Result<Auth, AuthError>;
}

/// Registry entry for authentication plugins.
pub struct AuthPluginRegistration {
    /// The authentication method implementation.
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

/// Registry entry for multifactor-capable authentication methods.
pub struct AuthMethodPluginRegistration {
    /// The authentication method implementation.
    pub method: &'static dyn OpenStackMultifactorAuthMethod,
}
inventory::collect!(AuthMethodPluginRegistration);

#[instrument(name="request", skip_all, fields(http.uri = request.url().as_str(), http.method = request.method().as_str(), openstack.ver=request.headers().get("openstack-api-version").map(|v| v.to_str().unwrap_or(""))))]
pub async fn execute_auth_request(
    client: &Client,
    request: Request,
) -> Result<Response, reqwest::Error> {
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
    pub fn set_header(&self, headers: &mut HeaderMap<HeaderValue>) -> Result<(), AuthError> {
        if let Auth::AuthToken(token) = self {
            token.set_header(headers)?;
        }

        Ok(())
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
    use crate::types::{AuthResponse, TokenInfo};
    use std::hash::{Hash, Hasher};

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
                token: TokenInfo {
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
                token: TokenInfo {
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
                token: TokenInfo {
                    expires_at: chrono::Utc::now() + chrono::TimeDelta::days(1),
                    ..Default::default()
                },
            }),
        );
        assert!(matches!(auth.get_state(None), AuthState::Valid));
    }

    #[test]
    fn test_auth_set_header_invalid_token() {
        let auth = Auth::AuthToken(Box::new(super::AuthToken::new(
            "invalid\nheader\nvalue",
            None,
        )));
        let mut headers = HeaderMap::new();
        let result = auth.set_header(&mut headers);
        assert!(result.is_err());
    }

    #[test]
    fn test_project_domain_eq_hash() {
        use std::collections::HashSet;

        let p1 = Project {
            id: Some("1".into()),
            name: Some("proj".into()),
            domain: Some(Domain {
                id: Some("d1".into()),
                name: Some("D".into()),
            }),
        };
        let p2 = Project {
            id: Some("1".into()),
            name: Some("proj".into()),
            domain: Some(Domain {
                id: Some("d1".into()),
                name: Some("D".into()),
            }),
        };
        assert_eq!(p1, p2);

        let h1 = {
            let mut h = std::collections::hash_map::DefaultHasher::new();
            p1.hash(&mut h);
            h.finish()
        };
        let h2 = {
            let mut h = std::collections::hash_map::DefaultHasher::new();
            p2.hash(&mut h);
            h.finish()
        };
        assert_eq!(h1, h2, "equal projects must have equal hashes");

        let mut pset = HashSet::new();
        pset.insert(p1.clone());
        assert!(pset.contains(&p2), "HashSet should contain equal project");
    }

    #[test]
    fn test_project_both_name_and_domain_none() {
        let p1 = Project {
            id: Some("x".into()),
            name: None,
            domain: None,
        };
        let p2 = Project {
            id: Some("y".into()),
            name: None,
            domain: None,
        };
        assert_ne!(p1, p2);
    }

    #[test]
    fn test_project_domain_none_vs_some() {
        let p1 = Project {
            id: None,
            name: Some("p".into()),
            domain: None,
        };
        let p2 = Project {
            id: None,
            name: Some("p".into()),
            domain: Some(Domain {
                id: None,
                name: None,
            }),
        };
        assert_ne!(p1, p2);
    }

    #[test]
    fn test_domain_both_names_none() {
        let d1 = Domain {
            id: Some("x".into()),
            name: None,
        };
        let d2 = Domain {
            id: Some("y".into()),
            name: None,
        };
        assert_ne!(d1, d2);
    }

    #[test]
    fn test_domain_none_vs_some_name() {
        let d1 = Domain {
            id: None,
            name: None,
        };
        let d2 = Domain {
            id: None,
            name: Some("D".into()),
        };
        assert_ne!(d1, d2);
    }

    #[test]
    fn test_project_hashset_dedup() {
        use std::collections::HashSet;
        let d = Domain {
            id: Some("d".into()),
            name: Some("D".into()),
        };
        let p1 = Project {
            id: Some("1".into()),
            name: Some("p".into()),
            domain: Some(d.clone()),
        };
        let p2 = Project {
            id: Some("1".into()),
            name: Some("p".into()),
            domain: Some(d.clone()),
        };
        let p3 = Project {
            id: Some("1".into()),
            name: Some("p".into()),
            domain: Some(d),
        };

        let mut set = HashSet::new();
        set.insert(p1);
        set.insert(p2);
        set.insert(p3);
        assert_eq!(
            set.len(),
            1,
            "HashSet should deduplicate identical projects"
        );
    }

    #[test]
    fn test_auth_scope_from_response_project() {
        let response = AuthResponse {
            token: TokenInfo {
                project: Some(Project {
                    id: Some("1".into()),
                    name: Some("p".into()),
                    domain: None,
                }),
                ..Default::default()
            },
        };
        let scope: AuthTokenScope = (&response).into();
        assert!(matches!(scope, AuthTokenScope::Project(_)));
    }

    #[test]
    fn test_auth_scope_from_response_domain() {
        let response = AuthResponse {
            token: TokenInfo {
                project: None,
                domain: Some(Domain {
                    id: Some("1".into()),
                    name: Some("D".into()),
                }),
                ..Default::default()
            },
        };
        let scope: AuthTokenScope = (&response).into();
        assert!(matches!(scope, AuthTokenScope::Domain(_)));
    }

    #[test]
    fn test_auth_scope_from_response_system() {
        let response = AuthResponse {
            token: TokenInfo {
                project: None,
                domain: None,
                system: Some(System { all: Some(true) }),
                ..Default::default()
            },
        };
        let scope: AuthTokenScope = (&response).into();
        assert!(matches!(scope, AuthTokenScope::System(_)));
    }

    #[test]
    fn test_auth_scope_from_response_unscoped() {
        let response = AuthResponse {
            token: TokenInfo {
                project: None,
                domain: None,
                system: None,
                ..Default::default()
            },
        };
        let scope: AuthTokenScope = (&response).into();
        assert!(matches!(scope, AuthTokenScope::Unscoped));
    }

    #[test]
    fn test_auth_scope_priority_project_over_domain() {
        let response = AuthResponse {
            token: TokenInfo {
                project: Some(Project {
                    id: Some("1".into()),
                    name: Some("p".into()),
                    domain: None,
                }),
                domain: Some(Domain {
                    id: Some("1".into()),
                    name: Some("D".into()),
                }),
                ..Default::default()
            },
        };
        let scope: AuthTokenScope = (&response).into();
        assert!(matches!(scope, AuthTokenScope::Project(_)));
    }

    #[test]
    fn test_auth_scope_priority_domain_over_system() {
        let response = AuthResponse {
            token: TokenInfo {
                project: None,
                domain: Some(Domain {
                    id: Some("1".into()),
                    name: Some("D".into()),
                }),
                system: Some(System { all: Some(true) }),
                ..Default::default()
            },
        };
        let scope: AuthTokenScope = (&response).into();
        assert!(matches!(scope, AuthTokenScope::Domain(_)));
    }

    #[test]
    fn test_auth_none_set_header_noop() {
        let auth = Auth::None;
        let mut headers = HeaderMap::new();
        let result = auth.set_header(&mut headers);
        assert!(result.is_ok());
        assert!(headers.is_empty());
    }

    #[test]
    fn test_auth_token_set_header() {
        let auth = Auth::AuthToken(Box::new(super::AuthToken::new("my-token", None)));
        let mut headers = HeaderMap::new();
        auth.set_header(&mut headers).unwrap();
        assert!(headers.contains_key("X-Auth-Token"));
    }

    #[test]
    fn test_auth_debug_token() {
        let auth = Auth::AuthToken(Box::new(super::AuthToken::new("tok", None)));
        let debug = format!("{:?}", auth);
        assert!(debug.contains("Token"));
    }

    #[test]
    fn test_auth_debug_none() {
        let auth = Auth::None;
        let debug = format!("{:?}", auth);
        assert!(debug.contains("unauthed"));
    }

    #[test]
    fn test_try_from_http_response_for_auth() {
        let auth = AuthResponse::default();
        let json = serde_json::to_string(&auth).unwrap();
        let body: bytes::Bytes = json.into_bytes().into();

        let http_response = http::Response::builder()
            .header("x-subject-token", "tok")
            .body(body)
            .unwrap();

        let result = Auth::try_from(http_response);
        assert!(result.is_ok());
        assert!(matches!(result.unwrap(), Auth::AuthToken(_)));
    }

    #[test]
    fn test_auth_error_plugin_constructor() {
        #[derive(Debug)]
        struct MyErr(&'static str);
        impl std::fmt::Display for MyErr {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.0)
            }
        }
        impl std::error::Error for MyErr {}

        let err = AuthError::plugin(MyErr("plug"));
        assert!(matches!(err, AuthError::Plugin { .. }));
        assert!(format!("{}", err).contains("plug"));
    }

    #[test]
    fn test_auth_error_display_unknown_auth_with_message() {
        let err = AuthError::UnknownAuth {
            code: 500,
            message: Some("upstream timeout".into()),
        };
        assert_eq!(format!("{}", err), "identity service error");
    }

    #[test]
    fn test_auth_error_display_unknown_auth_without_message() {
        let err = AuthError::UnknownAuth {
            code: 502,
            message: None,
        };
        assert_eq!(format!("{}", err), "identity service error");
    }

    #[test]
    fn test_auth_error_display_auth_value_not_supplied() {
        let err = AuthError::AuthValueNotSupplied("user".to_string());
        assert!(format!("{}", err).contains("value necessary"));
    }

    #[test]
    fn test_builder_error_from_string() {
        let err: BuilderError = "custom validation".to_string().into();
        assert!(matches!(err, BuilderError::Validation(_)));
        assert!(format!("{}", err).contains("custom validation"));
    }

    #[test]
    fn test_auth_token_scope_default() {
        let scope = AuthTokenScope::default();
        assert!(matches!(scope, AuthTokenScope::Unscoped));
    }

    #[test]
    fn test_auth_receipt_not_string_display() {
        let err = AuthError::AuthReceiptNotString;
        assert!(format!("{}", err).contains("receipt"));
    }

    #[test]
    fn test_auth_error_identity_display() {
        let err = AuthError::Identity(IdentityError {
            code: 401,
            message: "unauthorized".into(),
        });
        assert!(format!("{}", err).contains("authentication method error"));
        assert!(format!("{}", err).contains("unauthorized"));
    }

    #[test]
    fn test_name_or_id_serialize() {
        let id = NameOrId::Id("abc".into());
        let json = serde_json::to_string(&id).unwrap();
        assert!(json.contains("id"));
        assert!(json.contains("abc"));

        let name = NameOrId::Name("myproj".into());
        let json = serde_json::to_string(&name).unwrap();
        assert!(json.contains("name"));
        assert!(json.contains("myproj"));
    }

    #[test]
    fn test_name_or_id_deserialize() {
        let json = r#"{"id": "abc"}"#;
        let parsed: NameOrId = serde_json::from_str(json).unwrap();
        assert!(matches!(parsed, NameOrId::Id(ref s) if s == "abc"));

        let json = r#"{"name": "myproj"}"#;
        let parsed: NameOrId = serde_json::from_str(json).unwrap();
        assert!(matches!(parsed, NameOrId::Name(ref s) if s == "myproj"));
    }

    #[test]
    fn test_project_serialize_human_readable_skips_none() {
        let p = Project {
            id: Some("1".into()),
            name: None,
            domain: None,
        };
        let json = serde_json::to_string(&p).unwrap();
        assert!(json.contains("id"));
    }

    #[test]
    fn test_domain_serialize_human_readable_skips_none() {
        let d = Domain {
            id: Some("1".into()),
            name: None,
        };
        let json = serde_json::to_string(&d).unwrap();
        assert!(json.contains("id"));
    }

    #[test]
    fn test_project_serialize_all_fields() {
        let d = Domain {
            id: Some("d".into()),
            name: Some("D".into()),
        };
        let p = Project {
            id: Some("1".into()),
            name: Some("p".into()),
            domain: Some(d),
        };
        let json = serde_json::to_string(&p).unwrap();
        assert!(json.contains("id"));
        assert!(json.contains("name"));
        assert!(json.contains("domain"));
    }

    #[test]
    fn test_auth_state_debug() {
        assert_eq!(format!("{:?}", AuthState::Valid), "Valid");
        assert_eq!(format!("{:?}", AuthState::Expired), "Expired");
        assert_eq!(format!("{:?}", AuthState::AboutToExpire), "AboutToExpire");
        assert_eq!(format!("{:?}", AuthState::Unset), "Unset");
    }

    #[test]
    fn test_auth_receipt_response_serialization() {
        let receipt = AuthReceiptResponse {
            receipt: AuthReceipt {
                methods: vec!["password".to_string()],
                user: User {
                    id: "u".to_string(),
                    name: "user".to_string(),
                    ..Default::default()
                },
                expires_at: chrono::Local::now(),
                ..Default::default()
            },
            required_auth_methods: vec![vec!["totp".to_string()]],
            token: Some("tok".to_string()),
        };
        let json = serde_json::to_string(&receipt).unwrap();
        assert!(json.contains("receipt"));
        assert!(json.contains("password"));
        assert!(json.contains("totp"));
    }
}
