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
use std::str::FromStr;

use http::{HeaderMap, HeaderName, HeaderValue};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use tracing::{debug, error, trace};

use crate::api::identity::v3::auth::token::get as token_v3_info;
use crate::api::RestEndpoint;
use crate::auth::auth_token_endpoint as token_v3;
#[cfg(feature = "keystone_ng")]
use crate::auth::v3federation;
use crate::auth::{
    auth_helper::AuthHelper, authtoken_scope, v3applicationcredential, v3oidcaccesstoken,
    v3password, v3token, v3totp, v3websso, AuthState,
};
use crate::config;
use crate::types::identity::v3::{AuthReceiptResponse, AuthResponse};

pub use crate::auth::authtoken::authtoken_scope::AuthTokenScope;

/// AuthToken (X-Auth-Token) based auth errors
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum AuthTokenError {
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

    /// `Identity` request part build error
    #[error("Cannot construct identity auth information from config: {}", source)]
    AuthRequestIdentity {
        /// The error source
        #[from]
        source: token_v3::IdentityBuilderError,
    },

    /// `Auth` request part build error
    #[error("error preparing auth request: {}", source)]
    AuthRequestAuth {
        /// The error source
        #[from]
        source: token_v3::AuthBuilderError,
    },

    /// Auth request build error
    #[error("error preparing auth request: {}", source)]
    AuthRequest {
        /// The error source
        #[from]
        source: token_v3::RequestBuilderError,
    },

    /// Token `Info` request build error
    #[error("error preparing token info request: {}", source)]
    InfoRequest {
        /// The error source
        #[from]
        source: token_v3_info::RequestBuilderError,
    },

    /// Token Scope error
    #[error("Scope error: {}", source)]
    Scope {
        /// The error source
        #[from]
        source: authtoken_scope::AuthTokenScopeError,
    },

    /// ApplicationCredentials Identity error
    #[error("ApplicationCredential authentication error: {}", source)]
    ApplicationCredential {
        /// The error source
        #[from]
        source: v3applicationcredential::ApplicationCredentialError,
    },

    /// Oidc Access Token autherror
    #[error("OIDC access token authentication error: {}", source)]
    OidcAccessToken {
        /// The error source
        #[from]
        source: v3oidcaccesstoken::OidcAccessTokenError,
    },

    /// Password Identity error
    #[error("Password based authentication error: {}", source)]
    Password {
        /// The error source
        #[from]
        source: v3password::PasswordError,
    },

    /// Token Identity error
    #[error("Token based authentication error: {}", source)]
    Token {
        /// The error source
        #[from]
        source: v3token::TokenError,
    },

    /// TOTP Idetinty error
    #[error("Password based authentication error: {}", source)]
    Totp {
        /// The error source
        #[from]
        source: v3totp::TotpError,
    },

    /// WebSSO Identity error
    #[error("SSO based authentication error: {}", source)]
    WebSso {
        /// The error source
        #[from]
        source: v3websso::WebSsoError,
    },

    /// Federation Identity error
    #[cfg(feature = "keystone_ng")]
    #[error("Federation based authentication error: {}", source)]
    Federation {
        /// The error source
        #[from]
        source: v3federation::FederationError,
    },
}

type AuthResult<T> = Result<T, AuthTokenError>;

/// OpenStack AuthToken authorization structure
#[derive(Clone, Default, Deserialize, Serialize)]
pub struct AuthToken {
    /// Token itself
    pub(crate) token: String,
    /// Auth info reported by the server
    pub(crate) auth_info: Option<AuthResponse>,
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
            Some(ref data) => AuthTokenScope::from(data),
            _ => AuthTokenScope::Unscoped,
        }
    }
}

/// Supported AuthTypes
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd)]
#[allow(clippy::enum_variant_names)]
pub enum AuthType {
    /// v3 Application Credentials
    V3ApplicationCredential,
    #[cfg(feature = "keystone_ng")]
    /// Federation
    V3Federation,
    /// OIDC Access token
    V3OidcAccessToken,
    /// v3 Password
    V3Password,
    /// v3 Token
    V3Token,
    /// TOTP
    V3Totp,
    /// v3multifactor
    V3Multifactor,
    /// WebSSO
    V3WebSso,
}

impl FromStr for AuthType {
    type Err = AuthTokenError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "v3applicationcredential" | "applicationcredential" => {
                Ok(Self::V3ApplicationCredential)
            }
            "v3password" | "password" => Ok(Self::V3Password),
            #[cfg(feature = "keystone_ng")]
            "v3federation" | "federation" => Ok(Self::V3Federation),
            "v3oidcaccesstoken" | "accesstoken" => Ok(Self::V3OidcAccessToken),
            "v3token" | "token" => Ok(Self::V3Token),
            "v3totp" => Ok(Self::V3Totp),
            "v3multifactor" => Ok(Self::V3Multifactor),
            "v3websso" => Ok(Self::V3WebSso),
            other => Err(Self::Err::IdentityMethod {
                auth_type: other.into(),
            }),
        }
    }
}

impl AuthType {
    /// Get the auth_type of the cloud connection
    pub fn from_cloud_config(config: &config::CloudConfig) -> Result<Self, AuthTokenError> {
        if let Some(auth_type) = &config.auth_type {
            Self::from_str(auth_type)
        } else {
            Ok(Self::V3Password)
        }
    }

    /// Return String representation of the AuthType
    pub fn as_str(self) -> &'static str {
        match self {
            Self::V3ApplicationCredential => "v3applicationcredential",
            Self::V3Password => "v3password",
            #[cfg(feature = "keystone_ng")]
            Self::V3Federation => "v3federation",
            Self::V3OidcAccessToken => "v3oidcaccesstoken",
            Self::V3Token => "v3token",
            Self::V3Multifactor => "v3multifactor",
            Self::V3Totp => "v3totp",
            Self::V3WebSso => "v3websso",
        }
    }
}

/// Fill identity part of the v3 token auth builder using configured auth type data
async fn process_auth_type<A, S: AsRef<str>>(
    identity_builder: &mut token_v3::IdentityBuilder<'_>,
    auth_data: &config::Auth,
    connection_name: Option<S>,
    auth_helper: &mut A,
    auth_type: &AuthType,
) -> Result<(), AuthTokenError>
where
    A: AuthHelper,
{
    match auth_type {
        AuthType::V3ApplicationCredential => {
            v3applicationcredential::fill_identity(
                identity_builder,
                auth_data,
                connection_name,
                auth_helper,
            )
            .await?;
        }
        AuthType::V3Password => {
            v3password::fill_identity(identity_builder, auth_data, connection_name, auth_helper)
                .await?;
        }
        AuthType::V3Token => {
            v3token::fill_identity(identity_builder, auth_data, connection_name, auth_helper)
                .await?;
        }
        AuthType::V3Totp => {
            v3totp::fill_identity(identity_builder, auth_data, connection_name, auth_helper)
                .await?;
        }
        other => {
            return Err(AuthTokenError::IdentityMethod {
                auth_type: other.as_str().into(),
            });
        }
    };
    Ok(())
}

/// Prepare Token endpoint filling identity data from `CloudConfig`
pub(crate) async fn build_identity_data_from_config<'a, A>(
    config: &config::CloudConfig,
    auth_helper: &mut A,
) -> Result<token_v3::Identity<'a>, AuthTokenError>
where
    A: AuthHelper,
{
    let auth = config.auth.clone().ok_or(AuthTokenError::MissingAuthData)?;
    let auth_type = AuthType::from_cloud_config(config)?;
    let mut identity_builder = token_v3::IdentityBuilder::default();
    match auth_type {
        AuthType::V3Multifactor => {
            let mut methods: Vec<token_v3::Methods> = Vec::new();
            for auth_method in config
                .auth_methods
                .as_ref()
                .expect("`auth_methods` is an array of string when auth_type=`multifactor`")
            {
                let method_type = AuthType::from_str(auth_method)?;
                process_auth_type(
                    &mut identity_builder,
                    &auth,
                    config.name.as_ref(),
                    auth_helper,
                    &method_type,
                )
                .await?;
                // process_auth_type resets methods so we need to recover it
                match method_type {
                    AuthType::V3Password => {
                        methods.push(token_v3::Methods::Password);
                    }
                    AuthType::V3Token => {
                        methods.push(token_v3::Methods::Token);
                    }
                    AuthType::V3Totp => {
                        methods.push(token_v3::Methods::Totp);
                    }
                    _other => {}
                };
            }

            identity_builder.methods(methods);
        }
        other => {
            process_auth_type(
                &mut identity_builder,
                &auth,
                config.name.as_ref(),
                auth_helper,
                &other,
            )
            .await?;
        }
    };
    Ok(identity_builder.build()?)
}

/// Build Auth request from `Identity` and `AuthScope`
pub(crate) fn build_auth_request_with_identity_and_scope<'a>(
    auth: &token_v3::Identity<'a>,
    scope: &AuthTokenScope,
) -> Result<token_v3::Request<'a>, AuthTokenError> {
    let mut auth_request_data = token_v3::AuthBuilder::default();
    auth_request_data.identity(auth.clone());
    match scope {
        // For no scope we should not fill anything
        AuthTokenScope::Unscoped => {}
        _ => {
            if let Ok(scope_data) = token_v3::Scope::try_from(scope) {
                auth_request_data.scope(scope_data);
            }
        }
    }

    Ok(token_v3::RequestBuilder::default()
        .auth(auth_request_data.build()?)
        .build()?)
}

/// Build Auth request from `AuthToken` and `AuthScope
pub(crate) fn build_reauth_request<'a>(
    auth: &AuthToken,
    scope: &AuthTokenScope,
) -> Result<token_v3::Request<'a>, AuthTokenError> {
    let identity_data = token_v3::Identity::try_from(auth)?;
    let mut auth_request_data = token_v3::AuthBuilder::default();
    auth_request_data.identity(identity_data);
    match scope {
        // For no scope we should not fill anything
        AuthTokenScope::Unscoped => {}
        _ => {
            if let Ok(scope_data) = token_v3::Scope::try_from(scope) {
                auth_request_data.scope(scope_data);
            }
        }
    }

    Ok(token_v3::RequestBuilder::default()
        .auth(auth_request_data.build()?)
        .build()?)
}

/// Build Auth request from `Receipt`
pub(crate) async fn build_auth_request_from_receipt<'a, A>(
    config: &config::CloudConfig,
    receipt_header: HeaderValue,
    receipt_data: &AuthReceiptResponse,
    scope: &AuthTokenScope,
    auth_helper: &mut A,
) -> Result<impl RestEndpoint + 'a, AuthTokenError>
where
    A: AuthHelper,
{
    let mut identity_builder = token_v3::IdentityBuilder::default();
    let auth = config.auth.clone().ok_or(AuthTokenError::MissingAuthData)?;
    // Check required_auth_methods rules
    // Note: Keystone returns list of lists (as set of different rules)
    debug!(
        "Server requests additional authentication with one of: {:?}",
        &receipt_data.required_auth_methods
    );
    for auth_rule in &receipt_data.required_auth_methods {
        for required_method in auth_rule {
            if !receipt_data
                .receipt
                .methods
                .iter()
                .any(|x| x == required_method)
            {
                trace!("Adding {:?} auth data", required_method);
                process_auth_type(
                    &mut identity_builder,
                    &auth,
                    config.name.as_ref(),
                    auth_helper,
                    &AuthType::from_str(required_method)?,
                )
                .await?;
            }
        }
    }

    let mut auth_request_data = token_v3::AuthBuilder::default();
    auth_request_data.identity(identity_builder.build()?);

    if let Ok(scope_data) = token_v3::Scope::try_from(scope) {
        auth_request_data.scope(scope_data);
    }

    Ok(token_v3::RequestBuilder::default()
        .auth(auth_request_data.build()?)
        .headers(
            [(
                Some(HeaderName::from_static("openstack-auth-receipt")),
                receipt_header,
            )]
            .iter()
            .cloned(),
        )
        .build()?)
}

/// Prepare Endpoint for token info
pub(crate) fn build_token_info_endpoint<S: AsRef<str>>(
    subject_token: S,
) -> Result<token_v3_info::Request, AuthTokenError> {
    Ok(token_v3_info::RequestBuilder::default()
        .headers(
            [(
                Some(HeaderName::from_static("x-subject-token")),
                HeaderValue::from_str(subject_token.as_ref()).expect("Valid string"),
            )]
            .into_iter(),
        )
        .build()?)
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    use crate::auth::auth_helper::NonInteractive;
    use crate::config;

    #[tokio::test]
    async fn test_config_into_auth_password() -> Result<(), &'static str> {
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

        let auth_data = build_identity_data_from_config(&config, &mut NonInteractive::default())
            .await
            .unwrap();
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

    #[tokio::test]
    async fn test_config_into_auth_token() -> Result<(), &'static str> {
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

        let auth_data = build_identity_data_from_config(&config, &mut NonInteractive::default())
            .await
            .unwrap();
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
