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

//! Asynchronous OpenStack client

use std::collections::HashMap;
use std::convert::TryInto;
use std::fmt::{self, Debug};
use std::ops::{Deref, DerefMut};
use std::sync::Arc;
use std::time::{Duration, SystemTime};
use std::{fs::File, io::Read};

use arc_swap::ArcSwap;
use async_trait::async_trait;
use bytes::Bytes;
use chrono::TimeDelta;
use futures::io::{Error as IoError, ErrorKind as IoErrorKind};
use futures::stream::TryStreamExt;
use http::{HeaderMap, HeaderValue, Response as HttpResponse, StatusCode, header};
use parking_lot::RwLock;
use reqwest::{Certificate, Request, Response};
use secrecy::{ExposeSecret, SecretString};
use tokio_util::codec;
use tokio_util::compat::FuturesAsyncReadCompatExt;
use tracing::{Level, debug, enabled, error, event, info, instrument, trace, warn};

use openstack_sdk_auth_core::{
    Auth, AuthError, AuthPluginRegistration, AuthToken, OpenStackAuthType,
    authtoken::AuthTokenError,
    authtoken_scope::AuthTokenScope,
    types::{AuthResponse, Project, ServiceEndpoints},
};

// Private use of auth plugins for token refresh and receipt handling.
use openstack_sdk_auth_receipt as token_receipt;
use openstack_sdk_auth_token as token_auth;

use crate::auth::authtoken::{AuthType, build_token_info_endpoint};
use crate::session;

use openstack_sdk_core::api::{
    self, AsyncClient,
    query::{self, RawQueryAsync},
};
use openstack_sdk_core::auth::{
    AuthState,
    auth_helper::{AuthHelper, Dialoguer, Noop},
    gather_auth_data,
};
use openstack_sdk_core::catalog::{Catalog, CatalogError, ServiceEndpoint};
use openstack_sdk_core::config::CloudConfig;
use openstack_sdk_core::error::{OpenStackError, OpenStackResult, RestError};
use openstack_sdk_core::session::AuthSnapshot;
use openstack_sdk_core::types::{ApiVersion, BoxedAsyncRead, ServiceType};
use openstack_sdk_core::utils::expand_tilde;

/// Asynchronous client for the OpenStack API for a single user
///
/// Separate Identity (not the scope) should use separate instances of this.
/// ```rust
/// use openstack_sdk::api::{paged, Pagination, QueryAsync};
/// use openstack_sdk::{AsyncOpenStack, config::ConfigFile, OpenStackError};
/// use openstack_sdk::types::ServiceType;
/// use openstack_sdk::api::compute::v2::flavor::list;
///
/// async fn list_flavors() -> Result<(), OpenStackError> {
///     // Get the builder for the listing Flavors Endpoint
///     let mut ep_builder = list::Request::builder();
///     // Set the `min_disk` query param
///     ep_builder.min_disk("15");
///     let ep = ep_builder.build().unwrap();
///
///     let cfg = ConfigFile::new().unwrap();
///     // Get connection config from clouds.yaml/secure.yaml
///     let profile = cfg.get_cloud_config("devstack").unwrap().unwrap();
///     // Establish connection
///     let mut session = AsyncOpenStack::new(&profile).await?;
///
///     // Invoke service discovery when desired.
///     session.discover_service_endpoint(&ServiceType::Compute).await?;
///
///     // Execute the call with pagination limiting maximum amount of entries to 1000
///     let data: Vec<serde_json::Value> = paged(ep, Pagination::Limit(1000))
///         .query_async(&session)
///         .await.unwrap();
///
///     println!("Data = {:?}", data);
///     Ok(())
/// }
/// ```
pub struct AsyncOpenStack {
    /// The client to use for API calls.
    client: reqwest::Client,
    /// Cloud configuration
    config: CloudConfig,
    /// Session context (auth, catalog, state)
    session: Arc<RwLock<session::SessionContext>>,
    /// Lock-free snapshot of auth/catalog/region for the per-request hot
    /// path, republished by `SessionWriteGuard::drop` on every session write.
    auth_snapshot: Arc<ArcSwap<AuthSnapshot>>,
    /// Auth helper for re-authentication on 401.
    auth_helper: Option<Arc<dyn AuthHelper>>,
    /// Max retries on 401.
    max_auth_retries: u32,
    /// Single-flight guard for re-authentication, shared across all clones.
    /// Held for the duration of `handle_401_retry` so concurrent 401s queue
    /// on one re-auth instead of each running their own. Also intended to be
    /// shared with any future proactive token-renewal task.
    reauth_lock: Arc<tokio::sync::Mutex<()>>,
}

impl Clone for AsyncOpenStack {
    fn clone(&self) -> Self {
        Self {
            client: self.client.clone(),
            config: self.config.clone(),
            session: Arc::clone(&self.session),
            auth_snapshot: Arc::clone(&self.auth_snapshot),
            auth_helper: self.auth_helper.clone(),
            max_auth_retries: self.max_auth_retries,
            reauth_lock: Arc::clone(&self.reauth_lock),
        }
    }
}

impl Debug for AsyncOpenStack {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("OpenStack")
            .field("service_endpoints", &self.auth_snapshot.load().catalog)
            .finish()
    }
}

#[async_trait]
impl api::RestClient for AsyncOpenStack {
    type Error = RestError;

    fn get_current_project(&self) -> Option<Project> {
        let session = self.session.read();
        if let Auth::AuthToken(token) = &session.auth {
            return token.auth_info.clone().and_then(|x| x.token.project);
        }
        None
    }
}

#[async_trait]
impl api::AsyncClient for AsyncOpenStack {
    async fn rest_async(
        &self,
        request: http::request::Builder,
        body: Vec<u8>,
    ) -> Result<HttpResponse<Bytes>, api::ApiError<<Self as api::RestClient>::Error>> {
        let (mut auth, mut generation) = {
            let snap = self.auth_snapshot.load();
            (snap.auth.clone(), snap.auth_generation)
        };
        let mut retries = 0;

        let orig_method = request.method_ref().cloned().unwrap_or(http::Method::GET);
        let orig_uri = request.uri_ref().cloned().unwrap_or_default();
        let orig_headers = request.headers_ref().cloned().unwrap_or_default();

        let mut request = Some(request);
        loop {
            let current_request = match request.take() {
                Some(r) => r,
                None => unreachable!("request missing after retry setup"),
            };
            let result = self
                .rest_with_auth_async(current_request, body.clone(), &auth)
                .await;
            let is_401 = match &result {
                Ok(rsp) => rsp.status() == http::StatusCode::UNAUTHORIZED,
                Err(_) => false,
            };
            if is_401 && retries < self.max_auth_retries {
                warn!(
                    "Received 401 Unauthorized, retry {} of {}",
                    retries + 1,
                    self.max_auth_retries
                );
                match self.handle_401_retry(generation).await {
                    Ok(_) => {
                        (auth, generation) = {
                            let snap = self.auth_snapshot.load();
                            (snap.auth.clone(), snap.auth_generation)
                        };
                        retries += 1;
                        let mut new_builder = http::Request::builder()
                            .method(orig_method.clone())
                            .uri(orig_uri.clone());
                        for (name, value) in &orig_headers {
                            new_builder = new_builder.header(name.clone(), value.clone());
                        }
                        request = Some(new_builder);
                        continue;
                    }
                    Err(e) => {
                        warn!("401 retry failed: {}", e);
                        return result;
                    }
                }
            }
            return result;
        }
    }

    async fn rest_read_body_async(
        &self,
        request: http::request::Builder,
        body: BoxedAsyncRead,
    ) -> Result<HttpResponse<Bytes>, api::ApiError<<Self as api::RestClient>::Error>> {
        let auth = self.auth_snapshot.load().auth.clone();
        self.rest_with_auth_read_body_async(request, body, &auth)
            .await
    }

    async fn download_async(
        &self,
        request: http::request::Builder,
        body: Vec<u8>,
    ) -> Result<(HeaderMap, BoxedAsyncRead), api::ApiError<<Self as api::RestClient>::Error>> {
        let auth = self.auth_snapshot.load().auth.clone();
        self.download_with_auth_async(request, body, &auth).await
    }

    async fn get_service_endpoint(
        &self,
        service_type: &ServiceType,
        version: Option<&ApiVersion>,
    ) -> Result<ServiceEndpoint, api::ApiError<Self::Error>> {
        let lookup = || {
            let snap = self.auth_snapshot.load();
            let region = snap
                .region_name
                .as_ref()
                .or(self.config.region_name.as_ref());
            snap.catalog
                .get_service_endpoint(
                    service_type.to_string(),
                    version,
                    region,
                    self.config.interface.as_ref(),
                )
                .cloned()
                .map_err(api::ApiError::catalog)
        };

        let ep = match lookup() {
            Ok(ep) => return Ok(ep),
            Err(api::ApiError::Catalog { source }) => {
                if matches!(source, CatalogError::ServiceNotConfigured { .. }) {
                    info!(
                        "Service '{}' not in catalog, attempting refresh via auth/catalog",
                        service_type
                    );
                    if let Err(e) = self.refresh_catalog().await {
                        warn!("Catalog refresh failed: {}", e);
                    }
                    match lookup() {
                        Ok(ep) => {
                            debug!("Service '{}' found after catalog refresh", service_type);
                            self.persist_catalog_to_cache();
                            ep
                        }
                        Err(e) => return Err(e),
                    }
                } else {
                    return Err(api::ApiError::Catalog { source });
                }
            }
            Err(e) => return Err(e),
        };

        Ok(ep)
    }
}

/// Write guard for `SessionContext` that republishes `AuthSnapshot` on drop,
/// while the write lock is still held (the manual `Drop::drop` below runs
/// before the `guard` field's own destructor, so the store happens with the
/// lock still taken). This guarantees a concurrent hot-path reader never
/// sees a released lock with a stale snapshot: every write path that
/// mutates `auth`, `auth_generation`, `catalog`, or `region_name` is
/// automatically reflected, with no per-call-site bookkeeping required.
///
/// Trade-off: writes that only touch `state` (auth-cache bookkeeping, e.g.
/// `get_scope_auth`/`clear_all_auth`) still pay a full `AuthSnapshot`
/// rebuild — including a `Catalog` clone — since the guard can't tell which
/// fields changed. These are auth-flow writes, not per-request, so the cost
/// is bounded; revisit if `Catalog` grows large enough to matter.
struct SessionWriteGuard<'a> {
    guard: parking_lot::RwLockWriteGuard<'a, session::SessionContext>,
    snapshot: &'a ArcSwap<AuthSnapshot>,
}

impl Deref for SessionWriteGuard<'_> {
    type Target = session::SessionContext;
    fn deref(&self) -> &Self::Target {
        &self.guard
    }
}

impl DerefMut for SessionWriteGuard<'_> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.guard
    }
}

impl Drop for SessionWriteGuard<'_> {
    fn drop(&mut self) {
        self.snapshot
            .store(Arc::new(AuthSnapshot::from(&*self.guard)));
    }
}

impl AsyncOpenStack {
    /// Lock the session for reading. `location` is kept as a parameter for
    /// call-site self-documentation (parking_lot never poisons, so it no
    /// longer feeds an error message).
    #[allow(unused_variables)]
    fn session_read(
        &self,
        location: &'static str,
    ) -> parking_lot::RwLockReadGuard<'_, session::SessionContext> {
        self.session.read()
    }

    /// Lock the session for writing. Returns a guard that republishes
    /// `AuthSnapshot` on drop — see `SessionWriteGuard`.
    #[allow(unused_variables)]
    fn session_write(&self, location: &'static str) -> SessionWriteGuard<'_> {
        SessionWriteGuard {
            guard: self.session.write(),
            snapshot: &self.auth_snapshot,
        }
    }

    /// Basic constructor — visible to the sync facade.
    pub fn new_impl(config: &CloudConfig, auth: Auth) -> OpenStackResult<Self> {
        // Ensure auth plugin registries are populated even if the linker
        // would otherwise strip crates with no other references.
        openstack_sdk_auth_core::anchor_plugins();

        let mut client_builder = reqwest::Client::builder();

        if let Some(cacert) = &config.cacert {
            let mut buf = Vec::new();
            File::open(expand_tilde(cacert).unwrap_or(cacert.into()))
                .map_err(|e| OpenStackError::IOWithPath {
                    source: e,
                    path: cacert.into(),
                })?
                .read_to_end(&mut buf)
                .map_err(|e| OpenStackError::IOWithPath {
                    source: e,
                    path: cacert.into(),
                })?;
            for cert in Certificate::from_pem_bundle(&buf)? {
                client_builder = client_builder.add_root_certificate(cert);
            }
        }
        if let Some(false) = &config.verify {
            warn!(
                "SSL Verification is disabled! Please consider using `cacert` for adding custom certificate instead."
            );
            client_builder = client_builder.danger_accept_invalid_certs(true);
        }
        client_builder = client_builder.pool_max_idle_per_host(10);
        client_builder = client_builder.pool_idle_timeout(Duration::from_secs(30));
        client_builder = client_builder.timeout(Duration::from_secs(
            config
                .options
                .get("api_timeout")
                .and_then(|val| val.clone().into_uint().ok())
                .unwrap_or(30),
        ));
        client_builder = client_builder.connect_timeout(Duration::from_secs(5));
        client_builder = client_builder.tcp_keepalive(Duration::from_secs(60));
        client_builder = client_builder.gzip(true);
        client_builder = client_builder.deflate(true);

        // Pass CloudConfig.auth_cache as override; SessionContext resolves priority chain.
        let session_ctx = session::SessionContext::new(config, auth, config.auth_cache)?;
        let auth_snapshot = Arc::new(ArcSwap::new(Arc::new(AuthSnapshot::from(&session_ctx))));

        Ok(AsyncOpenStack {
            client: client_builder.build()?,
            config: config.clone(),
            session: Arc::new(RwLock::new(session_ctx)),
            auth_snapshot,
            auth_helper: None,
            max_auth_retries: 1,
            reauth_lock: Arc::new(tokio::sync::Mutex::new(())),
        })
    }

    /// Create a new OpenStack API session from CloudConfig
    #[instrument(name = "connect", level = "trace", skip(config))]
    pub async fn new(config: &CloudConfig) -> OpenStackResult<Self> {
        let session = Self::new_impl(config, Auth::None)?;

        // Ensure we resolve identity endpoint using version discovery
        session
            .discover_service_endpoint(&ServiceType::Identity)
            .await?;

        session.authorize(None, false, false).await?;

        Ok(session)
    }

    /// Create a new OpenStack API session from CloudConfig
    #[instrument(name = "connect", level = "trace", skip(config, auth_helper))]
    pub async fn new_with_authentication_helper<A>(
        config: &CloudConfig,
        auth_helper: A,
        renew_auth: bool,
    ) -> OpenStackResult<Self>
    where
        A: AuthHelper + Sync + Send + 'static,
    {
        let mut session = Self::new_impl(config, Auth::None)?;

        // Ensure we resolve identity endpoint using version discovery
        session
            .discover_service_endpoint(&ServiceType::Identity)
            .await?;

        session
            .authorize_with_auth_helper(None, &auth_helper, renew_auth)
            .await?;

        session.auth_helper = Some(Arc::new(auth_helper));

        Ok(session)
    }
    #[instrument(name = "connect", level = "trace", skip(config))]
    #[deprecated(
        since = "0.22.0",
        note = "please use `new_with_authentication_helper` instead"
    )]
    pub async fn new_interactive(config: &CloudConfig, renew_auth: bool) -> OpenStackResult<Self> {
        Self::new_with_authentication_helper(config, Dialoguer::default(), renew_auth).await
    }

    /// Set the authorization to be used by the client
    fn set_auth(&self, auth: Auth, skip_cache_update: bool) -> Result<(), OpenStackError> {
        let mut session = self.session_write("set_auth");
        session.auth = auth;
        session.auth_generation = session.auth_generation.wrapping_add(1);
        if !skip_cache_update && let Auth::AuthToken(a) = &session.auth {
            let a_clone = a.clone();
            let scope = match &a_clone.auth_info {
                Some(info) => {
                    if info.token.application_credential.is_some() {
                        AuthTokenScope::Unscoped
                    } else {
                        a_clone.get_scope()
                    }
                }
                _ => a_clone.get_scope(),
            };
            session.state.set_scope_auth(&scope, &a_clone);
        }
        Ok(())
    }

    /// Authorize against the cloud using provided credentials and get the session token.
    pub async fn authorize(
        &self,
        scope: Option<AuthTokenScope>,
        interactive: bool,
        renew_auth: bool,
    ) -> Result<(), OpenStackError> {
        if interactive {
            self.authorize_with_auth_helper(scope, &Dialoguer::default(), renew_auth)
                .await
        } else {
            self.authorize_with_auth_helper(scope, &Noop::default(), renew_auth)
                .await
        }
    }

    /// Re-authenticate with the existing auth for the given scope.
    async fn reauth(
        &self,
        auth: &AuthToken,
        scope: &AuthTokenScope,
    ) -> Result<Auth, OpenStackError> {
        Ok(token_auth::PLUGIN
            .auth(
                &self.client,
                self.get_service_endpoint(&ServiceType::Identity, Some(&ApiVersion::from((3, 0))))
                    .await?
                    .url(),
                &HashMap::from([("token".into(), auth.token.clone())]),
                Some(scope),
                None,
            )
            .await?)
    }

    /// Handle 401 Unauthorized by re-authenticating.
    ///
    /// `seen_generation` is the `auth_generation` observed by the caller
    /// before it hit the 401. Re-auth is single-flighted via `reauth_lock`:
    /// if another concurrent caller already completed a re-auth while this
    /// one was waiting for the lock, `auth_generation` will have moved on
    /// and this call skips its own re-auth.
    async fn handle_401_retry(&self, seen_generation: u64) -> Result<(), OpenStackError> {
        let _guard = self.reauth_lock.lock().await;

        let current_generation = self.auth_snapshot.load().auth_generation;
        if current_generation != seen_generation {
            // Another task already re-authed while we waited for the lock.
            return Ok(());
        }

        let scope = AuthTokenScope::try_from(&self.config)?;
        // Re-authenticate with current config. First try to reauth with existing token,
        // fall back to full authentication if auth_helper is available.
        let auth_helper_opt = self.auth_helper.clone();
        if let Some(auth_helper) = auth_helper_opt {
            {
                let mut session = self.session_write("handle_401_retry: clear state");
                session.state.clear_all_auth();
            }
            self.authorize_with_auth_helper(Some(scope), &auth_helper, true)
                .await?;
        } else {
            // No auth helper available - try to reauth with the current token
            if let Auth::AuthToken(token) = {
                let session = self.session_read("handle_401_retry: read auth");
                session.auth.clone()
            } && let Ok(new_auth) = self.reauth(&token, &scope).await
            {
                self.set_auth(new_auth, false)?;
                return Ok(());
            }
            // Reauth failed, clear state and try full auth as fallback
            {
                let mut session = self.session_write("handle_401_retry: clear state fallback");
                session.state.clear_all_auth();
            }
            self.authorize_with_auth_helper(Some(scope), &Noop::default(), true)
                .await?;
        }
        Ok(())
    }

    /// Disable authentication caching for this session.
    ///
    /// Clears any cached tokens and prevents further caching — both the
    /// in-memory store and the on-disk state file. Useful for scenarios
    /// requiring fresh authentication on every request.
    pub fn disable_auth_cache(&self) -> &Self {
        self.session.write().state.disable_auth_cache();
        self
    }

    /// Set the maximum number of retries on 401 responses.
    pub fn set_max_auth_retries(&mut self, n: u32) -> &mut Self {
        self.max_auth_retries = n;
        self
    }

    /// Set the auth helper used for 401 re-authentication.
    pub fn set_auth_helper<A>(&mut self, auth_helper: A)
    where
        A: AuthHelper + Sync + Send + 'static,
    {
        self.auth_helper = Some(Arc::new(auth_helper));
    }

    /// Authorize against the cloud using provided credentials and get the session token with the
    /// auth helper that may be invoked to interactively ask for the credentials.
    pub async fn authorize_with_auth_helper<A>(
        &self,
        scope: Option<AuthTokenScope>,
        auth_helper: &A,
        renew_auth: bool,
    ) -> Result<(), OpenStackError>
    where
        A: AuthHelper + Sync + Send + 'static,
    {
        let requested_scope =
            scope.map_or_else(|| AuthTokenScope::try_from(&self.config), |v| Ok(v.clone()))?;

        let auth_opt = {
            let mut session = self.session_write("authorize_with_auth_helper: get cache auth");
            session.state.get_scope_auth(&requested_scope)
        };
        if let Some(auth) = auth_opt
            && !renew_auth
        {
            // Valid authorization is already available and no renewal is required
            trace!("Auth already available");
            self.set_auth(Auth::AuthToken(Box::new(auth.clone())), true)?;
        } else {
            // No valid authorization data is available in the state or
            // renewal is requested
            let auth_type = AuthType::from_cloud_config(&self.config)?;
            let force_new_auth = matches!(auth_type, AuthType::V3ApplicationCredential);
            let available_auth_opt = {
                let mut session = self.session_write("authorize_with_auth_helper: reauthz");
                session.state.get_any_valid_auth()
            };
            if let (Some(available_auth), false) = (available_auth_opt, force_new_auth) {
                // State contain valid authentication for different scope/unscoped. It is possible
                // to request new authz using this other auth
                trace!("Valid Auth is available for reauthz: {:?}", available_auth);
                let token_auth = self.reauth(&available_auth, &requested_scope).await?;
                self.set_auth(token_auth.clone(), false)?;
            } else {
                // No auth/authz information available or force_new_auth. Proceed with new auth
                trace!("No Auth already available. Proceeding with new login");

                let auth_type = auth_type.as_str();
                // Find authenticator supporting the auth_type
                if let Some(authenticator) = inventory::iter::<AuthPluginRegistration>
                    .into_iter()
                    .find(|x| x.method.get_supported_auth_methods().contains(&auth_type))
                    .map(|x| x.method)
                {
                    // authenticate
                    let auth_hints = self
                        .config
                        .auth_methods
                        .as_ref()
                        .map(|methods| serde_json::json!({"auth_methods": methods}));
                    match authenticator
                        .auth(
                            &self.client,
                            self.get_service_endpoint(
                                &ServiceType::Identity,
                                Some(&ApiVersion::from(authenticator.api_version())),
                            )
                            .await?
                            .url(),
                            &gather_auth_data(
                                &authenticator.requirements(auth_hints.as_ref())?,
                                &self.config,
                                auth_helper,
                            )
                            .await?,
                            Some(&requested_scope),
                            auth_hints.as_ref(),
                        )
                        .await
                    {
                        Ok(token_auth) => {
                            self.set_auth(token_auth.clone(), false)?;
                        }
                        Err(AuthError::AuthReceipt(receipt)) => {
                            // Auth Receipt is received
                            // Find the receipt auth plugin
                            // Convert the receipt into auth hints
                            let auth_hints = serde_json::to_value(&receipt)?;
                            // Authenticate
                            let token_auth = token_receipt::PLUGIN
                                .auth(
                                    &self.client,
                                    self.get_service_endpoint(
                                        &ServiceType::Identity,
                                        Some(&ApiVersion::from(authenticator.api_version())),
                                    )
                                    .await?
                                    .url(),
                                    &gather_auth_data(
                                        &token_receipt::PLUGIN.requirements(Some(&auth_hints))?,
                                        &self.config,
                                        auth_helper,
                                    )
                                    .await?,
                                    Some(&requested_scope),
                                    Some(&auth_hints),
                                )
                                .await?;
                            self.set_auth(token_auth.clone(), false)?;
                        }
                        Err(other) => {
                            return Err(other.into());
                        }
                    }
                } else {
                    Err(AuthTokenError::IdentityMethod {
                        auth_type: auth_type.into(),
                    })?;
                }
            }
        }

        let auth_opt = {
            let session = self.session_read("authorize_with_auth_helper: get auth");
            session.auth.clone()
        };
        if let Auth::AuthToken(token_auth) = &auth_opt {
            // When unscoped auth is requested (maybe by not specifying the scope) we expect that
            // the Authenticator returns the necessary scope (i.e. for ApplicationCredentials it is
            // not possible to request scope, but the Keystone manages the scope) or unscoped.
            // In this case we just save the auth as unscoped (in addition to what was done above).
            // Otherwise we should rescope.

            if token_auth.auth_info.is_none() {
                let mut resolved_token = token_auth.clone();
                let token_info = self.fetch_token_info(token_auth.token.clone()).await?;
                resolved_token.auth_info = Some(token_info.clone());
                let scope = AuthTokenScope::from(&token_info);

                // Save unscoped token in the cache
                {
                    let mut session =
                        self.session_write("authorize_with_auth_helper: set unscoped cache");
                    session.state.set_scope_auth(&scope, &resolved_token);
                }
            }

            if requested_scope != AuthTokenScope::Unscoped
                && !token_auth
                    .auth_info
                    .as_ref()
                    .map(AuthTokenScope::from)
                    .is_some_and(|scope| requested_scope == scope)
            {
                // And now time to rescope the token
                let token_auth = self.reauth(token_auth, &requested_scope).await?;
                self.set_auth(token_auth.clone(), false)?;
            } else {
                // Client may not specify the target scope expecting the mapping to set
                // the proper token. Save the auth as unscope (similarly to the AppCred
                // handling).
                {
                    let mut session =
                        self.session_write("authorize_with_auth_helper: set unscoped");
                    session
                        .state
                        .set_scope_auth(&AuthTokenScope::Unscoped, token_auth);
                }
            }
        } else {
            Err(OpenStackError::NoAuth)?;
        }

        {
            let mut session = self.session_write("authorize_with_auth_helper: process catalog");
            let auth_data = match &session.auth {
                Auth::AuthToken(token_data) => match &token_data.auth_info {
                    Some(ad) => ad.clone(),
                    _ => return Err(OpenStackError::NoAuth),
                },
                _ => return Err(OpenStackError::NoAuth),
            };
            if let Some(project) = &auth_data.token.project {
                session.catalog.set_project_id(project.id.clone());
                session.catalog.configure(&self.config)?;
            }
            if let Some(endpoints) = &auth_data.token.catalog {
                session.catalog.process_catalog_endpoints(endpoints)?;
            } else {
                error!("No catalog information");
            }
        }
        Ok(())
    }

    /// Perform version discovery of a service
    #[instrument(skip(self))]
    pub async fn discover_service_endpoint(
        &self,
        service_type: &ServiceType,
    ) -> Result<(), OpenStackError> {
        let ep_opt = {
            let session = self.session_read("discover_service_endpoint: get ep");
            let region = session
                .region_name
                .as_ref()
                .or(self.config.region_name.as_ref());
            session
                .catalog
                .get_service_endpoint(
                    service_type.to_string(),
                    None,
                    region,
                    self.config.interface.as_ref(),
                )
                .ok()
                .cloned()
        };

        let ep = match ep_opt {
            Some(e) => e,
            None => return Ok(()),
        };

        let discovery_allowed = {
            let session = self.session_read("discover_service_endpoint: check discovery");
            session.catalog.discovery_allowed(service_type.to_string())
        };
        if !discovery_allowed {
            return Ok(());
        }

        info!("Performing `{}` endpoint version discovery", service_type);

        let orig_url = ep.url().clone();
        let mut try_url = ep.url().clone();
        // Version discovery document must logically end with "/" since API url goes even
        // deeper.
        try_url
            .path_segments_mut()
            .map_err(|_| CatalogError::cannot_be_base(ep.url()))?
            .pop_if_empty()
            .push("");
        let mut max_depth = 10;
        loop {
            let req = http::Request::builder()
                .method(http::Method::GET)
                .uri(query::url_to_http_uri(try_url.clone())?);

            let auth = {
                let session =
                    self.session_read("discover_service_endpoint: get auth for discovery");
                session.auth.clone()
            };
            match self.rest_with_auth_async(req, Vec::new(), &auth).await {
                Ok(rsp) => {
                    if rsp.status() != StatusCode::NOT_FOUND {
                        let region = {
                            let session =
                                self.session_read("discover_service_endpoint: get region");
                            session
                                .region_name
                                .as_deref()
                                .or(self.config.region_name.as_deref())
                                .map(|s| s.to_string())
                        };
                        let ok = {
                            let mut session =
                                self.session_write("discover_service_endpoint: process catalog");
                            session
                                .catalog
                                .process_endpoint_discovery(
                                    service_type,
                                    &try_url,
                                    rsp.body(),
                                    region.as_deref(),
                                    self.config.interface.as_ref(),
                                )
                                .is_ok()
                        };
                        if ok {
                            debug!("Finished service version discovery at {}", try_url.as_str());
                            debug!("catalog {:?}", self.session.read().catalog.clone());
                            return Ok(());
                        }
                    }
                }
                Err(err) => {
                    error!(
                        "Error querying {} for the version discovery. It is most likely a misconfiguration on the cloud side. {}",
                        try_url.as_str(),
                        err
                    );
                }
            };
            if try_url.path() != "/" {
                // We are not at the root yet and have not found a
                // valid version document so far, try one level up
                try_url
                    .path_segments_mut()
                    .map_err(|_| CatalogError::cannot_be_base(&orig_url))?
                    .pop();
            } else {
                return Err(OpenStackError::Discovery {
                    service: service_type.to_string(),
                    url: orig_url.into(),
                    msg: match service_type {
                        ServiceType::Identity => "Service is not working.".into(),
                        _ => "No Version document found. Either service is not supporting version discovery, or API is not working".into(),
                    }
                });
            }

            max_depth -= 1;
            if max_depth == 0 {
                break;
            }
        }
        Err(OpenStackError::Discovery {
            service: service_type.to_string(),
            url: orig_url.into(),
            msg: "Unknown".into(),
        })
    }

    /// Fetch auth catalog using current session token and update session catalog.
    ///
    /// This retrieves the service catalog from the Identity API using the
    /// `GET /v3/auth/catalog` endpoint and updates the session's catalog with
    /// the fetched endpoints. Useful when the token was obtained without a
    /// catalog or when the catalog needs to be refreshed.
    ///
    /// Updates `session.catalog` and `authz.auth_info.token.catalog` in memory.
    /// If `persist` is `true`, also persists to the on-disk auth cache via
    /// [`State::set_scope_auth`].
    pub async fn fetch_auth_catalog(&self, persist: bool) -> OpenStackResult<()> {
        self.refresh_catalog().await?;
        if persist {
            self.persist_catalog_to_cache();
        }
        Ok(())
    }

    /// Updates session catalog from the /v3/auth/catalog endpoint without
    /// persisting to the on-disk auth cache.
    async fn refresh_catalog(&self) -> OpenStackResult<()> {
        let mut catalog: Catalog = {
            let session = self.session_read("refresh_catalog");
            session.catalog.clone()
        };
        catalog.read_auth_catalog_async(self).await?;
        let mut session = self.session_write("refresh_catalog: update");

        // Update session catalog
        session.catalog = catalog.clone();

        // Update auth token's catalog
        if let Auth::AuthToken(authz) = &mut session.auth
            && let Some(ref mut auth_info) = authz.auth_info
        {
            auth_info.token.catalog = catalog.get_token_catalog();
        }
        Ok(())
    }

    /// Persist the session's updated catalog to the on-disk auth cache.
    fn persist_catalog_to_cache(&self) {
        let mut session = self.session_write("persist_catalog_to_cache");

        if let Auth::AuthToken(authz) = &session.auth {
            let scope = authz.get_scope();
            let authz_clone = authz.clone();
            session.state.set_scope_auth(&scope, &authz_clone);
        }
    }

    /// Return catalog information given in the token
    pub fn get_token_catalog(&self) -> Option<Vec<ServiceEndpoints>> {
        let session = self.session.read();
        session.catalog.get_token_catalog()
    }

    /// Return current authentication information
    pub fn get_auth_info(&self) -> Option<AuthResponse> {
        let session = self.session.read();
        if let Auth::AuthToken(token) = &session.auth {
            return token.auth_info.clone();
        }
        None
    }

    /// Return current authentication status
    ///
    /// Offset can be used to calculate imminent expiration.
    pub fn get_auth_state(&self, offset: Option<TimeDelta>) -> Option<AuthState> {
        let session = self.session.read();
        if let Auth::AuthToken(token) = &session.auth {
            return Some(token.get_state(offset));
        }
        None
    }

    /// Set the region name for endpoint resolution.
    ///
    /// This overrides the region from `CloudConfig` at runtime without requiring
    /// re-authentication. The token's service catalog already contains endpoints
    /// for all regions, so switching region only changes which endpoint is selected.
    pub fn set_region_name(&self, region: String) -> Result<(), OpenStackError> {
        let mut session = self.session_write("set_region_name");
        session.region_name = Some(region);
        Ok(())
    }

    /// Return the current effective region name.
    ///
    /// Returns the runtime override (if set via [`set_region_name`]) or falls back
    /// to the region from [`CloudConfig`].
    pub fn get_region_name(&self) -> Option<String> {
        let session = self.session.read();
        session
            .region_name
            .clone()
            .or_else(|| self.config.region_name.clone())
    }

    /// List available regions from catalog endpoints.
    ///
    /// Collects unique region names from both the service catalog and any
    /// discovered version endpoints.
    pub fn get_available_regions(&self) -> Option<Vec<String>> {
        let session = self.session.read();
        let regions = session.catalog.get_regions();
        if regions.is_empty() {
            None
        } else {
            Some(regions)
        }
    }

    /// Return current authentication token
    pub fn get_auth_token(&self) -> Option<SecretString> {
        let session = self.session.read();
        if let Auth::AuthToken(token) = &session.auth {
            return Some(token.token.clone());
        }
        None
    }

    /// Perform token introspection call
    pub async fn fetch_token_info(
        &self,
        token: SecretString,
    ) -> Result<AuthResponse, OpenStackError> {
        let auth_ep = build_token_info_endpoint(token.expose_secret())?;
        let rsp = auth_ep.raw_query_async(self).await?;
        let data: AuthResponse = serde_json::from_slice(rsp.body())?;
        Ok(data)
    }

    /// Perform HTTP request with given request and return raw response.
    #[instrument(name="request", skip_all, fields(http.uri = request.url().as_str(), http.method = request.method().as_str(), openstack.ver=request.headers().get("openstack-api-version").map(|v| v.to_str().unwrap_or(""))))]
    async fn execute_request(&self, request: Request) -> Result<Response, reqwest::Error> {
        info!("Sending request {:?}", request);
        let url = request.url().clone();
        let method = request.method().clone();

        if enabled!(Level::TRACE)
            && request.headers().get(header::CONTENT_TYPE)
                == Some(&HeaderValue::from_static("application/json"))
        {
            // Body may contain sensitive info - censor it but only when trace is on
            request
                .body()
                .and_then(|body| body.as_bytes())
                .and_then(|bytes| String::from_utf8(bytes.to_vec()).ok())
                .inspect(|rq| {
                    let censored = self
                        .config
                        .get_sensitive_values()
                        .iter()
                        .fold(rq.clone(), |sanitized, &secret| {
                            sanitized.replace(secret, "<CENSORED>")
                        });
                    trace!("Request Body: {:?}", censored);
                });
        }

        let start = SystemTime::now();
        let rsp = self.client.execute(request).await?;
        let elapsed = SystemTime::now().duration_since(start).unwrap_or_default();
        event!(
            name: "http_request",
            Level::INFO,
            url=url.as_str(),
            duration_ms=elapsed.as_millis(),
            status=rsp.status().as_u16(),
            method=method.as_str(),
            request_id=rsp.headers().get("x-openstack-request-id").map(|v| v.to_str().unwrap_or("")),
            "Request completed with status {}",
            rsp.status(),
        );
        Ok(rsp)
    }

    /// Perform a REST query with a given auth.
    async fn rest_with_auth_async(
        &self,
        mut request: http::request::Builder,
        body: Vec<u8>,
        auth: &Auth,
    ) -> Result<HttpResponse<Bytes>, api::ApiError<<Self as api::RestClient>::Error>> {
        use futures_util::TryFutureExt;
        let call = || async {
            if let Some(headers) = request.headers_mut() {
                auth.set_header(headers)?;
            }
            let http_request = request.body(body)?;
            let request: reqwest::Request = http_request.try_into()?;
            let rsp = self.execute_request(request).await?;
            let mut http_rsp = HttpResponse::builder()
                .status(rsp.status())
                .version(rsp.version());
            if let Some(headers) = http_rsp.headers_mut() {
                headers.extend(rsp.headers().clone())
            }
            Ok(http_rsp.body(rsp.bytes().await?)?)
        };
        call().map_err(api::ApiError::client).await
    }

    /// Perform a REST query with a given auth (streaming body).
    async fn rest_with_auth_read_body_async(
        &self,
        mut request: http::request::Builder,
        body_read: BoxedAsyncRead,
        auth: &Auth,
    ) -> Result<HttpResponse<Bytes>, api::ApiError<<Self as api::RestClient>::Error>> {
        use futures_util::TryFutureExt;
        let call = || async {
            if let Some(headers) = request.headers_mut() {
                auth.set_header(headers)?;
            }
            let stream = codec::FramedRead::new(body_read.compat(), codec::BytesCodec::new())
                .map_ok(|b| b.freeze());
            let http_request = request.body(reqwest::Body::wrap_stream(stream))?;
            let request: reqwest::Request = http_request.try_into()?;
            let rsp = self.execute_request(request).await?;
            let mut http_rsp = HttpResponse::builder()
                .status(rsp.status())
                .version(rsp.version());
            if let Some(headers) = http_rsp.headers_mut() {
                headers.extend(rsp.headers().clone())
            }
            Ok(http_rsp.body(rsp.bytes().await?)?)
        };
        call().map_err(api::ApiError::client).await
    }

    /// Perform a REST query with a given auth and return AsyncRead.
    async fn download_with_auth_async(
        &self,
        mut request: http::request::Builder,
        body: Vec<u8>,
        auth: &Auth,
    ) -> Result<(HeaderMap, BoxedAsyncRead), api::ApiError<<Self as api::RestClient>::Error>> {
        use futures_util::TryFutureExt;
        let call = || async {
            if let Some(headers) = request.headers_mut() {
                auth.set_header(headers)?;
            }
            let http_request = request.body(body)?;
            let request: reqwest::Request = http_request.try_into()?;
            let rsp = self.execute_request(request).await?;
            let mut resp_headers = HeaderMap::new();
            for (key, value) in rsp.headers() {
                resp_headers.insert(key, value.clone());
            }
            let boxed_async_read = BoxedAsyncRead::new(
                rsp.bytes_stream()
                    .map_err(|orig| {
                        let kind = if orig.is_timeout() {
                            IoErrorKind::TimedOut
                        } else {
                            IoErrorKind::Other
                        };
                        IoError::new(kind, orig)
                    })
                    .into_async_read(),
            );
            Ok((resp_headers, boxed_async_read))
        };
        call().map_err(api::ApiError::client).await
    }
}

#[cfg(test)]
mod tests {
    use chrono::{TimeDelta, Utc};
    use httpmock::MockServer;
    use secrecy::ExposeSecret;
    use serde_json::json;

    use openstack_sdk_core::api::{AsyncClient, RestClient};
    use openstack_sdk_core::catalog::Catalog;

    use super::*;
    use crate::config::Auth as ConfigAuth;

    /// Create a minimal AsyncOpenStack for testing 401 retry logic.
    /// Bypasses live identity server and discovery.
    fn create_test_client(
        server: &MockServer,
        token: &str,
        max_retries: u32,
        auth_helper: Option<Arc<dyn AuthHelper>>,
    ) -> AsyncOpenStack {
        let base_url = url::Url::parse(&server.base_url()).unwrap();

        let config = CloudConfig {
            auth: Some(ConfigAuth {
                auth_url: Some(base_url.as_str().to_string()),
                project_id: Some("test-project".into()),
                ..Default::default()
            }),
            region_name: Some("RegionOne".into()),
            ..Default::default()
        };

        let token_info = AuthResponse {
            token: openstack_sdk_auth_core::types::TokenInfo {
                expires_at: Utc::now() + chrono::TimeDelta::hours(1),
                project: Some(openstack_sdk_auth_core::types::Project {
                    id: Some("test-project".into()),
                    name: Some("TestProject".into()),
                    domain: None,
                }),
                user: openstack_sdk_auth_core::types::User {
                    id: "test-user".into(),
                    name: "test-user".into(),
                    domain: None,
                    password_expires_at: None,
                },
                ..Default::default()
            },
        };

        let auth = Auth::AuthToken(Box::new(openstack_sdk_auth_core::AuthToken {
            token: SecretString::from(token),
            auth_info: Some(token_info),
        }));

        let mut catalog = Catalog::default();
        catalog
            .register_catalog_endpoint(
                "identity",
                base_url.as_str(),
                Some("RegionOne"),
                Some("public"),
            )
            .unwrap();

        let mut state = session::state::State::new();
        state.set_auth_hash_key(0);

        let session_ctx = session::SessionContext {
            auth,
            catalog,
            state,
            region_name: None,
            auth_generation: 0,
        };
        let auth_snapshot = Arc::new(ArcSwap::new(Arc::new(AuthSnapshot::from(&session_ctx))));

        AsyncOpenStack {
            client: reqwest::Client::builder()
                .timeout(Duration::from_secs(10))
                .build()
                .unwrap(),
            config,
            session: Arc::new(RwLock::new(session_ctx)),
            auth_snapshot,
            auth_helper,
            max_auth_retries: max_retries,
            reauth_lock: Arc::new(tokio::sync::Mutex::new(())),
        }
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn test_401_no_retry_on_success() {
        let server = MockServer::start_async().await;

        let mock = server
            .mock_async(|when, then| {
                when.method(httpmock::Method::GET)
                    .path("/api/test")
                    .header("X-Auth-Token", "old-token");
                then.status(StatusCode::OK).body("{\"result\": \"ok\"}");
            })
            .await;

        let client = create_test_client(&server, "old-token", 1, None);

        let request = http::Request::builder()
            .method(http::Method::GET)
            .uri(format!("{}/api/test", server.base_url()));

        let result = client.rest_async(request, Vec::new()).await;
        assert!(result.is_ok());
        assert_eq!(result.as_ref().unwrap().status(), StatusCode::OK);
        mock.assert_async().await;
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn test_401_retry_succeeds() {
        let server = MockServer::start_async().await;

        // First call: API returns 401 for old token
        let mock_401 = server
            .mock_async(|when, then| {
                when.method(httpmock::Method::GET)
                    .path("/api/test")
                    .header("X-Auth-Token", "old-token");
                then.status(StatusCode::UNAUTHORIZED)
                    .body(r#"{"error": {"message": "Unauthorized"}, "message": "Unauthorized"}"#);
            })
            .await;

        // Re-auth: identity returns new token
        let expires = (Utc::now() + chrono::TimeDelta::hours(1)).to_rfc3339();
        let mock_reauth = server
            .mock_async(|when, then| {
                when.method(httpmock::Method::POST).path("/auth/tokens");
                then.status(StatusCode::CREATED)
                    .header("x-subject-token", "new-token")
                    .json_body(json!({
                        "token": {
                            "id": "token-id",
                            "name": "token-name",
                            "expires_at": &expires,
                            "project": {
                                "id": "test-project",
                                "name": "TestProject"
                            },
                            "user": {
                                "id": "test-user",
                                "name": "test-user"
                            }
                        }
                    }));
            })
            .await;

        // Second call: API returns 200 for new token
        let mock_200 = server
            .mock_async(|when, then| {
                when.method(httpmock::Method::GET)
                    .path("/api/test")
                    .header("X-Auth-Token", "new-token");
                then.status(StatusCode::OK).body("{\"result\": \"ok\"}");
            })
            .await;

        let client = create_test_client(&server, "old-token", 1, None);

        let request = http::Request::builder()
            .method(http::Method::GET)
            .uri(format!("{}/api/test", server.base_url()));

        let result = client.rest_async(request, Vec::new()).await;
        assert!(result.is_ok(), "expected Ok, got {:?}", result);
        assert_eq!(result.as_ref().unwrap().status(), StatusCode::OK);

        // Verify the correct sequence: 401 → re-auth → 200
        mock_401.assert_async().await;
        mock_reauth.assert_async().await;
        mock_200.assert_async().await;
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn test_401_non_401_not_retried() {
        let server = MockServer::start_async().await;

        let mock = server
            .mock_async(|when, then| {
                when.method(httpmock::Method::GET).path("/api/test");
                then.status(StatusCode::INTERNAL_SERVER_ERROR)
                    .body(r#"{"error": {"message": "Server Error"}, "message": "Server Error"}"#);
            })
            .await;

        let client = create_test_client(&server, "old-token", 5, None);

        let request = http::Request::builder()
            .method(http::Method::GET)
            .uri(format!("{}/api/test", server.base_url()));

        let result = client.rest_async(request, Vec::new()).await;
        assert!(result.is_ok());
        assert_eq!(
            result.as_ref().unwrap().status(),
            StatusCode::INTERNAL_SERVER_ERROR
        );
        mock.assert_async().await;
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn test_401_max_retries_zero() {
        let server = MockServer::start_async().await;

        let mock = server
            .mock_async(|when, then| {
                when.method(httpmock::Method::GET)
                    .path("/api/test")
                    .header("X-Auth-Token", "old-token");
                then.status(StatusCode::UNAUTHORIZED)
                    .body(r#"{"error": {"message": "Unauthorized"}, "message": "Unauthorized"}"#);
            })
            .await;

        let client = create_test_client(&server, "old-token", 0, None);

        let request = http::Request::builder()
            .method(http::Method::GET)
            .uri(format!("{}/api/test", server.base_url()));

        let result = client.rest_async(request, Vec::new()).await;
        assert!(result.is_ok());
        assert_eq!(result.as_ref().unwrap().status(), StatusCode::UNAUTHORIZED);
        mock.assert_async().await;
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn test_concurrent_requests() {
        let server = MockServer::start_async().await;

        let mock = server
            .mock_async(|when, then| {
                when.method(httpmock::Method::GET)
                    .path("/api/test")
                    .header("X-Auth-Token", "shared-token");
                then.status(StatusCode::OK).body("{\"result\": \"ok\"}");
            })
            .await;

        let client = create_test_client(&server, "shared-token", 0, None);

        // Spawn multiple concurrent requests sharing the same client
        let futures = (0..5)
            .map(|i| {
                let client = client.clone();
                let base_url = server.base_url();
                async move {
                    let request = http::Request::builder()
                        .method(http::Method::GET)
                        .uri(format!("{}/api/test?i={}", base_url, i));
                    client.rest_async(request, Vec::new()).await
                }
            })
            .collect::<Vec<_>>();

        let results = futures::future::join_all(futures).await;
        for r in results {
            assert!(r.is_ok(), "expected Ok, got {:?}", r);
            assert_eq!(r.unwrap().status(), StatusCode::OK);
        }
        mock.assert_calls(5);
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn test_clone_shares_session() {
        let server = MockServer::start_async().await;

        let _mock = server
            .mock_async(|when, then| {
                when.method(httpmock::Method::GET)
                    .path("/api/test")
                    .header("X-Auth-Token", "shared-token");
                then.status(StatusCode::OK).body("{\"result\": \"ok\"}");
            })
            .await;

        let client = create_test_client(&server, "shared-token", 0, None);

        // Verify both original and clone share the same token
        assert_eq!(
            client.get_auth_token().unwrap().expose_secret(),
            "shared-token"
        );

        let clone = client.clone();
        assert_eq!(
            clone.get_auth_token().unwrap().expose_secret(),
            "shared-token"
        );

        // Both should be able to make requests
        let request = http::Request::builder()
            .method(http::Method::GET)
            .uri(format!("{}/api/test", server.base_url()));
        let result = client.rest_async(request, Vec::new()).await;
        assert!(result.is_ok());

        let request = http::Request::builder()
            .method(http::Method::GET)
            .uri(format!("{}/api/test", server.base_url()));
        let result = clone.rest_async(request, Vec::new()).await;
        assert!(result.is_ok());
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn test_retry_token_update_visible_to_clone() {
        let server = MockServer::start_async().await;

        // First call: returns 401 for old token
        let mock_401 = server
            .mock_async(|when, then| {
                when.method(httpmock::Method::GET)
                    .path("/api/test")
                    .header("X-Auth-Token", "old-token");
                then.status(StatusCode::UNAUTHORIZED)
                    .body(r#"{"error": {"message": "Unauthorized"}, "message": "Unauthorized"}"#);
            })
            .await;

        // Re-auth: identity returns new token
        let expires = (Utc::now() + chrono::TimeDelta::hours(1)).to_rfc3339();
        let _mock_reauth = server
            .mock_async(|when, then| {
                when.method(httpmock::Method::POST).path("/auth/tokens");
                then.status(StatusCode::CREATED)
                    .header("x-subject-token", "new-token")
                    .json_body(json!({
                        "token": {
                            "id": "token-id",
                            "expires_at": &expires,
                            "project": { "id": "test-project", "name": "TestProject" },
                            "user": { "id": "test-user", "name": "test-user" }
                        }
                    }));
            })
            .await;

        // Subsequent calls: use new token
        let mock_new = server
            .mock_async(|when, then| {
                when.method(httpmock::Method::GET)
                    .path("/api/test")
                    .header("X-Auth-Token", "new-token");
                then.status(StatusCode::OK).body("{\"result\": \"ok\"}");
            })
            .await;

        let client = create_test_client(&server, "old-token", 1, None);
        let clone = client.clone();

        // Original client triggers 401 → re-auth → 200
        let request = http::Request::builder()
            .method(http::Method::GET)
            .uri(format!("{}/api/test", server.base_url()));
        let result = client.rest_async(request, Vec::new()).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap().status(), StatusCode::OK);

        // Clone should now also have the new token (shared session)
        assert_eq!(clone.get_auth_token().unwrap().expose_secret(), "new-token");

        // Clone should use the new token successfully
        let request = http::Request::builder()
            .method(http::Method::GET)
            .uri(format!("{}/api/test", server.base_url()));
        let result = clone.rest_async(request, Vec::new()).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap().status(), StatusCode::OK);

        mock_401.assert_async().await;
        mock_new.assert_calls(2);
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn test_multiple_clones_concurrent_with_retry() {
        let server = MockServer::start_async().await;

        // 401 for old token on all clones
        let _mock_401 = server
            .mock_async(|when, then| {
                when.method(httpmock::Method::GET)
                    .path("/api/test")
                    .header("X-Auth-Token", "old-token");
                then.status(StatusCode::UNAUTHORIZED)
                    .body(r#"{"error": {"message": "Unauthorized"}, "message": "Unauthorized"}"#);
            })
            .await;

        // Re-auth: identity returns new token
        let expires = (Utc::now() + chrono::TimeDelta::hours(1)).to_rfc3339();
        let _mock_reauth = server
            .mock_async(|when, then| {
                when.method(httpmock::Method::POST).path("/auth/tokens");
                then.status(StatusCode::CREATED)
                    .header("x-subject-token", "new-token")
                    .json_body(json!({
                        "token": {
                            "id": "token-id",
                            "expires_at": &expires,
                            "project": { "id": "test-project", "name": "TestProject" },
                            "user": { "id": "test-user", "name": "test-user" }
                        }
                    }));
            })
            .await;

        // Subsequent calls with new token
        let _mock_new = server
            .mock_async(|when, then| {
                when.method(httpmock::Method::GET)
                    .path("/api/test")
                    .header("X-Auth-Token", "new-token");
                then.status(StatusCode::OK).body("{\"result\": \"ok\"}");
            })
            .await;

        let client = create_test_client(&server, "old-token", 1, None);

        // Spawn 3 clones, each triggering 401 → re-auth → 200
        let futures = (0..3)
            .map(|i| {
                let client = client.clone();
                let base_url = server.base_url();
                async move {
                    let request = http::Request::builder()
                        .method(http::Method::GET)
                        .uri(format!("{}/api/test?i={}", base_url, i));
                    client.rest_async(request, Vec::new()).await
                }
            })
            .collect::<Vec<_>>();

        let results = futures::future::join_all(futures).await;
        for r in results {
            assert!(r.is_ok(), "expected Ok, got {:?}", r);
            assert_eq!(r.unwrap().status(), StatusCode::OK);
        }

        // Single-flight: 3 concurrent 401s must trigger exactly one re-auth.
        assert_eq!(
            _mock_reauth.calls_async().await,
            1,
            "expected exactly one re-auth call despite 3 concurrent 401s"
        );
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn test_get_service_endpoint_identity() {
        let server = MockServer::start_async().await;
        let client = create_test_client(&server, "test-token", 0, None);
        let base_url = server.base_url();

        let ep = client
            .get_service_endpoint(&ServiceType::Identity, None)
            .await
            .unwrap();
        assert!(ep.url_str().starts_with(&base_url));
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn test_get_service_endpoint_not_found() {
        let server = MockServer::start_async().await;
        let client = create_test_client(&server, "test-token", 0, None);

        let result = client
            .get_service_endpoint(&ServiceType::Compute, None)
            .await;
        assert!(result.is_err());
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn test_get_service_endpoint_with_version() {
        let server = MockServer::start_async().await;
        let client = create_test_client(&server, "test-token", 0, None);

        let v3 = ApiVersion::new(3, 0);
        let _ep = client
            .get_service_endpoint(&ServiceType::Identity, Some(&v3))
            .await
            .unwrap();
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn test_get_current_project() {
        let server = MockServer::start_async().await;
        let client = create_test_client(&server, "test-token", 0, None);

        let project = client.get_current_project();
        assert!(project.is_some());
        let project = project.unwrap();
        assert_eq!(project.id.as_deref(), Some("test-project"));
        assert_eq!(project.name.as_deref(), Some("TestProject"));
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn test_get_auth_token() {
        let server = MockServer::start_async().await;
        let client = create_test_client(&server, "my-secret-token", 0, None);

        let token = client.get_auth_token();
        assert!(token.is_some());
        assert_eq!(token.unwrap().expose_secret(), "my-secret-token");
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn test_get_auth_state() {
        let server = MockServer::start_async().await;
        let client = create_test_client(&server, "test-token", 0, None);

        let auth_state = client.get_auth_state(None);
        assert!(matches!(auth_state.unwrap(), AuthState::Valid));
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn test_get_auth_state_with_offset() {
        let server = MockServer::start_async().await;
        let client = create_test_client(&server, "test-token", 0, None);

        // Token expires in 1 hour, offset 2 hours → not valid
        let auth_state = client.get_auth_state(Some(TimeDelta::hours(2)));
        assert!(!matches!(auth_state.unwrap(), AuthState::Valid));

        // Token expires in 1 hour, offset 30 minutes → valid
        let auth_state = client.get_auth_state(Some(TimeDelta::minutes(30)));
        assert!(matches!(auth_state.unwrap(), AuthState::Valid));
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn test_get_auth_info() {
        let server = MockServer::start_async().await;
        let client = create_test_client(&server, "test-token", 0, None);

        let auth_info = client.get_auth_info();
        assert!(auth_info.is_some());
        let auth_info = auth_info.unwrap();
        assert!(auth_info.token.project.is_some());
        assert_eq!(
            auth_info.token.project.unwrap().id.as_deref(),
            Some("test-project")
        );
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn test_401_retry_exhaustion() {
        let server = MockServer::start_async().await;

        // Always 401 for old token
        let mock_401_old = server
            .mock_async(|when, then| {
                when.method(httpmock::Method::GET)
                    .path("/api/test")
                    .header("X-Auth-Token", "old-token");
                then.status(StatusCode::UNAUTHORIZED);
            })
            .await;

        // Re-auth: identity returns new token
        let expires = (Utc::now() + chrono::TimeDelta::hours(1)).to_rfc3339();
        let _mock_reauth = server
            .mock_async(|when, then| {
                when.method(httpmock::Method::POST).path("/auth/tokens");
                then.status(StatusCode::CREATED)
                    .header("x-subject-token", "new-token")
                    .json_body(json!({
                        "token": {
                            "id": "token-id",
                            "expires_at": &expires,
                            "project": { "id": "test-project", "name": "TestProject" },
                            "user": { "id": "test-user", "name": "test-user" }
                        }
                    }));
            })
            .await;

        // New token also gets 401
        let mock_401_new = server
            .mock_async(|when, then| {
                when.method(httpmock::Method::GET)
                    .path("/api/test")
                    .header("X-Auth-Token", "new-token");
                then.status(StatusCode::UNAUTHORIZED);
            })
            .await;

        let client = create_test_client(&server, "old-token", 1, None);

        let request = http::Request::builder()
            .method(http::Method::GET)
            .uri(format!("{}/api/test", server.base_url()));

        // Should return 401 after exhausting retries
        let result = client.rest_async(request, Vec::new()).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap().status(), StatusCode::UNAUTHORIZED);

        mock_401_old.assert_async().await;
        mock_401_new.assert_async().await;
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn test_mixed_read_write_concurrency() {
        let server = MockServer::start_async().await;

        let mock = server
            .mock_async(|when, then| {
                when.method(httpmock::Method::GET)
                    .path("/api/test")
                    .header("X-Auth-Token", "shared-token");
                then.status(StatusCode::OK).body("{\"result\": \"ok\"}");
            })
            .await;

        let client = create_test_client(&server, "shared-token", 0, None);

        let rest_futures: Vec<_> = (0..4)
            .map(|i| {
                let client = client.clone();
                let base_url = server.base_url();
                async move {
                    let request = http::Request::builder()
                        .method(http::Method::GET)
                        .uri(format!("{}/api/test?i={}", base_url, i));
                    client.rest_async(request, Vec::new()).await
                }
            })
            .collect();

        let auth_futures: Vec<_> = (0..4)
            .map(|_| {
                let client = client.clone();
                async move {
                    let _token = client.get_auth_token();
                    Some(client.get_auth_token())
                }
            })
            .collect();

        let rest_results = futures::future::join_all(rest_futures).await;
        for r in rest_results {
            assert!(r.is_ok(), "expected Ok, got {:?}", r);
            assert_eq!(r.unwrap().status(), StatusCode::OK);
        }

        let auth_results = futures::future::join_all(auth_futures).await;
        for r in auth_results {
            assert!(r.is_some());
        }

        mock.assert_calls(4);
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn test_rest_read_body_async() {
        let server = MockServer::start_async().await;

        let mock = server
            .mock_async(|when, then| {
                when.method(httpmock::Method::POST)
                    .path("/api/upload")
                    .header("X-Auth-Token", "test-token");
                then.status(StatusCode::OK)
                    .body("{\"result\": \"uploaded\"}");
            })
            .await;

        let client = create_test_client(&server, "test-token", 0, None);

        use futures::io::Cursor;
        let body = BoxedAsyncRead::new(Cursor::new(b"{\"data\": \"test\"}".to_vec()));

        let request = http::Request::builder()
            .method(http::Method::POST)
            .uri(format!("{}/api/upload", server.base_url()));

        let result = client.rest_read_body_async(request, body).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap().status(), StatusCode::OK);

        mock.assert_async().await;
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn test_download_async() {
        let server = MockServer::start_async().await;

        let mock = server
            .mock_async(|when, then| {
                when.method(httpmock::Method::GET)
                    .path("/api/download")
                    .header("X-Auth-Token", "test-token");
                then.status(StatusCode::OK)
                    .header("content-type", "application/octet-stream")
                    .body("binary-data-here");
            })
            .await;

        let client = create_test_client(&server, "test-token", 0, None);

        let request = http::Request::builder()
            .method(http::Method::GET)
            .uri(format!("{}/api/download", server.base_url()));

        let result = client.download_async(request, Vec::new()).await;
        assert!(result.is_ok());

        let (headers, _stream) = result.unwrap();
        assert!(headers.get("content-type").is_some());

        mock.assert_async().await;
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn test_clone_shares_service_endpoint() {
        let server = MockServer::start_async().await;
        let base_url = server.base_url();

        let client = create_test_client(&server, "test-token", 0, None);
        let clone = client.clone();

        // Both should be able to query the identity endpoint
        let ep1 = client
            .get_service_endpoint(&ServiceType::Identity, None)
            .await
            .unwrap();
        let ep2 = clone
            .get_service_endpoint(&ServiceType::Identity, None)
            .await
            .unwrap();

        assert!(ep1.url_str().starts_with(&base_url));
        assert!(ep2.url_str().starts_with(&base_url));
        assert_eq!(ep1.url_str(), ep2.url_str());
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn test_get_service_endpoint_refresh_catalog_on_missing() {
        let server = MockServer::start_async().await;

        // Mock the /auth/catalog endpoint that returns a catalog with compute
        let catalog_json = json!({
            "catalog": [
                {
                    "type": "compute",
                    "name": "nova",
                    "endpoints": [
                        {
                            "id": "ep-compute-1",
                            "region_id": "RegionOne",
                            "region": "RegionOne",
                            "interface": "public",
                            "url": "http://nova.example.com/v2.1",
                            "availability_zone_hints": []
                        }
                    ]
                }
            ]
        });

        let _mock_catalog = server
            .mock_async(|when, then| {
                when.method(httpmock::Method::GET).path("/auth/catalog");
                then.status(StatusCode::OK)
                    .header("content-type", "application/json")
                    .body(catalog_json.to_string());
            })
            .await;

        let client = create_test_client(&server, "test-token", 0, None);

        // Initially compute is not in catalog, but after refresh it should be found
        let ep = client
            .get_service_endpoint(&ServiceType::Compute, None)
            .await
            .unwrap();

        assert_eq!(ep.url_str(), "http://nova.example.com/v2.1");
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn test_get_service_endpoint_no_refresh_if_found() {
        let server = MockServer::start_async().await;

        let client = create_test_client(&server, "test-token", 0, None);

        // Identity is in the catalog, so no refresh should happen
        let ep = client
            .get_service_endpoint(&ServiceType::Identity, None)
            .await
            .unwrap();

        assert!(ep.url_str().starts_with(&server.base_url()));
        // No catalog fetch should occur since identity was in initial catalog
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn test_get_service_endpoint_refresh_failure_returns_error() {
        let server = MockServer::start_async().await;

        // Mock /auth/catalog to return 401 (unauthorized)
        let _mock_catalog = server
            .mock_async(|when, then| {
                when.method(httpmock::Method::GET).path("/auth/catalog");
                then.status(StatusCode::UNAUTHORIZED)
                    .body(r#"{"Error": "Unauthorized"}"#);
            })
            .await;

        let client = create_test_client(&server, "invalid-token", 0, None);

        // Attempt to get service not in catalog; refresh fails; error returned
        let result = client
            .get_service_endpoint(&ServiceType::Compute, None)
            .await;
        assert!(result.is_err());

        let err = result.unwrap_err();
        match err {
            api::ApiError::Catalog { source } => {
                assert!(matches!(source, CatalogError::ServiceNotConfigured { .. }));
            }
            other => panic!("Expected Catalog::ServiceNotConfigured, got {:?}", other),
        }
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn test_401_retry_preserves_headers() {
        let server = MockServer::start_async().await;

        let mock_401 = server
            .mock_async(|when, then| {
                when.method(httpmock::Method::GET)
                    .path("/api/test")
                    .header("x-auth-token", "old-token")
                    .header("content-type", "application/json")
                    .header("x-custom-header", "custom-value");
                then.status(StatusCode::UNAUTHORIZED)
                    .body(r#"{"error": {"message": "Unauthorized"}, "message": "Unauthorized"}"#);
            })
            .await;

        let expires = (Utc::now() + chrono::TimeDelta::hours(1)).to_rfc3339();
        let _mock_reauth = server
            .mock_async(|when, then| {
                when.method(httpmock::Method::POST).path("/auth/tokens");
                then.status(StatusCode::CREATED)
                    .header("x-subject-token", "new-token")
                    .json_body(json!({
                        "token": {
                            "id": "token-id",
                            "name": "token-name",
                            "expires_at": &expires,
                            "project": {
                                "id": "test-project",
                                "name": "TestProject"
                            },
                            "user": {
                                "id": "test-user",
                                "name": "test-user"
                            }
                        }
                    }));
            })
            .await;

        let mock_200 = server
            .mock_async(|when, then| {
                when.method(httpmock::Method::GET)
                    .path("/api/test")
                    .header("x-auth-token", "new-token")
                    .header("content-type", "application/json")
                    .header("x-custom-header", "custom-value");
                then.status(StatusCode::OK).body("{\"result\": \"ok\"}");
            })
            .await;

        let client = create_test_client(&server, "old-token", 1, None);

        let request = http::Request::builder()
            .method(http::Method::GET)
            .uri(format!("{}/api/test", server.base_url()))
            .header("Content-Type", "application/json")
            .header("X-Custom-Header", "custom-value");

        let result = client.rest_async(request, Vec::new()).await;
        assert!(result.is_ok(), "expected Ok, got {:?}", result);
        assert_eq!(result.as_ref().unwrap().status(), StatusCode::OK);

        mock_401.assert_async().await;
        mock_200.assert_async().await;
    }
}
