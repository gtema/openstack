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

//! On-disk record of installed plugins: which `name@version` pairs are
//! installed, their content hash, and which version is currently active for
//! each plugin name.
//!
//! Stored as JSON at `OSC_PLUGIN_LOCKFILE`, or `<config-dir>/osc/plugins.lock`
//! by default. Every write goes through [`atomic_write`]: the new content is
//! written to a sibling `.tmp` file and then renamed over the real path, so a
//! process killed mid-write leaves the previous, valid lockfile in place
//! rather than a truncated or partially-written one.

use std::collections::BTreeMap;
use std::fs;
use std::io::Read;
use std::path::{Path, PathBuf};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::error::WasmPluginError;

/// Trust metadata recorded for an installed plugin.
///
/// At this phase there is no signature or provenance source to check
/// against, so installing via `osc plugin install` always records
/// `confirmed_by_user: true` (the user explicitly pointed at the file) and
/// `allow_unsigned: true`.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct TrustInfo {
    /// Whether the user explicitly confirmed installing this plugin.
    pub confirmed_by_user: bool,
    /// Whether an unsigned plugin is allowed to be installed/loaded.
    pub allow_unsigned: bool,
}

/// A single installed `name@version` plugin record.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct PluginEntry {
    /// Plugin name.
    pub name: String,
    /// Plugin version (free-form; defaults to `"0.0.0"` when not specified
    /// at install time).
    pub version: String,
    /// Lowercase hex-encoded SHA-256 of the installed `.wasm` file.
    pub sha256: String,
    /// The path the plugin was installed from (for reference; not read
    /// again after install).
    pub source: PathBuf,
    /// When this entry was installed.
    pub installed_at: DateTime<Utc>,
    /// Trust metadata for this entry.
    pub trust: TrustInfo,
}

/// The full set of installed plugins and which version of each is active.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct PluginLockfile {
    /// Installed `name@version` entries, keyed by [`entry_key`].
    pub plugins: BTreeMap<String, PluginEntry>,
    /// The active version for each installed plugin name. The active
    /// version is the one the registry loads for auth-method resolution.
    pub active: BTreeMap<String, String>,
}

/// The `"name@version"` key an entry is stored under.
pub fn entry_key(name: &str, version: &str) -> String {
    format!("{name}@{version}")
}

/// Path to the lockfile: `OSC_PLUGIN_LOCKFILE` if set, otherwise
/// `<config-dir>/osc/plugins.lock`.
pub fn lockfile_path() -> Result<PathBuf, WasmPluginError> {
    if let Some(path) = std::env::var_os("OSC_PLUGIN_LOCKFILE") {
        return Ok(PathBuf::from(path));
    }
    dirs::config_dir()
        .map(|d| d.join("osc").join("plugins.lock"))
        .ok_or(WasmPluginError::PluginDirCannotBeIdentified)
}

impl PluginLockfile {
    /// Load the lockfile from [`lockfile_path`]. A missing file is treated
    /// as an empty lockfile rather than an error.
    pub fn load() -> Result<Self, WasmPluginError> {
        let path = lockfile_path()?;
        let bytes = match fs::read(&path) {
            Ok(bytes) => bytes,
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(Self::default()),
            Err(source) => return Err(WasmPluginError::Io { path, source }),
        };
        serde_json::from_slice(&bytes)
            .map_err(|source| WasmPluginError::LockfileFormat { path, source })
    }

    /// Serialize and atomically write this lockfile to [`lockfile_path`].
    pub fn save(&self) -> Result<(), WasmPluginError> {
        let path = lockfile_path()?;
        let bytes =
            serde_json::to_vec_pretty(self).map_err(|source| WasmPluginError::LockfileFormat {
                path: path.clone(),
                source,
            })?;
        atomic_write(&path, &bytes)
    }

    /// The entry for a specific `name@version`, if installed.
    pub fn entry(&self, name: &str, version: &str) -> Option<&PluginEntry> {
        self.plugins.get(&entry_key(name, version))
    }

    /// All installed versions of a given plugin name, in `entry_key` order.
    pub fn versions_of<'a>(&'a self, name: &'a str) -> impl Iterator<Item = &'a PluginEntry> {
        self.plugins.values().filter(move |e| e.name == name)
    }

    /// The entry for the currently active version of a plugin name, if any.
    pub fn active_entry(&self, name: &str) -> Option<&PluginEntry> {
        let version = self.active.get(name)?;
        self.entry(name, version)
    }
}

/// Write `bytes` to `path` atomically: write to a sibling `.tmp` file, then
/// rename over `path`. A crash after the write but before the rename leaves
/// `path` untouched; a crash after the rename leaves `path` fully updated.
/// There is no window in which `path` can observe partial content.
fn atomic_write(path: &Path, bytes: &[u8]) -> Result<(), WasmPluginError> {
    let dir = path.parent().unwrap_or_else(|| Path::new("."));
    fs::create_dir_all(dir).map_err(|source| WasmPluginError::Io {
        path: dir.to_path_buf(),
        source,
    })?;
    let mut tmp_name = path
        .file_name()
        .map(|n| n.to_os_string())
        .unwrap_or_default();
    tmp_name.push(".tmp");
    let tmp_path = path.with_file_name(tmp_name);

    fs::write(&tmp_path, bytes).map_err(|source| WasmPluginError::Io {
        path: tmp_path.clone(),
        source,
    })?;
    fs::rename(&tmp_path, path).map_err(|source| WasmPluginError::Io {
        path: path.to_path_buf(),
        source,
    })?;
    Ok(())
}

/// Lowercase hex-encoded SHA-256 of a file's contents, streamed rather than
/// read fully into memory.
pub fn sha256_hex(path: &Path) -> Result<String, WasmPluginError> {
    let mut file = fs::File::open(path).map_err(|source| WasmPluginError::Io {
        path: path.to_path_buf(),
        source,
    })?;
    let mut hasher = Sha256::new();
    let mut buf = [0u8; 64 * 1024];
    loop {
        let n = file.read(&mut buf).map_err(|source| WasmPluginError::Io {
            path: path.to_path_buf(),
            source,
        })?;
        if n == 0 {
            break;
        }
        hasher.update(&buf[..n]);
    }
    let digest = hasher.finalize();
    Ok(digest.iter().map(|b| format!("{b:02x}")).collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// All scenarios below share the `OSC_PLUGIN_LOCKFILE` env var, which is
    /// process-wide state; they're combined into one `#[test]` fn (run
    /// sequentially) rather than split across several, which the default
    /// multi-threaded test harness would race against each other.
    #[test]
    fn lockfile_round_trip_and_crash_safety() -> Result<(), Box<dyn std::error::Error>> {
        let dir = tempfile::tempdir()?;
        let path = dir.path().join("plugins.lock");
        // SAFETY: no other test in this process touches this env var.
        unsafe {
            std::env::set_var("OSC_PLUGIN_LOCKFILE", &path);
        }

        // A missing lockfile loads as an empty default rather than erroring.
        let lf = PluginLockfile::load()?;
        assert!(lf.plugins.is_empty());
        assert!(lf.active.is_empty());

        // Saving and reloading round-trips exactly.
        let mut lf = PluginLockfile::default();
        lf.plugins.insert(
            entry_key("demo", "1.0.0"),
            PluginEntry {
                name: "demo".into(),
                version: "1.0.0".into(),
                sha256: "deadbeef".into(),
                source: PathBuf::from("/tmp/demo.wasm"),
                installed_at: Utc::now(),
                trust: TrustInfo {
                    confirmed_by_user: true,
                    allow_unsigned: true,
                },
            },
        );
        lf.active.insert("demo".into(), "1.0.0".into());
        lf.save()?;

        let reloaded = PluginLockfile::load()?;
        assert_eq!(reloaded, lf);
        let active = reloaded
            .active_entry("demo")
            .ok_or("expected an active entry for demo")?;
        assert_eq!(active.sha256, "deadbeef");

        // Simulate a crash between the `.tmp` write and the rename: a
        // stale/garbage `.tmp` file left on disk must not affect what
        // `PluginLockfile::load` sees.
        let mut tmp_name = path
            .file_name()
            .ok_or("scratch lockfile path has no file name")?
            .to_os_string();
        tmp_name.push(".tmp");
        fs::write(path.with_file_name(tmp_name), b"not valid json")?;

        let reloaded = PluginLockfile::load()?;
        assert_eq!(reloaded, lf);

        // A later successful save still replaces the file cleanly,
        // overwriting the stale `.tmp` in the process.
        let mut lf2 = lf.clone();
        lf2.active.insert("demo".into(), "2.0.0".into());
        lf2.save()?;
        let reloaded2 = PluginLockfile::load()?;
        assert_eq!(reloaded2, lf2);

        Ok(())
    }
}
