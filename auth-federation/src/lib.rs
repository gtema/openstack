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

//! # Federated (OAUTH2/OIDC) login callback server handling
//!
//! This crate implements a tiny WebServer based on the Hyper library. It waits for a
//! /federation/oidc/callback endpoint to be invoked with POST or GET method and a form data
//! containing OAUTH2 authorization code. Once endpoint is invoked the server stops and returns
//! [`FederationAuthCodeCallbackResponse`] structure with the populated token.

use std::collections::HashMap;
use std::convert::Infallible;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use async_trait::async_trait;
use bytes::Bytes;
use dialoguer::Confirm;
use futures::io::Error as IoError;
use http_body_util::{BodyExt, Empty, Full, combinators::BoxBody};
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{Method, Request, Response, StatusCode, body::Incoming as IncomingBody};
use hyper_util::rt::TokioIo;
use secrecy::{ExposeSecret, SecretString};
use serde::Deserialize;
use serde_json::{Value, json};
use serde_urlencoded;
use thiserror::Error;
use tokio::net::TcpListener;
use tokio::signal;
use tokio_util::sync::CancellationToken;
use tracing::{Level, enabled, error, info, trace, warn};
use url::Url;

use openstack_sdk_auth_core::{
    Auth, AuthError, AuthPluginRegistration, AuthToken, OpenStackAuthType,
};

/// V4 OIDC Authentication for OpenStack SDK.
pub struct OidcAuthenticator;

// Submit the plugin to the registry at compile-time
static PLUGIN: OidcAuthenticator = OidcAuthenticator;
inventory::submit! {
    AuthPluginRegistration { method: &PLUGIN }
}

#[async_trait]
impl OpenStackAuthType for OidcAuthenticator {
    fn get_supported_auth_methods(&self) -> Vec<&'static str> {
        vec!["v4federation", "federation"]
    }

    fn requirements(&self) -> Value {
        json!({
            "type": "object",
            "required": ["identity_provider"],
            "properties": {
                "identity_provider": {
                    "type": "string",
                    "description": "Identity Provider ID"
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
        // Construct request for initializing authentication (POST call to keystone
        // `/federation/identity_providers/{idp_id}/auth`) to get the IDP url
        // client would need to contact.
        // TODO: If we know the scope we can request it from the very beginning
        // saving 1 call.
        let callback_addr = std::net::SocketAddr::from(([127, 0, 0, 1], 8050));

        let idp_id = values
            .get("identity_provider")
            .ok_or(FederationError::MissingAuthData)?
            .expose_secret();
        let auth_start = identity_url
            .join(
                format!(
                    "federation/identity_providers/{idp_id}/auth",
                    idp_id = idp_id,
                )
                .as_str(),
            )
            .map_err(FederationError::from)?;
        let mut body = json!({"redirect_uri": format!("http://localhost:{}/oidc/callback", callback_addr.port())});

        // TODO: check this
        if let Some(scope) = values.get("scope") {
            body["scope"] = scope.expose_secret().into();
        }

        let auth_info = http_client
            .post(auth_start)
            .json(&body)
            .send()
            .await
            .map_err(FederationError::from)?
            .json::<FederationAuthRequestResponse>()
            .await
            .map_err(FederationError::from)?;

        // Perform the magic directing user's browser at the IDP url and waiting
        // for the callback to be invoked with the authorization code
        let oauth2_code = get_auth_code(&auth_info.auth_url, callback_addr).await?;

        // Construct the request to Keystone to finish the authorization exchanging
        // received authorization code for the (unscoped) token
        //
        let auth_finish = identity_url
            .join("federation/oidc/callback")
            .map_err(FederationError::from)?;

        if let (Some(code), Some(state)) = (oauth2_code.code, oauth2_code.state) {
            let response = http_client
                .post(auth_finish)
                .json(&json!({"code": code, "state": state}))
                .send()
                .await
                .map_err(FederationError::from)?;

            let auth_token = AuthToken::from_reqwest_response(response).await?;

            Ok(Auth::AuthToken(Box::new(auth_token)))
        } else {
            return Err(AuthError::AuthTokenNotInResponse);
        }
    }
}

/// Federation related errors
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum FederationError {
    /// Auth data is missing
    #[error("auth data is missing")]
    MissingAuthData,

    /// Callback did not returned a token
    #[error("federation callback didn't return a token")]
    CallbackNoToken,

    /// Some failure in the SSO flow
    #[error("federation authentication failed")]
    CallbackFailed,

    /// IO communication error
    #[error("`IO` error: {}", source)]
    IO {
        /// The error source
        #[from]
        source: IoError,
    },

    #[error("failed to URL encode form parameters: {}", source)]
    UrlEncodedDeser {
        /// The source of the error.
        #[from]
        source: serde_urlencoded::de::Error,
    },

    /// Http error.
    #[error("http server error: {}", source)]
    Http {
        /// The source of the error.
        #[from]
        source: http::Error,
    },
    #[error("hyper error: {}", source)]
    Hyper {
        /// The source of the error.
        #[from]
        source: hyper::Error,
    },

    /// Thread join error
    #[error("`Join` error: {}", source)]
    Join {
        /// The error source
        #[from]
        source: tokio::task::JoinError,
    },

    /// Dialoguer error.
    #[error("error reading the user input: {}", source)]
    Dialoguer {
        /// The source of the error
        #[from]
        source: dialoguer::Error,
    },

    /// Reqwest error.
    #[error(transparent)]
    Reqwest {
        /// The source of the error.
        #[from]
        source: reqwest::Error,
    },
    /// Poisoned guard lock in the internal processing.
    #[error("internal error: poisoned lock: {}", context)]
    PoisonedLock {
        /// The source of the error.
        context: String,
    },
    /// Url error.
    #[error(transparent)]
    Url {
        /// The source of the error.
        #[from]
        source: url::ParseError,
    },
}

impl From<FederationError> for AuthError {
    fn from(source: FederationError) -> Self {
        Self::plugin(source)
    }
}

/// OAUTH2 Authentication request information
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct FederationAuthRequestResponse {
    /// Authentication URL the client should open in the browser
    pub auth_url: Url,
}

/// Information for finishing the authorization request (received as a callback from `/authorize`
/// call)
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct FederationAuthCodeCallbackResponse {
    /// Authorization code
    pub code: Option<String>,
    /// Authorization state
    pub state: Option<String>,
    /// IDP error
    pub error: Option<String>,
    /// IDP error description
    pub error_description: Option<String>,
}

/// Perform authorization request by opening a browser window with tiny webserver started to
/// capture the callback and return [`FederationAuthCodeCallbackResponse`]
///
/// - start callback server
/// - open browser pointing to the IDP authorization url
/// - wait for the response with the OpenIDC authorization code
async fn get_auth_code(
    url: &Url,
    socket_addr: SocketAddr,
) -> Result<FederationAuthCodeCallbackResponse, FederationError> {
    let confirmation = Confirm::new()
        .with_prompt(format!(
            "A default browser is going to be opened at `{}`. Do you want to continue?",
            url.as_str()
        ))
        .interact()?;
    if confirmation {
        info!("Opening browser at {:?}", url.as_str());
        let cancel_token = CancellationToken::new();
        let state: Arc<Mutex<Option<FederationAuthCodeCallbackResponse>>> =
            Arc::new(Mutex::new(None));

        tokio::spawn({
            let cancel_token = cancel_token.clone();
            async move {
                if let Ok(()) = signal::ctrl_c().await {
                    info!("received Ctrl-C, shutting down");
                    cancel_token.cancel();
                }
            }
        });

        let handle = tokio::spawn({
            let cancel_token = cancel_token.clone();
            let state = state.clone();
            async move { auth_callback_server(socket_addr, state, cancel_token).await }
        });
        open::that(url.as_str())?;

        let _res = handle.await?;

        let guard = state.lock().map_err(|_| FederationError::PoisonedLock {
            context: "getting auth_code guard lock".to_string(),
        })?;
        guard.clone().ok_or(FederationError::CallbackNoToken)
    } else {
        Err(FederationError::CallbackFailed)
    }
}

/// Start the OAUTH2 callback server
async fn auth_callback_server(
    addr: SocketAddr,
    state: Arc<Mutex<Option<FederationAuthCodeCallbackResponse>>>,
    cancel_token: CancellationToken,
) -> Result<(), FederationError> {
    let listener = TcpListener::bind(addr).await?;
    info!("Starting webserver to receive OAUTH2 authorization callback");
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
    state: Arc<Mutex<Option<FederationAuthCodeCallbackResponse>>>,
    cancel_token: CancellationToken,
) -> Result<Response<BoxBody<Bytes, Infallible>>, FederationError> {
    match (req.method(), req.uri().path()) {
        (&Method::GET, "/oidc/callback") => {
            if let Some(query) = req.uri().query() {
                if enabled!(Level::TRACE) {
                    let params = form_urlencoded::parse(query.as_bytes())
                        .into_owned()
                        .collect::<HashMap<String, String>>();
                    trace!("Params = {:?}", params);
                }

                let res: FederationAuthCodeCallbackResponse =
                    serde_urlencoded::from_bytes(query.as_bytes())?;

                if let Some(error_description) = res.error_description {
                    return Ok(Response::builder()
                        .status(StatusCode::INTERNAL_SERVER_ERROR)
                        .body(
                            Full::new(
                                format!(
                                    include_str!("../static/callback_error.html"),
                                    error = "Identity Provider returned error",
                                    error_description = error_description
                                )
                                .into(),
                            )
                            .boxed(),
                        )?);
                }
                let mut data = state.lock().map_err(|_| FederationError::PoisonedLock {
                    context: "getting auth_code guard lock in handle_request".to_string(),
                })?;

                *data = Some(res);
                cancel_token.cancel();

                Ok(Response::builder()
                    .body(Full::new(include_str!("../static/callback.html").into()).boxed())?)
            } else {
                Ok(Response::builder()
                    .status(StatusCode::NOT_FOUND)
                    .body(Empty::<Bytes>::new().boxed())?)
            }
        }
        (&Method::POST, "/oidc/callback") => {
            let mut error: Option<String> = None;
            let mut error_description: Option<String> = None;
            if let Some(Ok("application/x-www-form-urlencoded")) =
                req.headers().get("content-type").map(|x| x.to_str())
                && let Ok(body) = req.collect().await
            {
                let b = body.to_bytes();
                trace!("OIDC callback body is {:?}", b);
                if let Ok(res) =
                    serde_urlencoded::from_bytes::<FederationAuthCodeCallbackResponse>(&b)
                {
                    if let Some(error_descr) = res.error_description {
                        error = Some("Identity Provider returned error".into());
                        error_description = Some(error_descr);
                    } else if res.code.is_some() {
                        let mut data = state.lock().map_err(|_| FederationError::PoisonedLock {
                            context: "getting auth_code guard lock in handle_request".to_string(),
                        })?;

                        *data = Some(res);
                        cancel_token.cancel();

                        return Ok(Response::builder().body(
                            Full::new(include_str!("../static/callback.html").into()).boxed(),
                        )?);
                    }
                }
            }
            cancel_token.cancel();
            Ok(Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(
                    Full::new(
                        format!(
                            include_str!("../static/callback_error.html"),
                            error = error.unwrap_or("OIDC callback error".into()),
                            error_description = error_description.unwrap_or("Unsupported callback payload has been received. Cannot complete the authentication request".into())
                        )
                        .into(),
                    )
                    .boxed(),
                )
                ?)
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
    use reserve_port::ReservedSocketAddr;
    use std::sync::{Arc, Mutex};
    use tokio::signal;
    use tokio_util::sync::CancellationToken;
    use tracing_test::traced_test;

    use super::*;

    #[tokio::test]
    async fn test_callback_get() {
        let addr = ReservedSocketAddr::reserve_random_socket_addr()
            .expect("port available")
            .socket_addr();
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
        let handle = tokio::spawn({
            let cancel_token = cancel_token.clone();
            let state = state.clone();
            async move { auth_callback_server(addr, state, cancel_token).await }
        });

        let client = reqwest::Client::new();
        client
            .get(format!(
                "http://localhost:{}/oidc/callback?code=foo&state=bar",
                addr.port()
            ))
            .send()
            .await
            .unwrap();

        handle.await.unwrap().unwrap();
        assert_eq!(
            *state.lock().unwrap(),
            Some(FederationAuthCodeCallbackResponse {
                code: Some("foo".to_string()),
                state: Some("bar".to_string()),
                error: None,
                error_description: None
            })
        );
    }

    #[traced_test]
    #[tokio::test]
    async fn test_callback_post() {
        let addr = ReservedSocketAddr::reserve_random_socket_addr()
            .expect("port available")
            .socket_addr();
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
        let handle = tokio::spawn({
            let cancel_token = cancel_token.clone();
            let state = state.clone();
            async move { auth_callback_server(addr, state, cancel_token).await }
        });

        let params = [("code", "foo"), ("state", "bar")];
        let client = reqwest::Client::new();
        client
            .post(format!("http://localhost:{}/oidc/callback", addr.port()))
            .form(&params)
            .send()
            .await
            .unwrap();

        handle.await.unwrap().unwrap();
        assert_eq!(
            *state.lock().unwrap(),
            Some(FederationAuthCodeCallbackResponse {
                code: Some("foo".to_string()),
                state: Some("bar".to_string()),
                error: None,
                error_description: None
            })
        );
    }

    #[traced_test]
    #[tokio::test]
    async fn test_callback_no_token() {
        let addr = ReservedSocketAddr::reserve_random_socket_addr()
            .expect("port available")
            .socket_addr();
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
        let handle = tokio::spawn({
            let cancel_token = cancel_token.clone();
            let state = state.clone();
            async move { auth_callback_server(addr, state, cancel_token).await }
        });

        let client = reqwest::Client::new();
        client
            .post(format!("http://localhost:{}/oidc/callback", addr.port()))
            .send()
            .await
            .unwrap();

        handle.await.unwrap().unwrap();
        assert_eq!(*state.lock().unwrap(), None);
    }
}
