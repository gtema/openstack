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

//! # OIDC Client Credentials authentication for [`openstack_sdk`]
//!
//! This plugin implements the OIDC client credentials grant flow to authenticate
//! against OpenStack's Identity service (Keystone) via federation. It performs a
//! two-step authentication:
//!
//! 1. Exchange `client_id` and `client_secret` for an OIDC access token at the
//!    identity provider's token endpoint.
//! 2. Exchange the OIDC access token for a Keystone token via the federation
//!    endpoint using a Bearer credential.

use async_trait::async_trait;
use secrecy::ExposeSecret;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use thiserror::Error;

use openstack_sdk_auth_core::{
    Auth, AuthError, AuthPluginRegistration, AuthToken, AuthTokenScope, OpenStackAuthType,
    execute_auth_request,
};

/// OIDC Client Credentials authentication for OpenStack SDK.
///
/// Authenticates by first obtaining an OIDC access token via the client
/// credentials grant, then exchanging it for a Keystone federated token.
pub struct OidcClientCredentialsAuthenticator;

// Submit the plugin to the registry at compile-time
static PLUGIN: OidcClientCredentialsAuthenticator = OidcClientCredentialsAuthenticator;
inventory::submit! {
    AuthPluginRegistration { method: &PLUGIN }
}
#[used]
pub static ANCHOR: OidcClientCredentialsAuthenticator = OidcClientCredentialsAuthenticator;

/// Internal representation of the OIDC discovery document.
/// Only the fields required for token endpoint resolution and grant type validation
/// are extracted.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
struct DiscoveryDocument {
    token_endpoint: Option<String>,
    grant_types_supported: Option<Vec<String>>,
}

#[async_trait]
impl OpenStackAuthType for OidcClientCredentialsAuthenticator {
    /// Returns the list of supported authentication method names.
    fn get_supported_auth_methods(&self) -> Vec<&'static str> {
        vec!["v3oidcclientcredentials", "clientcredentials"]
    }

    /// Returns the Keystone API version required for this authentication method.
    fn api_version(&self) -> (u8, u8) {
        (3, 0)
    }

    /// Returns the JSON Schema describing the credential requirements.
    ///
    /// Required fields: `identity_provider`, `protocol`, `client_id`.
    /// Optional fields: `client_secret`, `scope`, `access_token_type`,
    /// `access_token_endpoint`, `discovery_endpoint`.
    fn requirements(&self, _hints: Option<&Value>) -> Result<Value, AuthError> {
        Ok(json!({
            "type": "object",
            "required": ["identity_provider", "protocol", "client_id"],
            "properties": {
                "identity_provider": {
                    "type": "string",
                    "description": "Identity Provider ID"
                },
                "protocol": {
                    "type": "string",
                    "description": "Federation protocol (e.g., oidc)"
                },
                "client_id": {
                    "type": "string",
                    "description": "OAuth 2.0 Client ID"
                },
                "client_secret": {
                    "type": "string",
                    "format": "password",
                    "description": "OAuth 2.0 Client Secret"
                },
                "access_token_type": {
                    "type": "string",
                    "description": "Token type to extract from the OIDC response (e.g., access_token, id_token)"
                },
                "scope": {
                    "type": "string",
                    "description": "OpenID Connect scope (e.g., openid profile)"
                },
                "access_token_endpoint": {
                    "type": "string",
                    "format": "uri",
                    "description": "OpenID Connect Provider Token Endpoint URL"
                },
                "discovery_endpoint": {
                    "type": "string",
                    "format": "uri",
                    "description": "OpenID Connect Discovery Document URL"
                },
            }
        }))
    }

    /// Performs the full OIDC client credentials authentication flow.
    ///
    /// 1. Resolves the OIDC token endpoint via explicit URL or discovery document.
    /// 2. Validates that `client_credentials` grant type is supported (when discovery is used).
    /// 3. Obtains an OIDC access token from the identity provider.
    /// 4. Exchanges the OIDC access token for a Keystone token via federation endpoint.
    async fn auth(
        &self,
        http_client: &reqwest::Client,
        identity_url: &url::Url,
        values: &std::collections::HashMap<String, secrecy::SecretString>,
        _scope: Option<&AuthTokenScope>,
        _hints: Option<&serde_json::Value>,
    ) -> Result<Auth, AuthError> {
        let idp_id = values
            .get("identity_provider")
            .ok_or(OidcClientCredentialsError::MissingIdentityProvider)?;
        let protocol = values
            .get("protocol")
            .ok_or(OidcClientCredentialsError::MissingProtocol)?;
        let client_id = values
            .get("client_id")
            .ok_or(OidcClientCredentialsError::MissingClientId)?;
        let client_secret = values.get("client_secret");
        let access_token_type = values
            .get("access_token_type")
            .map(|v| v.expose_secret().to_string())
            .unwrap_or_else(|| String::from("access_token"));
        let scope = values
            .get("scope")
            .map(|v| v.expose_secret().to_string())
            .unwrap_or_else(|| String::from("openid profile"));

        let (token_endpoint_url, discovery_document) = resolve_token_endpoint_and_discovery(
            http_client,
            values.get("discovery_endpoint"),
            values.get("access_token_endpoint"),
        )
        .await?;

        if let Some(ref doc) = discovery_document
            && let Some(ref grant_types) = doc.grant_types_supported
            && !grant_types.contains(&"client_credentials".to_string())
        {
            return Err(OidcClientCredentialsError::GrantTypeNotSupported.into());
        }

        let oidc_access_token = obtain_oidc_access_token(
            http_client,
            &token_endpoint_url,
            client_id,
            client_secret,
            &scope,
            &access_token_type,
        )
        .await?;

        let auth_endpoint = identity_url.join(
            format!(
                "OS-FEDERATION/identity_providers/{idp_id}/protocols/{protocol}/auth",
                idp_id = idp_id.expose_secret(),
                protocol = protocol.expose_secret(),
            )
            .as_str(),
        )?;

        let request = http_client
            .post(auth_endpoint)
            .bearer_auth(&oidc_access_token)
            .build()?;
        let response = execute_auth_request(http_client, request).await?;

        let auth_token = AuthToken::from_reqwest_response(response).await?;

        Ok(Auth::AuthToken(Box::new(auth_token)))
    }
}

/// Resolves the OIDC token endpoint URL and optionally fetches the discovery document.
///
/// Priority order for token endpoint resolution:
/// 1. Explicit `token_endpoint` URL (if provided).
/// 2. Endpoint from the OIDC discovery document (if `discovery_endpoint` is configured).
/// 3. Error if neither is available.
///
/// Returns a tuple of `(token_endpoint_url, discovery_document)`.
/// When explicit endpoint is used, the discovery document is `None` unless
/// `discovery_endpoint` was also configured (e.g., for grant type validation).
async fn resolve_token_endpoint_and_discovery(
    http_client: &reqwest::Client,
    discovery_endpoint: Option<&secrecy::SecretString>,
    token_endpoint: Option<&secrecy::SecretString>,
) -> Result<(url::Url, Option<DiscoveryDocument>), AuthError> {
    let discovery_document = if let Some(ep) = discovery_endpoint {
        let doc_url = url::Url::parse(ep.expose_secret())
            .map_err(OidcClientCredentialsError::InvalidDiscoveryEndpoint)?;
        let request = http_client.get(doc_url.clone()).build()?;
        let response = execute_auth_request(http_client, request).await?;
        let doc: DiscoveryDocument = response.json().await?;
        Some(doc)
    } else {
        None
    };

    let token_endpoint_url = if let Some(explicit) = token_endpoint {
        url::Url::parse(explicit.expose_secret())
            .map_err(OidcClientCredentialsError::InvalidTokenEndpoint)?
    } else if let Some(ref doc) = discovery_document {
        let endpoint = doc
            .token_endpoint
            .clone()
            .ok_or(OidcClientCredentialsError::TokenEndpointNotDiscovered)?;
        url::Url::parse(&endpoint).map_err(OidcClientCredentialsError::InvalidTokenEndpoint)?
    } else {
        return Err(OidcClientCredentialsError::TokenEndpointNotProvided.into());
    };

    Ok((token_endpoint_url, discovery_document))
}

/// Exchanges client credentials for an OIDC access token.
///
/// POSTs to the token endpoint with `grant_type=client_credentials`.
/// If `client_secret` is provided, authentication is performed via HTTP Basic auth.
/// Otherwise, `client_id` is included in the form body.
///
/// The returned access token is extracted from the field matching `access_token_type`
/// (default: `access_token`) in the JSON response.
async fn obtain_oidc_access_token(
    http_client: &reqwest::Client,
    token_endpoint: &url::Url,
    client_id: &secrecy::SecretString,
    client_secret: Option<&secrecy::SecretString>,
    scope: &str,
    access_token_type: &str,
) -> Result<String, AuthError> {
    let mut form = vec![("grant_type", "client_credentials"), ("scope", scope)];

    let request = if let Some(secret) = client_secret {
        http_client
            .post(token_endpoint.as_str())
            .basic_auth(client_id.expose_secret(), Some(secret.expose_secret()))
            .form(&form)
            .build()?
    } else {
        form.push(("client_id", client_id.expose_secret()));
        http_client
            .post(token_endpoint.as_str())
            .form(&form)
            .build()?
    };

    let response = execute_auth_request(http_client, request).await?;
    let body: serde_json::Value = response.json().await?;

    let access_token = body
        .get(access_token_type)
        .and_then(|v| v.as_str())
        .ok_or_else(|| OidcClientCredentialsError::AccessTokenNotInResponse {
            field: access_token_type.to_string(),
        })?
        .to_string();

    Ok(access_token)
}

/// Represents errors that can occur during OIDC client credentials authentication.
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum OidcClientCredentialsError {
    /// Identity provider ID is missing.
    #[error("identity_provider ID is missing")]
    MissingIdentityProvider,

    /// Federation protocol ID is missing.
    #[error("federation protocol ID is missing")]
    MissingProtocol,

    /// Client ID is missing.
    #[error("client_id is missing")]
    MissingClientId,

    /// Discovery endpoint URL is invalid.
    #[error("discovery endpoint URL is invalid: {0}")]
    InvalidDiscoveryEndpoint(url::ParseError),

    /// Token endpoint URL is invalid.
    #[error("token endpoint URL is invalid: {0}")]
    InvalidTokenEndpoint(url::ParseError),

    /// Token endpoint was not found in the discovery document.
    #[error("token_endpoint not found in discovery document")]
    TokenEndpointNotDiscovered,

    /// No token endpoint provided or discovered.
    #[error("access_token_endpoint not provided and no discovery_endpoint configured")]
    TokenEndpointNotProvided,

    /// The client_credentials grant type is not supported by the identity provider.
    #[error("grant type client_credentials not supported by the identity provider")]
    GrantTypeNotSupported,

    /// The access_token_type field was not present in the OIDC token response.
    #[error("field '{field}' not found in OIDC token response")]
    AccessTokenNotInResponse { field: String },
}

impl From<OidcClientCredentialsError> for AuthError {
    fn from(source: OidcClientCredentialsError) -> Self {
        Self::plugin(source)
    }
}

#[cfg(test)]
mod tests {
    use httpmock::MockServer;
    use reqwest::{Client, StatusCode};
    use secrecy::{ExposeSecret, SecretString};
    use serde_json::json;
    use std::collections::HashMap;
    use url::Url;

    use openstack_sdk_auth_core::Auth;

    use super::*;

    #[tokio::test]
    async fn test_auth_success_explicit_endpoint() {
        let server = MockServer::start_async().await;
        let base_url = Url::parse(&server.base_url()).unwrap();
        let _token_endpoint = &server.base_url();

        let mock = server
            .mock_async(|when, then| {
                when.method(httpmock::Method::POST)
                    .path("/OS-FEDERATION/identity_providers/idp1/protocols/oidc/auth")
                    .header("authorization", "Bearer oidc-token-value");
                then.status(StatusCode::CREATED)
                    .header("x-subject-token", "keystone-token")
                    .json_body(json!({
                        "token": {
                            "user": {
                                "id": "uid",
                                "name": "uname"
                            },
                            "expires_at": "2099-01-15T22:14:05.000000Z",
                        }
                    }));
            })
            .await;

        let oidc_server = MockServer::start_async().await;
        let _oidc_base = Url::parse(&oidc_server.base_url()).unwrap();

        let mock_oidc = oidc_server
            .mock_async(|when, then| {
                when.method(httpmock::Method::POST)
                    .header("content-type", "application/x-www-form-urlencoded")
                    .header("authorization", "Basic Y2xpZW50MTpzZWNyZXQ=");
                then.status(StatusCode::OK).json_body(json!({
                    "access_token": "oidc-token-value",
                    "token_type": "Bearer",
                    "expires_in": 3600
                }));
            })
            .await;

        let http_client = Client::new();
        let authenticator = OidcClientCredentialsAuthenticator;

        match authenticator
            .auth(
                &http_client,
                &base_url,
                &HashMap::from([
                    ("identity_provider".into(), SecretString::from("idp1")),
                    ("protocol".into(), SecretString::from("oidc")),
                    ("client_id".into(), SecretString::from("client1")),
                    ("client_secret".into(), SecretString::from("secret")),
                    (
                        "access_token_endpoint".into(),
                        SecretString::from(oidc_server.base_url()),
                    ),
                ]),
                None,
                None,
            )
            .await
        {
            Ok(Auth::AuthToken(token)) => {
                assert_eq!(token.token.expose_secret(), "keystone-token");
            }
            other => {
                panic!("success expected, got {:?}", other);
            }
        }
        mock.assert_async().await;
        mock_oidc.assert_async().await;
    }

    #[tokio::test]
    async fn test_auth_success_discovery() {
        let server = MockServer::start_async().await;
        let base_url = Url::parse(&server.base_url()).unwrap();

        let mock = server
            .mock_async(|when, then| {
                when.method(httpmock::Method::POST)
                    .path("/OS-FEDERATION/identity_providers/idp1/protocols/oidc/auth")
                    .header("authorization", "Bearer oidc-token-value");
                then.status(StatusCode::CREATED)
                    .header("x-subject-token", "keystone-token")
                    .json_body(json!({
                        "token": {
                            "user": {
                                "id": "uid",
                                "name": "uname"
                            },
                            "expires_at": "2099-01-15T22:14:05.000000Z",
                        }
                    }));
            })
            .await;

        let oidc_server = MockServer::start_async().await;

        let mock_discovery = oidc_server
            .mock_async(|when, then| {
                when.method(httpmock::Method::GET)
                    .path("/.well-known/openid-configuration");
                then.status(StatusCode::OK).json_body(json!({
                    "token_endpoint": format!("{}/token", oidc_server.base_url()),
                    "grant_types_supported": ["client_credentials", "authorization_code"]
                }));
            })
            .await;

        let mock_oidc = oidc_server
            .mock_async(|when, then| {
                when.method(httpmock::Method::POST)
                    .path("/token")
                    .header("content-type", "application/x-www-form-urlencoded");
                then.status(StatusCode::OK).json_body(json!({
                    "access_token": "oidc-token-value",
                    "token_type": "Bearer",
                    "expires_in": 3600
                }));
            })
            .await;

        let http_client = Client::new();
        let authenticator = OidcClientCredentialsAuthenticator;

        match authenticator
            .auth(
                &http_client,
                &base_url,
                &HashMap::from([
                    ("identity_provider".into(), SecretString::from("idp1")),
                    ("protocol".into(), SecretString::from("oidc")),
                    ("client_id".into(), SecretString::from("client1")),
                    ("client_secret".into(), SecretString::from("secret")),
                    (
                        "discovery_endpoint".into(),
                        SecretString::from(format!(
                            "{}/.well-known/openid-configuration",
                            oidc_server.base_url()
                        )),
                    ),
                ]),
                None,
                None,
            )
            .await
        {
            Ok(Auth::AuthToken(token)) => {
                assert_eq!(token.token.expose_secret(), "keystone-token");
            }
            other => {
                panic!("success expected, got {:?}", other);
            }
        }
        mock.assert_async().await;
        mock_discovery.assert_async().await;
        mock_oidc.assert_async().await;
    }

    #[tokio::test]
    async fn test_auth_missing_idp() {
        let authenticator = OidcClientCredentialsAuthenticator;
        let err = authenticator
            .auth(
                &Client::new(),
                &Url::parse("http://localhost").unwrap(),
                &HashMap::from([
                    ("protocol".into(), SecretString::from("oidc")),
                    ("client_id".into(), SecretString::from("client1")),
                    (
                        "access_token_endpoint".into(),
                        SecretString::from("http://oidc/token"),
                    ),
                ]),
                None,
                None,
            )
            .await;
        assert!(err.is_err());
    }

    #[tokio::test]
    async fn test_auth_missing_protocol() {
        let authenticator = OidcClientCredentialsAuthenticator;
        let err = authenticator
            .auth(
                &Client::new(),
                &Url::parse("http://localhost").unwrap(),
                &HashMap::from([
                    ("identity_provider".into(), SecretString::from("idp1")),
                    ("client_id".into(), SecretString::from("client1")),
                    (
                        "access_token_endpoint".into(),
                        SecretString::from("http://oidc/token"),
                    ),
                ]),
                None,
                None,
            )
            .await;
        assert!(err.is_err());
    }

    #[tokio::test]
    async fn test_auth_missing_client_id() {
        let authenticator = OidcClientCredentialsAuthenticator;
        let err = authenticator
            .auth(
                &Client::new(),
                &Url::parse("http://localhost").unwrap(),
                &HashMap::from([
                    ("identity_provider".into(), SecretString::from("idp1")),
                    ("protocol".into(), SecretString::from("oidc")),
                    (
                        "access_token_endpoint".into(),
                        SecretString::from("http://oidc/token"),
                    ),
                ]),
                None,
                None,
            )
            .await;
        assert!(err.is_err());
    }

    #[tokio::test]
    async fn test_auth_no_token_endpoint() {
        let authenticator = OidcClientCredentialsAuthenticator;
        let err = authenticator
            .auth(
                &Client::new(),
                &Url::parse("http://localhost").unwrap(),
                &HashMap::from([
                    ("identity_provider".into(), SecretString::from("idp1")),
                    ("protocol".into(), SecretString::from("oidc")),
                    ("client_id".into(), SecretString::from("client1")),
                ]),
                None,
                None,
            )
            .await;
        assert!(err.is_err());
    }

    #[tokio::test]
    async fn test_auth_grant_type_not_supported() {
        let server = MockServer::start_async().await;

        let mock_discovery = server
            .mock_async(|when, then| {
                when.method(httpmock::Method::GET)
                    .path("/.well-known/openid-configuration");
                then.status(StatusCode::OK).json_body(json!({
                    "token_endpoint": format!("{}/token", server.base_url()),
                    "grant_types_supported": ["authorization_code"]
                }));
            })
            .await;

        let http_client = Client::new();
        let authenticator = OidcClientCredentialsAuthenticator;
        let err = authenticator
            .auth(
                &http_client,
                &Url::parse("http://localhost").unwrap(),
                &HashMap::from([
                    ("identity_provider".into(), SecretString::from("idp1")),
                    ("protocol".into(), SecretString::from("oidc")),
                    ("client_id".into(), SecretString::from("client1")),
                    (
                        "discovery_endpoint".into(),
                        SecretString::from(format!(
                            "{}/.well-known/openid-configuration",
                            server.base_url()
                        )),
                    ),
                ]),
                None,
                None,
            )
            .await;
        assert!(err.is_err());
        mock_discovery.assert_async().await;
    }

    #[tokio::test]
    async fn test_auth_no_client_secret_uses_body() {
        let server = MockServer::start_async().await;
        let base_url = Url::parse(&server.base_url()).unwrap();

        let mock = server
            .mock_async(|when, then| {
                when.method(httpmock::Method::POST)
                    .path("/OS-FEDERATION/identity_providers/idp1/protocols/oidc/auth")
                    .header("authorization", "Bearer oidc-token-value");
                then.status(StatusCode::CREATED)
                    .header("x-subject-token", "keystone-token")
                    .json_body(json!({
                        "token": {
                            "user": {
                                "id": "uid",
                                "name": "uname"
                            },
                            "expires_at": "2099-01-15T22:14:05.000000Z",
                        }
                    }));
            })
            .await;

        let oidc_server = MockServer::start_async().await;

        let mock_oidc = oidc_server
            .mock_async(|when, then| {
                when.method(httpmock::Method::POST)
                    .header("content-type", "application/x-www-form-urlencoded");
                then.status(StatusCode::OK).json_body(json!({
                    "access_token": "oidc-token-value",
                    "token_type": "Bearer",
                    "expires_in": 3600
                }));
            })
            .await;

        let http_client = Client::new();
        let authenticator = OidcClientCredentialsAuthenticator;

        match authenticator
            .auth(
                &http_client,
                &base_url,
                &HashMap::from([
                    ("identity_provider".into(), SecretString::from("idp1")),
                    ("protocol".into(), SecretString::from("oidc")),
                    ("client_id".into(), SecretString::from("client1")),
                    (
                        "access_token_endpoint".into(),
                        SecretString::from(oidc_server.base_url()),
                    ),
                ]),
                None,
                None,
            )
            .await
        {
            Ok(Auth::AuthToken(token)) => {
                assert_eq!(token.token.expose_secret(), "keystone-token");
            }
            other => {
                panic!("success expected, got {:?}", other);
            }
        }
        mock.assert_async().await;
        mock_oidc.assert_async().await;
    }

    #[test]
    fn test_get_supported_auth_methods() {
        let authenticator = OidcClientCredentialsAuthenticator;
        assert!(
            authenticator
                .get_supported_auth_methods()
                .contains(&"v3oidcclientcredentials")
        );
        assert!(
            authenticator
                .get_supported_auth_methods()
                .contains(&"clientcredentials")
        );
    }

    #[test]
    fn test_requirements() {
        let authenticator = OidcClientCredentialsAuthenticator;
        let req = authenticator.requirements(None).unwrap();
        assert!(
            req["required"]
                .as_array()
                .unwrap()
                .contains(&"identity_provider".into())
        );
        assert!(
            req["required"]
                .as_array()
                .unwrap()
                .contains(&"protocol".into())
        );
        assert!(
            req["required"]
                .as_array()
                .unwrap()
                .contains(&"client_id".into())
        );
    }

    #[test]
    fn test_api_version() {
        let authenticator = OidcClientCredentialsAuthenticator;
        assert_eq!(authenticator.api_version(), (3, 0));
    }
}
