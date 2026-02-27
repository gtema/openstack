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

//! SDK connection state
//!
//! A session object keeps certain information that may need to be cached. This module implements
//! caching of the authentication/authorization information with certain functionality to manage
//! cache data.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::{DirBuilder, File};
use std::io::prelude::*;
#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;
use tracing::{debug, info, trace, warn};

use crate::auth::{
    AuthState,
    authtoken::{AuthToken, AuthTokenScope},
};

/// A HashMap of Scope to Token
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
            if let AuthTokenScope::Unscoped = k {
                if let AuthState::Valid = v.get_state(None) {
                    return Some(v.clone());
                }
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

/// OSC state
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
                if let (Some(state), true) = (self.load_auth_state(None), self.auth_cache_enabled) {
                    if let Some((scope, authz)) = self.find_scope_authz(&state, scope) {
                        trace!("Found valid authz in the state file");
                        self.auth_state.0.insert(scope, authz.clone());
                        return Some(authz);
                    }
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
        if let (Some(state), true) = (self.load_auth_state(None), self.auth_cache_enabled) {
            if let Some(auth) = self.find_valid_auth(&state) {
                return Some(auth);
            }
        }
        None
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
                            {
                                if (requested_domain.id.is_some()
                                    && requested_domain.id == state_domain.id)
                                    || (requested_domain.id.is_none()
                                        && requested_domain.name == state_domain.name)
                                {
                                    return Some((k.clone(), v.clone()));
                                }
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

    /// Get filename of the auth state file
    fn get_auth_state_filename(&self, auth_hash: u64) -> PathBuf {
        let mut fname_buf = self.base_dir.clone();
        fname_buf.push(auth_hash.to_string());
        fname_buf
    }

    /// Load auth state from the FS
    fn load_auth_state(&self, path: Option<PathBuf>) -> Option<ScopeAuths> {
        let fname = path.unwrap_or(self.get_auth_state_filename(self.auth_hash));
        match File::open(fname.as_path()) {
            Ok(mut file) => {
                let mut contents = vec![];
                match file.read_to_end(&mut contents) {
                    Ok(_) => match postcard::from_bytes::<ScopeAuths>(&contents) {
                        Ok(mut auth) => {
                            auth.filter_invalid_auths();
                            trace!("Cached Auth info: {:?}", auth);
                            Some(auth)
                        }
                        Err(x) => {
                            info!(
                                "Corrupted cache file `{}`: {:?}. Removing ",
                                fname.display(),
                                x
                            );
                            let _ = std::fs::remove_file(fname);
                            None
                        }
                    },
                    Err(e) => {
                        // Not able to read file, maybe it is corrupted. There is nothing user can
                        // or is expected to do about it, but it make sense to make user aware of.
                        info!("Error reading file `{}`: {:?}", fname.display(), e);
                        None
                    }
                }
            }
            Err(e) => {
                // Not able to open file, maybe it is missing. There is nothing user can or is
                // expected to do about it.
                debug!("Error opening file `{}`: {:?}", fname.display(), e);
                None
            }
        }
    }

    /// Save auth state to the FS
    pub fn save_scope_auth_to_file(&self, scope: &AuthTokenScope, data: &AuthToken) {
        let fname = self.get_auth_state_filename(self.auth_hash);
        let mut state = self
            .load_auth_state(Some(fname.clone()))
            .unwrap_or_default();

        let _ = state.0.insert(scope.clone(), data.clone());

        match File::create(fname.as_path()) {
            Ok(mut file) => {
                match file.metadata() {
                    Ok(metadata) => {
                        let mut permissions = metadata.permissions();
                        #[cfg(unix)]
                        {
                            // This code only exists on Linux/macOS/etc.
                            permissions.set_mode(0o600);
                            let _ = file.set_permissions(permissions);
                        }

                        #[cfg(windows)]
                        {
                            // On Windows, only readonly is possible and reasonable.
                            permissions.set_readonly(true);
                            let _ = file.set_permissions(permissions);
                        }
                    }
                    Err(_) => {
                        warn!("Cannot set permissions for the cache file");
                        return;
                    }
                }
                if let Err(e) = postcard::to_io(&state, &mut file) {
                    warn!("Error serializing state: {:?}", e);
                }
            }
            _ => {
                warn!("Error writing state file");
            }
        }
    }
}
