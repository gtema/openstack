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

use async_trait::async_trait;
use bytes::Bytes;
use chrono::TimeDelta;
use futures::io::{Error as IoError, ErrorKind as IoErrorKind};
use futures::stream::TryStreamExt;
use http::{header, HeaderMap, HeaderValue, Response as HttpResponse, StatusCode};
use reqwest::{Body, Certificate, Client as AsyncClient, Request, Response};
use std::convert::TryInto;
use std::fmt::{self, Debug};
use std::time::SystemTime;
use std::{fs::File, io::Read};
use tokio_util::codec;
use tokio_util::compat::FuturesAsyncReadCompatExt;
use tracing::{debug, enabled, error, event, info, instrument, trace, warn, Level};

use crate::api;
use crate::api::query;
#[cfg(feature = "keystone_ng")]
use crate::api::query::QueryAsync;
use crate::api::query::RawQueryAsync;
use crate::api::RestClient;
use crate::auth::{
    self,
    auth_helper::{AuthHelper, Dialoguer, Noop},
    authtoken::{self, AuthTokenError, AuthType},
    Auth, AuthError, AuthState,
};
use crate::catalog::{Catalog, CatalogError, ServiceEndpoint};
use crate::config::CloudConfig;
use crate::config::{get_config_identity_hash, ConfigFile};
use crate::error::{OpenStackError, OpenStackResult, RestError};
use crate::state;
use crate::types::identity::v3::{AuthReceiptResponse, AuthResponse, Project, ServiceEndpoints};
use crate::types::{ApiVersion, BoxedAsyncRead, ServiceType};
use crate::utils::expand_tilde;

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
#[derive(Clone)]
pub struct AsyncOpenStack {
    /// The client to use for API calls.
    client: reqwest::Client,
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

impl Debug for AsyncOpenStack {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("OpenStack")
            .field("service_endpoints", &self.catalog)
            .finish()
    }
}

#[async_trait]
impl api::RestClient for AsyncOpenStack {
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

    /// Get project id from the current scope
    fn get_current_project(&self) -> Option<Project> {
        if let Auth::AuthToken(token) = &self.auth {
            return token.auth_info.clone().and_then(|x| x.token.project);
        }
        None
    }
}

#[async_trait]
impl api::AsyncClient for AsyncOpenStack {
    // Perform REST request
    async fn rest_async(
        &self,
        request: http::request::Builder,
        body: Vec<u8>,
    ) -> Result<HttpResponse<Bytes>, api::ApiError<<Self as api::RestClient>::Error>> {
        self.rest_with_auth_async(request, body, &self.auth).await
    }

    /// Perform REST request with the body read from AsyncRead
    async fn rest_read_body_async(
        &self,
        request: http::request::Builder,
        body: BoxedAsyncRead,
    ) -> Result<HttpResponse<Bytes>, api::ApiError<<Self as api::RestClient>::Error>> {
        self.rest_with_auth_read_body_async(request, body, &self.auth)
            .await
    }

    /// Download result of HTTP operation.
    async fn download_async(
        &self,
        request: http::request::Builder,
        body: Vec<u8>,
    ) -> Result<(HeaderMap, BoxedAsyncRead), api::ApiError<<Self as api::RestClient>::Error>> {
        self.download_with_auth_async(request, body, &self.auth)
            .await
    }
}

impl AsyncOpenStack {
    /// Basic constructor
    fn new_impl(config: &CloudConfig, auth: Auth) -> OpenStackResult<Self> {
        let mut client_builder = AsyncClient::builder();

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

        let mut session = AsyncOpenStack {
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
    pub async fn new(config: &CloudConfig) -> OpenStackResult<Self> {
        let mut session = Self::new_impl(config, Auth::None)?;

        // Ensure we resolve identity endpoint using version discovery
        session
            .discover_service_endpoint(&ServiceType::Identity)
            .await?;

        session.authorize(None, false, false).await?;

        Ok(session)
    }

    /// Create a new OpenStack API session from CloudConfig
    #[instrument(name = "connect", level = "trace", skip(config, auth_helper))]
    pub async fn new_with_authentication_helper(
        config: &CloudConfig,
        auth_helper: &mut impl AuthHelper,
        renew_auth: bool,
    ) -> OpenStackResult<Self> {
        let mut session = Self::new_impl(config, Auth::None)?;

        // Ensure we resolve identity endpoint using version discovery
        session
            .discover_service_endpoint(&ServiceType::Identity)
            .await?;

        session
            .authorize_with_auth_helper(None, auth_helper, renew_auth)
            .await?;

        Ok(session)
    }

    /// Create a new OpenStack API session from CloudConfig
    #[instrument(name = "connect", level = "trace", skip(config))]
    #[deprecated(
        since = "0.22.0",
        note = "please use `new_with_authentication_helper` instead"
    )]
    pub async fn new_interactive(config: &CloudConfig, renew_auth: bool) -> OpenStackResult<Self> {
        Self::new_with_authentication_helper(config, &mut Dialoguer::default(), renew_auth).await
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
        self.set_auth(Auth::AuthToken(Box::new(token_auth.clone())), false);
        self
    }

    /// Authorize against the cloud using provided credentials and get the session token.
    pub async fn authorize(
        &mut self,
        scope: Option<authtoken::AuthTokenScope>,
        interactive: bool,
        renew_auth: bool,
    ) -> Result<(), OpenStackError> {
        if interactive {
            self.authorize_with_auth_helper(scope, &mut Dialoguer::default(), renew_auth)
                .await
        } else {
            self.authorize_with_auth_helper(scope, &mut Noop::default(), renew_auth)
                .await
        }
    }

    /// Authorize against the cloud using provided credentials and get the session token with the
    /// auth helper that may be invoked to interactively ask for the credentials.
    pub async fn authorize_with_auth_helper<A>(
        &mut self,
        scope: Option<authtoken::AuthTokenScope>,
        auth_helper: &mut A,
        renew_auth: bool,
    ) -> Result<(), OpenStackError>
    where
        A: AuthHelper,
    {
        let requested_scope = scope.map_or_else(
            || authtoken::AuthTokenScope::try_from(&self.config),
            |v| Ok(v.clone()),
        )?;

        if let (Some(auth), false) = (self.state.get_scope_auth(&requested_scope), renew_auth) {
            // Valid authorization is already available and no renewal is required
            trace!("Auth already available");
            self.set_auth(auth::Auth::AuthToken(Box::new(auth.clone())), true);
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
                // State contain valid authentication for different scope/unscoped. It is possible
                // to request new authz using this other auth
                trace!("Valid Auth is available for reauthz: {:?}", available_auth);
                let auth_ep = authtoken::build_reauth_request(&available_auth, &requested_scope)?;
                rsp = auth_ep.raw_query_async_ll(self, Some(false)).await?;
                // Check whether re-auth was successful
                api::check_response_error::<Self>(&rsp, None)?;
            } else {
                // No auth/authz information available. Proceed with new auth
                trace!("No Auth already available. Proceeding with new login");

                match auth_type {
                    AuthType::V3ApplicationCredential
                    | AuthType::V3Password
                    | AuthType::V3Token
                    | AuthType::V3Totp
                    | AuthType::V3Multifactor => {
                        let identity =
                            authtoken::build_identity_data_from_config(&self.config, auth_helper)
                                .await?;
                        let auth_ep = authtoken::build_auth_request_with_identity_and_scope(
                            &identity,
                            &requested_scope,
                        )?;
                        rsp = auth_ep.raw_query_async_ll(self, Some(false)).await?;

                        // Handle the MFA
                        if let StatusCode::UNAUTHORIZED = rsp.status() {
                            if let Some(receipt) = rsp.headers().get("openstack-auth-receipt") {
                                let receipt_data: AuthReceiptResponse =
                                    serde_json::from_slice(rsp.body())
                                        .expect("A valid OpenStack Auth receipt body");
                                let auth_endpoint = authtoken::build_auth_request_from_receipt(
                                    &self.config,
                                    receipt.clone(),
                                    &receipt_data,
                                    &requested_scope,
                                    auth_helper,
                                )
                                .await?;
                                rsp = auth_endpoint.raw_query_async(self).await?;
                            }
                        }
                        api::check_response_error::<Self>(&rsp, None)?;
                    }
                    AuthType::V3OidcAccessToken => {
                        let auth_ep =
                            auth::v3oidcaccesstoken::get_auth_ep(&self.config, auth_helper).await?;
                        rsp = auth_ep.raw_query_async(self).await?;

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
                        rsp = auth_ep.raw_query_async(self).await?;
                    }
                    AuthType::V3WebSso => {
                        let auth_url = auth::v3websso::get_auth_url(&self.config)?;
                        let identity_ep = self.get_service_endpoint(
                            &ServiceType::Identity,
                            Some(&ApiVersion::new(3, 0)),
                        )?;
                        let mut url = identity_ep.build_request_url(&auth_url)?;

                        let mut token_auth = auth::v3websso::get_token_auth(&mut url).await?;

                        // Set retrieved token as current auth
                        self.set_auth(auth::Auth::AuthToken(Box::new(token_auth.clone())), true);

                        // Get the token info (for the expiration)
                        let token_info = self.fetch_token_info(token_auth.token.clone()).await?;
                        token_auth.auth_info = Some(token_info.clone());
                        let scope = authtoken::AuthTokenScope::from(&token_info);

                        // Save unscoped token in the cache
                        self.state.set_scope_auth(&scope, &token_auth);

                        // And now time to rescope the token
                        let auth_ep =
                            authtoken::build_reauth_request(&token_auth, &requested_scope)?;
                        rsp = auth_ep.raw_query_async(self).await?;
                    }

                    #[cfg(feature = "keystone_ng")]
                    AuthType::V3Federation => {
                        // Construct request for initializing authentication (POST call to keystone
                        // `/federation/identity_providers/{idp_id}/auth`) to get the IDP url
                        // client would need to contact.
                        // TODO: If we know the scope we can request it from the very beginning
                        // saving 1 call.
                        let callback_addr = std::net::SocketAddr::from(([127, 0, 0, 1], 8050));
                        let init_auth_ep =
                            auth::v3federation::get_auth_ep(&self.config, callback_addr.port())?;
                        let auth_info: auth::v3federation::FederationAuthRequestResponse =
                            init_auth_ep.query_async(self).await?;

                        // Perform the magic directing user's browser at the IDP url and waiting
                        // for the callback to be invoked with the authorization code
                        let oauth2_code =
                            auth::v3federation::get_auth_code(&auth_info.auth_url, callback_addr)
                                .await?;

                        // Construct the request to Keystone to finish the authorization exchanging
                        // received authorization code for the (unscoped) token
                        let mut oidc_callback_builder =
                            auth::v3federation::OauthCallbackRequestBuilder::default();
                        if let (Some(code), Some(state)) = (oauth2_code.code, oauth2_code.state) {
                            oidc_callback_builder.code(code.clone());
                            oidc_callback_builder.state(state.clone());
                            let oidc_callback_ep = oidc_callback_builder
                                .build()
                                .map_err(auth::v3federation::FederationError::from)?;

                            rsp = oidc_callback_ep.raw_query_async(self).await?;
                        } else {
                            return Err(OpenStackError::NoAuth);
                        }
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

    /// Perform version discovery of a service
    #[instrument(skip(self))]
    pub async fn discover_service_endpoint(
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

                    let rsp = self
                        .rest_with_auth_async(req, Vec::new(), &self.auth)
                        .await?;
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
                        debug!("Finished service version discovery at {}", try_url.as_str());
                        return Ok(());
                    }
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

    // TODO(gtema): rename to `get_catalog`)
    /// Return catalog information given in the token
    pub fn get_token_catalog(&self) -> Option<Vec<ServiceEndpoints>> {
        self.catalog.get_token_catalog()
    }

    /// Return current authentication information
    pub fn get_auth_info(&self) -> Option<AuthResponse> {
        if let Auth::AuthToken(token) = &self.auth {
            return token.auth_info.clone();
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

    /// Return current authentication token
    pub fn get_auth_token(&self) -> Option<String> {
        if let Auth::AuthToken(token) = &self.auth {
            return Some(token.token.clone());
        }
        None
    }

    /// Perform token introspection call
    pub async fn fetch_token_info<S: AsRef<str>>(
        &self,
        token: S,
    ) -> Result<AuthResponse, OpenStackError> {
        let auth_ep = auth::authtoken::build_token_info_endpoint(token)?;
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
            auth.set_header(request.headers_mut().unwrap())?;
            let http_request = request.body(body)?;
            let request = http_request.try_into()?;

            let rsp = self.execute_request(request).await?;

            let mut http_rsp = HttpResponse::builder()
                .status(rsp.status())
                .version(rsp.version());
            let headers = http_rsp.headers_mut().unwrap();
            for (key, value) in rsp.headers() {
                headers.insert(key, value.clone());
            }
            Ok(http_rsp.body(rsp.bytes().await?)?)
        };
        call().map_err(api::ApiError::client).await
    }

    /// Perform a REST query with a given auth.
    async fn rest_with_auth_read_body_async(
        &self,
        mut request: http::request::Builder,
        body_read: BoxedAsyncRead,
        auth: &Auth,
    ) -> Result<HttpResponse<Bytes>, api::ApiError<<Self as api::RestClient>::Error>> {
        use futures_util::TryFutureExt;
        let call = || async {
            auth.set_header(request.headers_mut().unwrap())?;
            let stream = codec::FramedRead::new(body_read.compat(), codec::BytesCodec::new())
                .map_ok(|b| b.freeze());
            let http_request = request.body(Body::wrap_stream(stream))?;
            let request = http_request.try_into()?;

            let rsp = self.execute_request(request).await?;

            let mut http_rsp = HttpResponse::builder()
                .status(rsp.status())
                .version(rsp.version());
            let headers = http_rsp.headers_mut().unwrap();
            for (key, value) in rsp.headers() {
                headers.insert(key, value.clone());
            }
            Ok(http_rsp.body(rsp.bytes().await?)?)
        };
        call().map_err(api::ApiError::client).await
    }

    /// Perform a REST query with a given auth and return AsyncRead of the body.
    async fn download_with_auth_async(
        &self,
        mut request: http::request::Builder,
        body: Vec<u8>,
        auth: &Auth,
    ) -> Result<(HeaderMap, BoxedAsyncRead), api::ApiError<<Self as api::RestClient>::Error>> {
        use futures_util::TryFutureExt;
        let call = || async {
            auth.set_header(request.headers_mut().unwrap())?;
            let http_request = request.body(body)?;
            let request = http_request.try_into()?;
            let rsp = self.execute_request(request).await?;

            let mut headers = HeaderMap::new();
            for (key, value) in rsp.headers() {
                headers.insert(key, value.clone());
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
            Ok((headers, boxed_async_read))
        };
        call().map_err(api::ApiError::client).await
    }
}
