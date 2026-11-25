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

//! On-demand generation of the Extism plugin fixtures used by this crate's
//! integration tests.
//!
//! The `.wasm` files under `tests/fixtures/` are not checked in: the first
//! test that needs one builds it from the guest crate under
//! `fixtures/example-*-plugin/` (a nested, standalone cargo workspace) and
//! caches the result in `tests/fixtures/`, rebuilding only when the guest
//! sources are newer than the artifact. Building requires the
//! `wasm32-unknown-unknown` target (`rustup target add
//! wasm32-unknown-unknown`) and network access to crates.io on first build;
//! the guest crates' committed `Cargo.lock` pins their dependencies.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::Mutex;

/// `(fixture file name, guest crate dir, built cdylib file stem)`.
const FIXTURES: &[(&str, &str, &str)] = &[
    (
        "example_auth.wasm",
        "fixtures/example-auth-plugin",
        "example_wasm_auth_plugin",
    ),
    (
        "example_sso.wasm",
        "fixtures/example-sso-plugin",
        "example_wasm_sso_plugin",
    ),
];

/// Serializes fixture generation: tests within one binary run
/// multi-threaded, and sibling test binaries run in parallel under nextest,
/// so several processes can hit a missing fixture at the same time. Cargo
/// locks its target dir against concurrent builds, and the final copy is an
/// atomic rename, so concurrent generation is safe.
static GENERATION: Mutex<()> = Mutex::new(());

/// Path to the named fixture `.wasm`, generating it on demand if it is
/// missing or older than the guest crate's sources.
pub fn fixture(name: &str) -> Result<PathBuf, Box<dyn std::error::Error>> {
    let (file, guest_dir, lib_stem) = FIXTURES
        .iter()
        .copied()
        .find(|(file, _, _)| *file == name)
        .ok_or_else(|| -> Box<dyn std::error::Error> {
            format!("unknown fixture: {name:?}").into()
        })?;
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let out = manifest_dir.join("tests/fixtures").join(file);
    let guest = manifest_dir.join(guest_dir);
    if !is_stale(&out, &guest) {
        return Ok(out);
    }
    let _guard = GENERATION
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    if is_stale(&out, &guest) {
        generate(&guest, lib_stem, &out)?;
    }
    Ok(out)
}

fn is_stale(out: &Path, guest: &Path) -> bool {
    let Ok(out_modified) = fs::metadata(out).and_then(|m| m.modified()) else {
        return true;
    };
    let mut sources = vec![guest.join("Cargo.toml"), guest.join("Cargo.lock")];
    collect_sources(&guest.join("src"), &mut sources);
    sources.iter().any(|source| {
        fs::metadata(source)
            .and_then(|m| m.modified())
            .is_ok_and(|modified| modified > out_modified)
    })
}

fn collect_sources(dir: &Path, sources: &mut Vec<PathBuf>) {
    let Ok(entries) = fs::read_dir(dir) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            collect_sources(&path, sources);
        } else {
            sources.push(path);
        }
    }
}

fn generate(guest: &Path, lib_stem: &str, out: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let output = Command::new("cargo")
        .args([
            "build",
            "--release",
            "--locked",
            "--target",
            "wasm32-unknown-unknown",
        ])
        .arg("--manifest-path")
        .arg(guest.join("Cargo.toml"))
        // Pin the target dir so a developer's global CARGO_TARGET_DIR cannot
        // move the artifact out from under us; `target/` is gitignored.
        .env("CARGO_TARGET_DIR", guest.join("target"))
        .output()
        .map_err(|err| -> Box<dyn std::error::Error> {
            format!("failed to spawn `cargo build` for {lib_stem}: {err}").into()
        })?;
    if !output.status.success() {
        return Err(format!(
            "`cargo build` of fixture {lib_stem} failed ({}). If the \
            wasm32-unknown-unknown target is missing, install it with \
            `rustup target add wasm32-unknown-unknown`. Cargo output:\n{}",
            output.status,
            String::from_utf8_lossy(&output.stderr).trim()
        )
        .into());
    }
    let built = guest
        .join("target/wasm32-unknown-unknown/release")
        .join(format!("{lib_stem}.wasm"));
    if let Some(parent) = out.parent() {
        fs::create_dir_all(parent)?;
    }
    // Unique per process: sibling test binaries run in parallel under
    // nextest and may generate the same fixture concurrently.
    let tmp = out.with_extension(format!("wasm.tmp.{}", std::process::id()));
    fs::copy(&built, &tmp)?;
    fs::rename(&tmp, out)?;
    Ok(())
}
