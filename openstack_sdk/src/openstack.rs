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

//! Synchronous OpenStack client

use std::collections::HashMap;
use std::convert::TryInto;
use std::fmt::{self, Debug};
use std::sync::{Arc, RwLock};
use std::time::SystemTime;
use std::{fs::File, io::Read};

use bytes::Bytes;
use chrono::TimeDelta;
use http::{Response as HttpResponse, StatusCode};
use reqwest::{
    Certificate, Url,
    blocking::{Client as HttpClient, Request, Response},
};
use secrecy::SecretString;
use tokio::runtime::Runtime;
use tracing::{Level, debug, error, event, info, instrument, trace, warn};

// Force the linker to include crate plugins
use openstack_sdk_auth_applicationcredential as _;
use openstack_sdk_auth_oidcaccesstoken as _;
use openstack_sdk_auth_password as _;
use openstack_sdk_auth_receipt as token_receipt;
use openstack_sdk_auth_token as token_auth;
use openstack_sdk_auth_totp as _;

use openstack_sdk_auth_core::{
    Auth, AuthError, AuthPluginRegistration, AuthToken, OpenStackAuthType,
    authtoken::AuthTokenError, authtoken_scope::AuthTokenScope, types::Project,
};

use openstack_sdk_core::api::{
    self, Client,
    query::{self},
};
use openstack_sdk_core::auth::{
    AuthState,
    auth_helper::{AuthHelper, Dialoguer, Noop},
    gather_auth_data,
};
use openstack_sdk_core::catalog::{CatalogError, ServiceEndpoint};
use openstack_sdk_core::config::CloudConfig;
use openstack_sdk_core::error::{OpenStackError, OpenStackResult, RestError};
use openstack_sdk_core::types::{ApiVersion, ServiceType};
use openstack_sdk_core::utils::expand_tilde;

use crate::auth::authtoken::AuthType;
use crate::session;

// Private enum that enables the parsing of the cert bytes to be
// delayed until the client is built rather than when they're passed
// to a builder.
#[allow(dead_code)]
enum ClientCert {
    None,
    #[cfg(feature = "client_der")]
    Der(Vec<u8>, String),
    #[cfg(feature = "client_pem")]
    Pem(Vec<u8>),
}

/// Synchronous client for the OpenStack API for a single user.
///
/// Separate Identity (not the scope) should use separate instances of this.
/// ```rust
/// use openstack_sdk::api::{paged, Pagination, Query};
/// use openstack_sdk::{OpenStack, config::ConfigFile, OpenStackError};
/// use openstack_sdk::types::ServiceType;
/// use openstack_sdk::api::compute::v2::flavor::list;
///
/// fn list_flavors() -> Result<(), OpenStackError> {
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
///     let mut s = OpenStack::new(&profile)?;
///
///     // Invoke service discovery when desired.
///     s.discover_service_endpoint(&ServiceType::Compute)?;
///
///     // Execute the call with pagination limiting maximum amount of entries to 1000
///     let data: Vec<serde_json::Value> = paged(ep, Pagination::Limit(1000))
///         .query(&s)
///         .unwrap();
///
///     println!("Data = {:?}", data);
///     Ok(())
/// }
/// ```
pub struct OpenStack {
    /// The client to use for API calls.
    client: HttpClient,
    /// Cloud configuration
    config: CloudConfig,
    /// Session context (auth, catalog, state)
    session: Arc<RwLock<session::SessionContext>>,
    /// Auth helper for re-authentication on 401.
    auth_helper: Option<Arc<dyn AuthHelper>>,
    /// Max retries on 401.
    max_auth_retries: u32,
}

impl Clone for OpenStack {
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

impl Debug for OpenStack {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let catalog = self
            .session
            .read()
            .unwrap_or_else(|_| panic!("Debug: session lock poisoned"));
        f.debug_struct("OpenStack")
            .field("service_endpoints", &catalog.catalog)
            .finish()
    }
}

/// Should a certificate be validated in tls connections.
/// The Insecure option is used for self-signed certificates.
#[allow(dead_code)]
#[derive(Debug, Clone)]
enum CertPolicy {
    Default,
    Insecure,
}

impl OpenStack {
    /// Lock the session for reading.
    fn session_read(
        &self,
        location: &str,
    ) -> OpenStackResult<std::sync::RwLockReadGuard<'_, session::SessionContext>> {
        self.session
            .read()
            .map_err(|e| OpenStackError::SessionPoisoned {
                msg: format!("{}: session read lock poisoned: {}", location, e),
            })
    }

    /// Lock the session for writing.
    fn session_write(
        &self,
        location: &str,
    ) -> OpenStackResult<std::sync::RwLockWriteGuard<'_, session::SessionContext>> {
        self.session
            .write()
            .map_err(|e| OpenStackError::SessionPoisoned {
                msg: format!("{}: session write lock poisoned: {}", location, e),
            })
    }

    /// Lock the session for reading in RestClient/AsyncClient trait methods.
    fn session_read_rest(
        &self,
        location: &str,
    ) -> Result<std::sync::RwLockReadGuard<'_, session::SessionContext>, api::ApiError<RestError>>
    {
        self.session.read().map_err(|e| {
            api::ApiError::client(RestError::SessionPoisoned {
                msg: format!("{}: session read lock poisoned: {}", location, e),
            })
        })
    }

    fn new_impl(config: &CloudConfig, auth: Auth) -> OpenStackResult<Self> {
        let mut client_builder = HttpClient::builder();

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

        // Pass CloudConfig.auth_cache as override; SessionContext resolves priority chain.
        let session_ctx = session::SessionContext::new(config, auth, config.auth_cache)?;

        Ok(OpenStack {
            client: client_builder.build()?,
            config: config.clone(),
            session: Arc::new(RwLock::new(session_ctx)),
            auth_helper: None,
            max_auth_retries: 1,
        })
    }

    #[instrument(name = "connect", level = "trace", skip(config))]
    pub fn new(config: &CloudConfig) -> OpenStackResult<Self> {
        let session = Self::new_impl(config, Auth::None)?;
        session.discover_service_endpoint(&ServiceType::Identity)?;
        session.authorize(None, false, false)?;
        Ok(session)
    }

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

    pub fn authorize(
        &self,
        scope: Option<AuthTokenScope>,
        interactive: bool,
        renew_auth: bool,
    ) -> Result<(), OpenStackError> {
        if interactive {
            self.authorize_with_auth_helper(scope, &Dialoguer::default(), renew_auth)
        } else {
            self.authorize_with_auth_helper(scope, &Noop::default(), renew_auth)
        }
    }

    fn reauth(&self, auth: &AuthToken, scope: &AuthTokenScope) -> Result<Auth, OpenStackError> {
        let rt = Runtime::new()?;
        let client = reqwest::Client::new();
        Ok(rt.block_on(
            token_auth::PLUGIN.auth(
                &client,
                self.get_service_endpoint(&ServiceType::Identity, Some(&ApiVersion::from((3, 0))))?
                    .url(),
                HashMap::from([("token".into(), auth.token.clone())]),
                Some(scope),
                None,
            ),
        )?)
    }

    fn handle_401_retry(&self) -> Result<(), OpenStackError> {
        let scope = AuthTokenScope::try_from(&self.config)?;
        let auth_helper_opt = self.auth_helper.clone();
        if let Some(auth_helper) = auth_helper_opt {
            {
                let mut session = self.session_write("handle_401_retry")?;
                session.state.clear_all_auth();
            }
            self.authorize_with_auth_helper(Some(scope), &auth_helper, true)?;
        } else {
            // No auth helper available - try to reauth with the current token
            if let Auth::AuthToken(token) = {
                let session = self.session_read("handle_401_retry")?;
                session.auth.clone()
            } && let Ok(new_auth) = self.reauth(&token, &scope)
            {
                self.set_auth(new_auth, false)?;
                return Ok(());
            }
            // Reauth failed, clear state and try full auth as fallback
            {
                let mut session = self.session_write("handle_401_retry clear")?;
                session.state.clear_all_auth();
            }
            self.authorize_with_auth_helper(Some(scope), &Noop::default(), true)?;
        }
        Ok(())
    }

    pub fn set_max_auth_retries(&mut self, n: u32) -> &mut Self {
        self.max_auth_retries = n;
        self
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

    pub fn authorize_with_auth_helper<A>(
        &self,
        scope: Option<AuthTokenScope>,
        auth_helper: &A,
        renew_auth: bool,
    ) -> Result<(), OpenStackError>
    where
        A: AuthHelper + Send,
    {
        let rt = Runtime::new()?;
        let requested_scope = scope.unwrap_or(AuthTokenScope::try_from(&self.config)?);

        let cached_auth = {
            let mut session = self.session_write("authorize_with_auth_helper_cached")?;
            session.state.get_scope_auth(&requested_scope)
        };

        if let Some(auth) = cached_auth
            && !renew_auth
        {
            trace!("Auth already available");
            self.set_auth(Auth::AuthToken(Box::new(auth.clone())), true)?;
        } else {
            let auth_type = AuthType::from_cloud_config(&self.config)?;
            let force_new_auth = matches!(auth_type, AuthType::V3ApplicationCredential);
            let available_auth_opt = {
                let mut session = self.session_write("authorize_with_auth_helper_available")?;
                session.state.get_any_valid_auth()
            };
            if let (Some(available_auth), false) = (available_auth_opt, force_new_auth) {
                trace!("Valid Auth is available for reauthz: {:?}", available_auth);
                let token_auth = self.reauth(&available_auth, &requested_scope)?;
                self.set_auth(token_auth.clone(), false)?;
            } else {
                trace!("No Auth already available. Proceeding with new login");

                let auth_type = auth_type.as_str();
                if let Some(authenticator) = inventory::iter::<AuthPluginRegistration>
                    .into_iter()
                    .find(|x| x.method.get_supported_auth_methods().contains(&auth_type))
                    .map(|x| x.method)
                {
                    let auth_hints = self
                        .config
                        .auth_methods
                        .as_ref()
                        .map(|methods| serde_json::json!({"auth_methods": methods}));
                    let auth_data = rt.block_on(gather_auth_data(
                        &authenticator.requirements(auth_hints.as_ref())?,
                        &self.config,
                        auth_helper,
                    ))?;
                    let client = reqwest::Client::new();
                    match rt.block_on(
                        authenticator.auth(
                            &client,
                            self.get_service_endpoint(
                                &ServiceType::Identity,
                                Some(&ApiVersion::from(authenticator.api_version())),
                            )?
                            .url(),
                            auth_data,
                            Some(&requested_scope),
                            auth_hints.as_ref(),
                        ),
                    ) {
                        Ok(token_auth) => {
                            self.set_auth(token_auth.clone(), false)?;
                        }
                        Err(AuthError::AuthReceipt(receipt)) => {
                            if let Some(authenticator) = inventory::iter::<AuthPluginRegistration>
                                .into_iter()
                                .find(|x| {
                                    x.method.get_supported_auth_methods().contains(&"receipt")
                                })
                                .map(|x| x.method)
                            {
                                let auth_hints = serde_json::to_value(&receipt)?;
                                let auth_data = rt.block_on(gather_auth_data(
                                    &token_receipt::PLUGIN.requirements(Some(&auth_hints))?,
                                    &self.config,
                                    auth_helper,
                                ))?;
                                let token_auth = rt.block_on(
                                    token_receipt::PLUGIN.auth(
                                        &client,
                                        self.get_service_endpoint(
                                            &ServiceType::Identity,
                                            Some(&ApiVersion::from(authenticator.api_version())),
                                        )?
                                        .url(),
                                        auth_data,
                                        Some(&requested_scope),
                                        Some(&auth_hints),
                                    ),
                                )?;
                                self.set_auth(token_auth.clone(), false)?;
                            }
                        }
                        Err(other) => {
                            return Err(other.into());
                        }
                    }
                } else {
                    return Err(AuthTokenError::IdentityMethodSync {
                        auth_type: auth_type.into(),
                    })?;
                }
            }
        }

        // All paths fall through to catalog update
        {
            let mut session = self.session_write("authorize_with_auth_helper_catalog")?;
            let auth_data = {
                if let Auth::AuthToken(token_data) = &session.auth {
                    match &token_data.auth_info {
                        Some(ad) => ad.clone(),
                        _ => return Err(OpenStackError::NoAuth),
                    }
                } else {
                    return Err(OpenStackError::NoAuth);
                }
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

    #[instrument(skip(self))]
    pub fn discover_service_endpoint(
        &self,
        service_type: &ServiceType,
    ) -> Result<(), OpenStackError> {
        let ep_opt = {
            let session = self.session_read("discover_service_endpoint_ep")?;
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
            let session = self.session_read("discover_service_endpoint_discovery_allowed")?;
            session.catalog.discovery_allowed(service_type.to_string())
        };
        if !discovery_allowed {
            return Ok(());
        }

        info!("Performing `{}` endpoint version discovery", service_type);

        let orig_url = ep.url().clone();
        let mut try_url = ep.url().clone();
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
                let session = self.session_read("discover_loop_read_auth")?;
                session.auth.clone()
            };
            match self.rest_with_auth(req, Vec::new(), &auth) {
                Ok(rsp) => {
                    if rsp.status() != StatusCode::NOT_FOUND {
                        let ok = {
                            let mut session =
                                self.session_write("discover_loop_process_endpoint")?;
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

    pub fn get_auth_token(&self) -> Option<SecretString> {
        let session = self.session.read().unwrap_or_else(|e| e.into_inner());
        if let Auth::AuthToken(token) = &session.auth {
            return Some(token.token.clone());
        }
        None
    }

    pub fn get_auth_state(&self, offset: Option<TimeDelta>) -> Option<AuthState> {
        let session = self.session.read().unwrap_or_else(|e| e.into_inner());
        if let Auth::AuthToken(token) = &session.auth {
            return Some(token.get_state(offset));
        }
        None
    }

    #[instrument(name="request", skip_all, fields(http.uri = request.url().as_str(), http.method = request.method().as_str(), openstack.ver=request.headers().get("openstack-api-version").map(|v| v.to_str().unwrap_or(""))))]
    fn execute_request(&self, request: Request) -> Result<Response, reqwest::Error> {
        info!("Sending request {:?}", request);
        let url: Url = request.url().clone();
        let method = request.method().clone();

        let start = SystemTime::now();
        let rsp = self.client.execute(request)?;
        let elapsed = SystemTime::now().duration_since(start).unwrap_or_default();
        event!(
            name: "http_request",
            Level::INFO,
            url = url.as_str(),
            duration_ms = elapsed.as_millis(),
            status = rsp.status().as_u16(),
            method = method.as_str(),
            request_id = rsp.headers().get("x-openstack-request-id").map(|v| v.to_str().unwrap_or("")),
            "Request completed with status {}",
            rsp.status(),
        );
        Ok(rsp)
    }

    fn rest_with_auth(
        &self,
        mut request: http::request::Builder,
        body: Vec<u8>,
        auth: &Auth,
    ) -> Result<HttpResponse<Bytes>, api::ApiError<<Self as api::RestClient>::Error>> {
        let call = || -> Result<_, RestError> {
            if let Some(headers) = request.headers_mut() {
                auth.set_header(headers)?;
            }
            let http_request = request.body(body)?;
            let request = http_request.try_into()?;

            let rsp = self.execute_request(request)?;

            let mut http_rsp = HttpResponse::builder()
                .status(rsp.status())
                .version(rsp.version());

            if let Some(headers) = http_rsp.headers_mut() {
                headers.extend(rsp.headers().clone())
            }

            Ok(http_rsp.body(rsp.bytes()?)?)
        };
        call().map_err(api::ApiError::client)
    }
}

impl api::RestClient for OpenStack {
    type Error = RestError;

    fn get_current_project(&self) -> Option<Project> {
        let session = self.session.read().unwrap_or_else(|e| e.into_inner());
        if let Auth::AuthToken(token) = &session.auth {
            return token.auth_info.clone().and_then(|x| x.token.project);
        }
        None
    }
}

impl api::Client for OpenStack {
    fn rest(
        &self,
        request: http::request::Builder,
        body: Vec<u8>,
    ) -> Result<HttpResponse<Bytes>, api::ApiError<<Self as api::RestClient>::Error>> {
        let mut auth = {
            let session = self.session_read_rest("Client::rest")?;
            session.auth.clone()
        };
        let mut retries = 0;

        let orig_method = request.method_ref().cloned().unwrap_or(http::Method::GET);
        let orig_uri = request.uri_ref().cloned().unwrap_or_default();

        let mut request = request;
        loop {
            let result = self.rest_with_auth(request, body.clone(), &auth);
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
                match self.handle_401_retry() {
                    Ok(_) => {
                        auth = {
                            let session = self.session_read_rest("Client::rest_401_retry")?;
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

    fn get_service_endpoint(
        &self,
        service_type: &ServiceType,
        version: Option<&ApiVersion>,
    ) -> Result<ServiceEndpoint, api::ApiError<Self::Error>> {
        let session = self.session.read().map_err(|e| {
            api::ApiError::client(RestError::SessionPoisoned {
                msg: format!("get_service_endpoint: session read lock poisoned: {}", e),
            })
        })?;
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

#[cfg(test)]
mod tests {
    use chrono::Utc;
    use httpmock::MockServer;
    use secrecy::ExposeSecret;
    use serde_json::json;
    use std::time::Duration;

    use openstack_sdk_core::api::{Client, RestClient};
    use openstack_sdk_core::catalog::Catalog;

    use super::*;
    use crate::config::Auth as ConfigAuth;

    fn create_test_client(
        server: &MockServer,
        token: &str,
        max_retries: u32,
        auth_helper: Option<Arc<dyn AuthHelper>>,
    ) -> OpenStack {
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

        let token_info = openstack_sdk_auth_core::types::AuthResponse {
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

        OpenStack {
            client: reqwest::blocking::Client::builder()
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

    #[test]
    fn test_401_no_retry_on_success() {
        let server = MockServer::start();

        let mock = server.mock(|when, then| {
            when.method(httpmock::Method::GET)
                .path("/api/test")
                .header("X-Auth-Token", "old-token");
            then.status(StatusCode::OK).body("{\"result\": \"ok\"}");
        });

        let client = create_test_client(&server, "old-token", 1, None);

        let request = http::Request::builder()
            .method(http::Method::GET)
            .uri(format!("{}/api/test", server.base_url()));

        let result = client.rest(request, Vec::new());
        assert!(result.is_ok());
        assert_eq!(result.unwrap().status(), StatusCode::OK);
        mock.assert();
    }

    #[test]
    fn test_401_retry_succeeds() {
        let server = MockServer::start();

        let mock_401 = server.mock(|when, then| {
            when.method(httpmock::Method::GET)
                .path("/api/test")
                .header("X-Auth-Token", "old-token");
            then.status(StatusCode::UNAUTHORIZED)
                .body(r#"{"error": {"message": "Unauthorized"}, "message": "Unauthorized"}"#);
        });

        let expires = (Utc::now() + chrono::TimeDelta::hours(1)).to_rfc3339();
        let _mock_reauth = server.mock(|when, then| {
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
        });

        let mock_200 = server.mock(|when, then| {
            when.method(httpmock::Method::GET)
                .path("/api/test")
                .header("X-Auth-Token", "new-token");
            then.status(StatusCode::OK).body("{\"result\": \"ok\"}");
        });

        let client = create_test_client(&server, "old-token", 1, None);

        let request = http::Request::builder()
            .method(http::Method::GET)
            .uri(format!("{}/api/test", server.base_url()));

        let result = client.rest(request, Vec::new());
        assert!(result.is_ok(), "expected Ok, got {:?}", result);
        assert_eq!(result.unwrap().status(), StatusCode::OK);

        mock_401.assert();
        mock_200.assert();
    }

    #[test]
    fn test_401_non_401_not_retried() {
        let server = MockServer::start();

        let mock = server.mock(|when, then| {
            when.method(httpmock::Method::GET).path("/api/test");
            then.status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(r#"{"error": {"message": "Server Error"}, "message": "Server Error"}"#);
        });

        let client = create_test_client(&server, "old-token", 5, None);

        let request = http::Request::builder()
            .method(http::Method::GET)
            .uri(format!("{}/api/test", server.base_url()));

        let result = client.rest(request, Vec::new());
        assert!(result.is_ok());
        assert_eq!(result.unwrap().status(), StatusCode::INTERNAL_SERVER_ERROR);
        mock.assert();
    }

    #[test]
    fn test_401_max_retries_zero() {
        let server = MockServer::start();

        let mock = server.mock(|when, then| {
            when.method(httpmock::Method::GET)
                .path("/api/test")
                .header("X-Auth-Token", "old-token");
            then.status(StatusCode::UNAUTHORIZED)
                .body(r#"{"error": {"message": "Unauthorized"}, "message": "Unauthorized"}"#);
        });

        let client = create_test_client(&server, "old-token", 0, None);

        let request = http::Request::builder()
            .method(http::Method::GET)
            .uri(format!("{}/api/test", server.base_url()));

        let result = client.rest(request, Vec::new());
        assert!(result.is_ok());
        assert_eq!(result.unwrap().status(), StatusCode::UNAUTHORIZED);
        mock.assert();
    }

    #[test]
    fn test_clone_shares_session() {
        let server = MockServer::start();

        let mock = server.mock(|when, then| {
            when.method(httpmock::Method::GET)
                .path("/api/test")
                .header("X-Auth-Token", "shared-token");
            then.status(StatusCode::OK).body("{\"result\": \"ok\"}");
        });

        let client = create_test_client(&server, "shared-token", 0, None);

        assert_eq!(
            client.get_auth_token().unwrap().expose_secret(),
            "shared-token"
        );

        let clone = client.clone();
        assert_eq!(
            clone.get_auth_token().unwrap().expose_secret(),
            "shared-token"
        );

        let request = http::Request::builder()
            .method(http::Method::GET)
            .uri(format!("{}/api/test", server.base_url()));
        let result = client.rest(request, Vec::new());
        assert!(result.is_ok());

        let request = http::Request::builder()
            .method(http::Method::GET)
            .uri(format!("{}/api/test", server.base_url()));
        let result = clone.rest(request, Vec::new());
        assert!(result.is_ok());

        mock.assert_calls(2);
    }

    #[test]
    fn test_retry_token_update_visible_to_clone() {
        let server = MockServer::start();

        let mock_401 = server.mock(|when, then| {
            when.method(httpmock::Method::GET)
                .path("/api/test")
                .header("X-Auth-Token", "old-token");
            then.status(StatusCode::UNAUTHORIZED)
                .body(r#"{"error": {"message": "Unauthorized"}, "message": "Unauthorized"}"#);
        });

        let expires = (Utc::now() + chrono::TimeDelta::hours(1)).to_rfc3339();
        let _mock_reauth = server.mock(|when, then| {
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
        });

        let mock_new = server.mock(|when, then| {
            when.method(httpmock::Method::GET)
                .path("/api/test")
                .header("X-Auth-Token", "new-token");
            then.status(StatusCode::OK).body("{\"result\": \"ok\"}");
        });

        let client = create_test_client(&server, "old-token", 1, None);
        let clone = client.clone();

        // Original client triggers 401 → re-auth → 200
        let request = http::Request::builder()
            .method(http::Method::GET)
            .uri(format!("{}/api/test", server.base_url()));
        let result = client.rest(request, Vec::new());
        assert!(result.is_ok());
        assert_eq!(result.unwrap().status(), StatusCode::OK);

        // Clone should now have the new token (shared session)
        assert_eq!(clone.get_auth_token().unwrap().expose_secret(), "new-token");

        // Clone uses new token successfully
        let request = http::Request::builder()
            .method(http::Method::GET)
            .uri(format!("{}/api/test", server.base_url()));
        let result = clone.rest(request, Vec::new());
        assert!(result.is_ok());
        assert_eq!(result.unwrap().status(), StatusCode::OK);

        mock_401.assert();
        mock_new.assert_calls(2);
    }

    #[test]
    fn test_concurrent_clones_in_threads() {
        let server = MockServer::start();

        let mock = server.mock(|when, then| {
            when.method(httpmock::Method::GET)
                .path("/api/test")
                .header("X-Auth-Token", "shared-token");
            then.status(StatusCode::OK).body("{\"result\": \"ok\"}");
        });

        let client = create_test_client(&server, "shared-token", 0, None);

        let handles: Vec<_> = (0..4)
            .map(|i| {
                let client = client.clone();
                let base_url = server.base_url();
                std::thread::spawn(move || {
                    let request = http::Request::builder()
                        .method(http::Method::GET)
                        .uri(format!("{}/api/test?i={}", base_url, i));
                    client.rest(request, Vec::new())
                })
            })
            .collect();

        for h in handles {
            let result = h.join().unwrap();
            assert!(result.is_ok(), "expected Ok, got {:?}", result);
            assert_eq!(result.unwrap().status(), StatusCode::OK);
        }

        mock.assert_calls(4);
    }

    #[test]
    fn test_get_service_endpoint_identity() {
        let server = MockServer::start();
        let _client = create_test_client(&server, "test-token", 0, None);
        let base_url = server.base_url();

        let client = create_test_client(&server, "test-token", 0, None);
        let ep = client
            .get_service_endpoint(&ServiceType::Identity, None)
            .unwrap();

        assert!(ep.url_str().starts_with(&base_url));
    }

    #[test]
    fn test_get_service_endpoint_not_found() {
        let server = MockServer::start();
        let client = create_test_client(&server, "test-token", 0, None);

        let result = client.get_service_endpoint(&ServiceType::Compute, None);
        assert!(result.is_err());
    }

    #[test]
    fn test_get_service_endpoint_with_version() {
        let server = MockServer::start();
        let client = create_test_client(&server, "test-token", 0, None);

        let v3 = ApiVersion::new(3, 0);
        let ep = client
            .get_service_endpoint(&ServiceType::Identity, Some(&v3))
            .unwrap();

        assert!(ep.url_str().starts_with(server.base_url().as_str()));
    }

    #[test]
    fn test_get_current_project() {
        let server = MockServer::start();
        let client = create_test_client(&server, "test-token", 0, None);

        let project = client.get_current_project();
        assert!(project.is_some());
        let project = project.unwrap();
        assert_eq!(project.id.as_deref(), Some("test-project"));
        assert_eq!(project.name.as_deref(), Some("TestProject"));
    }

    #[test]
    fn test_get_auth_token() {
        let server = MockServer::start();
        let client = create_test_client(&server, "my-secret-token", 0, None);

        let token = client.get_auth_token();
        assert!(token.is_some());
        assert_eq!(token.unwrap().expose_secret(), "my-secret-token");
    }

    #[test]
    fn test_get_auth_state() {
        let server = MockServer::start();
        let client = create_test_client(&server, "test-token", 0, None);

        let auth_state = client.get_auth_state(None);
        assert!(matches!(auth_state.unwrap(), AuthState::Valid));
    }

    #[test]
    fn test_get_auth_state_with_offset() {
        let server = MockServer::start();
        let client = create_test_client(&server, "test-token", 0, None);

        // Token expires in 1 hour, offset 2 hours → not valid
        let auth_state = client.get_auth_state(Some(TimeDelta::hours(2)));
        assert!(!matches!(auth_state.unwrap(), AuthState::Valid));

        // Token expires in 1 hour, offset 30 minutes → valid
        let auth_state = client.get_auth_state(Some(TimeDelta::minutes(30)));
        assert!(matches!(auth_state.unwrap(), AuthState::Valid));
    }

    #[test]
    fn test_401_retry_exhaustion() {
        let server = MockServer::start();

        // Always 401 for old token
        let mock_401_old = server.mock(|when, then| {
            when.method(httpmock::Method::GET)
                .path("/api/test")
                .header("X-Auth-Token", "old-token");
            then.status(StatusCode::UNAUTHORIZED);
        });

        // Re-auth also returns 401-like (reauth returns new token, but it's still 401)
        let expires = (Utc::now() + chrono::TimeDelta::hours(1)).to_rfc3339();
        let _mock_reauth = server.mock(|when, then| {
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
        });

        // New token also gets 401
        let mock_401_new = server.mock(|when, then| {
            when.method(httpmock::Method::GET)
                .path("/api/test")
                .header("X-Auth-Token", "new-token");
            then.status(StatusCode::UNAUTHORIZED);
        });

        let client = create_test_client(&server, "old-token", 1, None);

        let request = http::Request::builder()
            .method(http::Method::GET)
            .uri(format!("{}/api/test", server.base_url()));

        // Should return 401 after exhausting retries
        let result = client.rest(request, Vec::new());
        assert!(result.is_ok());
        assert_eq!(result.unwrap().status(), StatusCode::UNAUTHORIZED);

        // Two 401s: one for old token, one after retry with new token
        mock_401_old.assert();
        mock_401_new.assert();
    }

    #[test]
    fn test_mixed_read_write_concurrency() {
        let server = MockServer::start();

        let mock = server.mock(|when, then| {
            when.method(httpmock::Method::GET)
                .path("/api/test")
                .header("X-Auth-Token", "shared-token");
            then.status(StatusCode::OK).body("{\"result\": \"ok\"}");
        });

        let client = create_test_client(&server, "shared-token", 0, None);

        let rest_handles: Vec<_> = (0..4)
            .map(|i| {
                let client = client.clone();
                let base_url = server.base_url();
                std::thread::spawn(move || {
                    let request = http::Request::builder()
                        .method(http::Method::GET)
                        .uri(format!("{}/api/test?i={}", base_url, i));
                    client.rest(request, Vec::new())
                })
            })
            .collect();

        let auth_handles: Vec<_> = (0..4)
            .map(|_| {
                let client = client.clone();
                std::thread::spawn(move || {
                    let _token = client.get_auth_token();
                    Some(client.get_auth_token())
                })
            })
            .collect();

        for h in rest_handles {
            let result = h.join().unwrap();
            assert!(result.is_ok(), "expected Ok, got {:?}", result);
            assert_eq!(result.unwrap().status(), StatusCode::OK);
        }

        for h in auth_handles {
            let result = h.join().unwrap();
            assert!(result.is_some());
        }

        mock.assert_calls(4);
    }
}
