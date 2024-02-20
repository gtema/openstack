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

use http::{HeaderMap, HeaderName, HeaderValue};
use serde::{Deserialize, Serialize};

use std::borrow::Cow;
use std::convert::TryFrom;
use std::fmt::{self, Debug};
use std::str::FromStr;
use tracing::{error, trace};

use thiserror::Error;

use futures::io::Error as IoError;

use dialoguer::{Input, Password};

use crate::api::identity::v3::auth::os_federation::{
    identity_provider::protocol::websso::get as fed_idp_sso_get, websso::get as fed_sso_get,
};
use crate::api::identity::v3::auth::token::create as token_v3;
use crate::api::identity::v3::auth::token::get as token_v3_info;
use crate::api::RestEndpoint;
use crate::config;
use crate::types::identity::v3::{AuthReceiptResponse, AuthResponse, Domain, Project};

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

    #[error(
        "AuthType `{}` is not a supported type for authenticating towards the cloud",
        auth_type
    )]
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
    #[error("MFA passcode is missing")]
    MissingPasscode,

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
    #[error("Cannot construct user domain information from config: {}", source)]
    AuthUserDomainBuild {
        #[from]
        source: token_v3::DomainBuilderError,
    },
    #[error("Cannot construct project scope information from config: {}", source)]
    AuthProjectScopeBuild {
        #[from]
        source: token_v3::ProjectBuilderError,
    },
    #[error(
        "Cannot construct project scope domain information from config: {}",
        source
    )]
    AuthProjectScopeDomainBuild {
        #[from]
        source: token_v3::ProjectDomainBuilderError,
    },
    #[error(
        "Cannot construct project scope domain information from config: {}",
        source
    )]
    AuthDomainScopeBuild {
        #[from]
        source: token_v3::ScopeDomainBuilderError,
    },
    #[error("Cannot construct TOTP user information: {}", source)]
    AuthTotpUserBuild {
        #[from]
        source: token_v3::TotpUserBuilderError,
    },
    #[error("Cannot construct TOTP user domain information: {}", source)]
    AuthTotpUserDomainBuild {
        #[from]
        source: token_v3::UserDomainStructBuilderError,
    },
    #[error("Cannot construct TOTP auth information: {}", source)]
    AuthTotpBuild {
        #[from]
        source: token_v3::TotpBuilderError,
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
    #[error("error preparing auth request: {}", source)]
    AuthWebSsoBuilderError {
        #[from]
        source: fed_sso_get::RequestBuilderError,
    },
    #[error("error preparing auth request: {}", source)]
    AuthWebIdpSsoBuilderError {
        #[from]
        source: fed_idp_sso_get::RequestBuilderError,
    },
    #[error("error preparing auth request: missing protocol information for federated login")]
    AuthSsoMissingProtocol,
    #[error("error preparing auth request: {}", source)]
    AuthTokenInfoBuilderError {
        #[from]
        source: token_v3_info::RequestBuilderError,
    },

    #[error("WebSSO callback didn't return a token")]
    WebSSONoToken,

    #[error("WebSSO authentication failed")]
    WebSSOFailed,

    #[error("`IO` error: {}", source)]
    IO {
        #[from]
        source: IoError,
    },

    #[error("`Join` error: {}", source)]
    Join {
        #[from]
        source: tokio::task::JoinError,
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
    pub fn auth_builder(err: token_v3::AuthBuilderError) -> Self {
        AuthError::AuthBuilderError { source: err }
    }
    pub fn auth_identity_builder(err: token_v3::IdentityBuilderError) -> Self {
        AuthError::AuthIdentityBuild { source: err }
    }
}

pub(crate) type AuthResult<T> = Result<T, AuthError>;

/// Supported AuthTypes
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd)]
#[allow(clippy::enum_variant_names)]
pub enum AuthType {
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
    type Err = AuthError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "v3password" | "password" => Ok(Self::V3Password),
            "v3token" | "token" => Ok(Self::V3Token),
            "v3totp" => Ok(Self::V3Totp),
            "v3multifactor" => Ok(Self::V3Multifactor),
            "v3websso" => Ok(Self::V3WebSso),
            other => Err(AuthError::auth_type(other)),
        }
    }
}

impl AuthType {
    /// Get the auth_type of the cloud connection
    pub fn from_cloud_config(config: &config::CloudConfig) -> Result<Self, AuthError> {
        if let Some(auth_type) = &config.auth_type {
            Self::from_str(auth_type)
        } else {
            Ok(Self::V3Password)
        }
    }

    /// Return String representation of the AuthType
    pub fn as_str(self) -> &'static str {
        match self {
            Self::V3Password => "v3password",
            Self::V3Token => "v3token",
            Self::V3Multifactor => "v3multifactor",
            Self::V3Totp => "v3totp",
            Self::V3WebSso => "v3websso",
        }
    }
}

/// Fill Auth Request Identity with user password data
fn fill_identity_using_password(
    identity_builder: &mut token_v3::IdentityBuilder<'_>,
    auth_data: &config::Auth,
    interactive: bool,
) -> Result<(), AuthError> {
    identity_builder.methods(Vec::from([token_v3::Methods::Password]));
    let mut user = token_v3::UserBuilder::default();
    // Set user_id or name
    if let Some(val) = &auth_data.user_id {
        user.id(val.clone());
    }
    if let Some(val) = &auth_data.username {
        user.name(val.clone());
    }
    if auth_data.user_id.is_none() && auth_data.username.is_none() {
        if interactive {
            // Or ask user for username in interactive mode
            let name: String = Input::new()
                .with_prompt("Username:")
                .interact_text()
                .unwrap();
            user.name(name);
        } else {
            return Err(AuthError::MissingUserId);
        }
    }
    // Fill password
    if let Some(val) = &auth_data.password {
        user.password(val.clone());
    } else if interactive {
        // Or ask user for password
        let password = Password::new()
            .with_prompt("User Password")
            .interact()
            .unwrap();
        user.password(password.to_string());
    } else {
        return Err(AuthError::MissingPassword);
    }

    // Process user domain information
    if auth_data.user_domain_id.is_some() || auth_data.user_domain_name.is_some() {
        let mut user_domain = token_v3::DomainBuilder::default();
        if let Some(val) = &auth_data.user_domain_id {
            user_domain.id(val.clone());
        }
        if let Some(val) = &auth_data.user_domain_name {
            user_domain.name(val.clone());
        }
        user.domain(user_domain.build()?);
    }

    let password = token_v3::PasswordBuilder::default()
        .user(user.build()?)
        .build()?;
    identity_builder.password(password);
    Ok(())
}

/// Fill Auth Request Identity with user token
fn fill_identity_using_token(
    identity_builder: &mut token_v3::IdentityBuilder<'_>,
    auth_data: &config::Auth,
    _interactive: bool,
) -> Result<(), AuthError> {
    identity_builder.methods(Vec::from([token_v3::Methods::Token]));
    let token = token_v3::TokenBuilder::default()
        .id(auth_data.token.clone().ok_or(AuthError::MissingToken)?)
        .build()?;
    identity_builder.token(token);
    Ok(())
}

/// Fill Auth Request Identity with MFA passcode
fn fill_identity_using_totp(
    identity_builder: &mut token_v3::IdentityBuilder<'_>,
    auth_data: &config::Auth,
    interactive: bool,
) -> Result<(), AuthError> {
    identity_builder.methods(Vec::from([token_v3::Methods::Totp]));
    let mut user = token_v3::TotpUserBuilder::default();
    if let Some(val) = &auth_data.user_id {
        user.id(val.clone());
    } else if let Some(val) = &auth_data.username {
        user.name(val.clone());
    } else if interactive {
        // Or ask user for username in interactive mode
        let name: String = Input::new()
            .with_prompt("Please provide the username:")
            .interact_text()
            .unwrap();
        user.name(name);
    } else {
        return Err(AuthError::MissingUserId);
    }
    // Process user domain information
    if auth_data.user_domain_id.is_some() || auth_data.user_domain_name.is_some() {
        let mut user_domain = token_v3::UserDomainStructBuilder::default();
        if let Some(val) = &auth_data.user_domain_id {
            user_domain.id(val.clone());
        }
        if let Some(val) = &auth_data.user_domain_name {
            user_domain.name(val.clone());
        }
        user.domain(user_domain.build()?);
    }

    if let Some(passcode) = &auth_data.passcode {
        user.passcode(passcode.clone());
    } else if interactive {
        // Or ask user for username in interactive mode
        let name: String = Input::new()
            .with_prompt("Please provide the MFA passcode:")
            .interact_text()
            .unwrap();
        user.passcode(name);
    } else {
        return Err(AuthError::MissingPasscode);
    }
    identity_builder.totp(
        token_v3::TotpBuilder::default()
            .user(user.build()?)
            .build()?,
    );
    Ok(())
}

fn process_auth_type(
    identity_builder: &mut token_v3::IdentityBuilder<'_>,
    auth_data: &config::Auth,
    interactive: bool,
    auth_type: &AuthType,
) -> Result<(), AuthError> {
    match auth_type {
        AuthType::V3Password => {
            fill_identity_using_password(identity_builder, auth_data, interactive)?;
        }
        AuthType::V3Token => {
            fill_identity_using_token(identity_builder, auth_data, interactive)?;
        }
        AuthType::V3Totp => {
            fill_identity_using_totp(identity_builder, auth_data, interactive)?;
        }
        other => {
            return Err(AuthError::IdentityMethod {
                auth_type: other.as_str().to_string(),
            });
        }
    };
    Ok(())
}

pub(crate) fn build_identity_data_from_config<'a>(
    config: &config::CloudConfig,
    interactive: bool,
) -> Result<token_v3::Identity<'a>, AuthError> {
    let auth = config.auth.clone().ok_or(AuthError::MissingAuthData)?;
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
                process_auth_type(&mut identity_builder, &auth, interactive, &method_type)?;
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
            process_auth_type(&mut identity_builder, &auth, interactive, &other)?;
        }
    };
    Ok(identity_builder.build()?)
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

/// Build Auth request from `Receipt`
pub(crate) fn build_auth_request_from_receipt<'a>(
    config: &config::CloudConfig,
    receipt_header: HeaderValue,
    receipt_data: &AuthReceiptResponse,
    scope: &AuthorizationScope,
    interactive: bool,
) -> Result<impl RestEndpoint + 'a, AuthError> {
    let mut identity_builder = token_v3::IdentityBuilder::default();
    let auth = config.auth.clone().ok_or(AuthError::MissingAuthData)?;
    // Check required_auth_methods rules
    // Note: Keystone returns list of lists (as set of different rules)
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
                    interactive,
                    &AuthType::from_str(required_method)?,
                )?;
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

/// Prepare Endpoint for the WebSSO authentication
pub(crate) fn build_sso_auth_endpoint(
    config: &config::CloudConfig,
) -> Result<Cow<'static, str>, AuthError> {
    if let Some(auth) = &config.auth {
        if let Some(identity_provider) = &auth.identity_provider {
            let mut ep = fed_idp_sso_get::RequestBuilder::default();
            ep.idp_id(identity_provider);
            if let Some(protocol) = &auth.protocol {
                ep.protocol_id(protocol);
            } else {
                return Err(AuthError::AuthSsoMissingProtocol);
            }
            return Ok(ep.build()?.endpoint());
        } else {
            let mut ep = fed_sso_get::RequestBuilder::default();
            if let Some(protocol) = &auth.protocol {
                ep.protocol_id(protocol);
            } else {
                return Err(AuthError::AuthSsoMissingProtocol);
            }
            return Ok(ep.build()?.endpoint());
        }
    }
    Err(AuthError::MissingAuthData)
}

pub(crate) fn build_token_info_endpoint<'a>(
    subject_token: String,
) -> Result<token_v3_info::Request<'a>, AuthError> {
    Ok(token_v3_info::RequestBuilder::default()
        .headers(
            [(
                Some(HeaderName::from_static("x-subject-token")),
                HeaderValue::from_str(subject_token.as_str()).expect("Valid string"),
            )]
            .into_iter(),
        )
        .build()?)
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

#[cfg(test)]
mod tests {

    use serde_json::json;

    use super::*;

    use crate::config;
    use crate::types::identity::v3::{AuthResponse, AuthToken};

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

        let auth_data = build_identity_data_from_config(&config, false).unwrap();
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

        let auth_data = build_identity_data_from_config(&config, false).unwrap();
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
