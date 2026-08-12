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

//! The single host function exposed to WASM auth plugins: a capability-restricted
//! HTTP client that may only reach the Identity endpoint the plugin was invoked
//! for. Plugins never get a socket, DNS resolver, or unrestricted HTTP client of
//! their own — every outbound request is proxied through here so the host can
//! enforce the destination.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

/// Per-call state made available to the `identity_http_request` host function.
///
/// Populated by [`crate::plugin::WasmAuthPlugin`] immediately before each guest
/// call that may need it, while the plugin's own mutex is held, and torn down
/// (client dropped) right after. `identity_origin` is scheme+host+port only; the
/// guest supplies a request-relative `path` which is joined against it, so a
/// plugin can never target a host other than the one the SDK resolved for the
/// configured cloud.
#[derive(Clone, Default)]
pub(crate) struct HostContextState {
    pub(crate) identity_origin: Option<url::Url>,
    pub(crate) client: Option<reqwest::blocking::Client>,
}

/// Request message a guest sends to `identity_http_request`.
#[derive(Debug, Deserialize)]
struct HttpRequestMsg {
    method: String,
    /// Must be relative (start with `/`); resolved against `identity_origin`.
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
/// part of `identity_http_request`'s handling of untrusted guest bytes that
/// doesn't require a live [`HostContextState`], split out so it can be
/// exercised directly (including by the `fuzzing`-feature entry point below)
/// without a real WASM call or network access.
fn resolve_request(
    origin: &url::Url,
    req: &HttpRequestMsg,
) -> Result<(url::Url, reqwest::Method), extism::Error> {
    if !req.path.starts_with('/') {
        return Err(extism::Error::msg(
            "identity_http_request: `path` must be relative to the identity endpoint (start with '/')",
        ));
    }
    let url = origin
        .join(&req.path)
        .map_err(|e| extism::Error::msg(format!("identity_http_request: invalid path: {e}")))?;
    let method = reqwest::Method::from_bytes(req.method.as_bytes())
        .map_err(|e| extism::Error::msg(format!("identity_http_request: invalid method: {e}")))?;
    Ok((url, method))
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
        let _ = resolve_request(origin, &req);
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

    let (url, method) = resolve_request(origin, &req)?;

    let mut builder = client.request(method, url);
    for (k, v) in &req.headers {
        builder = builder.header(k, v);
    }
    if let Some(body) = req.body {
        builder = builder.body(body);
    }

    let resp = builder
        .send()
        .map_err(|e| extism::Error::msg(format!("identity_http_request: request failed: {e}")))?;
    let status = resp.status().as_u16();
    let headers = resp
        .headers()
        .iter()
        .filter_map(|(k, v)| v.to_str().ok().map(|v| (k.to_string(), v.to_string())))
        .collect();
    let body = resp
        .text()
        .map_err(|e| extism::Error::msg(format!("identity_http_request: reading response body failed: {e}")))?;

    Ok(serde_json::to_string(&HttpResponseMsg { status, headers, body })?)
});
