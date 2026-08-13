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

//! Shared browser-based SSO callback host service.
//!
//! Both the native WebSSO auth plugin (`openstack-sdk-auth-websso`) and WASM
//! SSO auth plugins (the SSO ABI flavor of
//! `openstack_sdk_plugin_wasm::plugin::WasmAuthPlugin`) need the same three
//! host-controlled primitives:
//!
//! - bind a local callback listener and hand out its URL,
//! - generate and validate an anti-CSRF `state` token embedded in that URL,
//! - open the user's browser, optionally enforcing `https://`.
//!
//! Centralizing them here means the security-sensitive parts — the CSRF
//! check and the callback listener itself — are implemented once. A WASM
//! guest never gets a socket or a browser-opening capability of its own: it
//! only ever sees the already-bound callback URL as an input string and
//! hands back already-received callback data as an input string, both pure
//! JSON round trips through the host.

use std::collections::HashMap;
use std::convert::Infallible;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use bytes::Bytes;
use http_body_util::{BodyExt, Empty, Full, combinators::BoxBody};
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{Method, Request, Response, StatusCode, body::Incoming as IncomingBody};
use hyper_util::rt::TokioIo;
use ring::rand::{SecureRandom, SystemRandom};
use thiserror::Error;
use tokio::net::TcpListener;
use tracing::{info, warn};
use url::Url;

const CALLBACK_PATH: &str = "/callback";
const STATE_PARAM: &str = "state";
const CALLBACK_PAGE: &str = include_str!("../static/callback.html");
/// Number of random bytes used for the anti-CSRF `state` token (256 bits).
const STATE_BYTES: usize = 32;
/// Number of random bytes used for the PKCE `code_verifier` (256 bits,
/// base64url-no-pad-encodes to 43 characters — within RFC 7636's
/// 43-128 character range).
const CODE_VERIFIER_BYTES: usize = 32;
/// Number of random bytes used for the OIDC `nonce` (128 bits, hex-encodes
/// to a 32-character string).
const NONCE_BYTES: usize = 16;

/// Errors from the shared WebSSO/SSO host service.
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum WebssoHostError {
    /// Building an HTTP response failed.
    #[error("http server error: {}", source)]
    Http {
        /// The error source.
        #[from]
        source: http::Error,
    },

    /// Hyper server error.
    #[error("hyper (http server) error: {}", source)]
    Hyper {
        /// The error source.
        #[from]
        source: hyper::Error,
    },

    /// IO error binding the listener or opening the browser.
    #[error("`IO` error: {}", source)]
    Io {
        /// The error source.
        #[from]
        source: std::io::Error,
    },

    /// The URL a caller tried to open required `https://` and didn't have
    /// it.
    #[error("refusing to open a non-https URL (scheme was `{scheme}`)")]
    InsecureScheme {
        /// The scheme the URL actually had.
        scheme: String,
    },

    /// No callback (matching or not) arrived before the deadline.
    #[error("timed out after {:?} waiting for the SSO callback", .0)]
    Timeout(Duration),

    /// The wait was cancelled (e.g. Ctrl-C).
    #[error("SSO callback wait was cancelled")]
    Cancelled,

    /// The secure random generator failed.
    #[error("failed to generate a secure random value")]
    Random,

    /// Internal lock was poisoned.
    #[error("internal error: poisoned lock: {}", context)]
    PoisonedLock {
        /// Context describing which lock was poisoned.
        context: String,
    },

    /// Building the callback URL from the bound listener's address failed.
    #[error("failed to build the callback URL: {}", source)]
    UrlParse {
        /// The error source.
        #[from]
        source: url::ParseError,
    },
}

/// Whether [`open_browser`] should refuse to open a non-`https://` URL.
///
/// The native Keystone WebSSO flow may reasonably point at a plain `http://`
/// identity endpoint in a local/dev deployment, so it keeps this `false`
/// (unchanged, pre-existing behavior). WASM SSO plugins have no such
/// grandfathered use case and MUST set this `true`, per the SSO ABI's
/// fail-closed design.
#[derive(Clone, Copy, Debug, Default)]
pub struct BrowserOpenPolicy {
    /// Require the URL's scheme to be `https`.
    pub require_https: bool,
}

/// Open `url` in the user's default browser.
///
/// Refuses with [`WebssoHostError::InsecureScheme`] if `policy.require_https`
/// is set and `url`'s scheme isn't `https`. This check happens before any
/// attempt to actually launch a browser.
pub fn open_browser(url: &Url, policy: BrowserOpenPolicy) -> Result<(), WebssoHostError> {
    if policy.require_https && url.scheme() != "https" {
        return Err(WebssoHostError::InsecureScheme {
            scheme: url.scheme().to_string(),
        });
    }
    info!("Opening browser at {:?}", url.as_str());
    open::that(url.as_str())?;
    Ok(())
}

/// A bound local callback listener with a fresh anti-CSRF `state` token,
/// a fresh PKCE (RFC 7636) pair, and a fresh OIDC `nonce` already generated.
pub struct CallbackServer {
    listener: TcpListener,
    state: String,
    callback_url: Url,
    code_verifier: String,
    code_challenge: String,
    nonce: String,
}

impl CallbackServer {
    /// Bind a local callback listener on `port` (or an OS-assigned ephemeral
    /// port if `None`), generating a fresh `state` token (embedded in the
    /// returned callback URL's query string), a fresh PKCE pair (returned
    /// via [`Self::code_challenge`]/[`Self::code_verifier`], not embedded in
    /// the URL — callers thread these into the SSO ABI's request JSON
    /// instead), and a fresh OIDC `nonce` (returned via [`Self::nonce`]).
    pub async fn bind(port: Option<u16>) -> Result<Self, WebssoHostError> {
        let listener = TcpListener::bind(("127.0.0.1", port.unwrap_or(0))).await?;
        let addr = listener.local_addr()?;
        let state = generate_state()?;
        let (code_verifier, code_challenge) = generate_pkce_pair()?;
        let nonce = generate_nonce()?;
        let mut callback_url = Url::parse(&format!("http://{addr}{CALLBACK_PATH}"))?;
        callback_url
            .query_pairs_mut()
            .append_pair(STATE_PARAM, &state);
        Ok(Self {
            listener,
            state,
            callback_url,
            code_verifier,
            code_challenge,
            nonce,
        })
    }

    /// The full callback URL, including the embedded `state` token, that a
    /// caller should direct the identity provider (or SSO plugin) to POST
    /// back to.
    pub fn callback_url(&self) -> &Url {
        &self.callback_url
    }

    /// The `host:port` authority of the callback URL — what a plugin's
    /// self-declared redirect target must match.
    pub fn redirect_host(&self) -> String {
        match self.callback_url.port() {
            Some(port) => format!("{}:{port}", self.callback_url.host_str().unwrap_or("")),
            None => self.callback_url.host_str().unwrap_or("").to_string(),
        }
    }

    /// The PKCE `code_challenge` for this callback's authorization request
    /// (`base64url-no-pad(SHA256(code_verifier))`). Safe to read multiple
    /// times — unlike [`Self::wait_for_callback`], which consumes `self`.
    pub fn code_challenge(&self) -> &str {
        &self.code_challenge
    }

    /// The PKCE `code_verifier` for this callback's token exchange. Safe to
    /// read multiple times; callers needing it after [`Self::wait_for_callback`]
    /// consumes `self` must capture an owned copy first.
    pub fn code_verifier(&self) -> &str {
        &self.code_verifier
    }

    /// The OIDC `nonce` for this callback's authorization request. Not
    /// secret — a guest embeds it directly in the browser-visible authorize
    /// URL — so, like [`Self::code_challenge`], safe to read multiple
    /// times.
    pub fn nonce(&self) -> &str {
        &self.nonce
    }

    /// Wait (up to `timeout`, cancellable with Ctrl-C) for a single POST to
    /// the callback URL whose `state` parameter matches the one embedded in
    /// [`Self::callback_url`].
    ///
    /// The `state` token is read from the callback URL's own query string
    /// (not the POST body — the identity provider/plugin only ever POSTs
    /// back to the exact callback URL it was given, so the query string
    /// round-trips unchanged regardless of what body fields it sends). Any
    /// request with a missing or mismatched `state` is rejected with `403`
    /// and does **not** satisfy the wait — a forged callback (state
    /// omitted, guessed, or replayed from a previous run) can never
    /// complete the flow; the server keeps waiting for the real one until
    /// the timeout. Returns every form-encoded parameter from the accepted
    /// request's POST body.
    pub async fn wait_for_callback(
        self,
        timeout: Duration,
    ) -> Result<HashMap<String, String>, WebssoHostError> {
        tokio::select! {
            res = self.wait_for_callback_inner(timeout) => res,
            _ = tokio::signal::ctrl_c() => Err(WebssoHostError::Cancelled),
        }
    }

    async fn wait_for_callback_inner(
        self,
        timeout: Duration,
    ) -> Result<HashMap<String, String>, WebssoHostError> {
        let Self {
            listener, state, ..
        } = self;
        let result: Arc<Mutex<Option<HashMap<String, String>>>> = Arc::new(Mutex::new(None));

        loop {
            tokio::select! {
                accepted = listener.accept() => {
                    let (stream, _addr) = accepted?;
                    let io = TokioIo::new(stream);
                    let state = state.clone();
                    let conn_result = result.clone();
                    let service = service_fn(move |req| {
                        handle_callback(req, state.clone(), conn_result.clone())
                    });
                    // Single-shot server: force `Connection: close` so
                    // `serve_connection` returns as soon as the response is
                    // sent, instead of idling on a keep-alive connection
                    // until the client's pool eventually times it out.
                    if let Err(err) = http1::Builder::new()
                        .keep_alive(false)
                        .serve_connection(io, service)
                        .await
                    {
                        warn!("failed to serve SSO callback connection: {err:?}");
                    }
                    if result.lock().map_err(|_| WebssoHostError::PoisonedLock {
                        context: "SSO callback result".into(),
                    })?.is_some() {
                        break;
                    }
                }
                _ = tokio::time::sleep(timeout) => {
                    return Err(WebssoHostError::Timeout(timeout));
                }
            }
        }

        let guard = result.lock().map_err(|_| WebssoHostError::PoisonedLock {
            context: "SSO callback result".into(),
        })?;
        guard
            .clone()
            .ok_or(WebssoHostError::Timeout(Duration::default()))
    }
}

async fn handle_callback(
    req: Request<IncomingBody>,
    expected_state: String,
    result: Arc<Mutex<Option<HashMap<String, String>>>>,
) -> Result<Response<BoxBody<Bytes, Infallible>>, WebssoHostError> {
    match (req.method(), req.uri().path()) {
        (&Method::POST, CALLBACK_PATH) => {
            // The `state` token was embedded in the callback URL's query
            // string (see `CallbackServer::bind`), not the POST body: the
            // identity provider (or SSO plugin) is only ever told to POST
            // back to that exact URL, so the query string round-trips
            // unchanged regardless of what body fields the provider sends.
            let received_state = req.uri().query().and_then(|q| {
                form_urlencoded::parse(q.as_bytes())
                    .find(|(k, _)| k == STATE_PARAM)
                    .map(|(_, v)| v.into_owned())
            });
            let b = req.collect().await?.to_bytes();
            let params: HashMap<String, String> =
                form_urlencoded::parse(b.as_ref()).into_owned().collect();
            let state_ok = received_state
                .as_deref()
                .map(|s| constant_time_eq(s.as_bytes(), expected_state.as_bytes()))
                .unwrap_or(false);
            if !state_ok {
                warn!(
                    "rejected SSO callback with a missing/invalid `state` parameter (possible forged or replayed callback)"
                );
                return Ok(Response::builder()
                    .status(StatusCode::FORBIDDEN)
                    .body(Empty::<Bytes>::new().boxed())?);
            }

            let mut guard = result.lock().map_err(|_| WebssoHostError::PoisonedLock {
                context: "SSO callback result".into(),
            })?;
            *guard = Some(params);
            drop(guard);

            Ok(Response::builder().body(Full::new(Bytes::from(CALLBACK_PAGE)).boxed())?)
        }
        _ => Ok(Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(Empty::<Bytes>::new().boxed())?),
    }
}

fn generate_state() -> Result<String, WebssoHostError> {
    let rng = SystemRandom::new();
    let mut bytes = [0u8; STATE_BYTES];
    rng.fill(&mut bytes).map_err(|_| WebssoHostError::Random)?;
    Ok(bytes.iter().map(|b| format!("{b:02x}")).collect())
}

/// Generate a fresh OIDC `nonce`: 16 random bytes, hex-encoded to a
/// 32-character string. Mirrors [`generate_state`] exactly — same RNG, same
/// hex-encoding approach — just a different byte count and purpose.
fn generate_nonce() -> Result<String, WebssoHostError> {
    let rng = SystemRandom::new();
    let mut bytes = [0u8; NONCE_BYTES];
    rng.fill(&mut bytes).map_err(|_| WebssoHostError::Random)?;
    Ok(bytes.iter().map(|b| format!("{b:02x}")).collect())
}

/// Generate a fresh RFC 7636 PKCE pair: a `code_verifier` (43 random
/// base64url-no-pad characters, drawn from `[A-Za-z0-9-._~]` as required by
/// the RFC) and its `code_challenge` (`base64url-no-pad(SHA256(verifier))`).
/// Always uses the `S256` challenge method — the weaker `plain` method
/// isn't offered.
fn generate_pkce_pair() -> Result<(String, String), WebssoHostError> {
    use base64::Engine as _;

    let rng = SystemRandom::new();
    let mut bytes = [0u8; CODE_VERIFIER_BYTES];
    rng.fill(&mut bytes).map_err(|_| WebssoHostError::Random)?;
    let code_verifier = base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(bytes);

    let digest = ring::digest::digest(&ring::digest::SHA256, code_verifier.as_bytes());
    let code_challenge = base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(digest.as_ref());

    Ok((code_verifier, code_challenge))
}

/// Constant-time byte comparison, used for the `state` check so a mismatch
/// can't be timed to leak how many leading bytes matched.
fn constant_time_eq(a: &[u8], b: &[u8]) -> bool {
    if a.len() != b.len() {
        return false;
    }
    let mut diff = 0u8;
    for (x, y) in a.iter().zip(b.iter()) {
        diff |= x ^ y;
    }
    diff == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn accepts_matching_state_and_strips_it() {
        let server = CallbackServer::bind(None).await.expect("bind");
        let callback_url = server.callback_url().clone();
        assert_eq!(callback_url.path(), CALLBACK_PATH);
        let redirect_host = server.redirect_host();
        assert!(!redirect_host.is_empty());

        let wait = tokio::spawn(server.wait_for_callback(Duration::from_secs(5)));

        let client = reqwest::Client::new();
        let resp = client
            .post(callback_url.as_str())
            .form(&[("token", "secret-token"), ("extra", "1")])
            .send()
            .await
            .expect("post callback");
        assert!(resp.status().is_success());

        let params = wait.await.expect("join").expect("wait_for_callback");
        assert_eq!(
            params.get("token").map(String::as_str),
            Some("secret-token")
        );
        assert_eq!(params.get("extra").map(String::as_str), Some("1"));
        assert!(!params.contains_key(STATE_PARAM));
    }

    #[tokio::test]
    async fn rejects_forged_state_then_accepts_the_real_callback() {
        let server = CallbackServer::bind(None).await.expect("bind");
        let mut callback_url = server.callback_url().clone();
        let real_query = callback_url.query().unwrap_or("").to_string();

        let wait = tokio::spawn(server.wait_for_callback(Duration::from_secs(5)));

        // A forged callback with a guessed/omitted state must not satisfy
        // the wait.
        callback_url.set_query(Some("state=forged-state-value"));
        let client = reqwest::Client::new();
        let forged_resp = client
            .post(callback_url.as_str())
            .form(&[("token", "attacker-token")])
            .send()
            .await
            .expect("post forged callback");
        assert_eq!(forged_resp.status(), reqwest::StatusCode::FORBIDDEN);

        // The real callback, with the correct state, does.
        callback_url.set_query(Some(&real_query));
        let real_resp = client
            .post(callback_url.as_str())
            .form(&[("token", "real-token")])
            .send()
            .await
            .expect("post real callback");
        assert!(real_resp.status().is_success());

        let params = wait.await.expect("join").expect("wait_for_callback");
        assert_eq!(params.get("token").map(String::as_str), Some("real-token"));
    }

    #[tokio::test]
    async fn times_out_when_nothing_arrives() {
        let server = CallbackServer::bind(None).await.expect("bind");
        let err = server
            .wait_for_callback(Duration::from_millis(50))
            .await
            .expect_err("should time out");
        assert!(matches!(err, WebssoHostError::Timeout(_)));
    }

    #[test]
    fn open_browser_refuses_non_https_when_required() {
        let url = Url::parse("http://example.com").expect("valid url");
        let err = open_browser(
            &url,
            BrowserOpenPolicy {
                require_https: true,
            },
        )
        .expect_err("should refuse http");
        assert!(matches!(err, WebssoHostError::InsecureScheme { .. }));
    }

    #[test]
    fn constant_time_eq_matches_and_rejects() {
        assert!(constant_time_eq(b"abc", b"abc"));
        assert!(!constant_time_eq(b"abc", b"abd"));
        assert!(!constant_time_eq(b"abc", b"ab"));
    }

    #[test]
    fn pkce_pair_has_correct_shape() {
        let (verifier, challenge) = generate_pkce_pair().expect("generate pkce pair");
        assert_eq!(verifier.len(), 43);
        assert!(
            verifier
                .chars()
                .all(|c| c.is_ascii_alphanumeric() || matches!(c, '-' | '.' | '_' | '~')),
            "verifier contained a character outside RFC 7636's unreserved set: {verifier:?}"
        );

        let expected_challenge = {
            use base64::Engine as _;
            let digest = ring::digest::digest(&ring::digest::SHA256, verifier.as_bytes());
            base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(digest.as_ref())
        };
        assert_eq!(challenge, expected_challenge);
    }

    #[test]
    fn pkce_pair_verifiers_are_unique_across_calls() {
        let (verifier_a, _) = generate_pkce_pair().expect("generate pkce pair");
        let (verifier_b, _) = generate_pkce_pair().expect("generate pkce pair");
        assert_ne!(verifier_a, verifier_b);
    }

    #[tokio::test]
    async fn bind_exposes_code_challenge_and_verifier() {
        let server = CallbackServer::bind(None).await.expect("bind");
        assert_eq!(server.code_verifier().len(), 43);
        assert!(!server.code_challenge().is_empty());
        assert_ne!(server.code_verifier(), server.code_challenge());
    }

    #[tokio::test]
    async fn bind_exposes_nonce() {
        let server = CallbackServer::bind(None).await.expect("bind");
        assert_eq!(server.nonce().len(), 32);
        assert!(server.nonce().chars().all(|c| c.is_ascii_hexdigit()));
    }

    #[test]
    fn nonce_is_32_hex_characters() {
        let nonce = generate_nonce().expect("generate nonce");
        assert_eq!(nonce.len(), 32);
        assert!(nonce.chars().all(|c| c.is_ascii_hexdigit()));
    }

    #[test]
    fn nonce_is_unique_across_calls() {
        let a = generate_nonce().expect("generate nonce");
        let b = generate_nonce().expect("generate nonce");
        assert_ne!(a, b);
    }
}
