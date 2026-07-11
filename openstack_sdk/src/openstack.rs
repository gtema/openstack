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

//! Synchronous OpenStack client — thin blocking facade over [`AsyncOpenStack`].
//!
//! All orchestration logic (authorisation, 401 retry, catalog refresh, version
//! discovery) lives in [`crate::AsyncOpenStack`] and is delegated to via a
//! lazily-initialised, shared multi-threaded Tokio runtime.  This eliminates
//! the per-call [`tokio::runtime::Runtime::new()`] calls and the ~1 000 lines
//! of duplicated auth / catalog code that previously lived in this module.

use std::sync::{Arc, OnceLock};

use tracing::instrument;

use chrono::TimeDelta;
use openstack_sdk_auth_core::{
    Auth,
    authtoken_scope::AuthTokenScope,
    types::{AuthResponse, Project},
};
use openstack_sdk_core::api::{self, Client, RestClient};
use openstack_sdk_core::auth::{AuthState, auth_helper::AuthHelper};
use openstack_sdk_core::config::CloudConfig;
use openstack_sdk_core::error::{OpenStackResult, RestError};
use openstack_sdk_core::types::ServiceType;
use secrecy::SecretString;

use crate::{AsyncOpenStack, types::ApiVersion};

type Runtime = Arc<tokio::runtime::Runtime>;

/// Synchronous client for the OpenStack API for a single user.
///
/// This is a **thin blocking wrapper** around [`AsyncOpenStack`].  All
/// authorisation, 401-retry, catalog-refresh and version-discovery logic is
/// shared — there is no per-call `Runtime::new()` overhead.
///
/// ```rust
/// use openstack_sdk::config::ConfigFile;
/// use openstack_sdk::types::ServiceType;
///
/// fn list_flavors() -> Result<(), openstack_sdk::OpenStackError> {
///     let cfg = ConfigFile::new()?;
///     let profile = cfg.get_cloud_config("devstack")?.unwrap();
///     let session = openstack_sdk::OpenStack::new(&profile)?;
///     session.discover_service_endpoint(&ServiceType::Compute)?;
///     Ok(())
/// }
/// ```
///
/// # Runtime
///
/// A multi-threaded Tokio runtime is created lazily on the first blocking call
/// and shared among all clones of this `OpenStack` instance.  Calling any of the
/// synchronous methods from *within* an already-running Tokio runtime will
/// panic (`Runtime::block_on` is not reentrant).  If you need to use the SDK
/// from inside an async context, use [`AsyncOpenStack`] directly.
pub struct OpenStack {
    inner: AsyncOpenStack,
    runtime: Arc<OnceLock<Runtime>>,
}

impl Clone for OpenStack {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
            runtime: Arc::clone(&self.runtime),
        }
    }
}

impl std::fmt::Debug for OpenStack {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.inner, f)
    }
}

impl RestClient for OpenStack {
    type Error = RestError;

    fn get_current_project(&self) -> Option<Project> {
        self.inner.get_current_project()
    }
}

impl Client for OpenStack {
    fn rest(
        &self,
        request: http::request::Builder,
        body: Vec<u8>,
    ) -> Result<http::Response<bytes::Bytes>, api::ApiError<RestError>> {
        let rt = self.runtime_init();
        rt.block_on(api::AsyncClient::rest_async(&self.inner, request, body))
    }

    fn get_service_endpoint(
        &self,
        service_type: &ServiceType,
        version: Option<&ApiVersion>,
    ) -> Result<openstack_sdk_core::catalog::ServiceEndpoint, api::ApiError<RestError>> {
        let rt = self.runtime_init();
        rt.block_on(api::AsyncClient::get_service_endpoint(
            &self.inner,
            service_type,
            version,
        ))
    }
}

impl OpenStack {
    /// Create a new authenticated session from [`CloudConfig`].
    ///
    /// Performs version-discovery for the Identity service and runs the
    /// authorisation flow.
    #[instrument(name = "connect", level = "trace", skip(config))]
    pub fn new(config: &CloudConfig) -> OpenStackResult<Self> {
        let inner = AsyncOpenStack::new_impl(config, Auth::None)?;
        let s = Self {
            inner,
            runtime: Arc::new(OnceLock::new()),
        };
        s.discover_service_endpoint(&ServiceType::Identity)?;
        s.authorize(None, false, false)?;
        Ok(s)
    }

    /// Create a new session skipping initial service discovery and
    /// authorisation, so that the caller can invoke
    /// [`discover_service_endpoint`] and [`authorize`] manually.
    ///
    /// [`discover_service_endpoint`]: Self::discover_service_endpoint
    /// [`authorize`]: Self::authorize
    pub fn new_no_auth(config: &CloudConfig) -> OpenStackResult<Self> {
        let inner = AsyncOpenStack::new_impl(config, Auth::None)?;
        Ok(Self {
            inner,
            runtime: Arc::new(OnceLock::new()),
        })
    }

    /// Create a session that carries the given pre-constructed [`Auth`].
    pub fn new_with_auth(config: &CloudConfig, auth: Auth) -> OpenStackResult<Self> {
        let inner = AsyncOpenStack::new_impl(config, auth)?;
        Ok(Self {
            inner,
            runtime: Arc::new(OnceLock::new()),
        })
    }

    /// Lazily initialise the shared tokio runtime.
    fn runtime_init(&self) -> Runtime {
        self.runtime
            .get_or_init(|| {
                Arc::new(
                    tokio::runtime::Builder::new_multi_thread()
                        .enable_all()
                        .build()
                        .unwrap_or_else(|e| panic!("failed to create tokio runtime: {e}")),
                )
            })
            .clone()
    }

    // ---------- convenience setters ---------------------------------------

    /// Set the maximum number of retries on 401 responses.
    pub fn set_max_auth_retries(&mut self, n: u32) -> &mut Self {
        self.inner.set_max_auth_retries(n);
        self
    }

    /// Disable authentication caching for this session.
    pub fn disable_auth_cache(&self) -> &Self {
        self.inner.disable_auth_cache();
        self
    }

    /// Set the auth helper used for 401 re-authentication.
    pub fn set_auth_helper<A>(&mut self, auth_helper: A)
    where
        A: AuthHelper + Sync + Send + 'static,
    {
        self.inner.set_auth_helper(auth_helper);
    }

    // ---------- public API (all delegate to the async client) --------------

    /// Authorise the session.
    pub fn authorize(
        &self,
        scope: Option<AuthTokenScope>,
        interactive: bool,
        renew_auth: bool,
    ) -> OpenStackResult<()> {
        let rt = self.runtime_init();
        rt.block_on(self.inner.authorize(scope, interactive, renew_auth))
    }

    /// Authorise the session with a custom [`AuthHelper`].
    pub fn authorize_with_auth_helper<A>(
        &self,
        scope: Option<AuthTokenScope>,
        auth_helper: &A,
        renew_auth: bool,
    ) -> OpenStackResult<()>
    where
        A: AuthHelper + Sync + Send + 'static,
    {
        let rt = self.runtime_init();
        rt.block_on(
            self.inner
                .authorize_with_auth_helper(scope, auth_helper, renew_auth),
        )
    }

    /// Set the region name for endpoint resolution.
    pub fn set_region_name(&self, region: String) -> OpenStackResult<()> {
        self.inner.set_region_name(region)
    }

    /// Return the current effective region name.
    pub fn get_region_name(&self) -> Option<String> {
        self.inner.get_region_name()
    }

    /// Return the current authentication status.
    pub fn get_auth_state(&self, offset: Option<TimeDelta>) -> Option<AuthState> {
        self.inner.get_auth_state(offset)
    }

    /// Return the current auth token.
    pub fn get_auth_token(&self) -> Option<SecretString> {
        self.inner.get_auth_token()
    }

    /// Return the full authentication information (token data).
    pub fn get_auth_info(&self) -> Option<AuthResponse> {
        self.inner.get_auth_info()
    }

    /// Return current project information.
    pub fn get_current_project(&self) -> Option<Project> {
        self.inner.get_current_project()
    }

    /// List available regions from catalog endpoints.
    pub fn get_available_regions(&self) -> Option<Vec<String>> {
        self.inner.get_available_regions()
    }

    /// Perform version discovery of the given service.
    #[instrument(skip(self))]
    pub fn discover_service_endpoint(&self, service_type: &ServiceType) -> OpenStackResult<()> {
        let rt = self.runtime_init();
        rt.block_on(self.inner.discover_service_endpoint(service_type))
    }

    /// Fetch auth catalog using the `/v3/auth/catalog` endpoint and update
    /// the session catalog, persisting to cache.
    pub fn fetch_auth_catalog(&self) -> OpenStackResult<()> {
        let rt = self.runtime_init();
        rt.block_on(self.inner.fetch_auth_catalog(true))
    }

    /// Perform token introspection.
    pub fn fetch_token_info(&self, token: SecretString) -> OpenStackResult<AuthResponse> {
        let rt = self.runtime_init();
        rt.block_on(self.inner.fetch_token_info(token))
    }

    /// Return the inner async client.
    pub fn inner(&self) -> &AsyncOpenStack {
        &self.inner
    }

    /// Consume this client and return the inner async client, suitable for
    /// scenarios where a running tokio runtime already exists.
    pub fn into_inner(self) -> AsyncOpenStack {
        self.inner
    }
}
