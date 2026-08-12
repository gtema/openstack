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

use extism::{Manifest, Plugin, Wasm};
use secrecy::SecretString;

use openstack_sdk_auth_core::OpenStackAuthType;
use openstack_sdk_plugin_wasm::WasmAuthPlugin;
use openstack_sdk_websso_host::CallbackServer;

fn fixture_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/example_sso.wasm")
}

fn raw_plugin() -> Result<Plugin, Box<dyn std::error::Error>> {
    let manifest = Manifest::new([Wasm::file(fixture_path())]).disallow_all_hosts();
    Ok(Plugin::new(manifest, [], false)?)
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
