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

//! Session context for auth, catalog, and state management.

use crate::auth::Auth;
use crate::catalog::Catalog;
use crate::config::{CloudConfig, ConfigFile, get_config_identity_hash};
use crate::state::State;

use crate::error::OpenStackError;

use openstack_sdk_auth_core::authtoken::AuthTokenError;

/// Holds auth, catalog and state together to enable interior mutation.
///
/// Wrapping this behind an `Arc<RwLock>` allows session-mutating operations
/// to take `&self`, which is required by trait methods and eliminates the
/// need for a temporary `session_clone` in re-auth scenarios.
pub struct SessionContext {
    pub auth: Auth,
    pub catalog: Catalog,
    pub state: State,
    /// Runtime region override (set by region switch, takes priority over CloudConfig).
    pub region_name: Option<String>,
    /// Bumped every time `auth` is replaced. Lets a waiter on the re-auth
    /// lock detect that another task already refreshed the token while it
    /// was waiting, so it can skip its own re-auth.
    pub auth_generation: u64,
}

impl SessionContext {
    /// Construct a new SessionContext from CloudConfig.
    ///
    /// This is shared between sync and async clients — the ~30 lines of
    /// catalog/state initialization are identical.
    ///
    /// The `auth_cache` parameter is the session-level override. When `None`,
    /// the CloudConfig's `auth_cache` field is checked, then the global
    /// `ConfigFile` fallback, then defaults to `true`.
    pub fn new(
        config: &CloudConfig,
        auth: Auth,
        auth_cache: Option<bool>,
    ) -> Result<Self, OpenStackError> {
        let mut catalog = Catalog::default();

        let auth_data = config
            .auth
            .as_ref()
            .ok_or(AuthTokenError::MissingAuthData)?;

        let identity_service_url = auth_data
            .auth_url
            .as_ref()
            .ok_or(AuthTokenError::MissingAuthUrl)?;

        catalog.register_catalog_endpoint(
            "identity",
            identity_service_url,
            config.region_name.as_ref(),
            Some("public"),
        )?;

        catalog.configure(config)?;

        // Resolve auth cache priority:
        //   1. explicit parameter override
        //   2. CloudConfig.auth_cache
        //   3. ConfigFile.cache.auth
        //   4. default true
        let auth_cache_enabled = auth_cache
            .or(config.auth_cache)
            .or_else(|| {
                ConfigFile::new()
                    .ok()
                    .and_then(|cf| cf.cache.as_ref().and_then(|c| c.auth))
            })
            .unwrap_or(true);

        let mut state = State::new();
        state
            .set_auth_hash_key(get_config_identity_hash(config))
            .enable_auth_cache(auth_cache_enabled);

        Ok(Self {
            auth,
            catalog,
            state,
            region_name: None,
            auth_generation: 0,
        })
    }
}
