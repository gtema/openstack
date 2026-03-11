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

//#![deny(dead_code, unused_imports, unused_mut)]

use std::collections::HashMap;
use std::convert::TryInto;
use std::fmt::{self, Debug};
use std::time::SystemTime;
use std::{fs::File, io::Read};

use bytes::Bytes;
use chrono::TimeDelta;
use http::{Response as HttpResponse, StatusCode};
use reqwest::{
    Certificate, Url,
    blocking::{Client, Request, Response},
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
    self, RestClient,
    query::{self},
};
use openstack_sdk_core::auth::{
    AuthState,
    auth_helper::{AuthHelper, Dialoguer, Noop},
    gather_auth_data,
};
use openstack_sdk_core::catalog::{Catalog, CatalogError, ServiceEndpoint};
use openstack_sdk_core::config::{CloudConfig, ConfigFile, get_config_identity_hash};
use openstack_sdk_core::error::{OpenStackError, OpenStackResult, RestError};
use openstack_sdk_core::state;
use openstack_sdk_core::types::{ApiVersion, ServiceType};
use openstack_sdk_core::utils::expand_tilde;

use crate::auth::authtoken::AuthType;

// Private enum that enables the parsing of the cert bytes to be
// delayed until the client is built rather than when they're passed
// to a builder.
#[allow(dead_code)]
#[derive(Clone)]
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
///     let mut session = OpenStack::new(&profile)?;
///
///     // Invoke service discovery when desired.
///     session.discover_service_endpoint(&ServiceType::Compute)?;
///
///     // Execute the call with pagination limiting maximum amount of entries to 1000
///     let data: Vec<serde_json::Value> = paged(ep, Pagination::Limit(1000))
///         .query(&session)
///         .unwrap();
///
///     println!("Data = {:?}", data);
///     Ok(())
/// }
/// ```
#[derive(Clone)]
pub struct OpenStack {
    /// The client to use for API calls.
    client: Client,
    /// Cloud configuration
    config: CloudConfig,
    /// The authentication information to use when communicating with OpenStack.
    auth: Auth,
    /// Endpoints catalog
    catalog: Catalog,
    /// Session state.
    ///
    /// In order to save authentication roundtrips save/load authentication
    /// information in the file (similar to how other cli tools are doing)
    /// and check auth expiration upon load.
    state: state::State,
}

impl Debug for OpenStack {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("OpenStack")
            .field("service_endpoints", &self.catalog)
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
    /// Basic constructor
    fn new_impl(config: &CloudConfig, auth: Auth) -> OpenStackResult<Self> {
        let mut client_builder = Client::builder();

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

        let mut session = OpenStack {
            client: client_builder.build()?,
            config: config.clone(),
            auth,
            catalog: Catalog::default(),
            state: state::State::new(),
        };

        let auth_data = session
            .config
            .auth
            .as_ref()
            .ok_or(AuthTokenError::MissingAuthData)?;

        let identity_service_url = auth_data
            .auth_url
            .as_ref()
            .ok_or(AuthTokenError::MissingAuthUrl)?;

        session.catalog.register_catalog_endpoint(
            "identity",
            identity_service_url,
            config.region_name.as_ref(),
            Some("public"),
        )?;

        session.catalog.configure(config)?;

        session
            .state
            .set_auth_hash_key(get_config_identity_hash(config))
            .enable_auth_cache(ConfigFile::new()?.is_auth_cache_enabled());

        Ok(session)
    }

    /// Create a new OpenStack API session from CloudConfig
    #[instrument(name = "connect", level = "trace", skip(config))]
    pub fn new(config: &CloudConfig) -> OpenStackResult<Self> {
        let mut session = Self::new_impl(config, Auth::None)?;

        // Ensure we resolve identity endpoint using version discovery
        session.discover_service_endpoint(&ServiceType::Identity)?;

        session.authorize(None, false, false)?;

        Ok(session)
    }

    /// Set the authorization to be used by the client
    fn set_auth(&mut self, auth: Auth, skip_cache_update: bool) -> &mut Self {
        self.auth = auth;
        if !skip_cache_update && let Auth::AuthToken(auth) = &self.auth {
            // For app creds we should save auth as unscoped since:
            // - on request it is disallowed to specify scope
            // - response contain fixed scope
            // With this it is not possible to find auth in the cache if we use the real
            // scope
            let scope = match &auth.auth_info {
                Some(info) => {
                    if info.token.application_credential.is_some() {
                        AuthTokenScope::Unscoped
                    } else {
                        auth.get_scope()
                    }
                }
                _ => auth.get_scope(),
            };
            self.state.set_scope_auth(&scope, auth);
        }
        self
    }

    /// Authorize against the cloud using provided credentials and get the session token
    pub fn authorize(
        &mut self,
        scope: Option<AuthTokenScope>,
        interactive: bool,
        renew_auth: bool,
    ) -> Result<(), OpenStackError> {
        if interactive {
            self.authorize_with_auth_helper(scope, &mut Dialoguer::default(), renew_auth)
        } else {
            self.authorize_with_auth_helper(scope, &mut Noop::default(), renew_auth)
        }
    }

    /// Re-authenticate with the existing auth for the given scope.
    fn reauth(&self, auth: &AuthToken, scope: &AuthTokenScope) -> Result<Auth, OpenStackError> {
        // Create the runtime
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
    /// Authorize against the cloud using provided credentials and get the session token
    pub fn authorize_with_auth_helper<A>(
        &mut self,
        scope: Option<AuthTokenScope>,
        auth_helper: &mut A,
        renew_auth: bool,
    ) -> Result<(), OpenStackError>
    where
        A: AuthHelper + Send,
    {
        // Create the runtime
        let rt = Runtime::new()?;
        let requested_scope = scope.unwrap_or(AuthTokenScope::try_from(&self.config)?);

        if let (Some(auth), false) = (self.state.get_scope_auth(&requested_scope), renew_auth) {
            // Valid authorization is already available and no renewal is required
            trace!("Auth already available");
            self.auth = Auth::AuthToken(Box::new(auth.clone()));
        } else {
            // No valid authorization data is available in the state or
            // renewal is requested
            let auth_type = AuthType::from_cloud_config(&self.config)?;
            let mut force_new_auth = renew_auth;
            if let AuthType::V3ApplicationCredential = auth_type {
                // application_credentials token can not be used to get new token without again
                // supplying application credentials.
                // So for AppCred we just force a brand new auth
                force_new_auth = true;
            }
            if let (Some(available_auth), false) = (self.state.get_any_valid_auth(), force_new_auth)
            {
                // State contain valid authentication for different
                // scope/unscoped. It is possible to request new authz
                // using this other auth
                trace!("Valid Auth is available for reauthz: {:?}", available_auth);
                let token_auth = self.reauth(&available_auth, &requested_scope)?;
                self.set_auth(token_auth.clone(), false);
            } else {
                // No auth/authz information available. Proceed with new auth
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
                            self.set_auth(token_auth.clone(), false);
                        }

                        Err(AuthError::AuthReceipt(receipt)) => {
                            // Auth Receipt is received
                            // Find the receipt auth plugin
                            if let Some(authenticator) = inventory::iter::<AuthPluginRegistration>
                                .into_iter()
                                .find(|x| {
                                    x.method.get_supported_auth_methods().contains(&"receipt")
                                })
                                .map(|x| x.method)
                            {
                                // Convert the receipt into auth hints
                                let auth_hints = serde_json::to_value(&receipt)?;
                                let auth_data = rt.block_on(gather_auth_data(
                                    &token_receipt::PLUGIN.requirements(Some(&auth_hints))?,
                                    &self.config,
                                    auth_helper,
                                ))?;
                                // Authenticate
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
                                self.set_auth(token_auth.clone(), false);
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

        if let Auth::AuthToken(token_data) = &self.auth {
            match &token_data.auth_info {
                Some(auth_data) => {
                    if let Some(project) = &auth_data.token.project {
                        self.catalog.set_project_id(project.id.clone());
                        // Reconfigure catalog since we know now the project_id
                        self.catalog.configure(&self.config)?;
                    }
                    if let Some(endpoints) = &auth_data.token.catalog {
                        self.catalog
                            .process_catalog_endpoints(endpoints, Some("public"))?;
                    } else {
                        error!("No catalog information");
                    }
                }
                _ => return Err(OpenStackError::NoAuth),
            }
        }
        // TODO: without AuthToken authorization we may want to read catalog separately
        Ok(())
    }

    #[instrument(skip(self))]
    pub fn discover_service_endpoint(
        &mut self,
        service_type: &ServiceType,
    ) -> Result<(), OpenStackError> {
        if let Ok(ep) = self.catalog.get_service_endpoint(
            service_type.to_string(),
            None,
            self.config.region_name.as_ref(),
        ) {
            if self.catalog.discovery_allowed(service_type.to_string()) {
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

                    match self.rest_with_auth(req, Vec::new(), &self.auth) {
                        Ok(rsp) => {
                            if rsp.status() != StatusCode::NOT_FOUND
                                && self
                                    .catalog
                                    .process_endpoint_discovery(
                                        service_type,
                                        &try_url,
                                        rsp.body(),
                                        self.config.region_name.as_ref(),
                                    )
                                    .is_ok()
                            {
                                debug!(
                                    "Finished service version discovery at {}",
                                    try_url.as_str()
                                );
                                return Ok(());
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
                return Err(OpenStackError::Discovery {
                    service: service_type.to_string(),
                    url: orig_url.into(),
                    msg: "Unknown".into(),
                });
            }
            return Ok(());
        }
        Ok(())
    }

    /// Return current authentication token
    pub fn get_auth_token(&self) -> Option<SecretString> {
        if let Auth::AuthToken(token) = &self.auth {
            return Some(token.token.clone());
        }
        None
    }

    /// Return current authentication status
    ///
    /// Offset can be used to calculate imminent expiration.
    pub fn get_auth_state(&self, offset: Option<TimeDelta>) -> Option<AuthState> {
        if let Auth::AuthToken(token) = &self.auth {
            return Some(token.get_state(offset));
        }
        None
    }

    /// Perform HTTP request with given request and return raw response.
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

    /// Get service endpoint from the catalog
    fn get_service_endpoint(
        &self,
        service_type: &ServiceType,
        version: Option<&ApiVersion>,
    ) -> Result<&ServiceEndpoint, api::ApiError<Self::Error>> {
        Ok(self
            .catalog
            .get_service_endpoint(service_type.to_string(), version, None::<String>)?)
    }

    fn get_current_project(&self) -> Option<Project> {
        if let Auth::AuthToken(token) = &self.auth {
            return token.auth_info.clone().and_then(|x| x.token.project);
        }
        None
    }
}

impl api::Client for OpenStack {
    /// Perform the query with the client specifics
    fn rest(
        &self,
        request: http::request::Builder,
        body: Vec<u8>,
    ) -> Result<HttpResponse<Bytes>, api::ApiError<Self::Error>> {
        self.rest_with_auth(request, body, &self.auth)
    }
}
