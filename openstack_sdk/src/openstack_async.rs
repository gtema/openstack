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
use std::sync::{Arc, RwLock};
use std::time::{Duration, SystemTime};
use std::{fs::File, io::Read};

use async_trait::async_trait;
use bytes::Bytes;
use chrono::TimeDelta;
use futures::io::{Error as IoError, ErrorKind as IoErrorKind};
use futures::stream::TryStreamExt;
use http::{HeaderMap, HeaderValue, Response as HttpResponse, StatusCode, header};
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

// Force the linker to include crate plugins
use openstack_sdk_auth_applicationcredential as _;
#[cfg(feature = "keystone_ng")]
use openstack_sdk_auth_federation as _;
#[cfg(feature = "keystone_ng")]
use openstack_sdk_auth_jwt as _;
use openstack_sdk_auth_oidcaccesstoken as _;
#[cfg(feature = "passkey")]
use openstack_sdk_auth_passkey as _;
use openstack_sdk_auth_password as _;
use openstack_sdk_auth_receipt as token_receipt;
use openstack_sdk_auth_token as token_auth;
use openstack_sdk_auth_totp as _;
use openstack_sdk_auth_websso as _;

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
use openstack_sdk_core::catalog::{CatalogError, ServiceEndpoint};
use openstack_sdk_core::config::CloudConfig;
use openstack_sdk_core::error::{OpenStackError, OpenStackResult, RestError};
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
    /// Auth helper for re-authentication on 401.
    auth_helper: Option<Arc<dyn AuthHelper>>,
    /// Max retries on 401.
    max_auth_retries: u32,
}

impl Clone for AsyncOpenStack {
    fn clone(&self) -> Self {
        Self {
            client: self.client.clone(),
            config: self.config.clone(),
            session: Arc::clone(&self.session),
            auth_helper: self.auth_helper.clone(),
            max_auth_retries: self.max_auth_retries,
        }
    }
}

impl Debug for AsyncOpenStack {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Use blocking lock on the main thread for debug
        // This is only for Debug impl, not in async context
        f.debug_struct("OpenStack")
            .field(
                "service_endpoints",
                &self
                    .session
                    .read()
                    .unwrap_or_else(|_| panic!("Debug: session lock poisoned"))
                    .catalog,
            )
            .finish()
    }
}

#[async_trait]
impl api::RestClient for AsyncOpenStack {
    type Error = RestError;

    fn get_current_project(&self) -> Option<Project> {
        let session = self.session.read().unwrap_or_else(|e| e.into_inner());
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
        let mut auth = {
            let session = self.session_read_rest("AsyncClient::rest")?;
            session.auth.clone()
        };
        let mut retries = 0;

        let orig_method = request.method_ref().cloned().unwrap_or(http::Method::GET);
        let orig_uri = request.uri_ref().cloned().unwrap_or_default();

        let mut request = request;
        loop {
            let result = self
                .rest_with_auth_async(request, body.clone(), &auth)
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
                match self.handle_401_retry().await {
                    Ok(_) => {
                        auth = {
                            let session = self.session_read_rest("AsyncClient::rest_retry")?;
                            session.auth.clone()
                        };
                        retries += 1;
                        request = http::Request::builder()
                            .method(orig_method.clone())
                            .uri(orig_uri.clone());
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
        let auth = {
            let session = self.session_read_rest("AsyncClient::rest_read_body_async")?;
            session.auth.clone()
        };
        self.rest_with_auth_read_body_async(request, body, &auth)
            .await
    }

    async fn download_async(
        &self,
        request: http::request::Builder,
        body: Vec<u8>,
    ) -> Result<(HeaderMap, BoxedAsyncRead), api::ApiError<<Self as api::RestClient>::Error>> {
        let auth = {
            let session = self.session_read_rest("AsyncClient::download_async")?;
            session.auth.clone()
        };
        self.download_with_auth_async(request, body, &auth).await
    }

    async fn get_service_endpoint(
        &self,
        service_type: &ServiceType,
        version: Option<&ApiVersion>,
    ) -> Result<ServiceEndpoint, api::ApiError<Self::Error>> {
        let session = self.session_read_rest("AsyncClient::get_service_endpoint")?;
        session
            .catalog
            .get_service_endpoint(
                service_type.to_string(),
                version,
                self.config.region_name.as_ref(),
                self.config.interface.as_ref(),
            )
            .cloned()
            .map_err(api::ApiError::catalog)
    }
}

impl AsyncOpenStack {
    /// Lock the session for reading in trait methods (converts PoisonError to RestError).
    fn session_read_rest(
        &self,
        location: &'static str,
    ) -> Result<std::sync::RwLockReadGuard<'_, session::SessionContext>, api::ApiError<RestError>>
    {
        self.session.read().map_err(|e| {
            api::ApiError::client(RestError::SessionPoisoned {
                msg: format!("{}: session read lock poisoned: {}", location, e),
            })
        })
    }

    /// Lock the session for writing in trait methods.
    #[allow(dead_code)]
    fn session_write_rest(
        &self,
        location: &'static str,
    ) -> Result<std::sync::RwLockWriteGuard<'_, session::SessionContext>, api::ApiError<RestError>>
    {
        self.session.write().map_err(|e| {
            api::ApiError::client(RestError::SessionPoisoned {
                msg: format!("{}: session write lock poisoned: {}", location, e),
            })
        })
    }

    /// Lock the session for reading in non-trait methods.
    fn session_read(
        &self,
        location: &'static str,
    ) -> Result<std::sync::RwLockReadGuard<'_, session::SessionContext>, OpenStackError> {
        self.session
            .read()
            .map_err(|e| OpenStackError::SessionPoisoned {
                msg: format!("{}: session read lock poisoned: {}", location, e),
            })
    }

    /// Lock the session for writing in non-trait methods.
    fn session_write(
        &self,
        location: &'static str,
    ) -> Result<std::sync::RwLockWriteGuard<'_, session::SessionContext>, OpenStackError> {
        self.session
            .write()
            .map_err(|e| OpenStackError::SessionPoisoned {
                msg: format!("{}: session write lock poisoned: {}", location, e),
            })
    }

    /// Basic constructor
    fn new_impl(config: &CloudConfig, auth: Auth) -> OpenStackResult<Self> {
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

        Ok(AsyncOpenStack {
            client: client_builder.build()?,
            config: config.clone(),
            session: Arc::new(RwLock::new(session_ctx)),
            auth_helper: None,
            max_auth_retries: 1,
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
        let mut session = self.session_write("set_auth")?;
        session.auth = auth;
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
                HashMap::from([("token".into(), auth.token.clone())]),
                Some(scope),
                None,
            )
            .await?)
    }

    /// Handle 401 Unauthorized by re-authenticating.
    async fn handle_401_retry(&self) -> Result<(), OpenStackError> {
        let scope = AuthTokenScope::try_from(&self.config)?;
        // Re-authenticate with current config. First try to reauth with existing token,
        // fall back to full authentication if auth_helper is available.
        let auth_helper_opt = self.auth_helper.clone();
        if let Some(auth_helper) = auth_helper_opt {
            {
                let mut session = self.session_write("handle_401_retry: clear state")?;
                session.state.clear_all_auth();
            }
            self.authorize_with_auth_helper(Some(scope), &auth_helper, true)
                .await?;
        } else {
            // No auth helper available - try to reauth with the current token
            if let Auth::AuthToken(token) = {
                let session = self.session_read("handle_401_retry: read auth")?;
                session.auth.clone()
            } && let Ok(new_auth) = self.reauth(&token, &scope).await
            {
                self.set_auth(new_auth, false)?;
                return Ok(());
            }
            // Reauth failed, clear state and try full auth as fallback
            {
                let mut session = self.session_write("handle_401_retry: clear state fallback")?;
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
        if let Ok(mut session) = self.session.write() {
            session.state.disable_auth_cache();
        }
        self
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
            let mut session = self.session_write("authorize_with_auth_helper: get cache auth")?;
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
                let mut session = self.session_write("authorize_with_auth_helper: reauthz")?;
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
                            gather_auth_data(
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
                                    gather_auth_data(
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
                    return Err(AuthTokenError::IdentityMethod {
                        auth_type: auth_type.into(),
                    })?;
                }
            }
        }

        let auth_opt = {
            let session = self.session_read("authorize_with_auth_helper: get auth")?;
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
                        self.session_write("authorize_with_auth_helper: set unscoped cache")?;
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
                        self.session_write("authorize_with_auth_helper: set unscoped")?;
                    session
                        .state
                        .set_scope_auth(&AuthTokenScope::Unscoped, token_auth);
                }
            }
        } else {
            return Err(AuthError::AuthTokenNotInResponse)?;
        }

        {
            let mut session = self.session_write("authorize_with_auth_helper: process catalog")?;
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
            let session = self.session_read("discover_service_endpoint: get ep")?;
            session
                .catalog
                .get_service_endpoint(
                    service_type.to_string(),
                    None,
                    self.config.region_name.as_ref(),
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
            let session = self.session_read("discover_service_endpoint: check discovery")?;
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
                    self.session_read("discover_service_endpoint: get auth for discovery")?;
                session.auth.clone()
            };
            match self.rest_with_auth_async(req, Vec::new(), &auth).await {
                Ok(rsp) => {
                    if rsp.status() != StatusCode::NOT_FOUND {
                        let ok = {
                            let mut session =
                                self.session_write("discover_service_endpoint: process catalog")?;
                            session
                                .catalog
                                .process_endpoint_discovery(
                                    service_type,
                                    &try_url,
                                    rsp.body(),
                                    self.config.region_name.as_ref(),
                                    self.config.interface.as_ref(),
                                )
                                .is_ok()
                        };
                        if ok {
                            debug!("Finished service version discovery at {}", try_url.as_str());
                            debug!("catalog {:?}", {
                                self.session
                                    .read()
                                    .unwrap_or_else(|_| panic!("discover: session lock poisoned"))
                                    .catalog
                                    .clone()
                            });
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

    // TODO(gtema): rename to `get_catalog`)
    /// Return catalog information given in the token
    pub fn get_token_catalog(&self) -> Option<Vec<ServiceEndpoints>> {
        let session = self.session.read().unwrap_or_else(|e| e.into_inner());
        session.catalog.get_token_catalog()
    }

    /// Return current authentication information
    pub fn get_auth_info(&self) -> Option<AuthResponse> {
        let session = self.session.read().unwrap_or_else(|e| e.into_inner());
        if let Auth::AuthToken(token) = &session.auth {
            return token.auth_info.clone();
        }
        None
    }

    /// Return current authentication status
    ///
    /// Offset can be used to calculate imminent expiration.
    pub fn get_auth_state(&self, offset: Option<TimeDelta>) -> Option<AuthState> {
        let session = self.session.read().unwrap_or_else(|e| e.into_inner());
        if let Auth::AuthToken(token) = &session.auth {
            return Some(token.get_state(offset));
        }
        None
    }

    /// Return current authentication token
    pub fn get_auth_token(&self) -> Option<SecretString> {
        let session = self.session.read().unwrap_or_else(|e| e.into_inner());
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
            token: openstack_sdk_auth_core::types::AuthToken {
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

        AsyncOpenStack {
            client: reqwest::Client::builder()
                .timeout(Duration::from_secs(10))
                .build()
                .unwrap(),
            config,
            session: Arc::new(RwLock::new(session::SessionContext {
                auth,
                catalog,
                state,
            })),
            auth_helper,
            max_auth_retries: max_retries,
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
}
