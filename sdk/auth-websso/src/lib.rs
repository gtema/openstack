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

//! # WebSSO authentication for [`openstack_sdk`]
//!
//! This plugin implements single-sign-on (WebSSO) authentication against OpenStack's
//! Identity service (Keystone). It starts a temporary local HTTP callback server
//! to receive the authentication token from Keystone's WebSSO redirect URL.
//!
//! The flow is:
//! 1. Start a local HTTP callback server on an available port
//! 2. Construct the WebSSO URL with the callback origin
//! 3. Open the user's browser to the WebSSO page
//! 4. Receive the Keystone token via the callback server
//! 5. Return the authenticated token

use std::time::Duration;

use async_trait::async_trait;
use secrecy::{ExposeSecret, SecretString};
use serde_json::{Value, json};
use thiserror::Error;
use url::Url;

use dialoguer::Confirm;

use openstack_sdk_auth_core::{
    Auth, AuthError, AuthPluginRegistration, AuthToken, AuthTokenError, AuthTokenScope,
    OpenStackAuthType,
};
use openstack_sdk_websso_host::{BrowserOpenPolicy, CallbackServer, WebssoHostError};

/// WebSSO authentication for OpenStack SDK.
///
/// Authenticates via a browser-based single-sign-on (WebSSO) flow
/// with a local callback server to receive the token.
pub struct WebSSOAuthenticator;

// Submit the plugin to the registry at compile-time
static PLUGIN: WebSSOAuthenticator = WebSSOAuthenticator;
inventory::submit! {
    AuthPluginRegistration { method: &PLUGIN }
}
#[used]
pub static ANCHOR: WebSSOAuthenticator = WebSSOAuthenticator;

#[async_trait]
impl OpenStackAuthType for WebSSOAuthenticator {
    fn get_supported_auth_methods(&self) -> Vec<&'static str> {
        vec!["v3websso"]
    }

    fn requirements(&self, _hints: Option<&Value>) -> Result<Value, AuthError> {
        Ok(json!({
            "type": "object",
            "required": ["protocol"],
            "properties": {
                "identity_provider": {
                    "type": "string",
                    "description": "Identity Provider ID"
                },
                "protocol": {
                    "type": "string",
                    "description": "Protocol"
                },
                "callback_port": {
                    "type": "integer",
                    "description": "The local port to use for the authentication callback server. If omitted, the default (8050) is used."
                },
            }
        }))
    }

    fn api_version(&self) -> (u8, u8) {
        (3, 0)
    }

    async fn auth(
        &self,
        _http_client: &reqwest::Client,
        identity_url: &url::Url,
        values: &std::collections::HashMap<String, SecretString>,
        _scope: Option<&AuthTokenScope>,
        _hints: Option<&serde_json::Value>,
    ) -> Result<Auth, AuthError> {
        let protocol_id = values.get("protocol").ok_or(WebSsoError::MissingProtocol)?;

        let callback_port = values
            .get("callback_port")
            .and_then(|v| v.expose_secret().parse::<u16>().ok());

        let endpoint = if let Some(idp_id) = values.get("identity_provider") {
            format!(
                "auth/OS-FEDERATION/identity_providers/{idp_id}/protocols/{protocol_id}/websso",
                idp_id = idp_id.expose_secret(),
                protocol_id = protocol_id.expose_secret()
            )
        } else {
            format!(
                "auth/OS-FEDERATION/websso/{protocol_id}",
                protocol_id = protocol_id.expose_secret()
            )
        };

        let mut auth_url = identity_url.join(&endpoint)?;

        let token_auth = get_token_auth(&mut auth_url, callback_port).await?;

        Ok(Auth::AuthToken(Box::new(token_auth)))
    }
}

/// WebSSO related errors.
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum WebSsoError {
    /// WebSSO callback did not return a token.
    #[error("WebSSO callback didn't return a token")]
    CallbackNoToken,

    /// Some failure in the SSO flow.
    #[error("WebSSO authentication failed")]
    CallbackFailed,

    /// Error during invoking the dialoguer.
    #[error("error using the dialoguer: {}", source)]
    Dialoguer {
        /// The error source.
        #[from]
        source: dialoguer::Error,
    },

    /// Error from the shared WebSSO callback host service (binding the
    /// callback listener, serving/validating the callback, or opening the
    /// browser).
    #[error("WebSSO host service error: {}", source)]
    Host {
        /// The error source.
        #[from]
        source: WebssoHostError,
    },

    /// Protocol is missing.
    #[error("Federation protocol information is missing")]
    MissingProtocol,
}

impl From<WebSsoError> for AuthError {
    fn from(source: WebSsoError) -> Self {
        Self::plugin(source)
    }
}

/// Return [`AuthToken`] obtained using the WebSSO (Keystone behind mod_auth_oidc)
pub async fn get_token_auth(
    url: &mut Url,
    callback_port: Option<u16>,
) -> Result<AuthToken, AuthTokenError> {
    let token = get_token(url, callback_port)
        .await
        .map_err(AuthTokenError::plugin)?;
    Ok(AuthToken::new(token, None))
}

// Perform WebSSO by opening a browser window with tiny webserver started to capture the callback
///
/// - bind the callback server (host-generated anti-CSRF `state` embedded in
///   its URL)
/// - open browser pointing to the SSO url
/// - wait for the response with the OpenStack token
async fn get_token(url: &mut Url, callback_port: Option<u16>) -> Result<String, WebSsoError> {
    let server = CallbackServer::bind(callback_port).await?;
    url.set_query(Some(format!("origin={}", server.callback_url()).as_str()));
    let confirmation = Confirm::new()
        .with_prompt(format!(
            "A default browser is going to be opened at `{}`. Do you want to continue?",
            url.as_str()
        ))
        .interact()?;
    if !confirmation {
        return Err(WebSsoError::CallbackFailed);
    }
    // `require_https: false`: the Keystone identity endpoint this URL is
    // built from may legitimately be plain `http://` in a local/dev
    // deployment, unlike the WASM SSO plugin ABI which has no such
    // grandfathered use case and always requires `https://`.
    openstack_sdk_websso_host::open_browser(
        url,
        BrowserOpenPolicy {
            require_https: false,
        },
    )?;
    let mut params = server.wait_for_callback(Duration::from_secs(120)).await?;
    params.remove("token").ok_or(WebSsoError::CallbackNoToken)
}
