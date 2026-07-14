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

//! SDK connection state and auth cache
//!
//! A session object keeps certain information that may need to be cached. This module implements
//! caching of authentication/authorization tokens in memory and on disk.
//!
//! # On-disk cache
//!
//! When auth cache is enabled, tokens are persisted in `~/.osc/<auth_hash>` using
//! [postcard](https://crates.io/crates/postcard) (binary) serialization.
//! Each auth hash gets its own file, so different credentials do not interfere.
//!
//! # Concurrency and file locking
//!
//! Multiple processes may access the same cache file simultaneously. To protect the
//! read-modify-write cycle in [`State::save_scope_auth_to_file`], a dedicated lock file
//! (`<auth_hash>.lock`) is used. The lock is acquired exclusively before reading existing
//! state and held until the atomic write (`NamedTempFile::persist`) completes. This prevents
//! data loss from concurrent writers overwriting each other's updates.
//!
//! Reads (`load_auth_state`) use a shared lock on the same lock file, allowing multiple
//! concurrent readers while blocking writes.
//!
//! Lock acquisition failures are tolerated (logged as warnings) — the cache has
//! best-effort semantics, and a failed lock does not prevent a process from operating.

use std::collections::HashMap;
use std::fs::{DirBuilder, File, OpenOptions};
use std::io::prelude::*;
#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use tempfile::NamedTempFile;
use tracing::{debug, info, trace, warn};

use openstack_sdk_auth_core::{AuthState, authtoken::AuthToken, authtoken_scope::AuthTokenScope};

/// Margin applied when deciding whether a cached token is usable for a new request.
///
/// A token valid at read time but expiring a few seconds later would still be handed
/// out and immediately 401 (or worse, land a non-idempotent request exactly at expiry).
/// This margin is only for "is this usable right now" checks; persistence-time
/// filtering (deciding what is worth writing to the cache) keeps using `None`.
const VALIDITY_MARGIN: chrono::TimeDelta = chrono::TimeDelta::seconds(60);

/// Cache files older than this are considered abandoned (stale credentials, a
/// toolchain bump that changed the hash, ...) and are garbage-collected on
/// [`State::new()`]. Chosen well above any realistic token lifetime.
const MAX_CACHE_FILE_AGE: std::time::Duration = std::time::Duration::from_secs(7 * 24 * 60 * 60);

/// On-disk cache format version, written as the first byte of the file ahead of
/// the postcard payload. Lets a schema change be distinguished from corruption
/// (and logged as such) instead of both looking like "corrupted → removed".
const CACHE_FORMAT_VERSION: u8 = 1;

/// Discovered service endpoints almost never change for a given cloud, so a
/// fairly long TTL is used here (unlike the short [`VALIDITY_MARGIN`] applied
/// to auth tokens). Entries older than this are treated as a cache miss and
/// re-discovered over HTTP.
const MAX_DISCOVERY_CACHE_AGE: chrono::TimeDelta = chrono::TimeDelta::hours(24);

/// On-disk format version for the discovery cache file. Kept separate from
/// [`CACHE_FORMAT_VERSION`] since the two caches evolve independently.
const DISCOVERY_CACHE_FORMAT_VERSION: u8 = 1;

/// A single cached version-discovery result.
///
/// Rather than caching the parsed [`ServiceEndpoint`](crate::catalog::ServiceEndpoint)
/// (which is not `Serialize`), the raw discovery document response body is cached
/// together with the URL it was fetched from. On a cache hit the caller re-parses
/// the document locally (no network I/O), reusing the exact same parsing path as a
/// live discovery.
#[derive(Clone, Deserialize, Serialize, Debug)]
struct DiscoveryEntry {
    /// URL the discovery document was fetched from.
    url: String,
    /// Raw discovery document response body.
    data: Vec<u8>,
    /// Unix timestamp (seconds) when this entry was written.
    discovered_at: i64,
}

/// On-disk discovery cache: keyed by `"<service_type>|<region>|<interface>"`.
#[derive(Clone, Default, Deserialize, Serialize, Debug)]
struct DiscoveryEntries(HashMap<String, DiscoveryEntry>);

/// Build the cache key for a discovery cache entry.
fn discovery_cache_key(
    service_type: &str,
    region: Option<&str>,
    interface: Option<&str>,
) -> String {
    format!(
        "{}|{}|{}",
        service_type,
        region.unwrap_or(""),
        interface.unwrap_or("")
    )
}

/// Relative priority of a scope when several cached tokens could serve as a
/// generic "any valid auth" seed for re-scoping. Lower is preferred.
fn scope_priority(scope: &AuthTokenScope) -> u8 {
    match scope {
        AuthTokenScope::Unscoped => 0,
        AuthTokenScope::Project(_) => 1,
        AuthTokenScope::Domain(_) => 2,
        AuthTokenScope::System(_) => 3,
    }
}

/// In-memory store of authentication tokens keyed by scope.
///
/// Tokens are kept valid by [`ScopeAuths::filter_invalid_auths`] which is called
/// on every read/write operation.
#[derive(Clone, Default, Deserialize, Serialize, Debug)]
struct ScopeAuths(HashMap<AuthTokenScope, AuthToken>);

impl ScopeAuths {
    /// Filter out all invalid auth data keeping only valid ones
    fn filter_invalid_auths(&mut self) -> &mut Self {
        self.0.retain(|_, v| AuthState::Valid == v.get_state(None));
        self
    }

    /// Find the best token to seed a new request with: unscoped first, then by
    /// [`scope_priority`] (in order: unscoped, project, domain, system), deterministically rather
    /// than in arbitrary `HashMap` iteration order.
    fn find_valid_auth(&self) -> Option<AuthToken> {
        self.0
            .iter()
            .filter(|(_, v)| AuthState::Valid == v.get_state(Some(VALIDITY_MARGIN)))
            .min_by_key(|(k, _)| scope_priority(k))
            .map(|(_, v)| v.clone())
    }
}

/// SDK session state — manages auth tokens in memory and on disk.
///
/// Tokens are stored per-scope in a [`ScopeAuths`] map. When the auth cache
/// is enabled, new tokens are also persisted to a binary file in `~/.osc/`.
/// The cache is keyed by an auth hash so different credentials use separate files.
#[derive(Clone, Default, Deserialize, Serialize)]
pub struct State {
    /// Auth/Authz state
    auth_state: ScopeAuths,
    base_dir: PathBuf,
    auth_hash: String,
    auth_cache_enabled: bool,
}

impl State {
    pub fn new() -> Self {
        let state = Self {
            auth_hash: String::new(),
            auth_state: Default::default(),
            auth_cache_enabled: false,
            base_dir: dirs::home_dir()
                .unwrap_or_default()
                //.expect("Cannot determine users XDG_HOME")
                .join(".osc"),
        };
        DirBuilder::new()
            .recursive(true)
            .create(state.base_dir.clone())
            .ok();
        #[cfg(unix)]
        {
            if let Ok(metadata) = std::fs::metadata(&state.base_dir) {
                let mut permissions = metadata.permissions();
                permissions.set_mode(0o700);
                let _ = std::fs::set_permissions(&state.base_dir, permissions);
            }
        }
        state.gc_stale_cache_files();
        state
    }

    /// Best-effort removal of cache/lock files that have not been touched in
    /// [`MAX_CACHE_FILE_AGE`]. Every credential or toolchain change mints a new
    /// `<hash>`/`<hash>.lock` pair under `~/.osc/` and the old ones are never
    /// otherwise cleaned up.
    ///
    /// `State::new()` can run more than once per process (e.g. switching clouds
    /// in the TUI), so this is gated to a single directory scan per process
    /// rather than repeating the I/O on every construction.
    fn gc_stale_cache_files(&self) {
        static DONE: std::sync::Once = std::sync::Once::new();
        let mut ran = false;
        DONE.call_once(|| ran = true);
        if !ran {
            return;
        }

        let Ok(entries) = std::fs::read_dir(&self.base_dir) else {
            return;
        };
        for entry in entries.flatten() {
            let path = entry.path();
            let Ok(metadata) = entry.metadata() else {
                continue;
            };
            if !metadata.is_file() {
                continue;
            }
            let is_stale = metadata
                .modified()
                .ok()
                .and_then(|m| m.elapsed().ok())
                .is_some_and(|age| age > MAX_CACHE_FILE_AGE);
            if is_stale {
                let _ = std::fs::remove_file(&path);
            }
        }
    }
    /// Set the unique authentication hash key
    pub fn set_auth_hash_key(&mut self, auth_hash: impl Into<String>) -> &mut Self {
        self.auth_hash = auth_hash.into();
        self
    }

    /// Enable/Disable authorization caching
    pub fn enable_auth_cache(&mut self, val: bool) -> &mut Self {
        self.auth_cache_enabled = val;
        self
    }

    /// Disable all authorization caching (in-memory and on-disk).
    ///
    /// When disabled, `get_scope_auth()` and `get_any_valid_auth()` will
    /// always return `None`, and `set_scope_auth()` will skip both the
    /// in-memory store and file persistence.
    pub fn disable_auth_cache(&mut self) -> &mut Self {
        self.auth_cache_enabled = false;
        self.auth_state.0.clear();
        self
    }

    /// Set authz into the state
    pub fn set_scope_auth(&mut self, scope: &AuthTokenScope, authz: &AuthToken) {
        self.auth_state.filter_invalid_auths();
        self.auth_state.0.insert(scope.clone(), authz.clone());
        if self.auth_cache_enabled {
            self.save_scope_auth_to_file(scope, authz);
        }
    }

    /// Get authz for requested scope from the state
    pub fn get_scope_auth(&mut self, scope: &AuthTokenScope) -> Option<AuthToken> {
        trace!("Get authz information for {:?}", scope);
        self.auth_state.filter_invalid_auths();

        // Find the best matching scope using wildcard matching, applying the
        // validity margin since this token is about to be used for a request.
        if let Some((_, authz)) = self.auth_state.0.iter().find(|(k, v)| {
            scope.matches(k) && AuthState::Valid == v.get_state(Some(VALIDITY_MARGIN))
        }) {
            return Some(authz.clone());
        }

        if let (Some(state), true) = (self.load_auth_state(None), self.auth_cache_enabled)
            && let Some((scope, authz)) = self.find_scope_authz(&state, scope)
        {
            trace!("Found valid authz in the state file");
            self.auth_state.0.insert(scope, authz.clone());
            return Some(authz);
        }
        None
    }

    pub fn get_any_valid_auth(&mut self) -> Option<AuthToken> {
        if let Some(auth) = self.auth_state.find_valid_auth() {
            return Some(auth);
        }
        if let (Some(state), true) = (self.load_auth_state(None), self.auth_cache_enabled)
            && let Some(auth) = state.find_valid_auth()
        {
            return Some(auth);
        }
        None
    }

    /// Returns the path to the on-disk auth cache file that would be used for the
    /// current auth hash, if the on-disk cache is enabled.
    pub fn cache_file_path(&self) -> Option<PathBuf> {
        if self.auth_cache_enabled {
            Some(self.get_auth_state_filename(&self.auth_hash))
        } else {
            None
        }
    }

    /// Removes all cached auth tokens from memory and disk.
    ///
    /// When the cache is enabled, both the data file and the companion lock file
    /// are deleted.
    pub fn clear_all_auth(&mut self) {
        trace!("Clearing all cached auth");
        if self.auth_cache_enabled {
            let fname = self.get_auth_state_filename(&self.auth_hash);
            let _ = std::fs::remove_file(&fname);
            let _ = std::fs::remove_file(self.get_auth_state_lock_filename(&self.auth_hash));
        }
        self.auth_state.0.clear();
    }

    /// Returns the discovery document body previously cached for
    /// `(service_type, region, interface)`, along with the URL it was fetched from,
    /// if a fresh (within [`MAX_DISCOVERY_CACHE_AGE`]) entry exists.
    ///
    /// Returns `None` when the cache is disabled, the file is missing/corrupt, or
    /// the entry is stale.
    pub fn get_discovery_cache(
        &self,
        service_type: &str,
        region: Option<&str>,
        interface: Option<&str>,
    ) -> Option<(String, Vec<u8>)> {
        if !self.auth_cache_enabled {
            return None;
        }
        let entries = self.load_discovery_state()?;
        let key = discovery_cache_key(service_type, region, interface);
        let entry = entries.0.get(&key)?;
        let now = chrono::Utc::now().timestamp();
        if now - entry.discovered_at > MAX_DISCOVERY_CACHE_AGE.num_seconds() {
            trace!("Discovery cache entry for `{}` is stale", key);
            return None;
        }
        Some((entry.url.clone(), entry.data.clone()))
    }

    /// Persists a discovery document body for `(service_type, region, interface)`
    /// to the on-disk discovery cache.
    ///
    /// Best-effort: locking/IO failures are logged and otherwise ignored.
    pub fn set_discovery_cache(
        &self,
        service_type: &str,
        region: Option<&str>,
        interface: Option<&str>,
        url: &str,
        data: Vec<u8>,
    ) {
        if !self.auth_cache_enabled {
            return;
        }
        let key = discovery_cache_key(service_type, region, interface);
        let entry = DiscoveryEntry {
            url: url.to_string(),
            data,
            discovered_at: chrono::Utc::now().timestamp(),
        };
        self.save_discovery_entry_to_file(&key, &entry);
    }

    /// Returns the path to the on-disk discovery cache file for the current auth hash.
    fn get_discovery_state_filename(&self) -> PathBuf {
        let mut fname_buf = self.base_dir.clone();
        fname_buf.push(format!("{}.discovery", self.auth_hash));
        fname_buf
    }

    /// Returns the path to the lock file guarding the discovery cache file.
    fn get_discovery_state_lock_filename(&self) -> PathBuf {
        let mut fname_buf = self.get_discovery_state_filename();
        fname_buf.set_extension("discovery.lock");
        fname_buf
    }

    /// Reads and deserializes the discovery cache file, if present and valid.
    fn read_discovery_state_from_file(
        &self,
        file: &mut File,
        path: &PathBuf,
    ) -> Option<DiscoveryEntries> {
        let mut contents = vec![];
        match file.read_to_end(&mut contents) {
            Ok(_) => {
                let (&version, payload) = contents.split_first()?;
                if version != DISCOVERY_CACHE_FORMAT_VERSION {
                    info!(
                        "Discovery cache file `{}` has format version {} (expected {}); removing.",
                        path.display(),
                        version,
                        DISCOVERY_CACHE_FORMAT_VERSION
                    );
                    let _ = std::fs::remove_file(path);
                    return None;
                }
                match postcard::from_bytes::<DiscoveryEntries>(payload) {
                    Ok(entries) => Some(entries),
                    Err(x) => {
                        info!(
                            "Corrupted discovery cache file `{}`: {:?}. Removing",
                            path.display(),
                            x
                        );
                        let _ = std::fs::remove_file(path);
                        None
                    }
                }
            }
            Err(e) => {
                info!(
                    "Error reading discovery cache file `{}`: {:?}",
                    path.display(),
                    e
                );
                None
            }
        }
    }

    /// Loads the discovery cache with a shared lock (mirrors [`State::load_auth_state`]).
    fn load_discovery_state(&self) -> Option<DiscoveryEntries> {
        let fname = self.get_discovery_state_filename();
        let lock_fname = self.get_discovery_state_lock_filename();

        match File::open(&fname) {
            Ok(mut file) => {
                if let Ok(lock_file) = OpenOptions::new()
                    .create(true)
                    .write(true)
                    .truncate(false)
                    .read(true)
                    .open(&lock_fname)
                    && let Err(e) = lock_file.lock_shared()
                {
                    warn!(
                        "Failed to acquire shared lock on `{}`: {:?}",
                        lock_fname.display(),
                        e
                    );
                }
                self.read_discovery_state_from_file(&mut file, &fname)
            }
            Err(e) => {
                debug!(
                    "Error opening discovery cache file `{}`: {:?}",
                    fname.display(),
                    e
                );
                None
            }
        }
    }

    /// Persists a single discovery entry to the on-disk cache (mirrors
    /// [`State::save_scope_auth_to_file`]: exclusive lock, read-modify-write, atomic
    /// replace via [`NamedTempFile::persist`], with a direct-write fallback).
    fn save_discovery_entry_to_file(&self, key: &str, entry: &DiscoveryEntry) {
        let fname = self.get_discovery_state_filename();
        let lock_fname = self.get_discovery_state_lock_filename();

        let lock_file = match OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(false)
            .open(&lock_fname)
        {
            Ok(f) => f,
            Err(e) => {
                warn!(
                    "Failed to open discovery cache lock file `{}`: {:?}",
                    lock_fname.display(),
                    e
                );
                return;
            }
        };

        if let Err(e) = lock_file.lock() {
            warn!(
                "Failed to acquire exclusive lock on `{}`: {:?}",
                lock_fname.display(),
                e
            );
            return;
        }

        let opt_loaded = if let Ok(mut f) = File::open(&fname) {
            if let Err(e) = f.lock_shared() {
                warn!(
                    "Failed to acquire shared lock on `{}`: {:?}",
                    fname.display(),
                    e
                );
            }
            self.read_discovery_state_from_file(&mut f, &fname)
        } else {
            None
        };

        let mut state = opt_loaded.unwrap_or_default();
        state.0.insert(key.to_string(), entry.clone());

        if let Ok(mut temp_file) = NamedTempFile::new_in(&self.base_dir) {
            if temp_file
                .write_all(&[DISCOVERY_CACHE_FORMAT_VERSION])
                .is_ok()
                && postcard::to_io(&state, &mut temp_file).is_ok()
                && self.set_file_permissions(temp_file.as_file())
            {
                if temp_file.persist(&fname).is_err() {
                    warn!(
                        "Error persisting discovery cache file to `{}`",
                        fname.display()
                    );
                }
                return;
            }
        } else {
            warn!("Error creating temp file for discovery cache");
        }

        // Fallback: direct write
        if let Ok(mut write_file) = File::create(&fname) {
            if !self.set_file_permissions(&write_file) {
                warn!("Cannot set permissions for the discovery cache file");
                return;
            }
            if let Err(e) = write_file.write_all(&[DISCOVERY_CACHE_FORMAT_VERSION]) {
                warn!("Error writing discovery cache format version: {:?}", e);
            } else if let Err(e) = postcard::to_io(&state, &mut write_file) {
                warn!("Error serializing discovery cache state: {:?}", e);
            }
        } else {
            warn!("Error writing discovery cache file");
        }
    }

    /// Locate authz for requested scope in the state
    fn find_scope_authz(
        &self,
        state: &ScopeAuths,
        scope: &AuthTokenScope,
    ) -> Option<(AuthTokenScope, AuthToken)> {
        trace!("Searching requested scope authz in state");
        for (k, v) in state.0.iter() {
            trace!("Analyse known auth for scope {:?}", k);
            if scope.matches(k) && AuthState::Valid == v.get_state(Some(VALIDITY_MARGIN)) {
                return Some((k.clone(), v.clone()));
            }
        }
        None
    }

    /// Returns the path to the lock file for a given auth hash.
    ///
    /// The lock file (`<auth_hash>.lock`) is used for exclusive/shared file locking
    /// during cache read-modify-write operations. It is kept separate from the data
    /// file so that [`NamedTempFile::persist`] can atomically replace the data file
    /// without releasing the lock.
    fn get_auth_state_lock_filename(&self, auth_hash: &str) -> PathBuf {
        let mut fname_buf = self.get_auth_state_filename(auth_hash);
        fname_buf.set_extension("lock");
        fname_buf
    }

    /// Returns the path to the on-disk auth cache file for a given auth hash.
    fn get_auth_state_filename(&self, auth_hash: &str) -> PathBuf {
        let mut fname_buf = self.base_dir.clone();
        fname_buf.push(auth_hash);
        fname_buf
    }

    /// Restricts cache file permissions to owner-only (0o600 on Unix).
    ///
    /// On Windows this is a no-op: the profile directory's default ACL already
    /// restricts access to the owning user, and marking the file read-only does
    /// not add any real confidentiality (a read-only attribute is not an ACL —
    /// any process running as the same user can still clear it and read the
    /// file). It actively breaks cache *updates*: the next write goes through
    /// `NamedTempFile::persist`/`File::create`, both of which fail against a
    /// read-only target.
    #[cfg_attr(not(unix), allow(unused_variables, clippy::unnecessary_wraps))]
    fn set_file_permissions(&self, file: &File) -> bool {
        #[cfg(unix)]
        {
            match file.metadata() {
                Ok(metadata) => {
                    let mut permissions = metadata.permissions();
                    permissions.set_mode(0o600);
                    file.set_permissions(permissions).is_ok()
                }
                Err(_) => false,
            }
        }
        #[cfg(not(unix))]
        {
            true
        }
    }

    /// Reads and deserializes auth state from an already-open file.
    ///
    /// Invalid tokens (expired or unscoped with no auth info) are filtered out after
    /// deserialization. If the file contains corrupted data, the file is removed so
    /// it does not interfere with future cache operations.
    ///
    /// # Returns
    ///
    /// * `Some(ScopeAuths)` — deserialized state with only valid tokens
    /// * `None` — file is missing, empty, corrupt, wrong format version, or I/O error
    fn read_auth_state_from_file(&self, file: &mut File, path: &PathBuf) -> Option<ScopeAuths> {
        let mut contents = vec![];
        match file.read_to_end(&mut contents) {
            Ok(_) => {
                let (&version, payload) = contents.split_first()?;
                if version != CACHE_FORMAT_VERSION {
                    info!(
                        "Cache file `{}` has format version {} (expected {}); a newer/older \
                         osc wrote it. Removing.",
                        path.display(),
                        version,
                        CACHE_FORMAT_VERSION
                    );
                    let _ = std::fs::remove_file(path);
                    return None;
                }
                match postcard::from_bytes::<ScopeAuths>(payload) {
                    Ok(mut auth) => {
                        auth.filter_invalid_auths();
                        trace!("Cached Auth info loaded");
                        Some(auth)
                    }
                    Err(x) => {
                        info!(
                            "Corrupted cache file `{}`: {:?}. Removing ",
                            path.display(),
                            x
                        );
                        let _ = std::fs::remove_file(path);
                        None
                    }
                }
            }
            Err(e) => {
                info!("Error reading file `{}`: {:?}", path.display(), e);
                None
            }
        }
    }

    /// Loads auth state from the cache file with a shared lock.
    ///
    /// Opens the lock file and acquires a shared lock before reading the cache file,
    /// allowing concurrent readers while blocking writers. A failed lock acquisition
    /// is logged as a warning and does not prevent the read.
    ///
    /// # Arguments
    ///
    /// * `path` — optional explicit path; when `None` the default cache file is used
    fn load_auth_state(&self, path: Option<&PathBuf>) -> Option<ScopeAuths> {
        let fname = path
            .cloned()
            .unwrap_or_else(|| self.get_auth_state_filename(&self.auth_hash));
        let lock_fname = if path.is_some() {
            fname.with_extension("lock")
        } else {
            self.get_auth_state_lock_filename(&self.auth_hash)
        };

        match File::open(&fname) {
            Ok(mut file) => {
                if let Ok(lock_file) = OpenOptions::new()
                    .create(true)
                    .write(true)
                    .truncate(false)
                    .read(true)
                    .open(&lock_fname)
                    && let Err(e) = lock_file.lock_shared()
                {
                    warn!(
                        "Failed to acquire shared lock on `{}`: {:?}",
                        lock_fname.display(),
                        e
                    );
                }
                self.read_auth_state_from_file(&mut file, &fname)
            }
            Err(e) => {
                debug!("Error opening file `{}`: {:?}", fname.display(), e);
                None
            }
        }
    }

    /// Persists a single auth token to the on-disk cache.
    ///
    /// 1. Opens the lock file and acquires an exclusive lock
    /// 2. Reads any existing tokens from the cache file
    /// 3. Inserts the new token into the in-memory state
    /// 4. Writes the full state atomically via [`NamedTempFile::persist`]
    /// 5. Releases the lock when the `File` handle is dropped
    ///
    /// The exclusive lock is held on a separate `.lock` file, not the data file itself,
    /// so it survives the atomic replace performed by [`NamedTempFile::persist`]. This
    /// ensures concurrent writers cannot interleave their updates and lose data.
    ///
    /// If the atomic write fails, a fallback path writes directly to the cache file.
    pub fn save_scope_auth_to_file(&self, scope: &AuthTokenScope, data: &AuthToken) {
        let fname = self.get_auth_state_filename(&self.auth_hash);
        let lock_fname = self.get_auth_state_lock_filename(&self.auth_hash);

        // Acquire exclusive lock on lock file. The lock survives `persist` replacing
        // the data file, so concurrent writes are properly synchronized.
        let lock_file = match OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(false)
            .open(&lock_fname)
        {
            Ok(f) => f,
            Err(e) => {
                warn!(
                    "Failed to open lock file `{}`: {:?}",
                    lock_fname.display(),
                    e
                );
                return;
            }
        };

        if let Err(e) = lock_file.lock() {
            warn!(
                "Failed to acquire exclusive lock on `{}`: {:?}",
                lock_fname.display(),
                e
            );
            return;
        }

        // Read existing cache while holding the exclusive lock
        let opt_loaded = if let Ok(mut f) = File::open(&fname) {
            if let Err(e) = f.lock_shared() {
                warn!(
                    "Failed to acquire shared lock on `{}`: {:?}",
                    fname.display(),
                    e
                );
            }
            self.read_auth_state_from_file(&mut f, &fname)
        } else {
            None
        };

        let mut state = opt_loaded.unwrap_or_default();
        let _ = state.0.insert(scope.clone(), data.clone());

        // Try atomic write: create temp file in same directory, persist to replace data file
        if let Ok(mut temp_file) = NamedTempFile::new_in(&self.base_dir) {
            if temp_file.write_all(&[CACHE_FORMAT_VERSION]).is_ok()
                && postcard::to_io(&state, &mut temp_file).is_ok()
                && self.set_file_permissions(temp_file.as_file())
            {
                if temp_file.persist(&fname).is_err() {
                    warn!("Error persisting cache file to `{}`", fname.display());
                }
                // The exclusive lock on `lock_file` is still held; it will release on drop
                return;
            }
        } else {
            warn!("Error creating temp file for cache");
        }

        // Fallback: direct write
        if let Ok(mut write_file) = File::create(&fname) {
            if !self.set_file_permissions(&write_file) {
                warn!("Cannot set permissions for the cache file");
                return;
            }
            if let Err(e) = write_file.write_all(&[CACHE_FORMAT_VERSION]) {
                warn!("Error writing cache format version: {:?}", e);
            } else if let Err(e) = postcard::to_io(&state, &mut write_file) {
                warn!("Error serializing state: {:?}", e);
            }
        } else {
            warn!("Error writing state file");
        }
        // lock_file drops here, releasing exclusive lock
    }
}

#[cfg(test)]
mod tests {
    //! Tests for auth cache persistence, file locking, and concurrency.
    use std::fs::OpenOptions;
    use std::path::Path;
    use std::sync::Arc;
    use std::sync::OnceLock;

    use secrecy::ExposeSecret;

    use openstack_sdk_auth_core::types::{Domain, Project};

    use super::*;

    fn test_tmp_dir() -> &'static Arc<tempfile::TempDir> {
        static DIR: OnceLock<Arc<tempfile::TempDir>> = OnceLock::new();
        DIR.get_or_init(|| std::sync::Arc::new(tempfile::tempdir().unwrap()))
    }

    fn make_state_dir() -> PathBuf {
        let d = test_tmp_dir().path().join(format!(
            "test_{}_{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .subsec_nanos()
        ));
        std::fs::create_dir_all(&d).unwrap();
        d
    }

    fn new_state_in(tmp: &Path, hash: u64) -> State {
        State {
            auth_hash: hash.to_string(),
            auth_state: Default::default(),
            auth_cache_enabled: true,
            base_dir: tmp.to_path_buf(),
        }
    }

    fn make_token(name: &str) -> AuthToken {
        use openstack_sdk_auth_core::types::{AuthResponse, TokenInfo as AuthTokenType};
        AuthToken::new(
            name,
            Some(AuthResponse {
                token: AuthTokenType {
                    user: Default::default(),
                    catalog: Some(vec![]),
                    expires_at: chrono::Utc::now() + chrono::TimeDelta::hours(1),
                    ..Default::default()
                },
            }),
        )
    }

    #[test]
    fn test_postcard_roundtrip_token() {
        let token = make_token("tr");
        let bytes = postcard::to_stdvec(&token).unwrap();
        let deserialized: AuthToken = postcard::from_bytes(&bytes).unwrap();
        assert_eq!(deserialized.token.expose_secret(), "tr");
    }

    #[test]
    fn test_postcard_roundtrip_scope_auths() {
        let scope = make_project_scope("test");
        let token = make_token("tr");
        let mut sa = ScopeAuths::default();
        sa.0.insert(scope, token);
        let bytes = postcard::to_stdvec(&sa).unwrap();
        let deserialized: ScopeAuths = postcard::from_bytes(&bytes).unwrap();
        assert!(deserialized.0.len() == 1);
    }

    fn make_project_scope(name: &str) -> AuthTokenScope {
        AuthTokenScope::Project(Project {
            id: Some(name.to_string()),
            name: None,
            domain: None,
        })
    }

    #[test]
    fn test_save_and_load_token() {
        let dir = make_state_dir();
        let mut s = new_state_in(&dir, 1);
        let scope = make_project_scope("p1");
        let token = make_token("tok1");
        s.set_scope_auth(&scope, &token);
        let mut s2 = new_state_in(&dir, 1);
        let loaded = s2.get_scope_auth(&scope);
        assert!(loaded.is_some());
        assert_eq!(loaded.unwrap().token.expose_secret(), "tok1");
    }

    #[test]
    fn test_clear_all_auth_removes_file() {
        let dir = make_state_dir();
        let mut s = new_state_in(&dir, 2);
        let scope = make_project_scope("p2");
        let token = make_token("tok2");
        s.set_scope_auth(&scope, &token);
        let fname = s.get_auth_state_filename(&s.auth_hash);
        assert!(fname.exists());
        s.clear_all_auth();
        assert!(s.get_scope_auth(&scope).is_none());
        assert!(!fname.exists());
    }

    #[test]
    fn test_corrupted_file_is_removed() {
        let dir = make_state_dir();
        let s = new_state_in(&dir, 3);
        let fname = s.get_auth_state_filename("3");
        OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&fname)
            .unwrap()
            .write_all(b"corrupt")
            .unwrap();
        assert!(fname.exists());
        let loaded = s.load_auth_state(None);
        assert!(loaded.is_none());
        assert!(!fname.exists());
    }

    #[test]
    fn test_get_any_valid_auth_returns_token() {
        let dir = make_state_dir();
        let mut s = new_state_in(&dir, 4);
        let scope = make_project_scope("p4");
        let token = make_token("tok4");
        s.set_scope_auth(&scope, &token);
        let any = s.get_any_valid_auth();
        assert!(any.is_some());
        assert_eq!(any.unwrap().token.expose_secret(), "tok4");
    }

    #[test]
    fn test_get_any_valid_auth_falls_back_to_file() {
        let dir = make_state_dir();
        let mut s = new_state_in(&dir, 5);
        let scope = make_project_scope("p5");
        let token = make_token("tok5");
        s.set_scope_auth(&scope, &token);
        let mut s2 = new_state_in(&dir, 5);
        s2.auth_state.0.clear();
        let any = s2.get_any_valid_auth();
        assert!(any.is_some());
        assert_eq!(any.unwrap().token.expose_secret(), "tok5");
    }

    #[test]
    fn test_cache_disabled_skips_file_write() {
        let dir = make_state_dir();
        let mut s = new_state_in(&dir, 6);
        s.auth_cache_enabled = false;
        let scope = make_project_scope("p6");
        let token = make_token("tok6");
        s.set_scope_auth(&scope, &token);
        let fname = s.get_auth_state_filename("6");
        assert!(!fname.exists());
    }

    #[test]
    fn test_concurrent_writers_no_data_loss() {
        let dir = make_state_dir();
        let hash = 100;
        let path = std::sync::Arc::new(dir.clone());
        let handles = (0..4)
            .map(|i| {
                let p = path.clone();
                std::thread::spawn(move || {
                    let mut s = new_state_in(&p, hash);
                    let scope = make_project_scope(&format!("concurrent-p{i}"));
                    let token = make_token(&format!("tokc-{i}"));
                    s.set_scope_auth(&scope, &token);
                    (scope, token)
                })
            })
            .collect::<Vec<_>>();

        let results: Vec<_> = handles.into_iter().map(|h| h.join().unwrap()).collect();

        let mut s = new_state_in(&dir, hash);
        s.auth_state.0.clear();
        for (scope, _token) in &results {
            let loaded = s.get_scope_auth(scope);
            assert!(loaded.is_some(), "Token for scope {:?} was lost", scope);
        }
    }

    #[test]
    fn test_wildcard_lookup_by_partial_scope() {
        let dir = make_state_dir();
        let mut s = new_state_in(&dir, 20);
        let scope_cached = AuthTokenScope::Project(Project {
            id: Some("p-wildcard".to_string()),
            name: Some("Wildcard".to_string()),
            domain: None,
        });
        let token = make_token("tok-wildcard");
        s.set_scope_auth(&scope_cached, &token);

        // Lookup by name only (id=None) should find the cached token
        let scope_req_by_name = AuthTokenScope::Project(Project {
            id: None,
            name: Some("Wildcard".to_string()),
            domain: None,
        });
        let loaded = s.get_scope_auth(&scope_req_by_name);
        assert!(loaded.is_some());
        assert_eq!(loaded.unwrap().token.expose_secret(), "tok-wildcard");
    }

    #[test]
    fn test_multiple_scopes_in_same_cache() {
        let dir = make_state_dir();
        let mut s = new_state_in(&dir, 21);
        let scope1 = make_project_scope("multi-p1");
        let scope2 = make_project_scope("multi-p2");
        let scope3 = make_project_scope("multi-p3");
        let token1 = make_token("tok-multi-1");
        let token2 = make_token("tok-multi-2");
        let token3 = make_token("tok-multi-3");
        s.set_scope_auth(&scope1, &token1);
        s.set_scope_auth(&scope2, &token2);
        s.set_scope_auth(&scope3, &token3);

        assert_eq!(s.auth_state.0.len(), 3);

        let t1 = s.get_scope_auth(&scope1).unwrap();
        let t2 = s.get_scope_auth(&scope2).unwrap();
        let t3 = s.get_scope_auth(&scope3).unwrap();
        assert_eq!(t1.token.expose_secret(), "tok-multi-1");
        assert_eq!(t2.token.expose_secret(), "tok-multi-2");
        assert_eq!(t3.token.expose_secret(), "tok-multi-3");
    }

    #[test]
    fn test_domain_scope_cache() {
        let dir = make_state_dir();
        let mut s = new_state_in(&dir, 22);
        let scope_d = AuthTokenScope::Domain(Domain {
            id: Some("d-22".to_string()),
            name: Some("Default".to_string()),
        });
        let token_d = make_token("tok-domain");
        s.set_scope_auth(&scope_d, &token_d);

        let loaded = s.get_scope_auth(&scope_d);
        assert!(loaded.is_some());
        assert_eq!(loaded.unwrap().token.expose_secret(), "tok-domain");
    }

    #[test]
    fn test_unscoped_token_cache() {
        let dir = make_state_dir();
        let mut s = new_state_in(&dir, 23);
        let scope_u = AuthTokenScope::Unscoped;
        let token_u = make_token("tok-unscoped");
        s.set_scope_auth(&scope_u, &token_u);

        let loaded = s.get_scope_auth(&scope_u);
        assert!(loaded.is_some());
        assert_eq!(loaded.unwrap().token.expose_secret(), "tok-unscoped");
    }

    #[test]
    fn test_invalid_token_is_filtered_on_set() {
        use openstack_sdk_auth_core::{
            authtoken::AuthToken as TestAuthToken,
            types::{AuthResponse, TokenInfo},
        };
        let dir = make_state_dir();
        let mut s = new_state_in(&dir, 24);
        let scope = make_project_scope("p-24");
        let valid_token = make_token("tok-valid");
        s.set_scope_auth(&scope, &valid_token);
        assert_eq!(s.auth_state.0.len(), 1);

        // Create an expired token
        let expired_token = TestAuthToken::new(
            "tok-expired".to_string(),
            Some(AuthResponse {
                token: TokenInfo {
                    user: Default::default(),
                    catalog: Some(vec![]),
                    expires_at: chrono::Utc::now() - chrono::TimeDelta::hours(1),
                    ..Default::default()
                },
            }),
        );
        // Setting the expired token should filter out the valid one (since it's the only valid one and now a new expired one is set)
        s.set_scope_auth(&scope, &expired_token);
        // filter_invalid_auths removes all invalid tokens; the expired one should be removed too
        // Actually, filter runs before insert, so it keeps the old valid token but then the new one replaces it
        assert_eq!(s.auth_state.0.len(), 1);
        // The expired token should have been filtered out on the next get
        s.auth_state.filter_invalid_auths();
        assert_eq!(s.auth_state.0.len(), 0);
    }

    #[test]
    fn test_postcard_roundtrip_multiple_scopes() {
        let scope1 = make_project_scope("postcard-p1");
        let scope2 = make_project_scope("postcard-p2");
        let scope3 = AuthTokenScope::Domain(Domain {
            id: Some("d-postcard".to_string()),
            name: Some("Default".to_string()),
        });
        let token1 = make_token("tok-pc-1");
        let token2 = make_token("tok-pc-2");
        let token3 = make_token("tok-pc-3");
        let mut sa = ScopeAuths::default();
        sa.0.insert(scope1, token1);
        sa.0.insert(scope2, token2);
        sa.0.insert(scope3, token3);
        let bytes = postcard::to_stdvec(&sa).unwrap();
        let deserialized: ScopeAuths = postcard::from_bytes(&bytes).unwrap();
        assert_eq!(deserialized.0.len(), 3);
    }

    #[test]
    fn test_wildcard_lookup_in_file_cache() {
        let dir = make_state_dir();
        let mut s = new_state_in(&dir, 25);
        let scope_cached = AuthTokenScope::Project(Project {
            id: Some("file-wildcard".to_string()),
            name: Some("FileLookup".to_string()),
            domain: None,
        });
        let token = make_token("tok-file-wildcard");
        s.set_scope_auth(&scope_cached, &token);

        // Create a new state with cleared memory to force file lookup
        let mut s2 = new_state_in(&dir, 25);
        s2.auth_state.0.clear();

        // Lookup by name only (id=None) should find the cached token in the file
        let scope_req_by_name = AuthTokenScope::Project(Project {
            id: None,
            name: Some("FileLookup".to_string()),
            domain: None,
        });
        let loaded = s2.get_scope_auth(&scope_req_by_name);
        assert!(loaded.is_some());
        assert_eq!(loaded.unwrap().token.expose_secret(), "tok-file-wildcard");
    }

    #[test]
    fn test_project_and_domain_scope_coexist() {
        let dir = make_state_dir();
        let mut s = new_state_in(&dir, 26);
        let scope_p = make_project_scope("coexist-p");
        let scope_d = AuthTokenScope::Domain(Domain {
            id: Some("d-26".to_string()),
            name: Some("Default".to_string()),
        });
        let token_p = make_token("tok-p-coexist");
        let token_d = make_token("tok-d-coexist");
        s.set_scope_auth(&scope_p, &token_p);
        s.set_scope_auth(&scope_d, &token_d);
        assert_eq!(s.auth_state.0.len(), 2);

        let tp = s.get_scope_auth(&scope_p);
        let td = s.get_scope_auth(&scope_d);
        assert!(tp.is_some());
        assert!(td.is_some());
        assert_eq!(tp.unwrap().token.expose_secret(), "tok-p-coexist");
        assert_eq!(td.unwrap().token.expose_secret(), "tok-d-coexist");
    }

    #[test]
    fn test_discovery_cache_roundtrip() {
        let dir = make_state_dir();
        let s = new_state_in(&dir, 100);
        assert!(
            s.get_discovery_cache("compute", Some("RegionOne"), Some("public"))
                .is_none()
        );
        s.set_discovery_cache(
            "compute",
            Some("RegionOne"),
            Some("public"),
            "http://example.com/v2.1/",
            b"{\"version\": {}}".to_vec(),
        );
        let cached = s.get_discovery_cache("compute", Some("RegionOne"), Some("public"));
        assert!(cached.is_some());
        let (url, data) = cached.unwrap();
        assert_eq!(url, "http://example.com/v2.1/");
        assert_eq!(data, b"{\"version\": {}}".to_vec());

        // A different key (region) must miss.
        assert!(
            s.get_discovery_cache("compute", Some("RegionTwo"), Some("public"))
                .is_none()
        );
    }

    #[test]
    fn test_discovery_cache_disabled_is_noop() {
        let dir = make_state_dir();
        let mut s = new_state_in(&dir, 101);
        s.disable_auth_cache();
        s.set_discovery_cache("compute", None, None, "http://example.com/", vec![1, 2, 3]);
        assert!(s.get_discovery_cache("compute", None, None).is_none());
        // No file should have been written.
        assert!(!s.get_discovery_state_filename().exists());
    }

    #[test]
    fn test_discovery_cache_stale_entry_ignored() {
        let dir = make_state_dir();
        let s = new_state_in(&dir, 102);
        let key = discovery_cache_key("compute", None, None);
        let entry = DiscoveryEntry {
            url: "http://example.com/".into(),
            data: vec![9, 9, 9],
            discovered_at: chrono::Utc::now().timestamp()
                - MAX_DISCOVERY_CACHE_AGE.num_seconds()
                - 10,
        };
        s.save_discovery_entry_to_file(&key, &entry);
        assert!(s.get_discovery_cache("compute", None, None).is_none());
    }

    #[test]
    fn test_discovery_cache_persists_across_instances() {
        let dir = make_state_dir();
        let s1 = new_state_in(&dir, 103);
        s1.set_discovery_cache("identity", None, None, "http://example.com/v3/", vec![7, 7]);
        let s2 = new_state_in(&dir, 103);
        let cached = s2.get_discovery_cache("identity", None, None);
        assert!(cached.is_some());
        assert_eq!(cached.unwrap().1, vec![7, 7]);
    }
}
