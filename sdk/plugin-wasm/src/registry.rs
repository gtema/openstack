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

//! Process-wide registry of loaded [`WasmAuthPlugin`]s, backed by the
//! on-disk [`PluginLockfile`](crate::lockfile::PluginLockfile).
//!
//! Plugins live at `<plugin-root>/<name>/<version>/<name>.wasm` (by default
//! `<data-dir>/osc/plugins`, overridable via `OSC_PLUGIN_DIR`), one directory
//! per installed version. Multiple versions of the same plugin name may be
//! installed side by side; exactly one of them is the *active* version for
//! that name at any time, recorded in the lockfile. Only the active version
//! is ever loaded into the in-process registry for auth-method resolution.
//!
//! [`install`] makes the newly installed version active. [`remove`] falls
//! back to the most recently installed remaining version if the active one
//! is removed, or clears the name entirely if none remain. Every load
//! (whether at [`install`], [`ensure_loaded`], or explicit [`verify`])
//! re-checks the on-disk file's SHA-256 against the lockfile record before
//! trusting it.
//!
//! This is intentionally simple (no hot-reload of a *different* active
//! version while the process is running): the active plugin for a name is
//! resolved once per process, matching how the compiled-in, `inventory`-based
//! auth plugins already behave.

use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::{Arc, OnceLock, RwLock};

use chrono::Utc;

use crate::error::WasmPluginError;
use crate::lockfile::{self, PluginEntry, PluginLockfile, TrustInfo};
use crate::plugin::WasmAuthPlugin;

/// Version recorded for a plugin installed without an explicit version.
pub const DEFAULT_VERSION: &str = "0.0.0";

#[derive(Default)]
struct Registry {
    /// The active plugin for each installed name.
    by_name: HashMap<String, Arc<WasmAuthPlugin>>,
    /// Index from a supported auth method name to the plugin implementing
    /// it. First plugin to claim a method wins if more than one active
    /// plugin declares it.
    by_method: HashMap<&'static str, Arc<WasmAuthPlugin>>,
    loaded_default_dir: bool,
}

fn registry() -> &'static RwLock<Registry> {
    static REGISTRY: OnceLock<RwLock<Registry>> = OnceLock::new();
    REGISTRY.get_or_init(|| RwLock::new(Registry::default()))
}

fn lock_read() -> Result<std::sync::RwLockReadGuard<'static, Registry>, WasmPluginError> {
    registry()
        .read()
        .map_err(|_| WasmPluginError::Registry("registry lock poisoned".into()))
}

fn lock_write() -> Result<std::sync::RwLockWriteGuard<'static, Registry>, WasmPluginError> {
    registry()
        .write()
        .map_err(|_| WasmPluginError::Registry("registry lock poisoned".into()))
}

/// Root directory plugin versions are installed under:
/// `<plugin_root>/<name>/<version>/<name>.wasm`.
///
/// Overridable via the `OSC_PLUGIN_DIR` environment variable; otherwise
/// `<data-dir>/osc/plugins`.
pub fn plugin_root() -> Result<PathBuf, WasmPluginError> {
    if let Some(dir) = std::env::var_os("OSC_PLUGIN_DIR") {
        return Ok(PathBuf::from(dir));
    }
    dirs::data_dir()
        .map(|d| d.join("osc").join("plugins"))
        .ok_or(WasmPluginError::PluginDirCannotBeIdentified)
}

fn version_dir(name: &str, version: &str) -> Result<PathBuf, WasmPluginError> {
    Ok(plugin_root()?.join(name).join(version))
}

fn version_file(name: &str, version: &str) -> Result<PathBuf, WasmPluginError> {
    Ok(version_dir(name, version)?.join(format!("{name}.wasm")))
}

fn insert(reg: &mut Registry, plugin: WasmAuthPlugin) -> Arc<WasmAuthPlugin> {
    let plugin = Arc::new(plugin);
    for method in plugin.supported_methods() {
        reg.by_method
            .entry(*method)
            .or_insert_with(|| plugin.clone());
    }
    reg.by_name
        .insert(plugin.name().to_string(), plugin.clone());
    plugin
}

/// Recompute `entry`'s on-disk SHA-256 and compare it against the hash
/// recorded in the lockfile at install time.
fn verify_entry(entry: &PluginEntry) -> Result<(), WasmPluginError> {
    let path = version_file(&entry.name, &entry.version)?;
    let actual = lockfile::sha256_hex(&path)?;
    if actual != entry.sha256 {
        return Err(WasmPluginError::HashMismatch {
            name: entry.name.clone(),
            version: entry.version.clone(),
            expected: entry.sha256.clone(),
            actual,
        });
    }
    Ok(())
}

/// Hash-verify and load the active version of `name` into `reg`.
fn load_active(
    reg: &mut Registry,
    lockfile: &PluginLockfile,
    name: &str,
) -> Result<(), WasmPluginError> {
    let entry = lockfile
        .active_entry(name)
        .ok_or_else(|| WasmPluginError::NotInstalled {
            name: name.to_string(),
            version: None,
        })?;
    verify_entry(entry)?;
    let path = version_file(&entry.name, &entry.version)?;
    let plugin = WasmAuthPlugin::load(&path)?;
    insert(reg, plugin);
    Ok(())
}

/// Ensure the active version of every installed plugin has been loaded into
/// the in-process registry, exactly once per process. Safe to call
/// repeatedly and from multiple call sites (e.g. once from SDK client
/// construction, again defensively before auth dispatch).
pub fn ensure_loaded() -> Result<(), WasmPluginError> {
    {
        let reg = lock_read()?;
        if reg.loaded_default_dir {
            return Ok(());
        }
    }
    let lockfile = PluginLockfile::load()?;
    let mut reg = lock_write()?;
    for name in lockfile.active.keys() {
        load_active(&mut reg, &lockfile, name)?;
    }
    reg.loaded_default_dir = true;
    Ok(())
}

/// Look up the plugin implementing a given auth method name, if any is
/// currently loaded. Does not itself trigger loading; call [`ensure_loaded`]
/// first.
pub fn lookup(method: &str) -> Result<Option<Arc<WasmAuthPlugin>>, WasmPluginError> {
    Ok(lock_read()?.by_method.get(method).cloned())
}

/// The active plugins currently loaded into the in-process registry.
pub fn list_loaded() -> Result<Vec<Arc<WasmAuthPlugin>>, WasmPluginError> {
    Ok(lock_read()?.by_name.values().cloned().collect())
}

/// The full lockfile of installed `name@version` entries and their active
/// markers, read directly from disk (independent of what's currently loaded
/// into the in-process registry).
pub fn installed() -> Result<PluginLockfile, WasmPluginError> {
    PluginLockfile::load()
}

/// Validate a `.wasm` file at `src_path` and install it as `name@version`
/// (name is taken from the plugin's own ABI; version defaults to
/// [`DEFAULT_VERSION`] when not given), making it the active version for
/// that name.
///
/// Installing over an already-installed `name@version` is rejected unless
/// `force` is set, in which case the existing file and lockfile entry are
/// replaced.
pub fn install(
    src_path: &Path,
    version: Option<&str>,
    force: bool,
) -> Result<Arc<WasmAuthPlugin>, WasmPluginError> {
    // Validate before touching the plugin directory: a malformed module
    // should never be copied in.
    let probe = WasmAuthPlugin::load(src_path)?;
    let name = probe.name().to_string();
    let version = version.unwrap_or(DEFAULT_VERSION).to_string();

    let mut lf = PluginLockfile::load()?;
    let key = lockfile::entry_key(&name, &version);
    if lf.plugins.contains_key(&key) && !force {
        return Err(WasmPluginError::AlreadyInstalled(format!(
            "{name}@{version}"
        )));
    }

    let dest_dir = version_dir(&name, &version)?;
    fs::create_dir_all(&dest_dir).map_err(|source| WasmPluginError::Io {
        path: dest_dir.clone(),
        source,
    })?;
    let dest = dest_dir.join(format!("{name}.wasm"));
    fs::copy(src_path, &dest).map_err(|source| WasmPluginError::Io {
        path: dest.clone(),
        source,
    })?;
    let sha256 = lockfile::sha256_hex(&dest)?;

    lf.plugins.insert(
        key,
        PluginEntry {
            name: name.clone(),
            version: version.clone(),
            sha256,
            source: src_path.to_path_buf(),
            installed_at: Utc::now(),
            trust: TrustInfo {
                confirmed_by_user: true,
                allow_unsigned: true,
            },
        },
    );
    lf.active.insert(name.clone(), version.clone());
    lf.save()?;

    // Reload from the installed, hash-verified location so `source()`
    // reflects where the plugin actually lives from now on, and make it the
    // active plugin for this name in the in-process registry.
    let mut reg = lock_write()?;
    load_active(&mut reg, &lf, &name)?;
    reg.by_name
        .get(&name)
        .cloned()
        .ok_or_else(|| WasmPluginError::NotInstalled {
            name: name.clone(),
            version: Some(version.clone()),
        })
}

/// Remove an installed plugin. If `version` is `None`, every installed
/// version of `name` is removed; otherwise only that version.
///
/// If the removed version(s) included the active one, the most recently
/// installed remaining version (if any) becomes active. Returns an error if
/// nothing matching `name`/`version` is installed.
pub fn remove(name: &str, version: Option<&str>) -> Result<(), WasmPluginError> {
    let mut lf = PluginLockfile::load()?;

    let to_remove: Vec<String> = match version {
        Some(v) => {
            if lf.entry(name, v).is_none() {
                return Err(WasmPluginError::NotInstalled {
                    name: name.to_string(),
                    version: Some(v.to_string()),
                });
            }
            vec![v.to_string()]
        }
        None => {
            let versions: Vec<String> = lf.versions_of(name).map(|e| e.version.clone()).collect();
            if versions.is_empty() {
                return Err(WasmPluginError::NotInstalled {
                    name: name.to_string(),
                    version: None,
                });
            }
            versions
        }
    };

    for v in &to_remove {
        lf.plugins.remove(&lockfile::entry_key(name, v));
        let dir = version_dir(name, v)?;
        if dir.exists() {
            fs::remove_dir_all(&dir).map_err(|source| WasmPluginError::Io { path: dir, source })?;
        }
    }

    let removed_active = match version {
        Some(v) => lf.active.get(name).map(|a| a == v).unwrap_or(false),
        None => true,
    };
    if removed_active {
        lf.active.remove(name);
        let next = lf
            .versions_of(name)
            .max_by_key(|e| e.installed_at)
            .map(|e| e.version.clone());
        if let Some(v) = next {
            lf.active.insert(name.to_string(), v);
        }
    }
    lf.save()?;

    let mut reg = lock_write()?;
    reg.by_name.remove(name);
    reg.by_method.retain(|_, p| p.name() != name);
    if lf.active.contains_key(name) {
        load_active(&mut reg, &lf, name)?;
    }
    Ok(())
}

/// Re-check the on-disk SHA-256 of installed plugin file(s) against the
/// lockfile. If `version` is `None`, every installed version of `name` is
/// checked. Returns the entries that passed verification; the first mismatch
/// or missing file is returned as an error.
pub fn verify(name: &str, version: Option<&str>) -> Result<Vec<PluginEntry>, WasmPluginError> {
    let lf = PluginLockfile::load()?;
    let entries: Vec<PluginEntry> = match version {
        Some(v) => {
            let entry = lf
                .entry(name, v)
                .ok_or_else(|| WasmPluginError::NotInstalled {
                    name: name.to_string(),
                    version: Some(v.to_string()),
                })?;
            vec![entry.clone()]
        }
        None => {
            let entries: Vec<PluginEntry> = lf.versions_of(name).cloned().collect();
            if entries.is_empty() {
                return Err(WasmPluginError::NotInstalled {
                    name: name.to_string(),
                    version: None,
                });
            }
            entries
        }
    };
    for entry in &entries {
        verify_entry(entry)?;
    }
    Ok(entries)
}
