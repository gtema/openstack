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

//! The host functions exposed to WASM auth plugins: capability-restricted
//! HTTP clients that may only reach an origin the host itself resolved and
//! bound to this call — never one the plugin picks freely. `identity_http_request`
//! reaches the Identity endpoint `osc` resolved for the configured cloud,
//! available to both ABI flavors. `idp_http_request` reaches the external
//! identity-provider origin an `sso`-flavor plugin's own `sso_build_request`
//! response declared (after the host validates its scheme and SSRF-checks
//! its resolved address), and is only ever bound during that same plugin's
//! `sso_parse_callback` call. Plugins never get a socket, DNS resolver, or
//! unrestricted HTTP client of their own — every outbound request is
//! proxied through here so the host can enforce the destination.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

/// Per-call state made available to the host functions above.
///
/// Populated by [`crate::plugin::WasmAuthPlugin`] immediately before each
/// guest call that may need it, while the plugin's own mutex is held, and
/// torn down (client dropped) right after. Both origins are scheme+host+port
/// only; the guest supplies a request-relative `path` which is joined
/// against one of them, so a plugin can never target a host other than the
/// one the SDK bound for that specific call. `idp_origin` is `None` for
/// every call except `sso_parse_callback`.
#[derive(Clone, Default)]
pub(crate) struct HostContextState {
    pub(crate) identity_origin: Option<url::Url>,
    pub(crate) idp_origin: Option<url::Url>,
    pub(crate) client: Option<reqwest::blocking::Client>,
}

/// Request message a guest sends to `identity_http_request`/`idp_http_request`.
#[derive(Debug, Deserialize)]
struct HttpRequestMsg {
    method: String,
    /// Must be relative (start with `/`); resolved against the bound origin.
    path: String,
    #[serde(default)]
    headers: BTreeMap<String, String>,
    #[serde(default)]
    body: Option<String>,
}

/// Response message returned to the guest.
#[derive(Debug, Serialize)]
struct HttpResponseMsg {
    status: u16,
    headers: BTreeMap<String, String>,
    body: String,
}

/// Validate a guest-supplied request against `origin` and turn it into a
/// concrete URL + method, without performing any I/O. This is the entire
/// part of either host function's handling of untrusted guest bytes that
/// doesn't require a live [`HostContextState`], split out so it can be
/// exercised directly (including by the `fuzzing`-feature entry point below)
/// without a real WASM call or network access. `fn_name` is only used to
/// prefix error messages with the host function that produced them.
fn resolve_request(
    fn_name: &str,
    origin: &url::Url,
    req: &HttpRequestMsg,
) -> Result<(url::Url, reqwest::Method), extism::Error> {
    if !req.path.starts_with('/') {
        return Err(extism::Error::msg(format!(
            "{fn_name}: `path` must be relative to the identity endpoint (start with '/')"
        )));
    }
    let url = origin
        .join(&req.path)
        .map_err(|e| extism::Error::msg(format!("{fn_name}: invalid path: {e}")))?;
    if url.origin() != origin.origin() {
        return Err(extism::Error::msg(format!(
            "{fn_name}: `path` must not change the request origin"
        )));
    }
    let method = reqwest::Method::from_bytes(req.method.as_bytes())
        .map_err(|e| extism::Error::msg(format!("{fn_name}: invalid method: {e}")))?;
    Ok((url, method))
}

/// Send the already-resolved request and shape the response, shared by both
/// host functions so `idp_http_request` doesn't duplicate the proxy logic.
fn send_and_shape_response(
    fn_name: &str,
    url: url::Url,
    method: reqwest::Method,
    client: &reqwest::blocking::Client,
    req: &HttpRequestMsg,
) -> Result<String, extism::Error> {
    let mut builder = client.request(method, url);
    for (k, v) in &req.headers {
        builder = builder.header(k, v);
    }
    if let Some(body) = &req.body {
        builder = builder.body(body.clone());
    }

    let resp = builder
        .send()
        .map_err(|e| extism::Error::msg(format!("{fn_name}: request failed: {e}")))?;
    let status = resp.status().as_u16();
    let headers = resp
        .headers()
        .iter()
        .filter_map(|(k, v)| v.to_str().ok().map(|v| (k.to_string(), v.to_string())))
        .collect();
    let body = resp
        .text()
        .map_err(|e| extism::Error::msg(format!("{fn_name}: reading response body failed: {e}")))?;

    Ok(serde_json::to_string(&HttpResponseMsg {
        status,
        headers,
        body,
    })?)
}

/// Fuzz target entry point for the otherwise-private [`resolve_request`],
/// the part of `identity_http_request` that parses and validates raw bytes
/// a guest module controls (Extism's guest-to-host call boundary), without
/// making any network call.
///
/// Only compiled with the `fuzzing` feature; not part of the stable public
/// API.
#[cfg(feature = "fuzzing")]
pub fn fuzz_identity_http_request_parsing(origin: &url::Url, request: &str) {
    if let Ok(req) = serde_json::from_str::<HttpRequestMsg>(request) {
        let _ = resolve_request("identity_http_request", origin, &req);
    }
}

extism::host_fn!(pub(crate) identity_http_request(ctx: HostContextState; request: String) -> String {
    let req: HttpRequestMsg = serde_json::from_str(&request)
        .map_err(|e| extism::Error::msg(format!("invalid identity_http_request payload: {e}")))?;

    let state = ctx.get()?;
    let state = state
        .lock()
        .map_err(|_| extism::Error::msg("identity_http_request: host context lock poisoned"))?;
    let origin = state
        .identity_origin
        .as_ref()
        .ok_or_else(|| extism::Error::msg("identity_http_request: no identity endpoint bound to this call"))?;
    let client = state
        .client
        .as_ref()
        .ok_or_else(|| extism::Error::msg("identity_http_request: no http client bound to this call"))?;

    let (url, method) = resolve_request("identity_http_request", origin, &req)?;
    send_and_shape_response("identity_http_request", url, method, client, &req)
});

extism::host_fn!(pub(crate) idp_http_request(ctx: HostContextState; request: String) -> String {
    let req: HttpRequestMsg = serde_json::from_str(&request)
        .map_err(|e| extism::Error::msg(format!("invalid idp_http_request payload: {e}")))?;

    let state = ctx.get()?;
    let state = state
        .lock()
        .map_err(|_| extism::Error::msg("idp_http_request: host context lock poisoned"))?;
    let origin = state
        .idp_origin
        .as_ref()
        .ok_or_else(|| extism::Error::msg("idp_http_request: no IdP endpoint bound to this call"))?;
    let client = state
        .client
        .as_ref()
        .ok_or_else(|| extism::Error::msg("idp_http_request: no http client bound to this call"))?;

    let (url, method) = resolve_request("idp_http_request", origin, &req)?;

    // Re-resolve and re-check on every single call (not just once, back in
    // `auth_via_sso`) so a DNS answer that changed between validation and
    // now can't smuggle a request past the denylist.
    let host = url
        .host_str()
        .ok_or_else(|| extism::Error::msg("idp_http_request: url has no host"))?;
    let port = url.port_or_known_default().unwrap_or(443);
    crate::ssrf::resolve_and_check(host, port)
        .map_err(|reason| extism::Error::msg(format!("idp_http_request: {reason}")))?;

    send_and_shape_response("idp_http_request", url, method, client, &req)
});

#[cfg(test)]
mod tests {
    use super::*;

    fn req(method: &str, path: &str) -> HttpRequestMsg {
        HttpRequestMsg {
            method: method.to_string(),
            path: path.to_string(),
            headers: BTreeMap::new(),
            body: None,
        }
    }

    #[test]
    fn resolve_request_rejects_protocol_relative_path_escape() {
        let origin = url::Url::parse("https://idp.example.test/").unwrap();
        let r = req("GET", "//evil.example.test/token");
        let err = resolve_request("idp_http_request", &origin, &r).unwrap_err();
        assert!(
            err.to_string()
                .contains("must not change the request origin"),
            "unexpected error message: {err}"
        );
    }

    #[test]
    fn resolve_request_rejects_backslash_path_escape() {
        let origin = url::Url::parse("https://idp.example.test/").unwrap();
        let r = req("GET", "/\\evil.example.test/x");
        let err = resolve_request("idp_http_request", &origin, &r).unwrap_err();
        assert!(
            err.to_string()
                .contains("must not change the request origin"),
            "unexpected error message: {err}"
        );
    }

    #[test]
    fn resolve_request_accepts_same_origin_path() {
        let origin = url::Url::parse("https://idp.example.test/").unwrap();
        let r = req("GET", "/token");
        let (url, method) = resolve_request("idp_http_request", &origin, &r).unwrap();
        assert_eq!(url.as_str(), "https://idp.example.test/token");
        assert_eq!(method, reqwest::Method::GET);
    }
}
