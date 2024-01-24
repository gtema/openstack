use http::{HeaderMap, HeaderName, HeaderValue, Response};
use serde::{Deserialize, Serialize};
use std::collections::hash_map::DefaultHasher;
use std::convert::TryFrom;
use std::fmt::{self, Debug};
use tracing::{error, trace};

use thiserror::Error;
use url::Url;

use dialoguer::{Input, Password};

use crate::api::identity::v3::auth::token::create as token_v3;
use crate::config;
use crate::types::identity::v3::{
    self as types_v3, AuthReceiptResponse, AuthResponse, Domain, Project,
};
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

/// Fill Auth Request Identity with user password data
pub fn fill_identity_using_password(
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
pub fn fill_identity_using_token(
    identity_builder: &mut token_v3::IdentityBuilder<'_>,
    auth_data: &config::Auth,
    interactive: bool,
) -> Result<(), AuthError> {
    identity_builder.methods(Vec::from([token_v3::Methods::Token]));
    let token = token_v3::TokenBuilder::default()
        .id(auth_data.token.clone().ok_or(AuthError::MissingToken)?)
        .build()?;
    identity_builder.token(token);
    Ok(())
}

/// Fill Auth Request Identity with MFA passcode
pub fn fill_identity_using_totp(
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
    auth_type: &str,
) -> Result<(), AuthError> {
    match auth_type {
        "v3password" | "password" => {
            fill_identity_using_password(identity_builder, auth_data, interactive)?;
        }
        "v3token" | "token" => {
            fill_identity_using_token(identity_builder, auth_data, interactive)?;
        }
        "v3totp" | "totp" => {
            fill_identity_using_totp(identity_builder, auth_data, interactive)?;
        }
        other => {
            return Err(AuthError::auth_type(other));
        }
    };
    Ok(())
}

pub fn build_identity_data_from_config<'a>(
    config: &config::CloudConfig,
    interactive: bool,
) -> Result<token_v3::Identity<'a>, AuthError> {
    let auth = config.auth.clone().ok_or(AuthError::MissingAuthData)?;
    let auth_type = config.auth_type.clone().unwrap_or("password".to_string());
    let mut identity_builder = token_v3::IdentityBuilder::default();
    match auth_type.as_str() {
        "v3multifactor" | "multifactor" => {
            let mut methods: Vec<token_v3::Methods> = Vec::new();
            for auth_type in config
                .auth_methods
                .as_ref()
                .expect("`auth_methods` is an array of string when auth_type=`multifactor`")
            {
                process_auth_type(&mut identity_builder, &auth, interactive, auth_type)?;
                match auth_type.as_str() {
                    "v3password" | "password" => {
                        methods.push(token_v3::Methods::Password);
                    }
                    "v3token" | "token" => {
                        methods.push(token_v3::Methods::Token);
                    }
                    "v3totp" | "totp" => {
                        methods.push(token_v3::Methods::Totp);
                    }
                    other => {}
                };
            }

            identity_builder.methods(methods);
        }
        other => {
            process_auth_type(&mut identity_builder, &auth, interactive, other)?;
        }
    };
    Ok(identity_builder.build()?)
}

/// Build Auth `Identity` from existing `Auth` (use token)
impl TryFrom<&AuthToken> for token_v3::Identity<'_> {
    type Error = AuthError;

    fn try_from(auth: &AuthToken) -> Result<Self, Self::Error> {
        Ok(token_v3::IdentityBuilder::default()
            .methods(Vec::from([token_v3::Methods::Token]))
            .token(
                token_v3::TokenBuilder::default()
                    .id(auth.token.clone())
                    .build()?,
            )
            .build()?)
    }
}

/// Build Auth `Scope` data from `CloudConfig`
impl TryFrom<&config::CloudConfig> for token_v3::Scope<'_> {
    type Error = AuthError;
    fn try_from(config: &config::CloudConfig) -> Result<Self, Self::Error> {
        let auth = config.auth.clone().ok_or(AuthError::MissingAuthData)?;
        let mut scope = token_v3::ScopeBuilder::default();
        if auth.project_id.is_some() || auth.project_name.is_some() {
            // Project scope
            let mut project_scope = token_v3::ProjectBuilder::default();
            if auth.project_domain_name.is_some() || auth.project_domain_id.is_some() {
                let mut project_domain = token_v3::ProjectDomainBuilder::default();
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
            let mut domain_scope = token_v3::ScopeDomainBuilder::default();
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

/// Build Auth `Scope` data from existing `AuthorizationScope`
impl TryFrom<&AuthorizationScope> for token_v3::Scope<'_> {
    type Error = AuthError;
    fn try_from(scope: &AuthorizationScope) -> Result<Self, Self::Error> {
        let mut scope_builder = token_v3::ScopeBuilder::default();
        match scope {
            AuthorizationScope::Project(project) => {
                let mut project_builder = token_v3::ProjectBuilder::default();
                if let Some(val) = &project.id {
                    project_builder.id(val.clone());
                }
                if let Some(val) = &project.name {
                    project_builder.name(val.clone());
                }
                if let Some(domain) = &project.domain {
                    let mut domain_builder = token_v3::ProjectDomainBuilder::default();
                    if let Some(val) = &domain.id {
                        domain_builder.id(val.clone());
                    }
                    if let Some(val) = &domain.name {
                        domain_builder.name(val.clone());
                    }
                    project_builder.domain(domain_builder.build()?);
                }
                scope_builder.project(project_builder.build()?);
            }
            AuthorizationScope::Domain(domain) => {
                let mut domain_builder = token_v3::ScopeDomainBuilder::default();
                if let Some(val) = &domain.id {
                    domain_builder.id(val.clone());
                }
                if let Some(val) = &domain.name {
                    domain_builder.name(val.clone());
                }
                scope_builder.domain(domain_builder.build()?);
            }
            AuthorizationScope::Unscoped => {}
        }
        Ok(scope_builder.build()?)
    }
}

/// Build `AuthorizationScope` data from `CloudConfig`
impl TryFrom<&config::CloudConfig> for AuthorizationScope {
    type Error = AuthError;
    fn try_from(config: &config::CloudConfig) -> Result<Self, Self::Error> {
        let auth = config.auth.clone().ok_or(AuthError::MissingAuthData)?;
        if auth.project_id.is_some() || auth.project_name.is_some() {
            // Project scope
            Ok(AuthorizationScope::Project(Project {
                id: auth.project_id.clone(),
                name: auth.project_name.clone(),
                domain: types_v3::get_domain(auth.project_domain_id, auth.project_domain_name),
            }))
        } else if auth.domain_id.is_some() || auth.domain_name.is_some() {
            // Domain scope
            Ok(AuthorizationScope::Domain(Domain {
                id: auth.domain_id.clone(),
                name: auth.domain_name.clone(),
            }))
        } else {
            Ok(AuthorizationScope::Unscoped)
        }
    }
}

/// Build Auth request from `CloudConfig`
impl TryFrom<&config::CloudConfig> for token_v3::Request<'_> {
    type Error = AuthError;
    fn try_from(config: &config::CloudConfig) -> Result<Self, Self::Error> {
        let identity_data = build_identity_data_from_config(config, false)?;
        let mut auth_request_data = token_v3::AuthBuilder::default();
        auth_request_data.identity(identity_data);
        if let Ok(scope_data) = token_v3::Scope::try_from(config) {
            auth_request_data.scope(scope_data);
        }

        Ok(token_v3::RequestBuilder::default()
            .auth(auth_request_data.build()?)
            .build()?)
    }
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
) -> Result<token_v3::Request<'a>, AuthError> {
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
                process_auth_type(&mut identity_builder, &auth, interactive, required_method)?;
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

impl From<&AuthResponse> for AuthorizationScope {
    fn from(auth: &AuthResponse) -> Self {
        if let Some(project) = &auth.token.project {
            Self::Project(project.clone())
        } else if let Some(domain) = &auth.token.domain {
            Self::Domain(domain.clone())
        } else {
            Self::Unscoped
        }
    }
}

#[cfg(test)]
mod tests {
    use chrono::prelude::*;
    use serde::Serialize;
    use serde_json::json;
    use std::collections::HashMap;

    use super::*;
    use crate::api::identity::v3::auth::token::create as token_v3;
    use crate::config;
    use crate::types::identity::v3::{self as types_v3, AuthResponse, AuthToken};

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
