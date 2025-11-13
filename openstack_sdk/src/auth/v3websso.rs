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

//! WebSSO callback server handling
//!
//! This module implements a tiny WebServer based on the Hyper library. It waits for a /callback
//! endpoint to be invoked with POST method and a form data containing OpenStack token. Once
//! endpoint is invoked the server stops and returns `SsoState` structure with the populated token.

use std::borrow::Cow;
use std::collections::HashMap;
use std::convert::Infallible;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use tracing::{error, info, trace, warn};

use thiserror::Error;

use bytes::Bytes;
use futures::io::Error as IoError;
use http_body_util::{combinators::BoxBody, BodyExt, Empty, Full};
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{body::Incoming as IncomingBody, Method, Request, Response, StatusCode};
use hyper_util::rt::TokioIo;
use tokio::net::TcpListener;
use tokio::signal;
use tokio_util::sync::CancellationToken;
use url::Url;

use dialoguer::Confirm;

use crate::api::identity::v3::auth::os_federation::{
    identity_provider::protocol::websso::get as fed_idp_sso_get, websso::get as fed_sso_get,
};
use crate::api::RestEndpoint;
use crate::auth::authtoken::{AuthToken, AuthTokenError};
use crate::config;

/// WebSSO related errors.
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum WebSsoError {
    /// Callback did not returned a token.
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

    /// Federation Auth SSO with IDP and Protocol builder.
    #[error("error preparing auth request: {}", source)]
    FederationIdpSsoAuth {
        /// The error source.
        #[from]
        source: fed_idp_sso_get::RequestBuilderError,
    },

    /// Federation Auth builder.
    #[error("error preparing auth request: {}", source)]
    FederationSsoBuilder {
        /// The error source.
        #[from]
        source: fed_sso_get::RequestBuilderError,
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

/// Get URL as a string for the WebSSO authentication by constructing the [`RestEndpoint`] and
/// returning resulting `endpoint`
pub fn get_auth_url(config: &config::CloudConfig) -> Result<Cow<'static, str>, WebSsoError> {
    if let Some(auth) = &config.auth {
        if let Some(identity_provider) = &auth.identity_provider {
            let mut ep = fed_idp_sso_get::RequestBuilder::default();
            ep.idp_id(identity_provider);
            if let Some(protocol) = &auth.protocol {
                ep.protocol_id(protocol);
            } else {
                return Err(WebSsoError::MissingProtocol);
            }
            return Ok(ep.build()?.endpoint());
        } else {
            let mut ep = fed_sso_get::RequestBuilder::default();
            if let Some(protocol) = &auth.protocol {
                ep.protocol_id(protocol);
            } else {
                return Err(WebSsoError::MissingProtocol);
            }
            return Ok(ep.build()?.endpoint());
        }
    }
    Err(WebSsoError::MissingAuthData)
}

/// Return [`AuthToken`] obtained using the WebSSO (Keystone behind mod_auth_oidc)
pub async fn get_token_auth(url: &mut Url) -> Result<AuthToken, AuthTokenError> {
    let token = get_token(url, None).await?;
    Ok(AuthToken {
        token: token.clone(),
        auth_info: None,
    })
}

// Perform WebSSO by opening a browser window with tiny webserver started to capture the callback
///
/// - start callback server
/// - open browser pointing to the SSO url
/// - wait for the response with the OpenStack token
async fn get_token(url: &mut Url, socket_addr: Option<SocketAddr>) -> Result<String, WebSsoError> {
    url.set_query(Some("origin=http://localhost:8050/callback"));
    let confirmation = Confirm::new()
        .with_prompt(format!(
            "A default browser is going to be opened at `{}`. Do you want to continue?",
            url.as_str()
        ))
        .interact()?;
    if confirmation {
        info!("Opening browser at {:?}", url.as_str());
        let addr = socket_addr.unwrap_or(SocketAddr::from(([127, 0, 0, 1], 8050)));
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
            async move { websso_callback_server(addr, state, cancel_token).await }
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

/// Start the WebSSO callback server
async fn websso_callback_server(
    addr: SocketAddr,
    state: Arc<Mutex<Option<String>>>,
    cancel_token: CancellationToken,
) -> Result<(), WebSsoError> {
    let listener = TcpListener::bind(addr).await?;
    info!("Starting webserver to receive SSO callback");
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
                .body(Full::new(Bytes::from(include_str!("../../static/callback.html"))).boxed())?)
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

    use super::websso_callback_server;

    #[tokio::test]
    async fn test_callback() {
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
        let websso_handle = tokio::spawn({
            let cancel_token = cancel_token.clone();
            let state = state.clone();
            async move { websso_callback_server(addr, state, cancel_token).await }
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
        let websso_handle = tokio::spawn({
            let cancel_token = cancel_token.clone();
            let state = state.clone();
            async move { websso_callback_server(addr, state, cancel_token).await }
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
