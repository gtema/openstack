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

use std::fmt::Debug;
use std::str::FromStr;

use http::{HeaderName, HeaderValue};
use tracing::{debug, trace};

pub use openstack_sdk_auth_core::authtoken::{AuthToken, AuthTokenError};
use openstack_sdk_auth_core::types::AuthReceiptResponse;
use openstack_sdk_core::auth::auth_helper::AuthHelper;

use crate::api::RestEndpoint;
use crate::auth::auth_token_endpoint as token_v3;
use crate::auth::{authtoken_scope, v3_token_info, v3password, v3token, v3totp};
use crate::config;

pub use crate::auth::authtoken::authtoken_scope::AuthTokenScope;

/// Supported AuthTypes
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd)]
#[allow(clippy::enum_variant_names)]
pub enum AuthType {
    /// v3 Application Credentials
    V3ApplicationCredential,
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
    #[cfg(feature = "keystone_ng")]
    /// Federation.
    V4Federation,
    #[cfg(feature = "keystone_ng")]
    /// JWT.
    V4Jwt,
    #[cfg(feature = "passkey")]
    /// Passkey.
    V4Passkey,
}

impl FromStr for AuthType {
    type Err = AuthTokenError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "v3applicationcredential" | "applicationcredential" => {
                Ok(Self::V3ApplicationCredential)
            }
            "v3password" | "password" => Ok(Self::V3Password),
            "v3oidcaccesstoken" | "accesstoken" => Ok(Self::V3OidcAccessToken),
            "v3token" | "token" => Ok(Self::V3Token),
            "v3totp" => Ok(Self::V3Totp),
            "v3multifactor" => Ok(Self::V3Multifactor),
            "v3websso" => Ok(Self::V3WebSso),
            #[cfg(feature = "keystone_ng")]
            "v4federation" | "federation" => Ok(Self::V4Federation),
            #[cfg(feature = "keystone_ng")]
            "v4jwt" | "jwt" => Ok(Self::V4Jwt),
            #[cfg(feature = "passkey")]
            "v4passkey" | "passkey" => Ok(Self::V4Passkey),
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
            Self::V3OidcAccessToken => "v3oidcaccesstoken",
            Self::V3Token => "v3token",
            Self::V3Multifactor => "v3multifactor",
            Self::V3Totp => "v3totp",
            Self::V3WebSso => "v3websso",
            #[cfg(feature = "keystone_ng")]
            Self::V4Federation => "v4federation",
            #[cfg(feature = "keystone_ng")]
            Self::V4Jwt => "v4jwt",
            #[cfg(feature = "passkey")]
            Self::V4Passkey => "v4passkey",
        }
    }
}

/// Fill identity part of the v3 token auth builder using configured auth type data
async fn process_auth_type<S: AsRef<str>>(
    identity_builder: &mut token_v3::IdentityBuilder<'_>,
    auth_data: &config::Auth,
    connection_name: Option<S>,
    auth_helper: &mut (dyn AuthHelper + Send),
    auth_type: &AuthType,
) -> Result<(), AuthTokenError> {
    match auth_type {
        AuthType::V3Password => {
            v3password::fill_identity(identity_builder, auth_data, connection_name, auth_helper)
                .await
                .map_err(AuthTokenError::plugin)?;
        }
        AuthType::V3Token => {
            v3token::fill_identity(identity_builder, auth_data, connection_name, auth_helper)
                .await
                .map_err(AuthTokenError::plugin)?;
        }
        AuthType::V3Totp => {
            v3totp::fill_identity(identity_builder, auth_data, connection_name, auth_helper)
                .await
                .map_err(AuthTokenError::plugin)?;
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
pub(crate) async fn build_identity_data_from_config<'a>(
    config: &config::CloudConfig,
    auth_helper: &mut (dyn AuthHelper + Send),
) -> Result<token_v3::Identity<'a>, AuthTokenError> {
    let auth = config.auth.clone().ok_or(AuthTokenError::MissingAuthData)?;
    let auth_type = AuthType::from_cloud_config(config)?;
    let mut identity_builder = token_v3::IdentityBuilder::default();
    match auth_type {
        AuthType::V3Multifactor => {
            let mut methods: Vec<token_v3::Methods> = Vec::new();
            for auth_method in config
                .auth_methods
                .as_ref()
                .ok_or_else(|| AuthTokenError::MultifactorAuthMethodsList)?
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
    identity_builder.build().map_err(AuthTokenError::plugin)
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

    token_v3::RequestBuilder::default()
        .auth(auth_request_data.build().map_err(AuthTokenError::plugin)?)
        .build()
        .map_err(AuthTokenError::plugin)
}

/// Build Auth request from `AuthToken` and `AuthScope
pub(crate) fn build_reauth_request<'a>(
    auth: &AuthToken,
    scope: &AuthTokenScope,
) -> Result<token_v3::Request<'a>, AuthTokenError> {
    let identity_data = token_v3::Identity::try_from(auth).map_err(AuthTokenError::plugin)?;
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

    token_v3::RequestBuilder::default()
        .auth(auth_request_data.build().map_err(AuthTokenError::plugin)?)
        .build()
        .map_err(AuthTokenError::plugin)
}

/// Build Auth request from `Receipt`
pub(crate) async fn build_auth_request_from_receipt<'a>(
    config: &config::CloudConfig,
    receipt_header: HeaderValue,
    receipt_data: &AuthReceiptResponse,
    scope: &AuthTokenScope,
    auth_helper: &mut (dyn AuthHelper + Send),
) -> Result<impl RestEndpoint + 'a, AuthTokenError> {
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
    auth_request_data.identity(identity_builder.build().map_err(AuthTokenError::plugin)?);

    if let Ok(scope_data) = token_v3::Scope::try_from(scope) {
        auth_request_data.scope(scope_data);
    }

    token_v3::RequestBuilder::default()
        .auth(auth_request_data.build().map_err(AuthTokenError::plugin)?)
        .headers(
            [(
                Some(HeaderName::from_static("openstack-auth-receipt")),
                receipt_header,
            )]
            .iter()
            .cloned(),
        )
        .build()
        .map_err(AuthTokenError::plugin)
}

/// Prepare Endpoint for token info
pub(crate) fn build_token_info_endpoint<S: AsRef<str>>(
    subject_token: S,
) -> Result<v3_token_info::Request, AuthTokenError> {
    v3_token_info::RequestBuilder::default()
        .headers(
            [(
                Some(HeaderName::from_static("x-subject-token")),
                HeaderValue::from_str(subject_token.as_ref())?,
            )]
            .into_iter(),
        )
        .build()
        .map_err(AuthTokenError::plugin)
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    use crate::auth::auth_helper::Noop;
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

        let auth_data = build_identity_data_from_config(&config, &mut Noop::default())
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

        let auth_data = build_identity_data_from_config(&config, &mut Noop::default())
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
