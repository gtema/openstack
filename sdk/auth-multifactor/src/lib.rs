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

//! # Multifactor authentication method for [`openstack_sdk`]
use std::collections::BTreeSet;

use async_trait::async_trait;
use secrecy::SecretString;
use serde_json::{Value, json};
use thiserror::Error;

use openstack_sdk_auth_core::{
    Auth, AuthError, AuthMethodPluginRegistration, AuthPluginRegistration, AuthToken,
    AuthTokenScope, OpenStackAuthType, execute_auth_request,
};

// Force the linker to include the crate
use openstack_sdk_auth_password as _;
use openstack_sdk_auth_token as _;
use openstack_sdk_auth_totp as _;

/// Multifactor Authentication for OpenStack SDK.
pub struct MultifactorAuthenticator;

// Submit the plugin to the registry at compile-time
static PLUGIN: MultifactorAuthenticator = MultifactorAuthenticator;

inventory::submit! {
    AuthPluginRegistration { method: &PLUGIN }
}

fn deep_merge_value(a: &mut Value, b: Value) {
    match (a, b) {
        (Value::Object(a), Value::Object(b)) => {
            for (k, v) in b {
                deep_merge_value(a.entry(k).or_insert(Value::Null), v);
            }
        }
        (a, b) => *a = b,
    }
}

#[async_trait]
impl OpenStackAuthType for MultifactorAuthenticator {
    fn get_supported_auth_methods(&self) -> Vec<&'static str> {
        vec!["v3multifactor"]
    }

    fn requirements(&self, hints: Option<&Value>) -> Result<Value, AuthError> {
        let auth_methods: Vec<String> = serde_json::from_value(
            hints
                .as_ref()
                .ok_or(MultifactorAuthError::MissingRequirementsHints)?
                .get("auth_methods")
                .ok_or(MultifactorAuthError::MissingAuthMethods)?
                .clone(),
        )?;
        let mut required: BTreeSet<String> = BTreeSet::new();
        let mut props: Value = json!({});
        for method in auth_methods {
            if let Some(authenticator) = inventory::iter::<AuthMethodPluginRegistration>
                .into_iter()
                .find(|x| {
                    x.method
                        .get_supported_auth_methods()
                        .contains(&method.as_str())
                })
                .map(|x| x.method)
            {
                let method_requirements = authenticator.requirements(hints)?;
                required.extend(serde_json::from_value::<Vec<String>>(
                    method_requirements["required"].clone(),
                )?);

                if let Some(method_props) = method_requirements["properties"].as_object() {
                    for (name, val) in method_props.iter() {
                        props[name] = val.clone();
                    }
                }
            }
        }
        Ok(json!({
            "type": "object",
            "required": required,
            "properties": props
        }))
    }

    fn api_version(&self) -> (u8, u8) {
        (3, 0)
    }

    async fn auth(
        &self,
        http_client: &reqwest::Client,
        identity_url: &url::Url,
        values: std::collections::HashMap<String, SecretString>,
        scope: Option<&AuthTokenScope>,
        hints: Option<&serde_json::Value>,
    ) -> Result<Auth, AuthError> {
        let auth_methods: Vec<String> = serde_json::from_value(
            hints
                .as_ref()
                .ok_or(MultifactorAuthError::MissingRequirementsHints)?
                .get("auth_methods")
                .ok_or(MultifactorAuthError::MissingAuthMethods)?
                .clone(),
        )?;
        let mut methods: BTreeSet<String> = BTreeSet::new();
        let mut identity: Value = json!({});
        for method in auth_methods {
            if let Some(authenticator) = inventory::iter::<AuthMethodPluginRegistration>
                .into_iter()
                .find(|x| {
                    x.method
                        .get_supported_auth_methods()
                        .contains(&method.as_str())
                })
                .map(|x| x.method)
            {
                let (method, method_identity) = authenticator.get_auth_data(&values)?;
                methods.insert(method.into());
                deep_merge_value(&mut identity, method_identity);
            }
        }
        let endpoint = identity_url.join("auth/tokens")?;

        let mut body = json!({
            "auth": {
                "identity": identity
            }
        });
        body["auth"]["identity"]["methods"] = Vec::from_iter(methods).into();
        if let Some(scope) = scope {
            body["auth"]["scope"] = serde_json::to_value(scope)?;
        }

        let request = http_client.post(endpoint).json(&body).build()?;

        let response = execute_auth_request(http_client, request).await?;

        let auth_token = AuthToken::from_reqwest_response(response).await?;

        Ok(Auth::AuthToken(Box::new(auth_token)))
    }
}

/// Token related errors
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum MultifactorAuthError {
    /// Missing auth methods hint
    #[error("Auth methods information is missing")]
    MissingAuthMethods,

    /// Missing auth methods hint
    #[error("Plugin requirements hints missing")]
    MissingRequirementsHints,

    /// Missing secret
    #[error("Token is missing")]
    MissingToken,
}

impl From<MultifactorAuthError> for AuthError {
    fn from(source: MultifactorAuthError) -> Self {
        Self::plugin(source)
    }
}

#[cfg(test)]
mod tests {
    use httpmock::MockServer;
    use reqwest::Client;
    use reqwest::StatusCode;
    use secrecy::{ExposeSecret, SecretString};
    use serde_json::json;
    use std::collections::HashMap;
    use url::Url;

    use openstack_sdk_auth_core::Auth;

    use super::*;

    #[test]
    fn test_deep_merge() {
        let mut sot1 = json!({"a": "b"});
        deep_merge_value(&mut sot1, json!({"c": "d"}));
        assert_eq!(json!({"a": "b", "c": "d"}), sot1);
        let mut sot1 = json!({"a": {"b": "c"}});
        deep_merge_value(&mut sot1, json!({"a": {"d": "e"}}));
        assert_eq!(json!({"a": {"b": "c", "d": "e"}}), sot1);
    }

    #[test]
    fn test_requirements() {
        let authenticator = &PLUGIN;
        assert!(authenticator.requirements(None).is_err());
        assert!(authenticator.requirements(Some(&json!({}))).is_err());
        assert!(
            authenticator
                .requirements(Some(&json!({"foo": "bar"})))
                .is_err()
        );
        assert!(
            authenticator
                .requirements(Some(&json!({"auth_methods": "bar"})))
                .is_err()
        );
        assert_eq!(
            authenticator
                .requirements(Some(&json!({"auth_methods": ["v3token"]})))
                .unwrap(),
            json!({
                "type": "object",
                "required": ["token"],
                "properties": {
                    "token": {
                        "type": "string",
                        "format": "password",
                        "description": "Valid Keystone Token",
                    }
                }
            })
        );
        assert_eq!(
            authenticator
                .requirements(Some(&json!({"auth_methods": ["token"]})))
                .unwrap(),
            json!({
                "type": "object",
                "required": ["token"],
                "properties": {
                    "token": {
                        "type": "string",
                        "format": "password",
                        "description": "Valid Keystone Token",
                    }
                }
            })
        );
        assert_eq!(
            authenticator
                .requirements(Some(&json!({"auth_methods": ["token", "totp"]})))
                .unwrap(),
            json!({
                "type": "object",
                "required": ["passcode", "token"],
                "properties": {
                    "token": {
                        "type": "string",
                        "format": "password",
                        "description": "Valid Keystone Token",
                    },
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
                                "methods": ["token", "totp"],
                                "token": {
                                    "id": "secret",
                                },
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
        let mock_pass_totp = server
            .mock_async(|when, then| {
                when.method(httpmock::Method::POST)
                    .path("/auth/tokens")
                    .json_body(json!({
                        "auth": {
                            "identity": {
                                "methods": ["password", "totp"],
                                "password": {
                                    "user": {
                                        "id": "uid",
                                        "password": "password",
                                    }
                                },
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
                    ("token".into(), SecretString::from("secret")),
                    ("passcode".into(), SecretString::from("passcode")),
                    ("user_id".into(), SecretString::from("uid")),
                ]),
                None,
                Some(&json!({"auth_methods": ["token", "totp"]})),
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

        match authenticator
            .auth(
                &http_client,
                &base_url,
                HashMap::from([
                    ("password".into(), SecretString::from("password")),
                    ("passcode".into(), SecretString::from("passcode")),
                    ("user_id".into(), SecretString::from("uid")),
                ]),
                None,
                Some(&json!({"auth_methods": ["password", "totp"]})),
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
        mock_pass_totp.assert_async().await;
    }
}
