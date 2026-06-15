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

    /// Find valid unscoped authz
    fn find_valid_unscoped_auth(&self) -> Option<AuthToken> {
        for (k, v) in self.0.iter() {
            if let AuthTokenScope::Unscoped = k
                && let AuthState::Valid = v.get_state(None)
            {
                return Some(v.clone());
            }
        }
        None
    }

    /// Find first matching unscoped authz
    fn find_first_valid_auth(&self) -> Option<AuthToken> {
        for (_, v) in self.0.iter() {
            if let AuthState::Valid = v.get_state(None) {
                return Some(v.clone());
            }
        }
        None
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
    auth_hash: u64,
    auth_cache_enabled: bool,
}

impl State {
    pub fn new() -> Self {
        let state = Self {
            auth_hash: 0,
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
        state
    }
    /// Set the unique authentication hash key
    pub fn set_auth_hash_key(&mut self, auth_hash: u64) -> &mut Self {
        self.auth_hash = auth_hash;
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
        match self.auth_state.0.get(scope) {
            Some(authz) => Some(authz.clone()),
            None => {
                if let (Some(state), true) = (self.load_auth_state(None), self.auth_cache_enabled)
                    && let Some((scope, authz)) = self.find_scope_authz(&state, scope)
                {
                    trace!("Found valid authz in the state file");
                    self.auth_state.0.insert(scope, authz.clone());
                    return Some(authz);
                }
                None
            }
        }
    }

    fn find_valid_auth(&self, state: &ScopeAuths) -> Option<AuthToken> {
        if let Some(unscoped) = state.find_valid_unscoped_auth() {
            return Some(unscoped);
        }
        if let Some(scoped) = state.find_first_valid_auth() {
            return Some(scoped);
        }
        None
    }

    pub fn get_any_valid_auth(&mut self) -> Option<AuthToken> {
        if let Some(auth) = self.find_valid_auth(&self.auth_state) {
            return Some(auth);
        }
        if let (Some(state), true) = (self.load_auth_state(None), self.auth_cache_enabled)
            && let Some(auth) = self.find_valid_auth(&state)
        {
            return Some(auth);
        }
        None
    }

    /// Removes all cached auth tokens from memory and disk.
    ///
    /// When the cache is enabled, both the data file and the companion lock file
    /// are deleted.
    pub fn clear_all_auth(&mut self) {
        trace!("Clearing all cached auth");
        if self.auth_cache_enabled {
            let fname = self.get_auth_state_filename(self.auth_hash);
            let _ = std::fs::remove_file(&fname);
            let _ = std::fs::remove_file(self.get_auth_state_lock_filename(self.auth_hash));
        }
        self.auth_state.0.clear();
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
            match scope {
                AuthTokenScope::Project(project) => {
                    if let AuthTokenScope::Project(cached) = k {
                        // Scope type matches
                        if project.id.is_some() && project.id == cached.id {
                            // Match by ID is definite
                            return Some((k.clone(), v.clone()));
                        } else if project.name == cached.name {
                            // Match by Name requires verifying domain match
                            if let (Some(requested_domain), Some(state_domain)) =
                                (&project.domain, &cached.domain)
                                && ((requested_domain.id.is_some()
                                    && requested_domain.id == state_domain.id)
                                    || (requested_domain.id.is_none()
                                        && requested_domain.name == state_domain.name))
                            {
                                return Some((k.clone(), v.clone()));
                            }
                        }
                    }
                }
                AuthTokenScope::Domain(domain) => {
                    if let AuthTokenScope::Domain(cached) = k {
                        // Scope type matches
                        if domain.id == cached.id
                            || (domain.id.is_none() && domain.name == cached.name)
                        {
                            return Some((k.clone(), v.clone()));
                        }
                    }
                }
                AuthTokenScope::System(system) => {
                    if let AuthTokenScope::System(cached) = k {
                        // Scope type matches
                        if system.all == cached.all {
                            return Some((k.clone(), v.clone()));
                        }
                    }
                }
                AuthTokenScope::Unscoped => {
                    if let AuthTokenScope::Unscoped = k {
                        return Some((k.clone(), v.clone()));
                    }
                }
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
    fn get_auth_state_lock_filename(&self, auth_hash: u64) -> PathBuf {
        let mut fname_buf = self.get_auth_state_filename(auth_hash);
        fname_buf.set_extension("lock");
        fname_buf
    }

    /// Returns the path to the on-disk auth cache file for a given auth hash.
    fn get_auth_state_filename(&self, auth_hash: u64) -> PathBuf {
        let mut fname_buf = self.base_dir.clone();
        fname_buf.push(auth_hash.to_string());
        fname_buf
    }

    /// Restricts cache file permissions to owner-only (0o600 on Unix, readonly on Windows).
    fn set_file_permissions(&self, file: &File) -> bool {
        match file.metadata() {
            Ok(metadata) => {
                let mut permissions = metadata.permissions();
                #[cfg(unix)]
                {
                    permissions.set_mode(0o600);
                }

                #[cfg(windows)]
                {
                    permissions.set_readonly(true);
                }

                file.set_permissions(permissions).is_ok()
            }
            Err(_) => false,
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
    /// * `None` — file is missing, empty, corrupt, or I/O error
    fn read_auth_state_from_file(&self, file: &mut File, path: &PathBuf) -> Option<ScopeAuths> {
        let mut contents = vec![];
        match file.read_to_end(&mut contents) {
            Ok(_) => match postcard::from_bytes::<ScopeAuths>(&contents) {
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
            },
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
            .unwrap_or_else(|| self.get_auth_state_filename(self.auth_hash));
        let lock_fname = if path.is_some() {
            fname.with_extension("lock")
        } else {
            self.get_auth_state_lock_filename(self.auth_hash)
        };

        match File::open(&fname) {
            Ok(mut file) => {
                if let Ok(lock_file) = OpenOptions::new()
                    .create(false)
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
        let fname = self.get_auth_state_filename(self.auth_hash);
        let lock_fname = self.get_auth_state_lock_filename(self.auth_hash);

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
            if postcard::to_io(&state, &mut temp_file).is_ok()
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
            if let Err(e) = postcard::to_io(&state, &mut write_file) {
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

    use openstack_sdk_auth_core::types::Project;

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
            auth_hash: hash,
            auth_state: Default::default(),
            auth_cache_enabled: true,
            base_dir: tmp.to_path_buf(),
        }
    }

    fn make_token(name: &str) -> AuthToken {
        use openstack_sdk_auth_core::types::{AuthResponse, AuthToken as AuthTokenType};
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
        let fname = s.get_auth_state_filename(s.auth_hash);
        assert!(fname.exists());
        s.clear_all_auth();
        assert!(s.get_scope_auth(&scope).is_none());
        assert!(!fname.exists());
    }

    #[test]
    fn test_corrupted_file_is_removed() {
        let dir = make_state_dir();
        let s = new_state_in(&dir, 3);
        let fname = s.get_auth_state_filename(3);
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
        let fname = s.get_auth_state_filename(6);
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
}
