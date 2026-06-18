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

//! # OIDC Access Token authentication for [`openstack_sdk`]
//!
//! This plugin authenticates against OpenStack's Identity service (Keystone) using
//! a federated OIDC access token. It sends the access token as a Bearer credential
//! to the Keystone federation endpoint.

use async_trait::async_trait;
use secrecy::ExposeSecret;
use serde_json::{Value, json};
use thiserror::Error;

use openstack_sdk_auth_core::{
    Auth, AuthError, AuthPluginRegistration, AuthToken, AuthTokenScope, OpenStackAuthType,
    execute_auth_request,
};

/// OIDC Access Token authentication for OpenStack SDK.
///
/// Authenticates by presenting a Bearer token (OIDC access token) to the
/// Keystone federation endpoint.
pub struct OidcAccessTokenAuthenticator;

// Submit the plugin to the registry at compile-time
static PLUGIN: OidcAccessTokenAuthenticator = OidcAccessTokenAuthenticator;
inventory::submit! {
    AuthPluginRegistration { method: &PLUGIN }
}
#[used]
pub static ANCHOR: OidcAccessTokenAuthenticator = OidcAccessTokenAuthenticator;

#[async_trait]
impl OpenStackAuthType for OidcAccessTokenAuthenticator {
    fn get_supported_auth_methods(&self) -> Vec<&'static str> {
        vec!["v3oidcaccesstoken", "accesstoken"]
    }

    fn api_version(&self) -> (u8, u8) {
        (3, 0)
    }

    fn requirements(&self, _hints: Option<&Value>) -> Result<Value, AuthError> {
        Ok(json!({
            "type": "object",
            "required": ["identity_provider", "protocol", "access_token"],
            "properties": {
                "identity_provider": {
                    "type": "string",
                    "description": "Identity Provider ID"
                },
                "protocol": {
                    "type": "string",
                    "description": "Federation protocol (e.g., oidc)"
                },
                "access_token": {
                    "type": "string",
                    "format": "password",
                    "description": "OIDC access token from the identity provider"
                },
            }
        }))
    }

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
            .ok_or(OidcAccessTokenError::MissingIdpId)?;
        let protocol = values
            .get("protocol")
            .ok_or(OidcAccessTokenError::MissingProtocolId)?;
        let access_token = values
            .get("access_token")
            .ok_or(OidcAccessTokenError::MissingAccessToken)?;

        let endpoint = identity_url.join(
            format!(
                "OS-FEDERATION/identity_providers/{idp_id}/protocols/{protocol}/auth",
                idp_id = idp_id.expose_secret(),
                protocol = protocol.expose_secret()
            )
            .as_str(),
        )?;

        let request = http_client
            .post(endpoint)
            .bearer_auth(access_token.expose_secret())
            .build()?;
        let response = execute_auth_request(http_client, request).await?;

        let auth_token = AuthToken::from_reqwest_response(response).await?;

        Ok(Auth::AuthToken(Box::new(auth_token)))
    }
}
/// OIDC Access Token authentication errors.
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum OidcAccessTokenError {
    /// Auth data is missing.
    #[error("auth data is missing")]
    MissingAuthData,

    /// Identity provider ID is missing.
    #[error("identity_provider ID is missing")]
    MissingIdpId,

    /// Federation protocol ID is missing.
    #[error("federation protocol ID is missing")]
    MissingProtocolId,

    /// Access token is missing.
    #[error("OIDC access token is missing")]
    MissingAccessToken,
}

impl From<OidcAccessTokenError> for AuthError {
    fn from(source: OidcAccessTokenError) -> Self {
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
    async fn test_auth_success() {
        let server = MockServer::start_async().await;
        let base_url = Url::parse(&server.base_url()).unwrap();

        let mock = server
            .mock_async(|when, then| {
                when.method(httpmock::Method::POST)
                    .path("/OS-FEDERATION/identity_providers/idp1/protocols/oidc/auth")
                    .header("authorization", "Bearer my-access-token");
                then.status(StatusCode::CREATED)
                    .header("x-subject-token", "new-token")
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

        let http_client = Client::new();
        let authenticator = OidcAccessTokenAuthenticator;

        match authenticator
            .auth(
                &http_client,
                &base_url,
                &HashMap::from([
                    ("identity_provider".into(), SecretString::from("idp1")),
                    ("protocol".into(), SecretString::from("oidc")),
                    ("access_token".into(), SecretString::from("my-access-token")),
                ]),
                None,
                None,
            )
            .await
        {
            Ok(Auth::AuthToken(token)) => {
                assert_eq!(token.token.expose_secret(), "new-token");
            }
            other => {
                panic!("success expected, got {:?}", other);
            }
        }
        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_auth_missing_idp() {
        let authenticator = OidcAccessTokenAuthenticator;
        let err = authenticator
            .auth(
                &Client::new(),
                &Url::parse("http://localhost").unwrap(),
                &HashMap::from([
                    ("protocol".into(), SecretString::from("oidc")),
                    ("access_token".into(), SecretString::from("token")),
                ]),
                None,
                None,
            )
            .await;
        assert!(err.is_err());
    }

    #[tokio::test]
    async fn test_auth_missing_protocol() {
        let authenticator = OidcAccessTokenAuthenticator;
        let err = authenticator
            .auth(
                &Client::new(),
                &Url::parse("http://localhost").unwrap(),
                &HashMap::from([
                    ("identity_provider".into(), SecretString::from("idp1")),
                    ("access_token".into(), SecretString::from("token")),
                ]),
                None,
                None,
            )
            .await;
        assert!(err.is_err());
    }

    #[tokio::test]
    async fn test_auth_missing_access_token() {
        let authenticator = OidcAccessTokenAuthenticator;
        let err = authenticator
            .auth(
                &Client::new(),
                &Url::parse("http://localhost").unwrap(),
                &HashMap::from([
                    ("identity_provider".into(), SecretString::from("idp1")),
                    ("protocol".into(), SecretString::from("oidc")),
                ]),
                None,
                None,
            )
            .await;
        assert!(err.is_err());
    }

    #[test]
    fn test_get_supported_auth_methods() {
        let authenticator = OidcAccessTokenAuthenticator;
        assert!(
            authenticator
                .get_supported_auth_methods()
                .contains(&"v3oidcaccesstoken")
        );
        assert!(
            authenticator
                .get_supported_auth_methods()
                .contains(&"accesstoken")
        );
    }

    #[test]
    fn test_requirements() {
        let authenticator = OidcAccessTokenAuthenticator;
        let req = authenticator.requirements(None).unwrap();
        assert!(
            req["required"]
                .as_array()
                .unwrap()
                .contains(&"access_token".into())
        );
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
    }
}
