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

//! OpenStack API authentication
//!
//! Currently there are only 2 types of auth supported:
//!
//! - AuthToken (X-Auth-Token header)
//! - None (unauthenticated)

use http::{HeaderMap, HeaderValue};

use std::fmt::{self, Debug};
use tracing::error;

use thiserror::Error;

mod auth_token_endpoint;
pub mod authtoken;
pub mod authtoken_scope;
mod authtoken_utils;
pub mod v3applicationcredential;
pub mod v3password;
pub mod v3token;
pub mod v3totp;
pub mod v3websso;

use authtoken::{AuthToken, AuthTokenError};
use authtoken_scope::AuthTokenScopeError;
use v3websso::WebSsoError;

/// Authentication error
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum AuthError {
    /// Header error
    #[error("header value error: {}", source)]
    HeaderValue {
        /// Error source
        #[from]
        source: http::header::InvalidHeaderValue,
    },

    /// AuthToken error
    #[error("AuthToken error: {}", source)]
    AuthToken {
        /// Error source
        #[from]
        source: AuthTokenError,
    },

    #[error("token missing in the response")]
    AuthTokenNotInResponse,
}

// Explicitly implement From to easier propagate nested errors
impl From<AuthTokenScopeError> for AuthError {
    fn from(source: AuthTokenScopeError) -> Self {
        Self::AuthToken {
            source: AuthTokenError::Scope { source },
        }
    }
}
impl From<WebSsoError> for AuthError {
    fn from(source: v3websso::WebSsoError) -> Self {
        Self::AuthToken {
            source: source.into(),
        }
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::identity::v3::{AuthResponse, AuthToken};

    #[test]
    fn test_auth_validity_unset() {
        let auth = super::AuthToken::default();
        assert!(matches!(auth.get_state(None), AuthState::Unset));
    }

    #[test]
    fn test_auth_validity_expired() {
        let auth = super::AuthToken {
            token: String::new(),
            auth_info: Some(AuthResponse {
                token: AuthToken {
                    expires_at: chrono::Utc::now() - chrono::TimeDelta::days(1),
                    ..Default::default()
                },
            }),
        };
        assert!(matches!(auth.get_state(None), AuthState::Expired));
    }

    #[test]
    fn test_auth_validity_expire_soon() {
        let auth = super::AuthToken {
            token: String::new(),
            auth_info: Some(AuthResponse {
                token: AuthToken {
                    expires_at: chrono::Utc::now() + chrono::TimeDelta::minutes(10),
                    ..Default::default()
                },
            }),
        };
        assert!(matches!(
            auth.get_state(Some(chrono::TimeDelta::minutes(15))),
            AuthState::AboutToExpire
        ));
    }

    #[test]
    fn test_auth_validity_valid() {
        let auth = super::AuthToken {
            token: String::new(),
            auth_info: Some(AuthResponse {
                token: AuthToken {
                    expires_at: chrono::Utc::now() + chrono::TimeDelta::days(1),
                    ..Default::default()
                },
            }),
        };
        assert!(matches!(auth.get_state(None), AuthState::Valid));
    }
}
