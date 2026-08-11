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

//! Integration tests exercising a real Extism (WASM) auth plugin end-to-end:
//! loading and ABI probing, plus the `auth` guest export making an outbound
//! call through the sandboxed `identity_http_request` host function against a
//! mocked identity endpoint.
//!
//! The plugin under test is built from
//! `sdk/plugin-wasm/fixtures/example-auth-plugin` and checked in as
//! `tests/fixtures/example_auth.wasm` so these tests don't need the
//! `wasm32-wasip1` target or a nested `cargo build` at test time.

use std::collections::HashMap;
use std::path::PathBuf;

use httpmock::prelude::*;
use secrecy::{ExposeSecret, SecretString};

use openstack_sdk_auth_core::{Auth, OpenStackAuthType};
use openstack_sdk_plugin_wasm::WasmAuthPlugin;

fn fixture_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/example_auth.wasm")
}

#[test]
fn load_validates_abi() -> Result<(), Box<dyn std::error::Error>> {
    let plugin = WasmAuthPlugin::load(&fixture_path())?;
    assert_eq!(plugin.name(), "example_auth");
    assert_eq!(plugin.supported_methods(), &["v3exampleauth"]);
    assert_eq!(plugin.api_version(), (3, 0));
    assert_eq!(plugin.get_supported_auth_methods(), vec!["v3exampleauth"]);
    Ok(())
}

#[tokio::test(flavor = "multi_thread")]
async fn auth_round_trips_through_host_http() -> Result<(), Box<dyn std::error::Error>> {
    let server = MockServer::start();
    let mock = server.mock(|when, then| {
        when.method(POST).path("/v3/auth/tokens");
        then.status(201)
            .header("X-Subject-Token", "faketoken123")
            .header("Content-Type", "application/json")
            .body(
                r#"{"token": {"expires_at": "2030-01-01T00:00:00Z", "user": {"id": "u1", "name": "demo"}}}"#,
            );
    });

    let plugin = WasmAuthPlugin::load(&fixture_path())?;
    let identity_url = url::Url::parse(&server.base_url())?;
    let http_client = reqwest::Client::new();

    let mut values: HashMap<String, SecretString> = HashMap::new();
    values.insert("username".to_string(), SecretString::from("demo"));
    values.insert("password".to_string(), SecretString::from("secret"));

    let auth = plugin
        .auth(&http_client, &identity_url, &values, None, None)
        .await?;

    mock.assert();

    let Auth::AuthToken(token) = auth else {
        return Err("expected Auth::AuthToken".into());
    };
    assert_eq!(token.token.expose_secret(), "faketoken123");
    let auth_info = token.auth_info.ok_or("missing auth_info in response")?;
    assert_eq!(auth_info.token.user.id, "u1");
    assert_eq!(auth_info.token.user.name, "demo");

    Ok(())
}

#[tokio::test(flavor = "multi_thread")]
async fn auth_missing_credentials_is_rejected_without_a_network_call()
-> Result<(), Box<dyn std::error::Error>> {
    let server = MockServer::start();
    let mock = server.mock(|when, then| {
        when.method(POST).path("/v3/auth/tokens");
        then.status(201);
    });

    let plugin = WasmAuthPlugin::load(&fixture_path())?;
    let identity_url = url::Url::parse(&server.base_url())?;
    let http_client = reqwest::Client::new();
    let values: HashMap<String, SecretString> = HashMap::new();

    let result = plugin
        .auth(&http_client, &identity_url, &values, None, None)
        .await;

    assert!(result.is_err());
    mock.assert_calls(0);

    Ok(())
}
