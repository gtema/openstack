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

//! # User password authentication method for [`openstack_sdk`]
//!
//! Authorization using Token look like:
//!
//! ```json
//! {
//!     "auth": {
//!         "identity": {
//!             "methods": [
//!                 "password"
//!             ],
//!             "password": {
//!                 "user": {
//!                     "name": "admin",
//!                     "domain": {
//!                         "name": "Default"
//!                     },
//!                     "password": "devstacker"
//!                 }
//!             }
//!         }
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
pub struct PasswordAuthenticator;

// Submit the plugin to the registry at compile-time
static PLUGIN: PasswordAuthenticator = PasswordAuthenticator;
inventory::submit! {
    AuthPluginRegistration { method: &PLUGIN }
}
inventory::submit! {
    AuthMethodPluginRegistration { method: &PLUGIN }
}

impl PasswordAuthenticator {
    fn _get_supported_auth_methods(&self) -> Vec<&'static str> {
        vec!["v3password", "password"]
    }

    fn _requirements(&self) -> Value {
        json!({
            "type": "object",
            "required": ["password"],
            "properties": {
                "password": {
                    "type": "string",
                    "format": "password",
                    "description": "User password",
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
    ) -> Result<(&'static str, Value), PasswordAuthError> {
        let password = values
            .get("password")
            .ok_or(PasswordAuthError::MissingPassword)?
            .expose_secret();

        let mut user = json!({"password": password});
        if let Some(user_id) = values.get("user_id") {
            user["id"] = user_id.expose_secret().into();
        } else if let Some(user_name) = values.get("username") {
            user["name"] = user_name.expose_secret().into();
            if let Some(udi) = values.get("user_domain_id") {
                user["domain"]["id"] = udi.expose_secret().into();
            } else if let Some(udn) = values.get("user_domain_name") {
                user["domain"]["name"] = udn.expose_secret().into();
            } else {
                return Err(PasswordAuthError::MissingUserDomain)?;
            }
        } else {
            return Err(PasswordAuthError::MissingUser)?;
        }
        let body = json!({ "password": {"user": user } });
        Ok(("password", body))
    }
}

impl OpenStackMultifactorAuthMethod for PasswordAuthenticator {
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
impl OpenStackAuthType for PasswordAuthenticator {
    fn get_supported_auth_methods(&self) -> Vec<&'static str> {
        self._get_supported_auth_methods()
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
pub enum PasswordAuthError {
    /// Missing user password.
    #[error("user password is missing")]
    MissingPassword,

    /// Missing User info.
    #[error("User name/id is required")]
    MissingUser,

    /// Missing User info.
    #[error("User domain name/id is required")]
    MissingUserDomain,
}

impl From<PasswordAuthError> for AuthError {
    fn from(source: PasswordAuthError) -> Self {
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
                .contains(&"v3password")
        );
        assert!(
            openstack_sdk_auth_core::OpenStackAuthType::get_supported_auth_methods(authenticator)
                .contains(&"password")
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
                "password",
                json!({"password": {"user": {"id": "uid", "password": "password"}}})
            ),
            authenticator
                .get_auth_data(&HashMap::from([
                    ("password".into(), SecretString::from("password")),
                    ("user_id".into(), SecretString::from("uid")),
                ]))
                .unwrap()
        );
        assert!(
            authenticator
                .get_auth_data(&HashMap::from([
                    ("password".into(), SecretString::from("password")),
                    ("username".into(), SecretString::from("uname")),
                ]))
                .is_err()
        );
        assert_eq!(
            (
                "password",
                json!({"password": {"user": {"name": "uname", "password": "password", "domain": {"name": "udname"}}}})
            ),
            authenticator
                .get_auth_data(&HashMap::from([
                    ("password".into(), SecretString::from("password")),
                    ("username".into(), SecretString::from("uname")),
                    ("user_domain_name".into(), SecretString::from("udname")),
                ]))
                .unwrap()
        );
        assert_eq!(
            (
                "password",
                json!({"password": {"user": {"name": "uname", "password": "password", "domain": {"id": "udid"}}}})
            ),
            authenticator
                .get_auth_data(&HashMap::from([
                    ("password".into(), SecretString::from("password")),
                    ("username".into(), SecretString::from("uname")),
                    ("user_domain_id".into(), SecretString::from("udid")),
                ]))
                .unwrap()
        );
        assert_eq!(
            (
                "password",
                json!({"password": {"user": {"name": "uname", "password": "password", "domain": {"id": "udid"}}}})
            ),
            authenticator
                .get_auth_data(&HashMap::from([
                    ("password".into(), SecretString::from("password")),
                    ("username".into(), SecretString::from("uname")),
                    ("user_domain_id".into(), SecretString::from("udid")),
                    ("user_domain_name".into(), SecretString::from("udname")),
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
                                "methods": ["password"],
                                "password": {
                                    "user": {
                                        "id": "uid",
                                        "password": "password"
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
                    ("password".into(), SecretString::from("password")),
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
                                "methods": ["password"],
                                "password": {
                                    "user": {
                                        "id": "uid",
                                        "password": "password"
                                    }
                                }
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
                    ("password".into(), SecretString::from("password")),
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
}
