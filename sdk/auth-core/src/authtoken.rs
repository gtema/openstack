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

//! OpenStack AuthToken based authorization (X-Auth-Token)

use std::fmt;
use std::fmt::Debug;

use http::{HeaderMap, HeaderValue};
use reqwest::{Response, StatusCode};
use secrecy::{ExposeSecret, SecretString};
use serde::{Deserialize, Serialize, Serializer};
use thiserror::Error;

use crate::authtoken_scope::*;
use crate::types::*;
use crate::{AuthError, AuthState};

/// AuthToken (X-Auth-Token) based auth errors
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum AuthTokenError {
    /// Auth request preparation error.
    #[error("error preparing authentication request: {}", source)]
    AuthRequest {
        /// The source of the error.
        #[source]
        source: Box<dyn std::error::Error + Send + Sync + 'static>,
    },

    /// Token is missing in the authentication response.
    #[error("token missing in the response")]
    AuthTokenNotInResponse,

    /// X-Subject-Token cannot be converted to string.
    #[error("token missing cannot be converted to string")]
    AuthTokenNotString,

    /// Header error
    #[error("header value error: {}", source)]
    HeaderValue {
        #[from]
        source: http::header::InvalidHeaderValue,
    },

    /// Unsupported identity method
    #[error(
        "AuthType `{}` is not a supported type for authenticating towards the cloud",
        auth_type
    )]
    IdentityMethod { auth_type: String },

    /// Unsupported identity method in sync mode
    #[error(
        "AuthType `{}` is not a supported type for authenticating towards the cloud with sync interface",
        auth_type
    )]
    IdentityMethodSync { auth_type: String },

    /// Auth data is missing
    #[error("Auth data is missing")]
    MissingAuthData,

    /// auth_url is missing
    #[error("Auth URL is missing")]
    MissingAuthUrl,

    /// Multifactor `auth_type` requires `auth_methods` to be an array of strings.
    #[error("`auth_methods` must be an array of string when `auth_type=multifactor`")]
    MultifactorAuthMethodsList,

    /// Token Scope error
    #[error("Scope error: {}", source)]
    Scope {
        /// The error source
        #[from]
        source: AuthTokenScopeError,
    },

    /// (De)Serialization error.
    #[error("failed to deserialize response body: {}", source)]
    Serde {
        /// The source of the error.
        #[from]
        source: serde_json::Error,
    },

    /// AuthPlugin error.
    #[error("plugin error: {}", source)]
    Plugin {
        /// The source of the error.
        #[source]
        source: Box<dyn std::error::Error + Send + Sync + 'static>,
    },
}

impl AuthTokenError {
    pub fn auth_request<E>(error: E) -> Self
    where
        E: std::error::Error + Send + Sync + 'static,
    {
        Self::AuthRequest {
            source: Box::new(error),
        }
    }
    pub fn plugin<E>(error: E) -> Self
    where
        E: std::error::Error + Send + Sync + 'static,
    {
        Self::Plugin {
            source: Box::new(error),
        }
    }
}

/// Explicitly implement From to easier propagate nested errors
impl From<AuthTokenScopeError> for AuthError {
    fn from(source: AuthTokenScopeError) -> Self {
        Self::AuthToken {
            source: AuthTokenError::Scope { source },
        }
    }
}

type AuthResult<T> = Result<T, AuthTokenError>;

/// OpenStack AuthToken authorization structure
#[derive(Clone, Default, Deserialize, Serialize)]
pub struct AuthToken {
    /// Token itself
    #[serde(serialize_with = "serialize_secret_string")]
    pub token: SecretString,
    /// Auth info reported by the server
    pub auth_info: Option<AuthResponse>,
}

fn serialize_secret_string<S>(secret: &SecretString, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(secret.expose_secret())
}

impl From<&str> for AuthToken {
    fn from(value: &str) -> Self {
        Self {
            token: value.into(),
            ..Default::default()
        }
    }
}

impl Debug for AuthToken {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("AuthToken")
            .field("data", &self.auth_info)
            .finish()
    }
}

impl AuthToken {
    /// Construct new AuthToken instance.
    pub fn new<T>(token: T, auth_info: Option<AuthResponse>) -> Self
    where
        T: Into<SecretString>,
    {
        Self {
            token: token.into(),
            auth_info,
        }
    }
    /// Adds X-Auth-Token header to a request headers.
    ///
    /// Returns an error if the token string cannot be parsed as a header value.
    pub fn set_header(&self, headers: &mut HeaderMap<HeaderValue>) -> AuthResult<()> {
        let mut token_header_value = HeaderValue::from_str(self.token.expose_secret())?;
        token_header_value.set_sensitive(true);
        headers.insert("X-Auth-Token", token_header_value);

        Ok(())
    }

    /// Detect authentication validity (valid/expired/unset)
    ///
    /// Offset can be used to calculate imminent expiration.
    pub fn get_state(&self, expiration_offset: Option<chrono::TimeDelta>) -> AuthState {
        let expiration = chrono::Utc::now();
        let soon_expiration = match expiration_offset {
            Some(offset) => expiration + offset,
            None => expiration,
        };
        match &self.auth_info {
            Some(data) => {
                if data.token.expires_at <= expiration {
                    AuthState::Expired
                } else if data.token.expires_at <= soon_expiration {
                    AuthState::AboutToExpire
                } else {
                    AuthState::Valid
                }
            }
            None => AuthState::Unset,
        }
    }

    /// Get Token scope information
    pub fn get_scope(&self) -> AuthTokenScope {
        match &self.auth_info {
            Some(data) => AuthTokenScope::from(data),
            _ => AuthTokenScope::Unscoped,
        }
    }

    /// Parse [`Response`] into the AuthToken.
    pub async fn from_reqwest_response(response: Response) -> Result<Self, AuthError> {
        if !response.status().is_success() {
            let status = response.status();
            if let StatusCode::UNAUTHORIZED = status
                && let Some(receipt_header) = response.headers().get("openstack-auth-receipt")
            {
                let receipt_token = receipt_header
                    .to_str()
                    .map_err(|_| AuthError::AuthReceiptNotString)?
                    .into();

                let body_text = response.text().await?;
                if let Ok(mut receipt) = serde_json::from_str::<AuthReceiptResponse>(&body_text) {
                    receipt.token = Some(receipt_token);
                    return Err(AuthError::AuthReceipt(Box::new(receipt)));
                }

                if let Ok(data) = serde_json::from_str::<AuthErrorResponse>(&body_text) {
                    return Err(AuthError::Identity(data.error));
                } else {
                    return Err(AuthError::UnknownAuth {
                        code: status.into(),
                        message: Some(body_text),
                    });
                }
            }

            let body = response.text().await?;

            if let Ok(data) = serde_json::from_str::<AuthErrorResponse>(&body) {
                return Err(AuthError::Identity(data.error));
            } else {
                return Err(AuthError::UnknownAuth {
                    code: status.into(),
                    message: Some(body),
                });
            }
        }

        let token = response
            .headers()
            .get("x-subject-token")
            .ok_or(AuthError::AuthToken {
                source: AuthTokenError::AuthTokenNotInResponse,
            })?
            .to_str()
            .map_err(|_| AuthError::AuthToken {
                source: AuthTokenError::AuthTokenNotString,
            })?
            .to_string();

        let token_info: AuthResponse = response.json::<AuthResponse>().await?;

        Ok(Self {
            token: SecretString::from(token),
            auth_info: Some(token_info),
        })
    }
}

impl TryFrom<http::Response<bytes::Bytes>> for AuthToken {
    type Error = AuthTokenError;
    fn try_from(value: http::Response<bytes::Bytes>) -> Result<Self, Self::Error> {
        let token = value
            .headers()
            .get("x-subject-token")
            .ok_or(AuthTokenError::AuthTokenNotInResponse)?
            .to_str()
            .map_err(|_| AuthTokenError::AuthTokenNotString)?;

        let token_info: AuthResponse = serde_json::from_slice(value.body())?;
        Ok(Self::new(token, Some(token_info)))
    }
}

#[cfg(test)]
mod tests {
    use chrono::Local;
    use http::response::Builder;
    use reqwest::Response;
    use secrecy::ExposeSecret;
    use serde_json::to_string;

    use super::AuthError;
    use super::AuthToken;
    use super::AuthTokenError;
    use crate::AuthState;
    use crate::authtoken_scope::AuthTokenScope;
    use crate::authtoken_scope::AuthTokenScopeError;
    use crate::types::*;
    use http::HeaderMap;

    #[tokio::test]
    async fn test_from_reqwest_response_receipt() {
        let auth_receipt = AuthReceiptResponse {
            receipt: AuthReceipt {
                methods: vec!["password".into()],
                user: User {
                    id: "uid".into(),
                    name: "uname".into(),
                    ..Default::default()
                },
                expires_at: Local::now(),
                ..Default::default()
            },
            required_auth_methods: vec![vec!["totp".into(), "password".into()]],
            token: None,
        };
        let http_response = Builder::new()
            .status(401)
            .header("openstack-auth-receipt", "foobar")
            .header("content-type", "application/json")
            .body(to_string(&auth_receipt).unwrap())
            .unwrap();

        let response: Response = Response::from(http_response);

        let rsp = AuthToken::from_reqwest_response(response).await;
        match rsp {
            Err(AuthError::AuthReceipt(receipt)) => {
                let mut expected = auth_receipt.clone();
                expected.token = Some("foobar".into());
                assert_eq!(expected, *receipt);
            }
            other => {
                panic!("wrong response for the expected receipt error: {:?}", other);
            }
        }
    }

    #[tokio::test]
    async fn test_from_reqwest_response_error() {
        let err = AuthErrorResponse {
            error: IdentityError {
                code: 401,
                message: "internal error".into(),
            },
        };
        let http_response = Builder::new()
            .status(401)
            .header("content-type", "application/json")
            .body(to_string(&err).unwrap())
            .unwrap();

        let response: Response = Response::from(http_response);

        let rsp = AuthToken::from_reqwest_response(response).await;
        match rsp {
            Err(AuthError::Identity(error)) => {
                assert_eq!(error, err.error);
            }
            other => {
                panic!("wrong response: {:?}", other);
            }
        }
    }

    #[tokio::test]
    async fn test_from_reqwest_response_success() {
        let auth = AuthResponse::default();
        let http_response = Builder::new()
            .status(201)
            .header("content-type", "application/json")
            .header("x-subject-token", "foobar")
            .body(to_string(&auth).unwrap())
            .unwrap();

        let response: Response = Response::from(http_response);

        let rsp = AuthToken::from_reqwest_response(response).await;
        match rsp {
            Ok(rsp) => {
                assert_eq!(auth, rsp.auth_info.unwrap());
                assert_eq!("foobar", rsp.token.expose_secret());
            }
            other => {
                panic!("wrong response: {:?}", other);
            }
        }
    }

    #[tokio::test]
    async fn test_from_reqwest_response_success_no_token() {
        let auth = AuthResponse::default();
        let http_response = Builder::new()
            .status(201)
            .header("content-type", "application/json")
            .body(to_string(&auth).unwrap())
            .unwrap();

        let response: Response = Response::from(http_response);

        let rsp = AuthToken::from_reqwest_response(response).await;
        match rsp {
            Err(AuthError::AuthToken {
                source: AuthTokenError::AuthTokenNotInResponse,
            }) => {}
            other => {
                panic!("wrong response: {:?}", other);
            }
        }
    }

    #[tokio::test]
    async fn test_from_reqwest_response_malformed_json_success() {
        let http_response = Builder::new()
            .status(201)
            .header("content-type", "application/json")
            .header("x-subject-token", "foobar")
            .body(String::from("{invalid"))
            .unwrap();

        let response: Response = Response::from(http_response);

        let rsp = AuthToken::from_reqwest_response(response).await;
        // reqwest wraps serde errors in reqwest::Error, so it becomes AuthError::Serde or AuthError::Reqwest
        assert!(matches!(
            rsp,
            Err(AuthError::Serde { .. }) | Err(AuthError::Reqwest { .. })
        ));
    }

    #[tokio::test]
    async fn test_from_reqwest_response_500_json_error() {
        let err = AuthErrorResponse {
            error: IdentityError {
                code: 500,
                message: "internal server error".into(),
            },
        };
        let http_response = Builder::new()
            .status(500)
            .header("content-type", "application/json")
            .body(to_string(&err).unwrap())
            .unwrap();

        let response: Response = Response::from(http_response);

        let rsp = AuthToken::from_reqwest_response(response).await;
        match rsp {
            Err(AuthError::Identity(e)) => assert_eq!(e.code, 500),
            other => panic!("wrong response: {:?}", other),
        }
    }

    #[tokio::test]
    async fn test_from_reqwest_response_502_plain_text() {
        let http_response = Builder::new()
            .status(502)
            .body(String::from("upstream timeout"))
            .unwrap();

        let response: Response = Response::from(http_response);

        let rsp = AuthToken::from_reqwest_response(response).await;
        match rsp {
            Err(AuthError::UnknownAuth { code, message }) => {
                assert_eq!(code, 502);
                assert_eq!(message, Some("upstream timeout".into()));
            }
            other => panic!("wrong response: {:?}", other),
        }
    }

    #[tokio::test]
    async fn test_from_reqwest_response_403_identity_error() {
        let err = AuthErrorResponse {
            error: IdentityError {
                code: 403,
                message: "forbidden".into(),
            },
        };
        let http_response = Builder::new()
            .status(403)
            .header("content-type", "application/json")
            .body(to_string(&err).unwrap())
            .unwrap();

        let response: Response = Response::from(http_response);

        let rsp = AuthToken::from_reqwest_response(response).await;
        match rsp {
            Err(AuthError::Identity(e)) => assert_eq!(e.code, 403),
            other => panic!("wrong response: {:?}", other),
        }
    }

    #[tokio::test]
    async fn test_from_reqwest_response_401_receipt_malformed_json() {
        let http_response = Builder::new()
            .status(401)
            .header("openstack-auth-receipt", "foobar")
            .body(String::from("{malformed"))
            .unwrap();

        let response: Response = Response::from(http_response);

        let rsp = AuthToken::from_reqwest_response(response).await;
        match rsp {
            Err(AuthError::UnknownAuth { code, message }) => {
                assert_eq!(code, 401);
                assert_eq!(message, Some("{malformed".into()));
            }
            other => panic!("wrong response: {:?}", other),
        }
    }

    #[tokio::test]
    async fn test_from_reqwest_response_status_200() {
        let auth = AuthResponse::default();
        let http_response = Builder::new()
            .status(200)
            .header("content-type", "application/json")
            .header("x-subject-token", "tok")
            .body(to_string(&auth).unwrap())
            .unwrap();

        let response: Response = Response::from(http_response);

        let rsp = AuthToken::from_reqwest_response(response).await;
        assert!(rsp.is_ok());
        assert_eq!(rsp.unwrap().token.expose_secret(), "tok");
    }

    #[test]
    fn test_auth_token_from_str() {
        let auth: AuthToken = "my-secret-token".into();
        assert_eq!("my-secret-token", auth.token.expose_secret());
        assert!(auth.auth_info.is_none());
    }

    #[test]
    fn test_auth_token_set_header_success() {
        let auth = AuthToken::new("valid-token", None);
        let mut headers = HeaderMap::new();
        auth.set_header(&mut headers).unwrap();
        let val = headers.get("X-Auth-Token").expect("header missing");
        assert_eq!(val.to_str().unwrap(), "valid-token");
    }

    #[test]
    fn test_auth_token_debug_no_secret_leak() {
        let auth = AuthToken::new("super-secret", None);
        let debug = format!("{:?}", auth);
        assert!(debug.contains("AuthToken"));
        assert!(!debug.contains("super-secret"));
    }

    #[test]
    fn test_auth_token_get_scope_unscoped() {
        let auth = AuthToken::new("tok", None);
        assert!(matches!(auth.get_scope(), AuthTokenScope::Unscoped));
    }

    #[test]
    fn test_auth_token_get_scope_project() {
        let auth = AuthToken::new(
            "tok",
            Some(AuthResponse {
                token: TokenInfo {
                    project: Some(Project {
                        id: Some("1".into()),
                        name: Some("p".into()),
                        domain: None,
                    }),
                    ..Default::default()
                },
            }),
        );
        assert!(matches!(auth.get_scope(), AuthTokenScope::Project(_)));
    }

    #[test]
    fn test_auth_token_get_state_zero_offset() {
        let auth = AuthToken::new(
            String::new(),
            Some(AuthResponse {
                token: TokenInfo {
                    expires_at: chrono::Utc::now() + chrono::TimeDelta::milliseconds(500),
                    ..Default::default()
                },
            }),
        );
        // With zero offset, soon_expiration == expiration, so AboutToExpire is unreachable
        assert!(matches!(
            auth.get_state(Some(chrono::TimeDelta::zero())),
            AuthState::Valid
        ));
    }

    #[test]
    fn test_auth_token_get_state_expires_exactly_at_offset() {
        let expires = chrono::Utc::now() + chrono::TimeDelta::minutes(10);
        let auth = AuthToken::new(
            String::new(),
            Some(AuthResponse {
                token: TokenInfo {
                    expires_at: expires,
                    ..Default::default()
                },
            }),
        );
        assert!(matches!(
            auth.get_state(Some(chrono::TimeDelta::minutes(10))),
            AuthState::AboutToExpire
        ));
    }

    #[test]
    fn test_auth_token_get_state_no_offset_valid() {
        let auth = AuthToken::new(
            String::new(),
            Some(AuthResponse {
                token: TokenInfo {
                    expires_at: chrono::Utc::now() + chrono::TimeDelta::seconds(1),
                    ..Default::default()
                },
            }),
        );
        assert!(matches!(auth.get_state(None), AuthState::Valid));
    }

    #[test]
    fn test_auth_token_get_state_expires_exactly_now() {
        let now = chrono::Utc::now();
        let auth = AuthToken::new(
            String::new(),
            Some(AuthResponse {
                token: TokenInfo {
                    expires_at: now,
                    ..Default::default()
                },
            }),
        );
        assert!(matches!(
            auth.get_state(Some(chrono::TimeDelta::minutes(1))),
            AuthState::Expired
        ));
    }

    #[test]
    fn test_try_from_http_response_success() {
        let auth = AuthResponse::default();
        let body_bytes: bytes::Bytes = to_string(&auth).unwrap().into_bytes().into();

        let http_response = http::Response::builder()
            .status(201)
            .header("content-type", "application/json")
            .header("x-subject-token", "tok")
            .body(body_bytes)
            .unwrap();

        let result = AuthToken::try_from(http_response);
        assert!(result.is_ok());
        let token = result.unwrap();
        assert_eq!("tok", token.token.expose_secret());
    }

    #[test]
    fn test_try_from_http_response_no_token() {
        let auth = AuthResponse::default();
        let body_bytes: bytes::Bytes = to_string(&auth).unwrap().into_bytes().into();

        let http_response = http::Response::builder()
            .status(201)
            .header("content-type", "application/json")
            .body(body_bytes)
            .unwrap();

        let result = AuthToken::try_from(http_response);
        assert!(matches!(
            result,
            Err(AuthTokenError::AuthTokenNotInResponse)
        ));
    }

    #[test]
    fn test_try_from_http_response_malformed_json() {
        let http_response = http::Response::builder()
            .status(201)
            .header("x-subject-token", "tok")
            .body(bytes::Bytes::from_static(b"{bad"))
            .unwrap();

        let result = AuthToken::try_from(http_response);
        assert!(matches!(result, Err(AuthTokenError::Serde { .. })));
    }

    #[test]
    fn test_auth_token_error_auth_request() {
        #[derive(Debug)]
        struct MyErr(&'static str);
        impl std::fmt::Display for MyErr {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.0)
            }
        }
        impl std::error::Error for MyErr {}

        let err = AuthTokenError::auth_request(MyErr("req"));
        assert!(matches!(err, AuthTokenError::AuthRequest { .. }));
        assert!(format!("{}", err).contains("req"));
    }

    #[test]
    fn test_auth_token_error_plugin() {
        #[derive(Debug)]
        struct MyErr(&'static str);
        impl std::fmt::Display for MyErr {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.0)
            }
        }
        impl std::error::Error for MyErr {}

        let err = AuthTokenError::plugin(MyErr("plug"));
        assert!(matches!(err, AuthTokenError::Plugin { .. }));
        assert!(format!("{}", err).contains("plug"));
    }

    #[test]
    fn test_from_auth_token_scope_error_to_auth_error() {
        let scope_err = AuthTokenScopeError::MissingScope;
        let auth_err: AuthError = scope_err.into();
        assert!(matches!(
            auth_err,
            AuthError::AuthToken {
                source: AuthTokenError::Scope { .. }
            }
        ));
    }

    #[test]
    fn test_auth_token_error_identity_method_display() {
        let err = AuthTokenError::IdentityMethod {
            auth_type: "jwt".into(),
        };
        assert!(format!("{}", err).contains("jwt"));
    }

    #[test]
    fn test_auth_token_error_missing_data_display() {
        let err = AuthTokenError::MissingAuthData;
        assert_eq!(format!("{}", err), "Auth data is missing");
    }

    #[test]
    fn test_auth_token_error_missing_url_display() {
        let err = AuthTokenError::MissingAuthUrl;
        assert_eq!(format!("{}", err), "Auth URL is missing");
    }
}
