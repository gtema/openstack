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

use std::collections::HashMap;
use std::convert::Infallible;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use async_trait::async_trait;
use bytes::Bytes;
use futures::io::Error as IoError;
use http_body_util::{BodyExt, Empty, Full, combinators::BoxBody};
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{Method, Request, Response, StatusCode, body::Incoming as IncomingBody};
use hyper_util::rt::TokioIo;
use secrecy::{ExposeSecret, SecretString};
use serde_json::{Value, json};
use thiserror::Error;
use tokio::net::TcpListener;
use tokio::signal;
use tokio_util::sync::CancellationToken;
use tracing::{error, info, trace, warn};
use url::Url;

use dialoguer::Confirm;

use openstack_sdk_auth_core::{
    Auth, AuthError, AuthPluginRegistration, AuthToken, AuthTokenError, AuthTokenScope,
    OpenStackAuthType,
};

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

    /// Http error.
    #[error("http server error: {}", source)]
    Http {
        /// The source of the error.
        #[from]
        source: http::Error,
    },

    /// Hyper error.
    #[error("hyper (http server) error: {}", source)]
    Hyper {
        /// The source of the error.
        #[from]
        source: hyper::Error,
    },

    /// IO communication error.
    #[error("`IO` error: {}", source)]
    IO {
        /// The error source.
        #[from]
        source: IoError,
    },

    /// Thread join error.
    #[error("`Join` error: {}", source)]
    Join {
        /// The error source.
        #[from]
        source: tokio::task::JoinError,
    },

    /// Auth data is missing.
    #[error("Auth data is missing")]
    MissingAuthData,

    /// Protocol is missing.
    #[error("Federation protocol information is missing")]
    MissingProtocol,

    /// Poisoned guard lock in the internal processing.
    #[error("internal error: poisoned lock: {}", context)]
    PoisonedLock {
        /// The source of the error.
        context: String,
    },
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
/// - start callback server
/// - open browser pointing to the SSO url
/// - wait for the response with the OpenStack token
async fn get_token(url: &mut Url, callback_port: Option<u16>) -> Result<String, WebSsoError> {
    let port = callback_port.unwrap_or(8050);
    let listener = TcpListener::bind(format!("127.0.0.1:{}", port)).await?;
    let addr = listener
        .local_addr()
        .map_err(|_| WebSsoError::MissingAuthData)?;
    url.set_query(Some(
        format!("origin=http://localhost:{}/callback", addr.port()).as_str(),
    ));
    let confirmation = Confirm::new()
        .with_prompt(format!(
            "A default browser is going to be opened at `{}`. Do you want to continue?",
            url.as_str()
        ))
        .interact()?;
    if confirmation {
        info!("Opening browser at {:?}", url.as_str());
        let cancel_token = CancellationToken::new();
        let state: Arc<Mutex<Option<String>>> = Arc::new(Mutex::new(None));

        tokio::spawn({
            let cancel_token = cancel_token.clone();
            async move {
                if let Ok(()) = signal::ctrl_c().await {
                    info!("received Ctrl-C, shutting down");
                    cancel_token.cancel();
                }
            }
        });

        let websso_handle = tokio::spawn({
            let cancel_token = cancel_token.clone();
            let state = state.clone();
            async move { websso_callback_server(listener, state, cancel_token, None).await }
        });
        open::that(url.as_str())?;

        let _res = websso_handle.await?;

        let guard = state.lock().map_err(|_| WebSsoError::PoisonedLock {
            context: "locking WebSSO authentication state".to_string(),
        })?;
        guard.clone().ok_or(WebSsoError::CallbackNoToken)
    } else {
        Err(WebSsoError::CallbackFailed)
    }
}

/// Start the WebSSO callback server on a pre-bound listener
async fn websso_callback_server(
    listener: TcpListener,
    state: Arc<Mutex<Option<String>>>,
    cancel_token: CancellationToken,
    start_tx: Option<tokio::sync::oneshot::Sender<()>>,
) -> Result<(), WebSsoError> {
    let addr = listener
        .local_addr()
        .map_err(|_| WebSsoError::MissingAuthData)?;
    info!("Starting webserver to receive SSO callback on {}", addr);
    if let Some(tx) = start_tx {
        let _ = tx.send(());
    }
    // Wait maximum 2 minute for auth processing
    let webserver_timeout = Duration::from_secs(120);
    loop {
        let state_clone = state.clone();

        tokio::select! {
            Ok((stream, _addr)) = listener.accept() => {
                let io = TokioIo::new(stream);
                let cancel_token_srv = cancel_token.clone();
                let cancel_token_conn = cancel_token.clone();

                let service = service_fn(move |req| {
                    let state_clone = state_clone.clone();
                    let cancel_token = cancel_token_srv.clone();
                    handle_request(req, state_clone, cancel_token)
                });

                tokio::task::spawn(async move {
                    let cancel_token = cancel_token_conn.clone();
                    if let Err(err) = http1::Builder::new().serve_connection(io, service).await {
                        error!("Failed to serve connection: {:?}", err);
                        cancel_token.cancel();
                    }
                });
            },
            _ = cancel_token.cancelled() => {
                info!("Stopping webserver");
                break;
            },
            _ = tokio::time::sleep(webserver_timeout) => {
                warn!("Timeout of {} sec waiting for authentication expired. Shutting down", webserver_timeout.as_secs());
                cancel_token.cancel();
            }
        }
    }
    Ok(())
}

/// Server request handler function
async fn handle_request(
    req: Request<IncomingBody>,
    state: Arc<Mutex<Option<String>>>,
    cancel_token: CancellationToken,
    //) -> Result<Response<BoxBody<Bytes, Infallible>>, hyper::Error> {
) -> Result<Response<BoxBody<Bytes, Infallible>>, WebSsoError> {
    match (req.method(), req.uri().path()) {
        (&Method::POST, "/callback") => {
            let b = req.collect().await?.to_bytes();
            trace!("Body is {:?}", b);
            let params = form_urlencoded::parse(b.as_ref())
                .into_owned()
                .collect::<HashMap<String, String>>();
            trace!("Params = {:?}", params);

            let mut data = state.lock().map_err(|_| WebSsoError::PoisonedLock {
                context: "locking WebSSO authentication state".to_string(),
            })?;
            if let Some(token) = params.get("token") {
                *data = Some(token.clone());
            }
            cancel_token.cancel();

            Ok(Response::builder()
                .body(Full::new(Bytes::from(include_str!("../static/callback.html"))).boxed())?)
        }
        _ => {
            // Return 404 not found response.
            Ok(Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(Empty::<Bytes>::new().boxed())?)
        }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};
    use tokio::net::TcpListener;
    use tokio::signal;
    use tokio_util::sync::CancellationToken;
    use tracing::{info, warn};

    use super::WebSsoError;
    use super::handle_request;

    /// Test-only variant that accepts a pre-bound listener to avoid port reservation races
    async fn websso_callback_server_test(
        listener: TcpListener,
        state: Arc<Mutex<Option<String>>>,
        cancel_token: CancellationToken,
    ) -> Result<(), WebSsoError> {
        use hyper::server::conn::http1;
        use hyper::service::service_fn;

        use hyper_util::rt::TokioIo;
        use tracing::error;

        info!("Starting webserver to receive SSO callback");
        let webserver_timeout = std::time::Duration::from_secs(120);
        loop {
            let state_clone = state.clone();

            tokio::select! {
                Ok((stream, _addr)) = listener.accept() => {
                    let io = TokioIo::new(stream);
                    let cancel_token_srv = cancel_token.clone();
                    let cancel_token_conn = cancel_token.clone();

                    let service = service_fn(move |req| {
                        let state_clone = state_clone.clone();
                        let cancel_token = cancel_token_srv.clone();
                        handle_request(req, state_clone, cancel_token)
                    });

                    tokio::task::spawn(async move {
                        let cancel_token = cancel_token_conn.clone();
                        if let Err(err) = http1::Builder::new().serve_connection(io, service).await {
                            error!("Failed to serve connection: {:?}", err);
                            cancel_token.cancel();
                        }
                    });
                },
                _ = cancel_token.cancelled() => {
                    info!("Stopping webserver");
                    break;
                },
                _ = tokio::time::sleep(webserver_timeout) => {
                    warn!("Timeout of {} sec waiting for authentication expired. Shutting down", webserver_timeout.as_secs());
                    cancel_token.cancel();
                }
            }
        }
        Ok(())
    }

    #[tokio::test]
    async fn test_callback() {
        let listener = TcpListener::bind("127.0.0.1:0")
            .await
            .expect("port available");
        let addr = listener.local_addr().expect("listener address");
        let cancel_token = CancellationToken::new();

        tokio::spawn({
            let cancel_token = cancel_token.clone();
            async move {
                if let Ok(()) = signal::ctrl_c().await {
                    cancel_token.cancel();
                }
            }
        });

        let state = Arc::new(Mutex::new(None));
        let websso_handle = tokio::spawn({
            let cancel_token = cancel_token.clone();
            let state = state.clone();
            async move { websso_callback_server_test(listener, state, cancel_token).await }
        });

        let params = [("token", "foo_bar_baz")];
        let client = reqwest::Client::new();
        client
            .post(format!("http://localhost:{}/callback", addr.port()))
            .form(&params)
            .send()
            .await
            .unwrap();

        websso_handle.await.unwrap().unwrap();
        assert_eq!(*state.lock().unwrap(), Some(params[0].1.into()));
    }

    #[tokio::test]
    async fn test_callback_no_token() {
        let listener = TcpListener::bind("127.0.0.1:0")
            .await
            .expect("port available");
        let addr = listener.local_addr().expect("listener address");
        let cancel_token = CancellationToken::new();

        tokio::spawn({
            let cancel_token = cancel_token.clone();
            async move {
                if let Ok(()) = signal::ctrl_c().await {
                    cancel_token.cancel();
                }
            }
        });

        let state = Arc::new(Mutex::new(None));
        let websso_handle = tokio::spawn({
            let cancel_token = cancel_token.clone();
            let state = state.clone();
            async move { websso_callback_server_test(listener, state, cancel_token).await }
        });

        let client = reqwest::Client::new();
        client
            .post(format!("http://localhost:{}/callback", addr.port()))
            .send()
            .await
            .unwrap();

        websso_handle.await.unwrap().unwrap();
        assert_eq!(*state.lock().unwrap(), None);
    }
}
