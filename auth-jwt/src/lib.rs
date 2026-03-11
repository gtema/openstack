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

//! # JWT authentication for OpenStack based on the Keystone-rs
//!
//! This module implements login using the JWT token by exchanging it for a regular Keystone token.

use async_trait::async_trait;
use secrecy::ExposeSecret;
use serde_json::{Value, json};
use thiserror::Error;

use openstack_sdk_auth_core::{
    Auth, AuthError, AuthPluginRegistration, AuthToken, AuthTokenScope, OpenStackAuthType,
    execute_auth_request,
};

/// JWT related errors.
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum JwtError {
    /// Auth data is missing.
    #[error("auth data is missing")]
    MissingAuthData,

    /// Identity provider id is missing.
    #[error("identity provider id is missing")]
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

/// JWT Authentication for OpenStack SDK.
pub struct JwtAuthenticator;
// Submit the plugin to the registry at compile-time
static PLUGIN: JwtAuthenticator = JwtAuthenticator;
inventory::submit! {
    AuthPluginRegistration { method: &PLUGIN }
}

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
        values: std::collections::HashMap<String, secrecy::SecretString>,
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
