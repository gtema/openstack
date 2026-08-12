#![no_main]

//! Example SSO ABI v1 plugin, used as a test fixture by
//! `openstack-sdk-plugin-wasm`'s integration tests. It implements a toy
//! browser-based `v3examplesso` method: `sso_build_request` points the
//! browser at a fake identity-provider authorize page carrying the
//! host-provided callback URL as its `redirect_uri`, and
//! `sso_parse_callback` reads the `token` field out of whatever the
//! callback POST carried.
//!
//! Neither export gets a socket, DNS resolver, or browser-opening
//! capability of its own — the guest sandbox forbids sockets entirely
//! (`Manifest::disallow_all_hosts`) except for the host-mediated
//! `identity_http_request`/`idp_http_request` imports `sso_parse_callback`
//! uses in its `code`/`idp_code`-param paths below.
//!
//! A `mode` value (`values.mode`) lets the same fixture also exercise the
//! host's SSO security checks:
//! - `"bad_scheme"` — returns a plain `http://` url (must be rejected before
//!   any browser is opened).
//! - `"bad_host"` — returns a `redirect_host` that doesn't match the
//!   callback URL the host handed in (must be rejected before any browser
//!   is opened).
//! - `"ssrf_denylisted"` — returns a `url` whose host resolves to the
//!   cloud metadata address (must be rejected before any browser is
//!   opened, same as `"bad_scheme"`/`"bad_host"`).
//! - `"call_idp_during_build"` — attempts to call `idp_http_request` from
//!   within `sso_build_request` itself; must fail, since no IdP origin is
//!   bound until `sso_parse_callback`.
//! - `"pkce"` — like the default well-behaved path, but
//!   `sso_parse_callback`'s `idp_code` handling additionally forwards the
//!   received `code_verifier` as a `code_verifier` form field in the
//!   `idp_http_request` POST body, so a mock IdP token endpoint can verify
//!   the PKCE round trip end to end. If the mock IdP's response also
//!   carries an `id_token` field, its (unverified — no signature check,
//!   same as the ABI's stance generally) JWT payload's `nonce` claim is
//!   compared against the `nonce` this fixture embedded in the authorize
//!   URL during `sso_build_request` (carried across the two calls via
//!   Extism's `var_get`/`var_set`, since `nonce` isn't secret); a mismatch
//!   returns `{"error": ...}` instead of `{"ok": ...}`. Note: trusting an
//!   IdP-supplied POST param (`mode`) as a control-flow switch is a
//!   test-harness convenience specific to this fixture, not a pattern for
//!   real plugins to imitate — a real plugin should not let untrusted
//!   callback input select its security-relevant behavior.
//! - anything else (including absent) — well-behaved.
//!
//! Whenever the host-supplied request carries `code_challenge` /
//! `code_challenge_method` / `nonce` fields, `sso_build_request` always
//! embeds them in the authorize URL's query string — this isn't gated
//! behind any `mode`, since only the `idp_code` POST-body forwarding below
//! needs a dedicated mode.
//!
//! `sso_parse_callback` exercises both host-mediated HTTP imports:
//! - a `code` param is POSTed to a fake identity endpoint path
//!   (`/v3/auth/tokens/exchange`) via `identity_http_request`, and whatever
//!   `token` field the response echoes back becomes the final token.
//! - an `idp_code` param (optionally alongside a `mode: "pkce"` param) is
//!   POSTed to a fake IdP token endpoint path (`/token`) via
//!   `idp_http_request`; when `mode` is `"pkce"`, the POST body also
//!   includes the `code_verifier` the host handed to `sso_parse_callback`,
//!   and any `id_token` in the response is checked against the nonce as
//!   described above.
//! - a plain `token` param (the original fixture behavior) skips both round
//!   trips entirely.

use extism_pdk::*;
use serde_json::{Value, json};

#[host_fn]
extern "ExtismHost" {
    fn identity_http_request(request: String) -> String;
    fn idp_http_request(request: String) -> String;
}

#[plugin_fn]
pub fn plugin_abi_version(_input: String) -> FnResult<String> {
    Ok("1".to_string())
}

#[plugin_fn]
pub fn auth_supported_methods(_input: String) -> FnResult<String> {
    Ok(json!(["v3examplesso"]).to_string())
}

#[plugin_fn]
pub fn auth_api_version(_input: String) -> FnResult<String> {
    Ok(json!([3, 0]).to_string())
}

#[plugin_fn]
pub fn auth_requirements(_input: String) -> FnResult<String> {
    Ok(json!({
        "type": "object",
        "properties": {}
    })
    .to_string())
}

#[plugin_fn]
pub fn sso_build_request(input: String) -> FnResult<String> {
    let request: Value = serde_json::from_str(&input)?;
    let callback_url = request
        .get("callback_url")
        .and_then(|v| v.as_str())
        .unwrap_or_default();
    let mode = request
        .get("values")
        .and_then(|v| v.get("mode"))
        .and_then(|v| v.as_str())
        .unwrap_or("");

    if mode == "call_idp_during_build" {
        let http_request = json!({
            "method": "GET",
            "path": "/",
            "headers": {},
            "body": null,
        })
        .to_string();
        // Must fail: no IdP origin is bound during `sso_build_request`.
        unsafe { idp_http_request(http_request)? };
        return Ok(json!({"url": "https://unused.example.test/", "redirect_host": "unused"})
            .to_string());
    }

    let parsed = url::Url::parse(callback_url)
        .map_err(|e| Error::msg(format!("invalid callback_url: {e}")))?;
    let real_redirect_host = match parsed.port() {
        Some(port) => format!("{}:{port}", parsed.host_str().unwrap_or("")),
        None => parsed.host_str().unwrap_or("").to_string(),
    };

    let (scheme, host, redirect_host) = match mode {
        "bad_scheme" => ("http", "idp.example.test", real_redirect_host.as_str()),
        "bad_host" => ("https", "idp.example.test", "evil.example.test:1"),
        "ssrf_denylisted" => ("https", "169.254.169.254", real_redirect_host.as_str()),
        _ => ("https", "idp.example.test", real_redirect_host.as_str()),
    };

    let code_challenge = request
        .get("code_challenge")
        .and_then(|v| v.as_str())
        .unwrap_or_default();
    let code_challenge_method = request
        .get("code_challenge_method")
        .and_then(|v| v.as_str())
        .unwrap_or_default();
    let nonce = request
        .get("nonce")
        .and_then(|v| v.as_str())
        .unwrap_or_default();

    // `nonce` isn't secret, but it's only handed to the guest once, here —
    // stash it so `sso_parse_callback` can compare it against the
    // `id_token`'s `nonce` claim later.
    var::set("nonce", nonce)?;

    let url = format!(
        "{scheme}://{host}/authorize?client_id=demo&redirect_uri={}&code_challenge={}&code_challenge_method={}&nonce={}",
        urlencode(callback_url),
        urlencode(code_challenge),
        urlencode(code_challenge_method),
        urlencode(nonce)
    );

    Ok(json!({"url": url, "redirect_host": redirect_host}).to_string())
}

#[plugin_fn]
pub fn sso_parse_callback(input: String) -> FnResult<String> {
    let request: Value = serde_json::from_str(&input)?;
    let params = request
        .get("params")
        .and_then(|p| p.as_object())
        .cloned()
        .unwrap_or_default();

    // An `idp_code` param means the callback carried a code that must be
    // exchanged against the *IdP's own* token endpoint — exercises
    // `idp_http_request`.
    if let Some(idp_code) = params.get("idp_code").and_then(|v| v.as_str()) {
        let code_verifier = request
            .get("code_verifier")
            .and_then(|v| v.as_str())
            .unwrap_or_default();
        let use_pkce = params.get("mode").and_then(|v| v.as_str()) == Some("pkce");

        let body = if use_pkce {
            json!({"code": idp_code, "code_verifier": code_verifier}).to_string()
        } else {
            json!({"code": idp_code}).to_string()
        };

        let http_request = json!({
            "method": "POST",
            "path": "/token",
            "headers": {"Content-Type": "application/json"},
            "body": body,
        })
        .to_string();
        let response_json = unsafe { idp_http_request(http_request)? };
        let response: Value = serde_json::from_str(&response_json)?;

        let status = response.get("status").and_then(|v| v.as_u64()).unwrap_or(0);
        if !(200..300).contains(&status) {
            return Ok(json!({"error": format!("idp endpoint returned status {status}")})
                .to_string());
        }
        let body: Value = response
            .get("body")
            .and_then(|b| b.as_str())
            .and_then(|s| serde_json::from_str(s).ok())
            .unwrap_or(Value::Null);
        let token = body.get("token").and_then(|v| v.as_str()).unwrap_or_default();
        if token.is_empty() {
            return Ok(json!({"error": "idp endpoint did not return a token"}).to_string());
        }

        if let Some(id_token) = body.get("id_token").and_then(|v| v.as_str()) {
            let claims = decode_jwt_payload(id_token)
                .map_err(|e| Error::msg(format!("invalid id_token: {e}")))?;
            let got_nonce = claims.get("nonce").and_then(|v| v.as_str()).unwrap_or_default();
            let expected_nonce: String = var::get("nonce")?.unwrap_or_default();
            if got_nonce != expected_nonce {
                return Ok(json!({"error": "id_token nonce did not match"}).to_string());
            }
        }

        return Ok(json!({"ok": {"token": token, "auth_info": null}}).to_string());
    }

    // A `code` param means the callback carried a raw IdP-issued code/token
    // that still needs exchanging against the identity endpoint itself —
    // exercises `identity_http_request`. A `token` param (the pre-existing
    // fixture behavior) means the callback already carried a usable token
    // directly, no round trip needed.
    if let Some(code) = params.get("code").and_then(|v| v.as_str()) {
        let http_request = json!({
            "method": "POST",
            "path": "/v3/auth/tokens/exchange",
            "headers": {"Content-Type": "application/json"},
            "body": json!({"code": code}).to_string(),
        })
        .to_string();
        let response_json = unsafe { identity_http_request(http_request)? };
        let response: Value = serde_json::from_str(&response_json)?;

        let status = response.get("status").and_then(|v| v.as_u64()).unwrap_or(0);
        if !(200..300).contains(&status) {
            return Ok(
                json!({"error": format!("identity endpoint returned status {status}")})
                    .to_string(),
            );
        }
        let body: Value = response
            .get("body")
            .and_then(|b| b.as_str())
            .and_then(|s| serde_json::from_str(s).ok())
            .unwrap_or(Value::Null);
        let token = body.get("token").and_then(|v| v.as_str()).unwrap_or_default();
        if token.is_empty() {
            return Ok(json!({"error": "identity endpoint did not return a token"}).to_string());
        }
        return Ok(json!({"ok": {"token": token, "auth_info": null}}).to_string());
    }

    let token = params.get("token").and_then(|v| v.as_str()).unwrap_or_default();
    if token.is_empty() {
        return Ok(json!({"error": "callback didn't carry a token"}).to_string());
    }

    Ok(json!({"ok": {"token": token, "auth_info": null}}).to_string())
}

/// Decode a JWT's payload claims (the middle `.`-separated segment),
/// base64url-no-pad. No signature verification — matches this ABI's
/// stance on `id_token` generally (out of scope; the channel it arrived
/// over is already origin-pinned and SSRF-checked).
fn decode_jwt_payload(jwt: &str) -> Result<Value, Error> {
    use base64::Engine as _;

    let payload_segment = jwt
        .split('.')
        .nth(1)
        .ok_or_else(|| Error::msg("jwt did not have a payload segment"))?;
    let decoded = base64::engine::general_purpose::URL_SAFE_NO_PAD.decode(payload_segment)?;
    Ok(serde_json::from_slice(&decoded)?)
}

/// Minimal query-value percent-encoding, just enough for the test fixture's
/// own callback URL (which itself only ever contains `[A-Za-z0-9:/._-]` plus
/// `?`, `=`, `&`) — not a general-purpose encoder.
fn urlencode(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for b in s.bytes() {
        match b {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                out.push(b as char)
            }
            _ => out.push_str(&format!("%{b:02X}")),
        }
    }
    out
}
