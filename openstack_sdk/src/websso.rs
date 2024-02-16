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

use std::collections::HashMap;
use std::convert::Infallible;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use tracing::{error, info, trace, warn};

use bytes::Bytes;
use http_body_util::{combinators::BoxBody, BodyExt, Empty, Full};
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{body::Incoming as IncomingBody, Method, Request, Response, StatusCode};
use hyper_util::rt::TokioIo;
use tokio::net::TcpListener;
use tokio_util::sync::CancellationToken;

use crate::OpenStackError;

/// SSOState structure to return data from the WebServer back to the invoker
#[derive(Debug)]
pub(crate) struct SsoState {
    /// Federation token
    pub(crate) token: String,
}

#[derive(Debug)]
struct CallbackServerState {
    token: Option<String>,
    cancel_token: CancellationToken,
}

/// Start the WebSSO callback server
pub(crate) async fn websso_callback_server(
    addr: SocketAddr,
    cancel_token: CancellationToken,
) -> Result<SsoState, OpenStackError> {
    let listener = TcpListener::bind(addr).await?;
    info!("Starting webserver to receive SSO callback");
    let state: Arc<Mutex<CallbackServerState>> = Arc::new(Mutex::new(CallbackServerState {
        token: None,
        cancel_token: cancel_token.clone(),
    }));
    // Wait maximum 2 minute for auth processing
    let webserver_timeout = Duration::from_secs(120);
    loop {
        let state_clone = state.clone();

        tokio::select! {
            Ok((stream, _addr)) = listener.accept() => {
                let io = TokioIo::new(stream);
                let cancel_token = cancel_token.clone();

                let service = service_fn(move |req| {
                    let state_clone = state_clone.clone();
                    handle_request(req, state_clone)
                });

                tokio::task::spawn(async move {
                    let cancel_token = cancel_token.clone();
                    if let Err(err) = http1::Builder::new().serve_connection(io, service).await {
                        error!("Failed to serve connection: {:?}", err);
                        cancel_token.cancel();
                    }
                });
            },
            _ = cancel_token.cancelled() => {
                info!("Stopping webserver");
                return Ok(SsoState { token: state_clone.lock().unwrap().token.clone().ok_or(OpenStackError::WebSSONoToken)?});
            },
            _ = tokio::time::sleep(webserver_timeout) => {
                warn!("Timeout of {} sec waiting for authentication expired. Shutting down", webserver_timeout.as_secs());
                cancel_token.cancel();
            }
        }
    }
}

/// Server request handler function
async fn handle_request(
    req: Request<IncomingBody>,
    state: Arc<Mutex<CallbackServerState>>,
) -> Result<Response<BoxBody<Bytes, Infallible>>, hyper::Error> {
    match (req.method(), req.uri().path()) {
        (&Method::POST, "/callback") => {
            let b = req.collect().await?.to_bytes();
            trace!("Body is {:?}", b);
            let params = form_urlencoded::parse(b.as_ref())
                .into_owned()
                .collect::<HashMap<String, String>>();
            trace!("Params = {:?}", params);

            let mut data = state.lock().unwrap();
            if let Some(token) = params.get("token") {
                data.token = Some(token.clone());
                //data.cancel_token.cancel();
            }
            data.cancel_token.cancel();

            Ok(Response::builder()
                .body(
                    Full::new(Bytes::from(
                        include_str!("../static/callback.html").to_string(),
                    ))
                    .boxed(),
                )
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
    use tokio::signal;
    use tokio_util::sync::CancellationToken;

    use super::websso_callback_server;
    use crate::OpenStackError;

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

        let websso_handle = tokio::spawn({
            let cancel_token = cancel_token.clone();
            async move { websso_callback_server(addr, cancel_token).await }
        });

        let params = [("token", "foo_bar_baz")];
        let client = reqwest::Client::new();
        client
            .post(format!("http://localhost:{}/callback", addr.port()))
            .form(&params)
            .send()
            .await
            .unwrap();

        let res = websso_handle.await.unwrap().unwrap();
        assert_eq!(res.token, params[0].1.to_string());
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

        let websso_handle = tokio::spawn({
            let cancel_token = cancel_token.clone();
            async move { websso_callback_server(addr, cancel_token).await }
        });

        let client = reqwest::Client::new();
        client
            .post(format!("http://localhost:{}/callback", addr.port()))
            .send()
            .await
            .unwrap();

        match websso_handle.await.unwrap().unwrap_err() {
            OpenStackError::WebSSONoToken => {}
            _ => {
                panic!("Unexpected error")
            }
        }
    }
}
