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

//! Process-wide registry of loaded [`WasmAuthPlugin`]s.
//!
//! Plugins live in a single directory on disk (by default
//! `<data-dir>/osc/plugins`, overridable via `OSC_PLUGIN_DIR`). On first use
//! the registry lazily loads every `*.wasm` file found there; [`install`]
//! validates and copies a new plugin into that directory and loads it
//! immediately so it's available for the rest of the process's lifetime.
//!
//! This is intentionally simple (no hot-reload, no uninstall-while-running):
//! plugins are resolved once per process, matching how the compiled-in,
//! `inventory`-based auth plugins already behave.

use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::{Arc, OnceLock, RwLock};

use crate::error::WasmPluginError;
use crate::plugin::WasmAuthPlugin;

#[derive(Default)]
struct Registry {
    /// All successfully loaded plugins, keyed by plugin (file stem) name.
    by_name: HashMap<String, Arc<WasmAuthPlugin>>,
    /// Index from a supported auth method name to the plugin implementing it.
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

/// The directory plugins are loaded from and installed into.
///
/// Overridable via the `OSC_PLUGIN_DIR` environment variable; otherwise
/// `<data-dir>/osc/plugins`.
pub fn plugin_dir() -> Result<PathBuf, WasmPluginError> {
    if let Some(dir) = std::env::var_os("OSC_PLUGIN_DIR") {
        return Ok(PathBuf::from(dir));
    }
    dirs::data_dir()
        .map(|d| d.join("osc").join("plugins"))
        .ok_or(WasmPluginError::PluginDirCannotBeIdentified)
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

/// Load every `*.wasm` file directly inside `dir` into the registry.
///
/// Missing directories are treated as "no plugins installed" rather than an
/// error. Returns the number of plugins successfully loaded.
pub fn load_dir(dir: &Path) -> Result<usize, WasmPluginError> {
    let entries = match fs::read_dir(dir) {
        Ok(entries) => entries,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(0),
        Err(source) => {
            return Err(WasmPluginError::Io {
                path: dir.to_path_buf(),
                source,
            });
        }
    };

    let mut loaded = 0;
    let mut reg = lock_write()?;
    for entry in entries {
        let entry = entry.map_err(|source| WasmPluginError::Io {
            path: dir.to_path_buf(),
            source,
        })?;
        let path = entry.path();
        if path.extension().and_then(|e| e.to_str()) != Some("wasm") {
            continue;
        }
        let plugin = WasmAuthPlugin::load(&path)?;
        insert(&mut reg, plugin);
        loaded += 1;
    }
    Ok(loaded)
}

/// Ensure the default plugin directory has been scanned and loaded, exactly
/// once per process. Safe to call repeatedly and from multiple call sites
/// (e.g. once from SDK client construction, again defensively before auth
/// dispatch).
pub fn ensure_loaded() -> Result<(), WasmPluginError> {
    {
        let reg = lock_read()?;
        if reg.loaded_default_dir {
            return Ok(());
        }
    }
    let dir = plugin_dir()?;
    load_dir(&dir)?;
    let mut reg = lock_write()?;
    reg.loaded_default_dir = true;
    Ok(())
}

/// Look up the plugin implementing a given auth method name, if any is
/// currently loaded. Does not itself trigger loading; call [`ensure_loaded`]
/// first.
pub fn lookup(method: &str) -> Result<Option<Arc<WasmAuthPlugin>>, WasmPluginError> {
    Ok(lock_read()?.by_method.get(method).cloned())
}

/// All currently loaded plugins.
pub fn list_loaded() -> Result<Vec<Arc<WasmAuthPlugin>>, WasmPluginError> {
    Ok(lock_read()?.by_name.values().cloned().collect())
}

/// Validate a `.wasm` file at `src_path` and install it into the plugin
/// directory, loading it into the registry immediately.
///
/// Installing a plugin under a name that's already installed is rejected;
/// remove the existing file from the plugin directory first to replace it.
pub fn install(src_path: &Path) -> Result<Arc<WasmAuthPlugin>, WasmPluginError> {
    // Validate before touching the plugin directory: a malformed module
    // should never be copied in.
    let probe = WasmAuthPlugin::load(src_path)?;
    let name = probe.name().to_string();

    let dir = plugin_dir()?;
    fs::create_dir_all(&dir).map_err(|source| WasmPluginError::Io {
        path: dir.clone(),
        source,
    })?;
    let dest = dir.join(format!("{name}.wasm"));
    if dest.exists() {
        return Err(WasmPluginError::AlreadyInstalled(name));
    }
    fs::copy(src_path, &dest).map_err(|source| WasmPluginError::Io {
        path: dest.clone(),
        source,
    })?;

    // Reload from the installed location so `source()` reflects where the
    // plugin actually lives from now on.
    let plugin = WasmAuthPlugin::load(&dest)?;
    let mut reg = lock_write()?;
    Ok(insert(&mut reg, plugin))
}
