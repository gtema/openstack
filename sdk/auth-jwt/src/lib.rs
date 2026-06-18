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

//! # JWT authentication for OpenStack
//!
//! This plugin authenticates against the OpenStack Identity service (Keystone) using
//! a JSON Web Token (JWT). The JWT is sent as a Bearer token to the Keystone
//! federation JWT endpoint along with the identity provider ID and attribute mapping name.

use async_trait::async_trait;
use secrecy::ExposeSecret;
use serde_json::{Value, json};
use thiserror::Error;

use openstack_sdk_auth_core::{
    Auth, AuthError, AuthPluginRegistration, AuthToken, AuthTokenScope, OpenStackAuthType,
    execute_auth_request,
};

/// JWT authentication errors.
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum JwtError {
    /// Auth data is missing.
    #[error("auth data is missing")]
    MissingAuthData,

    /// Identity provider ID is missing.
    #[error("identity provider ID is missing")]
    MissingIdentityProvider,

    /// Attribute mapping name is missing.
    #[error("attribute mapping name is missing")]
    MissingAttributeMapping,

    /// JWT is missing.
    #[error("JWT is missing")]
    MissingJwt,
}

impl From<JwtError> for AuthError {
    fn from(source: JwtError) -> Self {
        Self::plugin(source)
    }
}

/// JWT authentication for OpenStack SDK.
///
/// Authenticates by presenting a JWT Bearer token to the Keystone federation
/// endpoint along with the identity provider ID and attribute mapping name.
pub struct JwtAuthenticator;

// Submit the plugin to the registry at compile-time
static PLUGIN: JwtAuthenticator = JwtAuthenticator;
inventory::submit! {
    AuthPluginRegistration { method: &PLUGIN }
}
#[used]
pub static ANCHOR: JwtAuthenticator = JwtAuthenticator;

#[async_trait]
impl OpenStackAuthType for JwtAuthenticator {
    fn get_supported_auth_methods(&self) -> Vec<&'static str> {
        vec!["v4jwt", "jwt"]
    }

    fn api_version(&self) -> (u8, u8) {
        (4, 0)
    }

    fn requirements(&self, _hints: Option<&Value>) -> Result<Value, AuthError> {
        Ok(json!({
            "type": "object",
            "required": ["identity_provider", "attribute_mapping_name", "jwt"],
            "properties": {
                "identity_provider": {
                    "type": "string",
                    "description": "Identity Provider ID"
                },
                "attribute_mapping_name": {
                    "type": "string",
                    "description": "Attribute mapping name"
                },
                "jwt": {
                    "type": "string",
                    "format": "password",
                    "description": "JWT token"
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
            .ok_or(JwtError::MissingIdentityProvider)?;
        let endpoint = identity_url.join(
            format!(
                "federation/identity_providers/{idp_id}/jwt",
                idp_id = idp_id.expose_secret(),
            )
            .as_str(),
        )?;
        let mut request = http_client.post(endpoint);
        request = request
            .bearer_auth(
                values
                    .get("jwt")
                    .ok_or(JwtError::MissingJwt)?
                    .expose_secret(),
            )
            .header(
                "openstack-mapping",
                values
                    .get("attribute_mapping_name")
                    .ok_or(JwtError::MissingAttributeMapping)?
                    .expose_secret(),
            );

        let response = execute_auth_request(http_client, request.build()?).await?;

        let auth_token = AuthToken::from_reqwest_response(response).await?;

        Ok(Auth::AuthToken(Box::new(auth_token)))
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
                    .path("/federation/identity_providers/idp1/jwt")
                    .header("authorization", "Bearer my-jwt-token")
                    .header("openstack-mapping", "mapping1");
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

        let authenticator = JwtAuthenticator;

        match authenticator
            .auth(
                &http_client,
                &base_url,
                &HashMap::from([
                    ("identity_provider".into(), SecretString::from("idp1")),
                    (
                        "attribute_mapping_name".into(),
                        SecretString::from("mapping1"),
                    ),
                    ("jwt".into(), SecretString::from("my-jwt-token")),
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
        let authenticator = JwtAuthenticator;
        let err = authenticator
            .auth(
                &Client::new(),
                &Url::parse("http://localhost").unwrap(),
                &HashMap::from([
                    (
                        "attribute_mapping_name".into(),
                        SecretString::from("mapping1"),
                    ),
                    ("jwt".into(), SecretString::from("token")),
                ]),
                None,
                None,
            )
            .await;
        assert!(err.is_err());
    }

    #[tokio::test]
    async fn test_auth_missing_jwt() {
        let authenticator = JwtAuthenticator;
        let err = authenticator
            .auth(
                &Client::new(),
                &Url::parse("http://localhost").unwrap(),
                &HashMap::from([
                    ("identity_provider".into(), SecretString::from("idp1")),
                    (
                        "attribute_mapping_name".into(),
                        SecretString::from("mapping1"),
                    ),
                ]),
                None,
                None,
            )
            .await;
        assert!(err.is_err());
    }

    #[tokio::test]
    async fn test_auth_missing_mapping() {
        let authenticator = JwtAuthenticator;
        let err = authenticator
            .auth(
                &Client::new(),
                &Url::parse("http://localhost").unwrap(),
                &HashMap::from([
                    ("identity_provider".into(), SecretString::from("idp1")),
                    ("jwt".into(), SecretString::from("token")),
                ]),
                None,
                None,
            )
            .await;
        assert!(err.is_err());
    }

    #[test]
    fn test_get_supported_auth_methods() {
        let authenticator = JwtAuthenticator;
        assert!(
            authenticator
                .get_supported_auth_methods()
                .contains(&"v4jwt")
        );
        assert!(authenticator.get_supported_auth_methods().contains(&"jwt"));
    }

    #[test]
    fn test_requirements() {
        let authenticator = JwtAuthenticator;
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
                .contains(&"attribute_mapping_name".into())
        );
        assert!(req["required"].as_array().unwrap().contains(&"jwt".into()));
    }

    #[test]
    fn test_api_version() {
        let authenticator = JwtAuthenticator;
        assert_eq!(authenticator.api_version(), (4, 0));
    }
}
