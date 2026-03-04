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
//! # Passkey authentication for OpenStack based on the Keystone-rs project

use async_trait::async_trait;

mod error;
mod finish;
mod start;

use secrecy::{ExposeSecret, SecretString};
use serde_json::{Value, json};

use openstack_sdk_auth_core::{
    Auth, AuthError, AuthPluginRegistration, AuthToken, OpenStackAuthType,
};

pub use error::PasskeyError;
use finish::*;
use start::PasskeyAuthenticationStartResponse;

/// WebAuthN Authentication for OpenStack SDK.
pub struct WebAuthnAuthenticator;

// Submit the plugin to the registry at compile-time
static PLUGIN: WebAuthnAuthenticator = WebAuthnAuthenticator;
inventory::submit! {
    AuthPluginRegistration { method: &PLUGIN }
}

#[async_trait]
impl OpenStackAuthType for WebAuthnAuthenticator {
    fn get_supported_auth_methods(&self) -> Vec<&'static str> {
        vec!["v4passkey", "passkey"]
    }

    fn requirements(&self) -> Value {
        json!({
            "type": "object",
            "required": ["user_id"],
            "properties": {
                "user_id": {
                    "type": "string",
                    "description": "User ID"
                },
            }
        })
    }

    fn api_version(&self) -> (u8, u8) {
        (4, 0)
    }

    async fn auth(
        &self,
        http_client: &reqwest::Client,
        identity_url: &url::Url,
        values: std::collections::HashMap<String, SecretString>,
    ) -> Result<Auth, AuthError> {
        let user_id = values
            .get("user_id")
            .ok_or(PasskeyError::MissingAuthData)?
            .expose_secret();
        let auth_start_ep = identity_url
            .join("auth/passkey/start")
            .map_err(PasskeyError::from)?;

        let req: PasskeyAuthenticationStartResponse = http_client
            .post(auth_start_ep)
            .json(&json!({"passkey": {"user_id": user_id}}))
            .send()
            .await
            .map_err(PasskeyError::from)?
            .json::<PasskeyAuthenticationStartResponse>()
            .await
            .map_err(PasskeyError::from)?;

        use webauthn_authenticator_rs::WebauthnAuthenticator;

        let mut auth = WebauthnAuthenticator::new(
            webauthn_authenticator_rs::mozilla::MozillaAuthenticator::new(),
        );
        let mut passkey_auth: AuthFinishRequestBuilder = AuthFinishRequestBuilder::try_from(
            auth.do_authentication(
                // TODO: the url contains path, while authenticator may only want domain info
                identity_url.clone(),
                req.try_into()?,
            )
            .map_err(PasskeyError::from)?,
        )?;

        let auth_finish_ep = identity_url
            .join("auth/passkey/finish")
            .map_err(PasskeyError::from)?;
        passkey_auth.user_id(user_id);

        let response = http_client
            .post(auth_finish_ep)
            .json(&passkey_auth.build().map_err(PasskeyError::from)?)
            .send()
            .await
            .map_err(PasskeyError::from)?;

        let auth_token = AuthToken::from_reqwest_response(response).await?;

        Ok(Auth::AuthToken(Box::new(auth_token)))
    }
}
