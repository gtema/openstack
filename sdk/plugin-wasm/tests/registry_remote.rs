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

//! End-to-end test of the remote registry install path: a mocked registry
//! index + download served over HTTP, resolved and checksum-verified by
//! [`registry::plan_remote_install`], then finalized by
//! [`registry::finalize_install`].
//!
//! The plugin's declared `source_repo` here is deliberately not a real,
//! attested repository, so provenance verification comes back `Unverified`
//! deterministically (whether this sandbox's egress proxy blocks the GitHub
//! API call outright or a reachable GitHub API simply has no attestations
//! for this made-up repo, the result is the same `Unverified` outcome
//! either way) — exactly the case that must fail closed without
//! `--allow-unsigned`, and succeed (loudly) with it. Provenance
//! *verification itself* (valid/tampered synthetic attestation) is covered
//! separately, and without any network dependency, by the `rcgen`-based
//! unit tests in `src/provenance.rs`.
//!
//! This is the only test in this binary so it can safely set the
//! process-wide `OSC_PLUGIN_DIR`/`OSC_PLUGIN_LOCKFILE` environment variables,
//! mirroring `tests/registry.rs`.

use std::path::PathBuf;

use openstack_sdk_plugin_wasm::index;
use openstack_sdk_plugin_wasm::lockfile;
use openstack_sdk_plugin_wasm::registry;

fn fixture_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/example_auth.wasm")
}

#[tokio::test(flavor = "multi_thread")]
async fn untrusted_install_fails_closed_and_allow_unsigned_is_the_only_way_past_it()
-> Result<(), Box<dyn std::error::Error>> {
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
                            // Not a real, attested repository: provenance
                            // verification is expected to come back
                            // Unverified for it, deterministically.
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

    // Refused before anything is written to disk, without --allow-unsigned.
    let pending =
        registry::plan_remote_install("example_auth", None, &registry_url, &client).await?;
    assert!(matches!(
        pending.provenance,
        registry::ProvenanceOutcome::Unverified { .. }
    ));
    let err = registry::finalize_install(pending, false, false, false).unwrap_err();
    assert!(matches!(
        err,
        openstack_sdk_plugin_wasm::error::WasmPluginError::Untrusted { .. }
    ));
    assert!(
        !registry::plugin_root()?.join("example_auth").exists(),
        "a refused install must not leave anything on disk"
    );
    assert!(
        registry::installed()?
            .entry("example_auth", "1.0.0")
            .is_none()
    );

    // The same plan, but with the explicit escape hatch, succeeds and is
    // recorded as unsigned.
    let pending =
        registry::plan_remote_install("example_auth", None, &registry_url, &client).await?;
    registry::finalize_install(pending, true, false, false)?;

    let lockfile = registry::installed()?;
    let entry = lockfile
        .entry("example_auth", "1.0.0")
        .ok_or("expected example_auth@1.0.0 to be installed")?;
    assert!(entry.trust.allow_unsigned);
    assert!(entry.provenance.is_none());
    assert_eq!(
        lockfile.active.get("example_auth").map(String::as_str),
        Some("1.0.0")
    );

    Ok(())
}
