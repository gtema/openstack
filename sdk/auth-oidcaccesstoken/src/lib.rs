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

//! # Federated (OAUTH2/OIDC) authentication using the OIDC access token
//!

use async_trait::async_trait;
use secrecy::ExposeSecret;
use serde_json::{Value, json};
use thiserror::Error;

use openstack_sdk_auth_core::{
    Auth, AuthError, AuthPluginRegistration, AuthToken, AuthTokenScope, OpenStackAuthType,
    execute_auth_request,
};

/// V3 OIDCAccessToken Authentication for OpenStack SDK.
pub struct OidcAccessTokenAuthenticator;

// Submit the plugin to the registry at compile-time
static PLUGIN: OidcAccessTokenAuthenticator = OidcAccessTokenAuthenticator;
inventory::submit! {
    AuthPluginRegistration { method: &PLUGIN }
}

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
                    "description": "Protocol"
                },
                "access_token": {
                    "type": "string",
                    "format": "password",
                    "description": "Access token"
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
            .ok_or(OidcAccessTokenError::MissingIdpId)?;
        let protocol = values
            .get("protocol")
            .ok_or(OidcAccessTokenError::MissingProtocolId)?;

        let endpoint = identity_url.join(
            format!(
                "OS-FEDERATION/identity_providers/{idp_id}/protocols/{protocol}/auth",
                idp_id = idp_id.expose_secret(),
                protocol = protocol.expose_secret()
            )
            .as_str(),
        )?;

        let request = http_client.post(endpoint).build()?;
        let response = execute_auth_request(http_client, request).await?;

        let auth_token = AuthToken::from_reqwest_response(response).await?;

        Ok(Auth::AuthToken(Box::new(auth_token)))
    }
}
/// OidcAccessToken related errors
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum OidcAccessTokenError {
    /// Auth data is missing
    #[error("auth data is missing")]
    MissingAuthData,

    /// Identity provider ID is missing
    #[error("identity_provider ID is missing")]
    MissingIdpId,

    /// Federation protocol ID is missing
    #[error("federation protocol ID is missing")]
    MissingProtocolId,

    /// Access token is missing
    #[error("access token is missing")]
    MissingAccessToken,
}

impl From<OidcAccessTokenError> for AuthError {
    fn from(source: OidcAccessTokenError) -> Self {
        Self::plugin(source)
    }
}

#[cfg(test)]
mod tests {}
