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

//! # Application credentials authentication method for [`openstack_sdk`]
//!
//! Authorization using Application Credentials look like:
//!
//! With ID and Secret
//! ```json
//! {
//!    "auth": {
//!        "identity": {
//!            "methods": [
//!                "application_credential"
//!            ],
//!            "application_credential": {
//!                "id": "423f19a4ac1e4f48bbb4180756e6eb6c",
//!                "secret": "rEaqvJka48mpv"
//!            }
//!        }
//!    }
//!}
//! ```
//!
//! With Name and Secret and user information
//! ```json
//! {
//!     "auth": {
//!         "identity": {
//!             "methods": [
//!                 "application_credential"
//!             ],
//!             "application_credential": {
//!                 "name": "monitoring",
//!                 "secret": "rEaqvJka48mpv",
//!                 "user": {
//!                     "id": "423f19a4ac1e4f48bbb4180756e6eb6c"
//!                 }
//!             }
//!         }
//!     }
//! }
//! ```

use async_trait::async_trait;
use thiserror::Error;

use secrecy::{ExposeSecret, SecretString};
use serde_json::{Value, json};

use openstack_sdk_auth_core::{
    Auth, AuthError, AuthPluginRegistration, AuthToken, OpenStackAuthType,
};

/// Application Credential Authentication for OpenStack SDK.
pub struct AppilcationCredentialAuthenticator;

// Submit the plugin to the registry at compile-time
static PLUGIN: AppilcationCredentialAuthenticator = AppilcationCredentialAuthenticator;
inventory::submit! {
    AuthPluginRegistration { method: &PLUGIN }
}

#[async_trait]
impl OpenStackAuthType for AppilcationCredentialAuthenticator {
    fn get_supported_auth_methods(&self) -> Vec<&'static str> {
        vec!["v3applicationcredential", "applicationcredential"]
    }

    fn requirements(&self) -> Value {
        json!({
            "type": "object",
            "required": ["application_credential_secret"],
            "properties": {
                "application_credential_id": {
                    "type": "string",
                    "description": "Application credential ID",
                },
                "application_credential_secret": {
                    "type": "string",
                    "format": "password",
                    "description": "Application credential secret",
                },
                "user_id": {
                    "type": "string",
                    "description": "User ID",
                },
                "user_name": {
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

    fn api_version(&self) -> (u8, u8) {
        (3, 0)
    }

    async fn auth(
        &self,
        http_client: &reqwest::Client,
        identity_url: &url::Url,
        values: std::collections::HashMap<String, SecretString>,
    ) -> Result<Auth, AuthError> {
        let endpoint = identity_url
            .join("auth/tokens")
            .map_err(ApplicationCredentialError::from)?;
        let mut app_cred = json!({
            "secret": values
                .get("application_credential_secret")
                .ok_or(ApplicationCredentialError::MissingSecret)?.expose_secret()
        });
        if let Some(app_cred_id) = values.get("application_credential_id") {
            app_cred["id"] = app_cred_id.expose_secret().into();
        } else if let Some(app_cred_name) = values.get("application_credential_name") {
            app_cred["name"] = app_cred_name.expose_secret().into();
            let mut user = json!({});
            if let Some(user_id) = values.get("user_id") {
                user["id"] = user_id.expose_secret().into();
            } else if let Some(user_name) = values.get("user_name") {
                user["name"] = user_name.expose_secret().into();
                if let Some(udi) = values.get("user_domain_id") {
                    user["user_domain_id"] = udi.expose_secret().into();
                } else if let Some(udn) = values.get("user_domain_name") {
                    user["user_domain_name"] = udn.expose_secret().into();
                } else {
                    return Err(ApplicationCredentialError::MissingUserDomain)?;
                }
            }
            app_cred["user"] = user;
        } else {
            return Err(ApplicationCredentialError::MissingIdOrName)?;
        }
        let body = json!({
            "auth": {
                "identity": {
                    "methods": ["application_credential"],
                    "application_credential": app_cred
                }
            }
        });

        let response = http_client
            .post(endpoint)
            .json(&body)
            .send()
            .await
            .map_err(ApplicationCredentialError::from)?;

        let auth_token = AuthToken::from_reqwest_response(response).await?;

        Ok(Auth::AuthToken(Box::new(auth_token)))
    }
}

/// Application Credential related errors
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum ApplicationCredentialError {
    /// Missing secret
    #[error("Application credential secret is missing")]
    MissingSecret,

    /// Missing ID and Name
    #[error("Application credential id or name must be present")]
    MissingIdOrName,

    /// Missing User info
    #[error("User name/id is required when application credential name is used")]
    MissingUser,

    /// Missing User info
    #[error(
        "User domain name/id is required when application credential name and user name are used"
    )]
    MissingUserDomain,

    /// Reqwest error.
    #[error("error sending query: {}", source)]
    Reqwest {
        /// The error source.
        #[from]
        source: reqwest::Error,
    },

    /// Url parse error.
    #[error(transparent)]
    Url {
        /// The error source
        #[from]
        source: url::ParseError,
    },
}

impl From<ApplicationCredentialError> for AuthError {
    fn from(source: ApplicationCredentialError) -> Self {
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

    use super::*;

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
                                "methods": ["application_credential"],
                                "application_credential": {
                                    "id": "app_cred_id",
                                    "secret": "secret"
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

        let authenticator = AppilcationCredentialAuthenticator {};

        match authenticator
            .auth(
                &http_client,
                &base_url,
                HashMap::from([
                    (
                        "application_credential_id".into(),
                        SecretString::from("app_cred_id"),
                    ),
                    (
                        "application_credential_secret".into(),
                        SecretString::from("secret"),
                    ),
                ]),
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

        if let Err(AuthError::Plugin { .. }) = authenticator
            .auth(
                &http_client,
                &base_url,
                HashMap::from([(
                    "application_credential_secret".into(),
                    SecretString::from("secret"),
                )]),
            )
            .await
        {
        } else {
            panic!("id or name missing must raise error");
        }
    }
}
