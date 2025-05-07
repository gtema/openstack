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

//! Federated (OAUTH2/OIDC) login callback server handling
//!
//! This module implements a tiny WebServer based on the Hyper library. It waits for a
//! /federation/oidc/callback endpoint to be invoked with POST or GET method and a form data
//! containing OAUTH2 authorization code. Once endpoint is invoked the server stops and returns
//! [`FederationAuthCodeCallbackResponse`] structure with the populated token.

use bytes::Bytes;
use derive_builder::Builder;
use dialoguer::Confirm;
use futures::io::Error as IoError;
use http_body_util::{combinators::BoxBody, BodyExt, Empty, Full};
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{body::Incoming as IncomingBody, Method, Request, Response, StatusCode};
use hyper_util::rt::TokioIo;
use serde::Deserialize;
use std::borrow::Cow;
use std::collections::HashMap;
use std::convert::Infallible;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use thiserror::Error;
use tokio::net::TcpListener;
use tokio::signal;
use tokio_util::sync::CancellationToken;
use tracing::{error, info, trace, warn};
use url::Url;

use crate::api::rest_endpoint_prelude::*;
use crate::api::RestEndpoint;
use crate::config;
use crate::types::{ApiVersion, ServiceType};

/// Federation related errors
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum FederationError {
    /// Auth data is missing
    #[error("Auth data is missing")]
    MissingAuthData,

    /// Callback did not returned a token
    #[error("WebSSO callback didn't return a token")]
    CallbackNoToken,

    /// Some failure in the SSO flow
    #[error("WebSSO authentication failed")]
    CallbackFailed,

    /// Federation Auth builder
    #[error("error preparing auth request: {}", source)]
    InitAuthBuilder {
        /// The error source
        #[from]
        source: OauthAuthorizeRequestBuilderError,
    },

    /// Federation Auth builder
    #[error("error preparing auth request: {}", source)]
    OidcCallbackBuilder {
        /// The error source
        #[from]
        source: OauthCallbackRequestBuilderError,
    },

    /// IO communication error
    #[error("`IO` error: {}", source)]
    IO {
        /// The error source
        #[from]
        source: IoError,
    },

    /// Thread join error
    #[error("`Join` error: {}", source)]
    Join {
        /// The error source
        #[from]
        source: tokio::task::JoinError,
    },
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
    pub code: String,
    /// Authorization state
    pub state: String,
}

/// Endpoint for initializing oauth2 authorization
#[derive(Builder, Debug, Clone)]
#[builder(setter(strip_option))]
pub struct OauthAuthorizeRequest<'a> {
    /// idp_id parameter for
    #[builder(setter(into))]
    idp_id: Cow<'a, str>,

    #[builder(default, setter(into))]
    mapping_id: Option<Cow<'a, str>>,

    #[builder(setter(into))]
    redirect_uri: Cow<'a, str>,
}
impl<'a> OauthAuthorizeRequest<'a> {
    /// Create a builder for the endpoint.
    pub fn builder() -> OauthAuthorizeRequestBuilder<'a> {
        OauthAuthorizeRequestBuilder::default()
    }
}

impl RestEndpoint for OauthAuthorizeRequest<'_> {
    fn method(&self) -> http::Method {
        http::Method::POST
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!(
            "federation/identity_providers/{idp_id}/auth",
            idp_id = self.idp_id.as_ref(),
        )
        .into()
    }

    fn body(&self) -> Result<Option<(&'static str, Vec<u8>)>, BodyError> {
        let mut params = JsonBodyParams::default();

        params.push("redirect_uri", &self.redirect_uri);
        params.push_opt("mapping_id", self.mapping_id.as_ref());

        params.into_body()
    }

    fn service_type(&self) -> ServiceType {
        ServiceType::Identity
    }

    /// Returns required API version
    fn api_version(&self) -> Option<ApiVersion> {
        Some(ApiVersion::new(3, 0))
    }
}

/// Endpoint for finishing oauth2 authorization (callback with auth code)
#[derive(Builder, Debug, Clone)]
#[builder(setter(strip_option))]
pub struct OauthCallbackRequest<'a> {
    /// code parameter
    #[builder(setter(into))]
    code: Cow<'a, str>,

    /// state parameter
    #[builder(setter(into))]
    state: Cow<'a, str>,
}
impl<'a> OauthCallbackRequest<'a> {
    /// Create a builder for the endpoint.
    pub fn builder() -> OauthCallbackRequestBuilder<'a> {
        OauthCallbackRequestBuilder::default()
    }
}

impl RestEndpoint for OauthCallbackRequest<'_> {
    fn method(&self) -> http::Method {
        http::Method::POST
    }

    fn endpoint(&self) -> Cow<'static, str> {
        "federation/oidc/callback".to_string().into()
    }

    fn body(&self) -> Result<Option<(&'static str, Vec<u8>)>, BodyError> {
        let mut params = JsonBodyParams::default();
        params.push("code", &self.code);
        params.push("state", &self.state);

        params.into_body()
    }

    fn service_type(&self) -> ServiceType {
        ServiceType::Identity
    }

    fn response_key(&self) -> Option<Cow<'static, str>> {
        Some("token".into())
    }

    /// Returns required API version
    fn api_version(&self) -> Option<ApiVersion> {
        Some(ApiVersion::new(3, 0))
    }
}

/// Get [`RestEndpoint`] for initializing the OIDC authentication
pub fn get_auth_ep(
    config: &config::CloudConfig,
    callback_port: u16,
) -> Result<impl RestEndpoint, FederationError> {
    if let Some(auth) = &config.auth {
        if let Some(identity_provider) = &auth.identity_provider {
            let mut ep = OauthAuthorizeRequest::builder();
            ep.idp_id(identity_provider.clone());
            ep.redirect_uri(format!("http://localhost:{}/oidc/callback", callback_port));
            return Ok(ep.build()?);
        }
    }
    Err(FederationError::MissingAuthData)
}

/// Perform authorization request by opening a browser window with tiny webserver started to
/// capture the callback and return [`FederationAuthCodeCallbackResponse`]
///
/// - start callback server
/// - open browser pointing to the IDP authorization url
/// - wait for the response with the OpenIDC authorization code
pub async fn get_auth_code(
    url: &Url,
    socket_addr: SocketAddr,
) -> Result<FederationAuthCodeCallbackResponse, FederationError> {
    let confirmation = Confirm::new()
        .with_prompt(format!(
            "A default browser is going to be opened at `{}`. Do you want to continue?",
            url.as_str()
        ))
        .interact()
        .unwrap();
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

        let guard = state.lock().expect("poisoned guard");
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
) -> Result<Response<BoxBody<Bytes, Infallible>>, hyper::Error> {
    println!("Got request {:?}, {:?}", req.method(), req.uri());
    match (req.method(), req.uri().path()) {
        (&Method::GET, "/oidc/callback") => {
            if let Some(query) = req.uri().query() {
                let params = form_urlencoded::parse(query.as_bytes())
                    .into_owned()
                    .collect::<HashMap<String, String>>();
                trace!("Params = {:?}", params);

                let mut data = state.lock().expect("state lock can be obtained");
                if let (Some(code), Some(state)) = (params.get("code"), params.get("state")) {
                    *data = Some(FederationAuthCodeCallbackResponse {
                        code: code.clone(),
                        state: state.clone(),
                    });
                }
                cancel_token.cancel();

                Ok(Response::builder()
                    .body(
                        Full::new(Bytes::from(include_str!("../../static/callback.html"))).boxed(),
                    )
                    .unwrap())
            } else {
                Ok(Response::builder()
                    .status(StatusCode::NOT_FOUND)
                    .body(Empty::<Bytes>::new().boxed())
                    .unwrap())
            }
        }
        (&Method::POST, "/oidc/callback") => {
            let b = req.collect().await?.to_bytes();
            trace!("Body is {:?}", b);
            let params = form_urlencoded::parse(b.as_ref())
                .into_owned()
                .collect::<HashMap<String, String>>();
            trace!("Params = {:?}", params);

            let mut data = state.lock().expect("state lock can be obtained");
            if let (Some(code), Some(state)) = (params.get("code"), params.get("state")) {
                *data = Some(FederationAuthCodeCallbackResponse {
                    code: code.clone(),
                    state: state.clone(),
                });
            }
            cancel_token.cancel();

            Ok(Response::builder()
                .body(Full::new(Bytes::from(include_str!("../../static/callback.html"))).boxed())
                .unwrap())
        }
        _ => {
            // Return 404 not found response.
            Ok(Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(Empty::<Bytes>::new().boxed())
                .unwrap())
        }
    }
}

#[cfg(test)]
mod tests {
    use reserve_port::ReservedSocketAddr;
    use std::sync::{Arc, Mutex};
    use tokio::signal;
    use tokio_util::sync::CancellationToken;

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
                code: "foo".to_string(),
                state: "bar".to_string()
            })
        );
    }

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
                code: "foo".to_string(),
                state: "bar".to_string()
            })
        );
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
