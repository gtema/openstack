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

//! Integration test for the runtime plugin registry: install a plugin
//! (across multiple versions) into a scratch plugin directory and lockfile,
//! then confirm it's discoverable and manageable via
//! `ensure_loaded`/`list_loaded`/`lookup`/`installed`/`remove`/`verify`.
//!
//! This is the only test in this binary so it can safely set the
//! process-wide `OSC_PLUGIN_DIR`/`OSC_PLUGIN_LOCKFILE` environment variables
//! and rely on the registry's own process-lifetime, load-once semantics
//! without racing another test thread.

use std::fs;
use std::path::PathBuf;

use openstack_sdk_plugin_wasm::registry;

fn fixture_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/example_auth.wasm")
}

#[test]
fn install_then_list_then_lookup() -> Result<(), Box<dyn std::error::Error>> {
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

    // Installing without an explicit version uses the default and becomes
    // active.
    let installed = registry::install(&fixture_path(), Some("1.0.0"), false)?;
    assert_eq!(installed.name(), "example_auth");

    // Installing the same name@version again is rejected without --force.
    assert!(registry::install(&fixture_path(), Some("1.0.0"), false).is_err());
    // ...but succeeds with force.
    registry::install(&fixture_path(), Some("1.0.0"), true)?;

    // A second version installs alongside the first and becomes active.
    registry::install(&fixture_path(), Some("2.0.0"), false)?;

    let lockfile = registry::installed()?;
    let versions: Vec<&str> = lockfile
        .versions_of("example_auth")
        .map(|e| e.version.as_str())
        .collect();
    assert_eq!(versions.len(), 2);
    assert_eq!(
        lockfile.active.get("example_auth").map(String::as_str),
        Some("2.0.0")
    );

    registry::ensure_loaded()?;
    let all = registry::list_loaded()?;
    assert_eq!(all.len(), 1);
    assert_eq!(all[0].name(), "example_auth");

    let found = registry::lookup("v3exampleauth")?;
    assert!(found.is_some());

    let missing = registry::lookup("does-not-exist")?;
    assert!(missing.is_none());

    // Verifying an untampered install succeeds for every installed version.
    let verified = registry::verify("example_auth", None)?;
    assert_eq!(verified.len(), 2);

    // Removing the active version (2.0.0) falls back to the remaining one
    // (1.0.0), which is still untampered at this point so the fallback load
    // succeeds.
    registry::remove("example_auth", Some("2.0.0"))?;
    let lockfile = registry::installed()?;
    assert_eq!(
        lockfile.active.get("example_auth").map(String::as_str),
        Some("1.0.0")
    );

    // Tampering with the remaining installed file is caught on the next
    // verify.
    let entry = lockfile
        .entry("example_auth", "1.0.0")
        .ok_or("expected example_auth@1.0.0 to be installed")?;
    let tampered_path = registry::plugin_root()?
        .join(&entry.name)
        .join(&entry.version)
        .join(format!("{}.wasm", entry.name));
    fs::write(&tampered_path, b"not a real wasm module")?;
    assert!(registry::verify("example_auth", Some("1.0.0")).is_err());

    // Removing every remaining version clears the plugin entirely, from
    // both the lockfile and the in-process registry. This doesn't attempt
    // to load the (tampered) file since no version remains active.
    registry::remove("example_auth", None)?;
    let lockfile = registry::installed()?;
    assert!(lockfile.versions_of("example_auth").next().is_none());
    assert!(!lockfile.active.contains_key("example_auth"));
    assert!(registry::lookup("v3exampleauth")?.is_none());

    Ok(())
}
