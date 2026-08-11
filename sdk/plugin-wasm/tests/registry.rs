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

//! Integration test for the runtime plugin registry: install a plugin into a
//! scratch `OSC_PLUGIN_DIR`, then confirm it's discoverable via
//! `ensure_loaded`/`list_loaded`/`lookup`.
//!
//! This is the only test in this binary so it can safely set the
//! process-wide `OSC_PLUGIN_DIR` environment variable and rely on the
//! registry's own process-lifetime, load-once semantics without racing
//! another test thread.

use std::path::PathBuf;

use openstack_sdk_plugin_wasm::registry;

fn fixture_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/example_auth.wasm")
}

#[test]
fn install_then_list_then_lookup() -> Result<(), Box<dyn std::error::Error>> {
    let dir = tempfile::tempdir()?;
    // SAFETY: this is the only test in this binary, so nothing else races on
    // the process environment.
    unsafe {
        std::env::set_var("OSC_PLUGIN_DIR", dir.path());
    }

    let installed = registry::install(&fixture_path())?;
    assert_eq!(installed.name(), "example_auth");

    // Installing a plugin under a name that's already installed is rejected.
    assert!(registry::install(&fixture_path()).is_err());

    registry::ensure_loaded()?;
    let all = registry::list_loaded()?;
    assert_eq!(all.len(), 1);
    assert_eq!(all[0].name(), "example_auth");

    let found = registry::lookup("v3exampleauth")?;
    assert!(found.is_some());

    let missing = registry::lookup("does-not-exist")?;
    assert!(missing.is_none());

    Ok(())
}
