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

//! Integration tests for the `sso` ABI flavor, exercising a real Extism
//! (WASM) SSO plugin built from
//! `sdk/plugin-wasm/fixtures/example-sso-plugin` and checked in as
//! `tests/fixtures/example_sso.wasm`.
//!
//! What's automatable in a headless sandbox vs. not:
//!
//! - The host-side security checks (`https://`-only, redirect-host must
//!   match the host-bound callback listener) run *before*
//!   [`WasmAuthPlugin::auth`] ever prompts for confirmation or opens a
//!   browser, so they're fully exercised here through the real public
//!   `auth()` entry point.
//! - A full happy-path run additionally needs an interactive confirmation
//!   (`dialoguer::Confirm`, reads a real terminal) and a real browser —
//!   neither exists in this test environment, matching the same
//!   can't-verify-live-here caveat already documented for GitHub
//!   attestations in `provenance.rs`'s tests. Instead, the guest ABI's own
//!   correctness (`sso_build_request`/`sso_parse_callback` shapes) is
//!   verified directly against a raw [`extism::Plugin`], and the shared
//!   anti-CSRF callback listener `auth_via_sso` relies on
//!   (`openstack_sdk_websso_host::CallbackServer`) is exercised end-to-end
//!   here too, forged state included.

use std::collections::HashMap;
use std::path::PathBuf;
use std::time::Duration;

use extism::{Function, Manifest, Plugin, UserData, ValType, Wasm};
use httpmock::prelude::*;
use secrecy::SecretString;

use openstack_sdk_auth_core::OpenStackAuthType;
use openstack_sdk_plugin_wasm::WasmAuthPlugin;
use openstack_sdk_websso_host::CallbackServer;

fn fixture_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/example_sso.wasm")
}

// `sso_parse_callback` now unconditionally imports `identity_http_request`
// (used only on its `code`-param path), so any raw `extism::Plugin` built
// from the fixture must resolve that import even when the tests using it
// never take that path. This stub is never actually invoked by
// `sso_guest_round_trip_is_well_formed` below, which only ever exercises
// the `token`-param path.
extism::host_fn!(stub_identity_http_request(_request: String) -> String {
    let result: Result<String, extism::Error> = Err(extism::Error::msg(
        "raw_plugin's identity_http_request stub should never be called",
    ));
    result
});

extism::host_fn!(stub_idp_http_request(_request: String) -> String {
    let result: Result<String, extism::Error> = Err(extism::Error::msg(
        "raw_plugin's idp_http_request stub should never be called",
    ));
    result
});

fn raw_plugin() -> Result<Plugin, Box<dyn std::error::Error>> {
    let manifest = Manifest::new([Wasm::file(fixture_path())]).disallow_all_hosts();
    let functions = vec![
        Function::new(
            "identity_http_request",
            [ValType::I64],
            [ValType::I64],
            UserData::new(()),
            stub_identity_http_request,
        ),
        Function::new(
            "idp_http_request",
            [ValType::I64],
            [ValType::I64],
            UserData::new(()),
            stub_idp_http_request,
        ),
    ];
    Ok(Plugin::new(manifest, functions, false)?)
}

#[test]
fn load_validates_sso_abi() -> Result<(), Box<dyn std::error::Error>> {
    let plugin = WasmAuthPlugin::load(&fixture_path())?;
    assert_eq!(plugin.name(), "example_sso");
    assert_eq!(plugin.supported_methods(), &["v3examplesso"]);
    assert_eq!(plugin.api_version(), (3, 0));
    Ok(())
}

#[tokio::test(flavor = "multi_thread")]
async fn bad_scheme_is_rejected_before_any_prompt() -> Result<(), Box<dyn std::error::Error>> {
    let plugin = WasmAuthPlugin::load(&fixture_path())?;
    let identity_url = url::Url::parse("https://keystone.example.test/v3")?;
    let http_client = reqwest::Client::new();

    let mut values: HashMap<String, SecretString> = HashMap::new();
    values.insert("mode".to_string(), SecretString::from("bad_scheme"));

    // If this reached the interactive confirmation prompt it would hang or
    // error on this environment's non-interactive stdin; a bounded timeout
    // makes that failure mode loud (test failure) instead of a silent hang.
    let result = tokio::time::timeout(
        Duration::from_secs(10),
        plugin.auth(&http_client, &identity_url, &values, None, None),
    )
    .await
    .expect("must not block on a confirmation prompt");

    let err = result.expect_err("a non-https redirect must be rejected");
    assert!(
        err.to_string().to_lowercase().contains("https")
            || err.to_string().to_lowercase().contains("scheme"),
        "unexpected error: {err}"
    );
    Ok(())
}

#[tokio::test(flavor = "multi_thread")]
async fn undeclared_redirect_host_is_rejected_before_any_prompt()
-> Result<(), Box<dyn std::error::Error>> {
    let plugin = WasmAuthPlugin::load(&fixture_path())?;
    let identity_url = url::Url::parse("https://keystone.example.test/v3")?;
    let http_client = reqwest::Client::new();

    let mut values: HashMap<String, SecretString> = HashMap::new();
    values.insert("mode".to_string(), SecretString::from("bad_host"));

    let result = tokio::time::timeout(
        Duration::from_secs(10),
        plugin.auth(&http_client, &identity_url, &values, None, None),
    )
    .await
    .expect("must not block on a confirmation prompt");

    let err = result.expect_err("an undeclared redirect host must be rejected outright");
    assert!(
        err.to_string().to_lowercase().contains("redirect host"),
        "unexpected error: {err}"
    );
    Ok(())
}

#[test]
fn sso_guest_round_trip_is_well_formed() -> Result<(), Box<dyn std::error::Error>> {
    let mut plugin = raw_plugin()?;

    let build_request = serde_json::json!({
        "identity_url": "https://keystone.example.test/v3",
        "callback_url": "http://127.0.0.1:54321/callback?state=abc123",
        "values": {},
        "scope": null,
        "hints": null,
    })
    .to_string();
    let build_output: String = plugin.call("sso_build_request", build_request.as_str())?;
    let build: serde_json::Value = serde_json::from_str(&build_output)?;

    let url = build["url"].as_str().ok_or("missing url")?;
    assert!(url.starts_with("https://idp.example.test/authorize"));
    assert!(url.contains("redirect_uri="));
    assert_eq!(build["redirect_host"].as_str(), Some("127.0.0.1:54321"));

    let callback_request = serde_json::json!({"params": {"token": "guest-token"}}).to_string();
    let callback_output: String = plugin.call("sso_parse_callback", callback_request.as_str())?;
    let callback: serde_json::Value = serde_json::from_str(&callback_output)?;
    assert_eq!(callback["ok"]["token"].as_str(), Some("guest-token"));

    let empty_callback = serde_json::json!({"params": {}}).to_string();
    let empty_output: String = plugin.call("sso_parse_callback", empty_callback.as_str())?;
    let empty: serde_json::Value = serde_json::from_str(&empty_output)?;
    assert!(empty.get("error").is_some());

    Ok(())
}

/// The exact primitive `auth_via_sso` waits on: a forged `state` never
/// satisfies the callback wait, and the real one does. Covered in depth in
/// `openstack-sdk-websso-host`'s own test suite; repeated here narrowly to
/// document that the SSO ABI flavor's anti-CSRF protection is this same
/// host-owned primitive, not something plugin-wasm reimplements.
#[tokio::test]
async fn callback_server_rejects_forged_state() -> Result<(), Box<dyn std::error::Error>> {
    let server = CallbackServer::bind(None).await?;
    let mut forged = server.callback_url().clone();
    forged.set_query(Some("state=forged"));

    let wait = tokio::spawn(server.wait_for_callback(Duration::from_secs(5)));

    let client = reqwest::Client::new();
    let forged_resp = client
        .post(forged.as_str())
        .form(&[("token", "attacker-token")])
        .send()
        .await?;
    assert_eq!(forged_resp.status(), reqwest::StatusCode::FORBIDDEN);

    // wait is still pending: cancel it by letting it time out quickly is
    // unnecessary here since we don't send the real callback in this test —
    // dropping the task is enough, nothing else depends on its outcome.
    wait.abort();
    Ok(())
}

/// `sso_parse_callback`'s new `identity_http_request` path: a callback
/// carrying a `code` param gets POSTed to the identity endpoint, and
/// whatever token the mock identity server echoes back becomes the guest's
/// returned token. Exercised directly via `call_guest_for_test`, bypassing
/// `auth_via_sso`'s browser/confirm/callback-listener steps — those need a
/// real terminal and browser this test environment doesn't have (see the
/// module docs above).
#[tokio::test(flavor = "multi_thread")]
async fn sso_parse_callback_exchanges_code_via_identity_endpoint()
-> Result<(), Box<dyn std::error::Error>> {
    let server = MockServer::start();
    let mock = server.mock(|when, then| {
        when.method(POST)
            .path("/v3/auth/tokens/exchange")
            .json_body(serde_json::json!({"code": "raw-idp-code"}));
        then.status(200)
            .header("Content-Type", "application/json")
            .body(r#"{"token": "exchanged-token-xyz"}"#);
    });

    let plugin = WasmAuthPlugin::load(&fixture_path())?;
    let identity_url = url::Url::parse(&server.base_url())?;

    let callback_json = serde_json::json!({"params": {"code": "raw-idp-code"}}).to_string();
    let output = plugin
        .call_guest_for_test("sso_parse_callback", callback_json, &identity_url, None)
        .await?;
    let parsed: serde_json::Value = serde_json::from_str(&output)?;

    mock.assert();
    assert_eq!(parsed["ok"]["token"].as_str(), Some("exchanged-token-xyz"));
    Ok(())
}

/// `sso_build_request` returning a `url` whose host resolves to a
/// denylisted (loopback) address must be rejected before any confirmation
/// prompt or browser-open — same shape as `bad_scheme_is_rejected_before_any_prompt`
/// / `undeclared_redirect_host_is_rejected_before_any_prompt` above.
#[tokio::test(flavor = "multi_thread")]
async fn ssrf_denylisted_redirect_is_rejected_before_any_prompt()
-> Result<(), Box<dyn std::error::Error>> {
    let plugin = WasmAuthPlugin::load(&fixture_path())?;
    let identity_url = url::Url::parse("https://keystone.example.test/v3")?;
    let http_client = reqwest::Client::new();

    let mut values: HashMap<String, SecretString> = HashMap::new();
    values.insert("mode".to_string(), SecretString::from("ssrf_denylisted"));

    let result = tokio::time::timeout(
        Duration::from_secs(10),
        plugin.auth(&http_client, &identity_url, &values, None, None),
    )
    .await
    .expect("must not block on a confirmation prompt");

    let err = result.expect_err("a url resolving to a denylisted address must be rejected");
    assert!(
        err.to_string().to_lowercase().contains("disallowed")
            || err.to_string().to_lowercase().contains("resolves to"),
        "unexpected error: {err}"
    );
    Ok(())
}

/// `idp_http_request` must not be reachable during `sso_build_request` —
/// no IdP origin exists yet at that point (it's derived from
/// `sso_build_request`'s own response). Exercised directly via
/// `call_guest_for_test` with `idp_origin: None`, bypassing
/// `auth_via_sso` entirely.
#[tokio::test(flavor = "multi_thread")]
async fn idp_http_request_during_sso_build_request_errors() -> Result<(), Box<dyn std::error::Error>>
{
    let plugin = WasmAuthPlugin::load(&fixture_path())?;
    let identity_url = url::Url::parse("https://keystone.example.test/v3")?;

    let request_json = serde_json::json!({
        "identity_url": "https://keystone.example.test/v3",
        "callback_url": "http://127.0.0.1:54321/callback?state=abc123",
        "values": {"mode": "call_idp_during_build"},
        "scope": null,
        "hints": null,
    })
    .to_string();

    let result = plugin
        .call_guest_for_test("sso_build_request", request_json, &identity_url, None)
        .await;

    let err = result.expect_err("idp_http_request must not be reachable during sso_build_request");
    // `WasmPluginError::Call`'s `Display` only shows the generic
    // "error while executing at wasm backtrace" message Extism wraps every
    // guest-trap error in; the actual host-function message
    // (`idp_http_request: no IdP endpoint bound to this call`) only shows up
    // in the anyhow-style cause chain, which requires `{:?}` (Debug), not
    // `{}` (Display), to see.
    assert!(
        format!("{err:?}")
            .to_lowercase()
            .contains("no idp endpoint bound"),
        "unexpected error: {err:?}"
    );
    Ok(())
}

/// Happy path: `sso_parse_callback` reaches a mock IdP token endpoint via
/// `idp_http_request`, and the response drives the resulting token.
/// Exercised directly via `call_guest_for_test` with a `Some(idp_origin)`
/// pointed at the mock server, bypassing `auth_via_sso`'s browser/confirm/
/// callback-listener steps (those need a real terminal and browser this
/// test environment doesn't have).
///
/// This test relies on `ssrf.rs`'s `fuzzing`-feature loopback exemption:
/// `httpmock::MockServer` always binds `127.0.0.1`, which the SSRF denylist
/// would otherwise reject. This test binary is built with
/// `required-features = ["fuzzing"]` (see `Cargo.toml`), so the exemption is
/// active; every other denylisted range stays enforced even under that
/// feature, and it is never compiled into a release `osc` binary.
#[tokio::test(flavor = "multi_thread")]
async fn sso_parse_callback_exchanges_code_via_idp_endpoint()
-> Result<(), Box<dyn std::error::Error>> {
    let server = MockServer::start();
    let mock = server.mock(|when, then| {
        when.method(POST)
            .path("/token")
            .json_body(serde_json::json!({"code": "raw-code-from-callback"}));
        then.status(200)
            .header("Content-Type", "application/json")
            .body(r#"{"token": "idp-exchanged-token"}"#);
    });

    let plugin = WasmAuthPlugin::load(&fixture_path())?;
    let identity_url = url::Url::parse("https://keystone.example.test/v3")?;
    let idp_origin = url::Url::parse(&server.base_url())?;

    let callback_json =
        serde_json::json!({"params": {"idp_code": "raw-code-from-callback"}}).to_string();
    let output = plugin
        .call_guest_for_test(
            "sso_parse_callback",
            callback_json,
            &identity_url,
            Some(idp_origin),
        )
        .await?;
    let parsed: serde_json::Value = serde_json::from_str(&output)?;

    mock.assert();
    assert_eq!(parsed["ok"]["token"].as_str(), Some("idp-exchanged-token"));
    Ok(())
}

/// Build a minimal (unsigned, unverified — matches this ABI's stance on
/// `id_token` generally) JWT carrying `nonce` as its only payload claim,
/// for feeding to the mock IdP token endpoint in the nonce round-trip
/// tests below.
fn make_id_token(nonce: &str) -> String {
    use base64::Engine as _;

    let encode = |v: &serde_json::Value| {
        base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(v.to_string())
    };
    let header = encode(&serde_json::json!({"alg": "none", "typ": "JWT"}));
    let payload = encode(&serde_json::json!({"nonce": nonce}));
    format!("{header}.{payload}.")
}

/// End-to-end PKCE round trip: `sso_build_request` embeds the
/// host-generated `code_challenge`/`code_challenge_method` into the
/// authorize URL's query string, and `sso_parse_callback` forwards the
/// matching `code_verifier` in the `idp_http_request` POST body sent for
/// token exchange. The mock IdP independently recomputes
/// SHA256(code_verifier) and compares it against the `code_challenge` it
/// saw in the earlier authorize URL, proving the two round-tripped
/// correctly end to end — this exercises the real `CallbackServer::bind`
/// PKCE generation, driven through `call_guest_for_test` (not
/// `auth_via_sso`, which needs a terminal and a browser this test
/// environment doesn't have).
#[tokio::test(flavor = "multi_thread")]
async fn sso_pkce_round_trips_from_build_request_to_callback()
-> Result<(), Box<dyn std::error::Error>> {
    use base64::Engine as _;

    let plugin = WasmAuthPlugin::load(&fixture_path())?;
    let identity_url = url::Url::parse("https://keystone.example.test/v3")?;

    let server = CallbackServer::bind(None).await?;
    let code_challenge = server.code_challenge().to_string();
    let code_verifier = server.code_verifier().to_string();
    let nonce = server.nonce().to_string();

    let build_request = serde_json::json!({
        "identity_url": identity_url.as_str(),
        "callback_url": server.callback_url().as_str(),
        "values": {},
        "scope": null,
        "hints": null,
        "code_challenge": code_challenge,
        "code_challenge_method": "S256",
        "nonce": nonce,
    })
    .to_string();
    let build_output = plugin
        .call_guest_for_test("sso_build_request", build_request, &identity_url, None)
        .await?;
    let build: serde_json::Value = serde_json::from_str(&build_output)?;
    let authorize_url = build["url"].as_str().ok_or("missing url")?;
    assert!(
        authorize_url.contains(&format!("code_challenge={code_challenge}")),
        "authorize url did not embed the code_challenge: {authorize_url}"
    );
    assert!(
        authorize_url.contains("code_challenge_method=S256"),
        "authorize url did not embed the code_challenge_method: {authorize_url}"
    );
    assert!(
        authorize_url.contains(&format!("nonce={nonce}")),
        "authorize url did not embed the nonce: {authorize_url}"
    );

    let idp_server = MockServer::start();
    let expected_challenge = code_challenge.clone();
    let id_token = make_id_token(&nonce);
    let mock = idp_server.mock(|when, then| {
        when.method(POST).path("/token").is_true(move |req| {
            let body = serde_json::from_slice::<serde_json::Value>(&req.body_vec()).ok();
            let Some(body) = body else { return false };
            let Some(sent_verifier) = body.get("code_verifier").and_then(|v| v.as_str()) else {
                return false;
            };
            let digest = ring::digest::digest(&ring::digest::SHA256, sent_verifier.as_bytes());
            let recomputed_challenge =
                base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(digest.as_ref());
            recomputed_challenge == expected_challenge
        });
        then.status(200)
            .header("Content-Type", "application/json")
            .body(
                serde_json::json!({"token": "pkce-verified-token", "id_token": id_token})
                    .to_string(),
            );
    });

    let idp_origin = url::Url::parse(&idp_server.base_url())?;
    let callback_json = serde_json::json!({
        "params": {"idp_code": "raw-code-from-callback", "mode": "pkce"},
        "code_verifier": code_verifier,
    })
    .to_string();
    let output = plugin
        .call_guest_for_test(
            "sso_parse_callback",
            callback_json,
            &identity_url,
            Some(idp_origin),
        )
        .await?;
    let parsed: serde_json::Value = serde_json::from_str(&output)?;

    mock.assert();
    assert_eq!(parsed["ok"]["token"].as_str(), Some("pkce-verified-token"));
    Ok(())
}

/// Same round trip as above, but the mock IdP's `id_token` carries a
/// `nonce` claim that doesn't match the one `sso_build_request` embedded in
/// the authorize URL — proving nonce validation is guest-side, recoverable
/// logic (an `{"error": ...}` result) rather than a host-enforced
/// rejection: the host never inspects `id_token` contents at all.
#[tokio::test(flavor = "multi_thread")]
async fn sso_nonce_mismatch_is_reported_as_a_guest_error() -> Result<(), Box<dyn std::error::Error>>
{
    let plugin = WasmAuthPlugin::load(&fixture_path())?;
    let identity_url = url::Url::parse("https://keystone.example.test/v3")?;

    let server = CallbackServer::bind(None).await?;
    let code_challenge = server.code_challenge().to_string();
    let code_verifier = server.code_verifier().to_string();
    let nonce = server.nonce().to_string();

    let build_request = serde_json::json!({
        "identity_url": identity_url.as_str(),
        "callback_url": server.callback_url().as_str(),
        "values": {},
        "scope": null,
        "hints": null,
        "code_challenge": code_challenge,
        "code_challenge_method": "S256",
        "nonce": nonce,
    })
    .to_string();
    plugin
        .call_guest_for_test("sso_build_request", build_request, &identity_url, None)
        .await?;

    let idp_server = MockServer::start();
    let id_token = make_id_token("a-completely-different-nonce");
    let mock = idp_server.mock(|when, then| {
        when.method(POST).path("/token");
        then.status(200)
            .header("Content-Type", "application/json")
            .body(
                serde_json::json!({"token": "pkce-verified-token", "id_token": id_token})
                    .to_string(),
            );
    });

    let idp_origin = url::Url::parse(&idp_server.base_url())?;
    let callback_json = serde_json::json!({
        "params": {"idp_code": "raw-code-from-callback", "mode": "pkce"},
        "code_verifier": code_verifier,
    })
    .to_string();
    let output = plugin
        .call_guest_for_test(
            "sso_parse_callback",
            callback_json,
            &identity_url,
            Some(idp_origin),
        )
        .await?;
    let parsed: serde_json::Value = serde_json::from_str(&output)?;

    mock.assert();
    let error = parsed
        .get("error")
        .and_then(|v| v.as_str())
        .ok_or("expected a mismatched nonce to surface as a guest-level error")?;
    assert!(
        error.contains("nonce"),
        "expected the error to mention the nonce mismatch, got: {error}"
    );
    Ok(())
}

/// The blocking HTTP client `idp_http_request`'s call site builds must not
/// silently follow redirects: every SSRF control this module's other tests
/// exercise (the `auth_via_sso` pre-check, the per-call re-check inside
/// `idp_http_request`, the denylist itself) only ever inspects the *first*
/// hop's destination, so a redirect response would otherwise bypass all of
/// them. The mock IdP token endpoint answers with a `302` pointing
/// elsewhere on the same mock server; if the client followed it, the guest
/// would see whatever that second, unmocked path returns (a `404` from
/// `httpmock`'s default) instead of the `302` itself, and the mock would
/// see two requests instead of one.
#[tokio::test(flavor = "multi_thread")]
async fn idp_http_request_does_not_follow_redirects() -> Result<(), Box<dyn std::error::Error>> {
    let server = MockServer::start();
    let mock = server.mock(|when, then| {
        when.method(POST)
            .path("/token")
            .json_body(serde_json::json!({"code": "raw-code-from-callback"}));
        then.status(302)
            .header("Location", "/token-followed-elsewhere");
    });

    let plugin = WasmAuthPlugin::load(&fixture_path())?;
    let identity_url = url::Url::parse("https://keystone.example.test/v3")?;
    let idp_origin = url::Url::parse(&server.base_url())?;

    let callback_json =
        serde_json::json!({"params": {"idp_code": "raw-code-from-callback"}}).to_string();
    let output = plugin
        .call_guest_for_test(
            "sso_parse_callback",
            callback_json,
            &identity_url,
            Some(idp_origin),
        )
        .await?;
    let parsed: serde_json::Value = serde_json::from_str(&output)?;

    // Exactly one request reached the mocked `/token` route: a followed
    // redirect would have produced a second request to
    // `/token-followed-elsewhere`.
    mock.assert_calls(1);
    let error = parsed
        .get("error")
        .and_then(|v| v.as_str())
        .ok_or("expected the guest to surface a non-2xx status as an error")?;
    assert!(
        error.contains("302"),
        "expected the raw 302 status to reach the guest untouched, got: {error}"
    );
    Ok(())
}

/// The per-call SSRF re-check inside `idp_http_request` itself (`host.rs`,
/// distinct from the one-time pre-prompt check in `auth_via_sso`): even
/// though the host already validated `sso_build_request`'s declared origin
/// before ever prompting, every `idp_http_request` call re-resolves and
/// re-checks the bound origin, so a DNS answer that changed between that
/// validation and now (DNS rebinding) can't smuggle a request past the
/// denylist. Uses a link-local address (not loopback) since loopback is
/// exempted under the `fuzzing` feature this test binary builds with (see
/// the happy-path test above) — link-local stays enforced regardless.
#[tokio::test(flavor = "multi_thread")]
async fn idp_http_request_rejects_denylisted_bound_origin_on_every_call()
-> Result<(), Box<dyn std::error::Error>> {
    let plugin = WasmAuthPlugin::load(&fixture_path())?;
    let identity_url = url::Url::parse("https://keystone.example.test/v3")?;
    let idp_origin = url::Url::parse("https://169.254.169.254/")?;

    let callback_json =
        serde_json::json!({"params": {"idp_code": "raw-code-from-callback"}}).to_string();
    let result = plugin
        .call_guest_for_test(
            "sso_parse_callback",
            callback_json,
            &identity_url,
            Some(idp_origin),
        )
        .await;

    let err =
        result.expect_err("a bound idp_origin resolving to a disallowed address must be rejected");
    // Same Extism guest-trap wrapping caveat as
    // `idp_http_request_during_sso_build_request_errors` above: the
    // host-function error text only shows up via `{:?}` (Debug), not `{}`.
    assert!(
        format!("{err:?}")
            .to_lowercase()
            .contains("resolves to a disallowed address"),
        "unexpected error: {err:?}"
    );
    Ok(())
}
