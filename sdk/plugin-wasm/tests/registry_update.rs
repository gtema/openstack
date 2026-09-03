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

//! End-to-end test of [`registry::update`]: a mocked registry index serving
//! two versions of the fixture plugin, exercised through every
//! [`registry::UpdateOutcome`] variant (`Updated`, `Declined`, `UpToDate`,
//! `SkippedPinned`, `NotInIndex`) plus its `name`/`all` argument validation
//! and the pinned-by-name error path.
//!
//! This is the only test in this binary so it can safely set the
//! process-wide `OSC_PLUGIN_DIR`/`OSC_PLUGIN_LOCKFILE` environment variables
//! and rely on the registry's own process-lifetime state, mirroring
//! `tests/registry.rs` and `tests/registry_remote.rs`.

use std::path::PathBuf;

use chrono::Utc;

use openstack_sdk_plugin_wasm::error::WasmPluginError;
use openstack_sdk_plugin_wasm::lockfile::{self, PluginEntry, PluginLockfile, TrustInfo};
use openstack_sdk_plugin_wasm::{index, registry};

fn fixture_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/example_auth.wasm")
}

#[tokio::test(flavor = "multi_thread")]
async fn update_walks_every_outcome() -> Result<(), Box<dyn std::error::Error>> {
    let plugin_dir = tempfile::tempdir()?;
    let lockfile_dir = tempfile::tempdir()?;
    // SAFETY: this is the only test in this binary, so nothing else races on
    // the process environment.
    unsafe {
        std::env::set_var("OSC_PLUGIN_DIR", plugin_dir.path());
        std::env::set_var(
            "OSC_PLUGIN_LOCKFILE",
            lockfile_dir.path().join("plugins.lock"),
        );
    }

    let fixture = fixture_path();
    let sha256 = lockfile::sha256_hex(&fixture)?;
    let bytes = std::fs::read(&fixture)?;

    // Serve the same fixture bytes under two declared versions; the index
    // metadata (not the wasm module itself) is what `update` resolves
    // "latest" against.
    let server = httpmock::MockServer::start();
    server.mock(|when, then| {
        when.method(httpmock::Method::GET).path("/index.json");
        then.status(200).json_body(serde_json::json!({
            "schema_version": 1,
            "plugins": [
                {
                    "name": "example_auth",
                    "description": "Example auth plugin",
                    "versions": [
                        {
                            "version": "1.0.0",
                            "download_url": format!("{}/example_auth.wasm", server.base_url()),
                            "sha256": sha256,
                            "source_repo": "gtema/nonexistent-plugin-fixture-xyz"
                        },
                        {
                            "version": "2.0.0",
                            "download_url": format!("{}/example_auth.wasm", server.base_url()),
                            "sha256": sha256,
                            "source_repo": "gtema/nonexistent-plugin-fixture-xyz"
                        }
                    ]
                }
            ]
        }));
    });
    server.mock(|when, then| {
        when.method(httpmock::Method::GET)
            .path("/example_auth.wasm");
        then.status(200).body(bytes);
    });

    let client = index::http_client()?;
    let registry_url = format!("{}/index.json", server.base_url());

    // A name with nothing installed under it is a hard error, not an
    // UpdateOutcome.
    let err = registry::update(
        Some("does-not-exist"),
        false,
        &registry_url,
        &client,
        true,
        |_| true,
    )
    .await
    .unwrap_err();
    assert!(matches!(err, WasmPluginError::NotInstalled { .. }));

    // Neither a name nor --all is also a hard error.
    let err = registry::update(None, false, &registry_url, &client, true, |_| true)
        .await
        .unwrap_err();
    assert!(matches!(err, WasmPluginError::Registry(_)));

    // Install the older version locally so `update` has something to act on.
    registry::install(&fixture, Some("1.0.0"), false)?;

    // Declining the offered update leaves the active version untouched.
    let outcomes = registry::update(
        Some("example_auth"),
        false,
        &registry_url,
        &client,
        true,
        |_| false,
    )
    .await?;
    assert_eq!(outcomes.len(), 1);
    assert!(matches!(
        &outcomes[0],
        registry::UpdateOutcome::Declined { name, version }
            if name == "example_auth" && version == "1.0.0"
    ));
    assert_eq!(
        registry::installed()?.active.get("example_auth"),
        Some(&"1.0.0".to_string())
    );

    // Confirming it updates to the latest version in the index.
    let outcomes = registry::update(
        Some("example_auth"),
        false,
        &registry_url,
        &client,
        true,
        |_| true,
    )
    .await?;
    assert_eq!(outcomes.len(), 1);
    assert!(matches!(
        &outcomes[0],
        registry::UpdateOutcome::Updated { name, from, to }
            if name == "example_auth" && from == "1.0.0" && to == "2.0.0"
    ));

    // Already at the latest version in the index: no confirm callback needed.
    let outcomes = registry::update(
        Some("example_auth"),
        false,
        &registry_url,
        &client,
        true,
        |_| panic!("confirm must not be called when already up to date"),
    )
    .await?;
    assert_eq!(outcomes.len(), 1);
    assert!(matches!(
        &outcomes[0],
        registry::UpdateOutcome::UpToDate { name, version }
            if name == "example_auth" && version == "2.0.0"
    ));

    // Pin the now-active version directly via the lockfile (the only way
    // `update` itself ever sets `pinned` is `false`), then confirm
    // requesting it by name is rejected...
    let mut lf = PluginLockfile::load()?;
    lf.plugins
        .get_mut(&lockfile::entry_key("example_auth", "2.0.0"))
        .ok_or("expected example_auth@2.0.0 to be installed")?
        .pinned = true;
    lf.save()?;

    let err = registry::update(
        Some("example_auth"),
        false,
        &registry_url,
        &client,
        true,
        |_| true,
    )
    .await
    .unwrap_err();
    assert!(matches!(err, WasmPluginError::Registry(_)));

    // ...and that `update --all` reports it as skipped rather than erroring
    // or touching it.
    let mut lf = PluginLockfile::load()?;
    lf.plugins.insert(
        lockfile::entry_key("ghost_plugin", "1.0.0"),
        PluginEntry {
            name: "ghost_plugin".into(),
            version: "1.0.0".into(),
            sha256: "deadbeef".into(),
            source: PathBuf::from("/tmp/ghost.wasm"),
            installed_at: Utc::now(),
            trust: TrustInfo {
                confirmed_by_user: true,
                allow_unsigned: true,
            },
            pinned: false,
            provenance: None,
        },
    );
    lf.active.insert("ghost_plugin".into(), "1.0.0".into());
    lf.save()?;

    let outcomes = registry::update(None, true, &registry_url, &client, true, |_| true).await?;
    assert_eq!(outcomes.len(), 2);
    assert!(matches!(
        &outcomes[0],
        registry::UpdateOutcome::SkippedPinned { name, version }
            if name == "example_auth" && version == "2.0.0"
    ));
    assert!(matches!(
        &outcomes[1],
        registry::UpdateOutcome::NotInIndex { name } if name == "ghost_plugin"
    ));

    Ok(())
}
