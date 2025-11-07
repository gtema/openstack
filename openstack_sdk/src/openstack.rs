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

#![deny(dead_code, unused_imports, unused_mut)]

use bytes::Bytes;
use chrono::TimeDelta;
use http::{Response as HttpResponse, StatusCode};
use reqwest::{
    blocking::{Client, Request, Response},
    Certificate, Url,
};
use std::convert::TryInto;
use std::fmt::{self, Debug};
use std::time::SystemTime;
use std::{fs::File, io::Read};
use tokio::runtime::Runtime;
use tracing::{debug, error, event, info, instrument, trace, warn, Level};

use crate::api;
use crate::api::query;
use crate::api::query::RawQuery;
use crate::auth::{
    self,
    auth_helper::{AuthHelper, Dialoguer, Noop},
    authtoken,
    authtoken::{AuthTokenError, AuthType},
    Auth, AuthError, AuthState,
};
use crate::catalog::{Catalog, CatalogError, ServiceEndpoint};
use crate::config::CloudConfig;
use crate::config::{get_config_identity_hash, ConfigFile};
use crate::error::{OpenStackError, OpenStackResult, RestError};
use crate::state;
use crate::types::identity::v3::{AuthReceiptResponse, AuthResponse, Project};
use crate::types::{ApiVersion, ServiceType};
use crate::utils::expand_tilde;

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
                .map_err(|e| OpenStackError::IO {
                    source: e,
                    path: cacert.into(),
                })?
                .read_to_end(&mut buf)
                .map_err(|e| OpenStackError::IO {
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
    fn set_auth(&mut self, auth: auth::Auth, skip_cache_update: bool) -> &mut Self {
        self.auth = auth;
        if !skip_cache_update {
            if let Auth::AuthToken(auth) = &self.auth {
                // For app creds we should save auth as unscoped since:
                // - on request it is disallowed to specify scope
                // - response contain fixed scope
                // With this it is not possible to find auth in the cache if we use the real
                // scope
                let scope = match &auth.auth_info {
                    Some(info) => {
                        if info.token.application_credential.is_some() {
                            authtoken::AuthTokenScope::Unscoped
                        } else {
                            auth.get_scope()
                        }
                    }
                    _ => auth.get_scope(),
                };
                self.state.set_scope_auth(&scope, auth);
            }
        }
        self
    }

    /// Set TokenAuth as current authorization
    fn set_token_auth(&mut self, token: String, token_info: Option<AuthResponse>) -> &mut Self {
        let token_auth = authtoken::AuthToken {
            token,
            auth_info: token_info,
        };
        self.set_auth(auth::Auth::AuthToken(Box::new(token_auth.clone())), false);
        self
    }

    /// Authorize against the cloud using provided credentials and get the session token
    pub fn authorize(
        &mut self,
        scope: Option<authtoken::AuthTokenScope>,
        interactive: bool,
        renew_auth: bool,
    ) -> Result<(), OpenStackError> {
        if interactive {
            self.authorize_with_auth_helper(scope, &mut Dialoguer::default(), renew_auth)
        } else {
            self.authorize_with_auth_helper(scope, &mut Noop::default(), renew_auth)
        }
    }

    /// Authorize against the cloud using provided credentials and get the session token
    pub fn authorize_with_auth_helper<A>(
        &mut self,
        scope: Option<authtoken::AuthTokenScope>,
        auth_helper: &mut A,
        renew_auth: bool,
    ) -> Result<(), OpenStackError>
    where
        A: AuthHelper,
    {
        // Create the runtime
        let rt = Runtime::new().unwrap();
        let requested_scope = scope.unwrap_or(authtoken::AuthTokenScope::try_from(&self.config)?);

        if let (Some(auth), false) = (self.state.get_scope_auth(&requested_scope), renew_auth) {
            // Valid authorization is already available and no renewal is required
            trace!("Auth already available");
            self.auth = auth::Auth::AuthToken(Box::new(auth.clone()));
        } else {
            // No valid authorization data is available in the state or
            // renewal is requested
            let auth_type = AuthType::from_cloud_config(&self.config)?;
            let mut force_new_auth = renew_auth;
            if let AuthType::V3ApplicationCredential = auth_type {
                // application_credentials token can not be used to get new token without again
                // supplying application credentials (bug in Keystone?)
                // So for AppCred we just force a brand new auth
                force_new_auth = true;
            }
            let mut rsp;
            if let (Some(available_auth), false) = (self.state.get_any_valid_auth(), force_new_auth)
            {
                // State contain valid authentication for different
                // scope/unscoped. It is possible to request new authz
                // using this other auth
                trace!("Valid Auth is available for reauthz: {:?}", available_auth);
                let auth_ep = authtoken::build_reauth_request(&available_auth, &requested_scope)?;
                rsp = auth_ep.raw_query(self)?;
            } else {
                // No auth/authz information available. Proceed with new auth
                trace!("No Auth already available. Proceeding with new login");

                match AuthType::from_cloud_config(&self.config)? {
                    AuthType::V3ApplicationCredential
                    | AuthType::V3Password
                    | AuthType::V3Token
                    | AuthType::V3Totp
                    | AuthType::V3Multifactor => {
                        let identity = rt.block_on(authtoken::build_identity_data_from_config(
                            &self.config,
                            auth_helper,
                        ))?;
                        let auth_ep = authtoken::build_auth_request_with_identity_and_scope(
                            &identity,
                            &requested_scope,
                        )?;
                        rsp = auth_ep.raw_query(self)?;

                        // Handle the MFA
                        if let StatusCode::UNAUTHORIZED = rsp.status() {
                            if let Some(receipt) = rsp.headers().get("openstack-auth-receipt") {
                                let receipt_data: AuthReceiptResponse =
                                    serde_json::from_slice(rsp.body())
                                        .expect("A valid OpenStack Auth receipt body");
                                let auth_endpoint =
                                    rt.block_on(authtoken::build_auth_request_from_receipt(
                                        &self.config,
                                        receipt.clone(),
                                        &receipt_data,
                                        &requested_scope,
                                        auth_helper,
                                    ))?;
                                rsp = auth_endpoint.raw_query(self)?;
                            }
                        }
                        api::check_response_error::<Self>(&rsp, None)?;
                    }
                    AuthType::V3OidcAccessToken => {
                        let auth_ep = rt.block_on(auth::v3oidcaccesstoken::get_auth_ep(
                            &self.config,
                            auth_helper,
                        ))?;
                        rsp = auth_ep.raw_query(self)?;

                        let token = rsp
                            .headers()
                            .get("x-subject-token")
                            .ok_or(AuthError::AuthTokenNotInResponse)?
                            .to_str()
                            .expect("x-subject-token is a string");

                        // Set retrieved token as current auth
                        let token_info: AuthResponse = serde_json::from_slice(rsp.body())?;
                        let token_auth = authtoken::AuthToken {
                            token: token.to_string(),
                            auth_info: Some(token_info),
                        };
                        self.set_auth(Auth::AuthToken(Box::new(token_auth.clone())), false);

                        // And now time to rescope the token
                        let auth_ep =
                            authtoken::build_reauth_request(&token_auth, &requested_scope)?;
                        rsp = auth_ep.raw_query(self)?;
                    }
                    other => {
                        return Err(AuthTokenError::IdentityMethodSync {
                            auth_type: other.as_str().into(),
                        })?;
                    }
                }
            };

            let data: AuthResponse = serde_json::from_slice(rsp.body())?;
            debug!("Auth token is {:?}", data);

            let token = rsp
                .headers()
                .get("x-subject-token")
                .ok_or(AuthError::AuthTokenNotInResponse)?
                .to_str()
                .expect("x-subject-token is a string");

            self.set_token_auth(token.into(), Some(data));
        }

        if let auth::Auth::AuthToken(token_data) = &self.auth {
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
                        .uri(query::url_to_http_uri(try_url.clone()));

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
    pub fn get_auth_token(&self) -> Option<String> {
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
            auth.set_header(request.headers_mut().unwrap())?;
            let http_request = request.body(body)?;
            let request = http_request.try_into()?;

            let rsp = self.execute_request(request)?;

            let mut http_rsp = HttpResponse::builder()
                .status(rsp.status())
                .version(rsp.version());
            let headers = http_rsp.headers_mut().unwrap();
            for (key, value) in rsp.headers() {
                headers.insert(key, value.clone());
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
