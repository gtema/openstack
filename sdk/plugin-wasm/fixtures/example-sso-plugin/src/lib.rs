#![no_main]

//! Example SSO ABI v1 plugin, used as a test fixture by
//! `openstack-sdk-plugin-wasm`'s integration tests. It implements a toy
//! browser-based `v3examplesso` method: `sso_build_request` points the
//! browser at a fake identity-provider authorize page carrying the
//! host-provided callback URL as its `redirect_uri`, and
//! `sso_parse_callback` reads the `token` field out of whatever the
//! callback POST carried.
//!
//! Neither export performs any I/O — the guest sandbox forbids sockets
//! entirely (`Manifest::disallow_all_hosts`), so this is enforced
//! structurally, not just by convention.
//!
//! A `mode` value (`values.mode`) lets the same fixture also exercise the
//! host's SSO security checks:
//! - `"bad_scheme"` — returns a plain `http://` url (must be rejected before
//!   any browser is opened).
//! - `"bad_host"` — returns a `redirect_host` that doesn't match the
//!   callback URL the host handed in (must be rejected before any browser
//!   is opened).
//! - anything else (including absent) — well-behaved.

use extism_pdk::*;
use serde_json::{Value, json};

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

    let parsed = url::Url::parse(callback_url)
        .map_err(|e| Error::msg(format!("invalid callback_url: {e}")))?;
    let real_redirect_host = match parsed.port() {
        Some(port) => format!("{}:{port}", parsed.host_str().unwrap_or("")),
        None => parsed.host_str().unwrap_or("").to_string(),
    };

    let (scheme, redirect_host) = match mode {
        "bad_scheme" => ("http", real_redirect_host.as_str()),
        "bad_host" => ("https", "evil.example.test:1"),
        _ => ("https", real_redirect_host.as_str()),
    };

    let url = format!(
        "{scheme}://idp.example.test/authorize?client_id=demo&redirect_uri={}",
        urlencode(callback_url)
    );

    Ok(json!({"url": url, "redirect_host": redirect_host}).to_string())
}

#[plugin_fn]
pub fn sso_parse_callback(input: String) -> FnResult<String> {
    let request: Value = serde_json::from_str(&input)?;
    let token = request
        .get("params")
        .and_then(|p| p.get("token"))
        .and_then(|v| v.as_str())
        .unwrap_or_default();

    if token.is_empty() {
        return Ok(json!({"error": "callback didn't carry a token"}).to_string());
    }

    Ok(json!({"ok": {"token": token, "auth_info": null}}).to_string())
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
