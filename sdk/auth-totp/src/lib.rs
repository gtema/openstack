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

//! # Token authentication method for [`openstack_sdk`]
//!
//! Authorization using Token look like:
//!
//! ```json
//! {
//!     "auth": {
//!         "identity": {
//!             "methods": [
//!                 "totp"
//!             ],
//!             "totp": {
//!                 "user": {
//!                     "id": "ee4dfb6e5540447cb3741905149d9b6e",
//!                     "passcode": "123456"
//!                 }
//!             }
//!         },
//!     }
//! }
//! ```

use std::collections::HashMap;

use async_trait::async_trait;
use thiserror::Error;

use secrecy::{ExposeSecret, SecretString};
use serde_json::{Value, json};

use openstack_sdk_auth_core::{
    Auth, AuthError, AuthMethodPluginRegistration, AuthPluginRegistration, AuthToken,
    AuthTokenScope, OpenStackAuthType, OpenStackMultifactorAuthMethod, execute_auth_request,
};

/// Token Authentication for OpenStack SDK.
pub struct TotpAuthenticator;

// Submit the plugin to the registry at compile-time
static PLUGIN: TotpAuthenticator = TotpAuthenticator;
inventory::submit! {
    AuthPluginRegistration { method: &PLUGIN }
}
inventory::submit! {
    AuthMethodPluginRegistration { method: &PLUGIN }
}

impl TotpAuthenticator {
    fn _get_supported_auth_methods(&self) -> Vec<&'static str> {
        vec!["v3totp", "totp"]
    }

    fn _requirements(&self) -> Value {
        json!({
            "type": "object",
            "required": ["passcode"],
            "properties": {
                "passcode": {
                    "type": "string",
                    "format": "password",
                    "description": "TOTP passcode",
                },
                "user_id": {
                    "type": "string",
                    "description": "User ID",
                },
                "username": {
                    "type": "string",
                    "description": "User name",
                },
                "user_domain_id": {
                    "type": "string",
                    "description": "User domain ID",
                },
                "user_domain_name": {
                    "type": "string",
                    "description": "User domain name",
                },
            }
        })
    }

    fn _get_auth_data(
        &self,
        values: &HashMap<String, SecretString>,
    ) -> Result<(&'static str, Value), TotpAuthError> {
        let passcode = values
            .get("passcode")
            .ok_or(TotpAuthError::MissingPasscode)?
            .expose_secret();

        let mut user = json!({"passcode": passcode});
        if let Some(user_id) = values.get("user_id") {
            user["id"] = user_id.expose_secret().into();
        } else if let Some(user_name) = values.get("username") {
            user["name"] = user_name.expose_secret().into();
            if let Some(udi) = values.get("user_domain_id") {
                user["domain"]["id"] = udi.expose_secret().into();
            } else if let Some(udn) = values.get("user_domain_name") {
                user["domain"]["name"] = udn.expose_secret().into();
            } else {
                return Err(TotpAuthError::MissingUserDomain)?;
            }
        } else {
            return Err(TotpAuthError::MissingUser)?;
        }
        let body = json!({ "totp": { "user": user } });
        Ok(("totp", body))
    }
}

impl OpenStackMultifactorAuthMethod for TotpAuthenticator {
    /// Return list of supported authentication methods.
    fn get_supported_auth_methods(&self) -> Vec<&'static str> {
        self._get_supported_auth_methods()
    }

    /// Get the json schema of the data the plugin requires to complete the authentication.
    fn requirements(
        &self,
        _hints: Option<&serde_json::Value>,
    ) -> Result<serde_json::Value, AuthError> {
        Ok(self._requirements())
    }

    /// Authenticate the client with the configuration.
    fn get_auth_data(
        &self,
        values: &HashMap<String, SecretString>,
    ) -> Result<(&'static str, serde_json::Value), AuthError> {
        Ok(self._get_auth_data(values)?)
    }
}

#[async_trait]
impl OpenStackAuthType for TotpAuthenticator {
    fn get_supported_auth_methods(&self) -> Vec<&'static str> {
        vec!["v3totp", "totp"]
    }

    fn requirements(&self, _hints: Option<&Value>) -> Result<Value, AuthError> {
        Ok(self._requirements())
    }

    fn api_version(&self) -> (u8, u8) {
        (3, 0)
    }

    async fn auth(
        &self,
        http_client: &reqwest::Client,
        identity_url: &url::Url,
        values: HashMap<String, SecretString>,
        scope: Option<&AuthTokenScope>,
        _hints: Option<&serde_json::Value>,
    ) -> Result<Auth, AuthError> {
        let (method, data) = self._get_auth_data(&values)?;
        let mut body = json!({ "auth": { "identity": data } });
        body["auth"]["identity"]["methods"] = [method].into();
        if let Some(scope) = scope {
            body["auth"]["scope"] = serde_json::to_value(scope)?;
        }

        let endpoint = identity_url.join("auth/tokens")?;

        let request = http_client.post(endpoint).json(&body).build()?;

        let response = execute_auth_request(http_client, request).await?;

        let auth_token = AuthToken::from_reqwest_response(response).await?;

        Ok(Auth::AuthToken(Box::new(auth_token)))
    }
}

/// Token related errors
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum TotpAuthError {
    /// Missing totp passcode
    #[error("TOTP passcode is missing")]
    MissingPasscode,

    /// Missing User info
    #[error("User name/id is required for TOTP authentication")]
    MissingUser,

    /// Missing User info
    #[error("User domain name/id is required for TOTP authentication")]
    MissingUserDomain,
}

impl From<TotpAuthError> for AuthError {
    fn from(source: TotpAuthError) -> Self {
        Self::plugin(source)
    }
}

#[cfg(test)]
mod tests {
    use httpmock::MockServer;
    use reqwest::Client;
    use reqwest::StatusCode;
    use secrecy::SecretString;
    use serde_json::json;
    use std::collections::HashMap;
    use url::Url;

    use openstack_sdk_auth_core::Auth;
    use openstack_sdk_auth_core::types::*;

    use super::*;

    #[test]
    fn test_get_supported_auth_methods() {
        let authenticator = &PLUGIN;
        assert!(
            openstack_sdk_auth_core::OpenStackAuthType::get_supported_auth_methods(authenticator)
                .contains(&"v3totp")
        );
        assert!(
            openstack_sdk_auth_core::OpenStackAuthType::get_supported_auth_methods(authenticator)
                .contains(&"totp")
        );
    }

    #[test]
    fn test_requirements() {
        let authenticator = &PLUGIN;
        assert!(
            openstack_sdk_auth_core::OpenStackAuthType::requirements(authenticator, None).is_ok(),
        );
    }

    #[test]
    fn test_get_auth_data() {
        let authenticator = &PLUGIN;
        assert_eq!(
            (
                "totp",
                json!({"totp": {"user": {"id": "uid", "passcode": "passcode"}}})
            ),
            authenticator
                .get_auth_data(&HashMap::from([
                    ("passcode".into(), SecretString::from("passcode")),
                    ("user_id".into(), SecretString::from("uid")),
                ]))
                .unwrap()
        );
    }

    #[tokio::test]
    async fn test_auth() {
        let server = MockServer::start_async().await;
        let base_url = Url::parse(&server.base_url()).unwrap();

        let mock = server
            .mock_async(|when, then| {
                when.method(httpmock::Method::POST)
                    .path("/auth/tokens")
                    .json_body(json!({
                        "auth": {
                            "identity": {
                                "methods": ["totp"],
                                "totp": {
                                    "user": {
                                        "id": "uid",
                                        "passcode": "passcode"
                                    }
                                },
                            },
                            "scope": {
                                "project": {
                                    "id": "pid"
                                }
                            }
                        }
                    }));
                then.status(StatusCode::CREATED)
                    .header("x-subject-token", "foo")
                    .json_body(json!({
                        "token": {
                            "user": {
                                "id": "uid",
                                "name": "uname"
                            },
                            "expires_at": "2018-01-15T22:14:05.000000Z",
                        }
                    }));
            })
            .await;
        let http_client = Client::new();

        let authenticator = &PLUGIN;

        match authenticator
            .auth(
                &http_client,
                &base_url,
                HashMap::from([
                    ("passcode".into(), SecretString::from("passcode")),
                    ("user_id".into(), SecretString::from("uid")),
                ]),
                Some(&AuthTokenScope::Project(Project {
                    id: Some("pid".into()),
                    ..Default::default()
                })),
                None,
            )
            .await
        {
            Ok(Auth::AuthToken(token)) => {
                assert_eq!(token.token.expose_secret(), "foo");
            }

            other => {
                panic!("success was expected, instead it is {:?}", other);
            }
        }
        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_auth_scope() {
        let server = MockServer::start_async().await;
        let base_url = Url::parse(&server.base_url()).unwrap();

        let mock = server
            .mock_async(|when, then| {
                when.method(httpmock::Method::POST)
                    .path("/auth/tokens")
                    .json_body(json!({
                        "auth": {
                            "identity": {
                                "methods": ["totp"],
                                "totp": {
                                    "user": {
                                        "id": "uid",
                                        "passcode": "passcode"
                                    }
                                }
                            }
                        }
                    }));
                then.status(StatusCode::CREATED)
                    .header("x-subject-token", "foo")
                    .json_body(json!({
                        "token": {
                            "user": {
                                "id": "uid",
                                "name": "uname"
                            },
                            "expires_at": "2018-01-15T22:14:05.000000Z",
                        }
                    }));
            })
            .await;
        let http_client = Client::new();

        let authenticator = &PLUGIN;

        match authenticator
            .auth(
                &http_client,
                &base_url,
                HashMap::from([
                    ("passcode".into(), SecretString::from("passcode")),
                    ("user_id".into(), SecretString::from("uid")),
                ]),
                None,
                None,
            )
            .await
        {
            Ok(Auth::AuthToken(token)) => {
                assert_eq!(token.token.expose_secret(), "foo");
            }

            other => {
                panic!("success was expected, instead it is {:?}", other);
            }
        }
        mock.assert_async().await;
    }
}
