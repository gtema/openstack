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

use std::convert::TryInto;
use std::fmt::{self, Debug};
use std::time::SystemTime;
use tracing::{debug, error, info, span, trace, Level};

use anyhow::anyhow;
use bytes::Bytes;
use http::{Response as HttpResponse, StatusCode};

use reqwest::blocking::Client;
use url::Url;

use crate::config::CloudConfig;

use crate::api;
use crate::api::query;
use crate::api::query::RawQuery;
use crate::auth::{self, Auth, AuthError, AuthorizationScope};
use crate::config::{get_config_identity_hash, ConfigFile};
use crate::state;
use crate::types::identity::v3::{AuthReceiptResponse, AuthResponse, Project};
use crate::types::ServiceType;

use crate::api::identity::v3::auth::token::create as token_v3;

use crate::catalog::{Catalog, ServiceEndpoint};

use crate::error::{OpenStackError, OpenStackResult, RestError};

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

/// A representation of the OpenStack API for a single user.
///
/// Separate users should use separate instances of this.
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
        let span = span!(Level::DEBUG, "new_impl");
        let _enter = span.enter();

        let mut session = OpenStack {
            client: Client::new(),
            config: config.clone(),
            auth,
            catalog: Catalog::default(),
            state: state::State::new(),
        };

        let auth_data = session
            .config
            .auth
            .as_ref()
            .ok_or(AuthError::MissingAuthData)?;

        let identity_service_url = auth_data
            .auth_url
            .as_ref()
            .ok_or(AuthError::MissingAuthUrl)?;

        session
            .catalog
            .add_service_endpoint("identity", identity_service_url)?;

        session.catalog.set_endpoint_overrides(config)?;

        session
            .state
            .set_auth_hash_key(get_config_identity_hash(config))
            .enable_auth_cache(ConfigFile::new()?.is_auth_cache_enabled());

        Ok(session)
    }

    /// Create a new OpenStack API session from CloudConfig
    pub fn new(config: &CloudConfig) -> OpenStackResult<Self> {
        trace!("Building new session");
        let span = span!(Level::TRACE, "Session span");
        let _enter = span.enter();

        let mut session = Self::new_impl(config, Auth::None)?;

        // Ensure we resolve identity endpoint using version discovery
        session.discover_service_endpoint(&ServiceType::Identity)?;

        session.authorize(None, None, false, false)?;

        Ok(session)
    }

    /// Authorize against the cloud using provided credentials and get the session token
    pub fn authorize(
        &mut self,
        auth_data: Option<token_v3::Identity<'_>>,
        scope: Option<AuthorizationScope>,
        interactive: bool,
        renew_auth: bool,
    ) -> Result<(), OpenStackError> {
        let requested_scope = scope.unwrap_or(AuthorizationScope::try_from(&self.config)?);

        if let (Some(auth), false) = (self.state.get_scope_auth(&requested_scope), renew_auth) {
            // Valid authorization is already available and no renewal is required
            trace!("Auth already available");
            self.auth = auth::Auth::AuthToken(Box::new(auth.clone()));
        } else {
            // No valid authorization data is available in the state or
            // renewal is requested
            let mut rsp;
            if let (Some(available_auth), false) = (self.state.get_any_valid_auth(), renew_auth) {
                // State contain valid authentication for different
                // scope/unscoped. It is possible to request new authz
                // using this other auth
                trace!("Valid Auth is available for reauthz: {:?}", available_auth);
                let auth_ep = auth::build_reauth_request(&available_auth, &requested_scope)?;
                rsp = auth_ep.raw_query(self)?;
            } else {
                // No auth/authz information available. Proceed with new auth
                trace!("No Auth already available. Proceeding with new login");

                // TODO: How to deal with SSO in sync?
                let auth_data =
                    auth_data.unwrap_or(auth::build_identity_data_from_config(&self.config, true)?);

                let auth_ep =
                    auth::build_auth_request_with_identity_and_scope(&auth_data, &requested_scope)?;
                rsp = auth_ep.raw_query(self)?;
            };

            // Handle the MFA
            if let StatusCode::UNAUTHORIZED = rsp.status() {
                if let Some(receipt) = rsp.headers().get("openstack-auth-receipt") {
                    let receipt_data: AuthReceiptResponse = serde_json::from_slice(rsp.body())
                        .expect("A valid OpenStack Auth receipt body");
                    let auth_endpoint = auth::build_auth_request_from_receipt(
                        &self.config,
                        receipt.clone(),
                        &receipt_data,
                        &requested_scope,
                        interactive,
                    )?;
                    rsp = auth_endpoint.raw_query(self)?;
                }
            }
            api::check_response_error::<Self>(&rsp)?;

            let data: AuthResponse = serde_json::from_slice(rsp.body()).unwrap();
            debug!("Auth token is {:?}", data);

            let token = rsp
                .headers()
                .get("x-subject-token")
                .unwrap()
                .to_str()
                .unwrap();

            let token_auth = auth::AuthToken {
                token: token.to_string().clone(),
                auth_info: Some(data.clone()),
            };
            let full_scope = AuthorizationScope::from(&data);

            self.auth = auth::Auth::AuthToken(Box::new(token_auth.clone()));

            self.state.set_scope_auth(&full_scope, &token_auth);
        }

        if let auth::Auth::AuthToken(token_data) = &self.auth {
            match &token_data.auth_info {
                Some(auth_data) => {
                    if let Some(endpoints) = &auth_data.token.catalog {
                        self.catalog
                            .process_catalog_endpoints(endpoints, Some("public"))?;
                    } else {
                        error!("No catalog information");
                    }
                }
                _ => return Err(anyhow!("No authentication information available").into()),
            }
        }
        // TODO: without AuthToken authorization we may want to read catalog separately
        Ok(())
    }

    pub fn discover_service_endpoint(
        &mut self,
        service_type: &ServiceType,
    ) -> Result<(), OpenStackError> {
        if let Some(ep) = self.catalog.get_service_endpoint(service_type) {
            if !ep.discovered {
                info!("Performing `{}` endpoint version discovery", service_type);

                let mut try_url = ep.url.clone();
                let mut max_depth = 10;
                loop {
                    let req = http::Request::builder()
                        .method(http::Method::GET)
                        .uri(query::url_to_http_uri(try_url.clone()));

                    let rsp = self.rest_with_auth(req, Vec::new(), &self.auth)?;
                    if rsp.status() != StatusCode::NOT_FOUND {
                        return Ok(self
                            .catalog
                            .process_endpoint_discovery(service_type, rsp.body())?);
                    }
                    if try_url.path() != "/" {
                        // We are not at the root yet and have not found a
                        // valid version document so far, try one level up
                        try_url = try_url.join("../")?;
                    } else {
                        return Err(OpenStackError::Discovery {
                            msg: "No Version document discovered".to_string(),
                        });
                    }

                    max_depth -= 1;
                    if max_depth == 0 {
                        break;
                    }
                }
                return Err(OpenStackError::Discovery {
                    msg: "Unknown".to_string(),
                });
            }
            return Ok(());
        }
        Ok(())
    }

    /// Perform a REST query with a given auth.
    pub fn rest_with_auth(
        &self,
        mut request: http::request::Builder,
        body: Vec<u8>,
        auth: &Auth,
    ) -> Result<HttpResponse<Bytes>, api::ApiError<<Self as api::RestClient>::Error>> {
        let call = || -> Result<_, RestError> {
            auth.set_header(request.headers_mut().unwrap())?;
            let http_request = request.body(body)?;
            let request = http_request.try_into()?;

            info!("Sending request {:?}", request);
            let start = SystemTime::now();
            let rsp = self.client.execute(request)?;
            let elapsed = SystemTime::now().duration_since(start);
            info!(
                "Request completed with status {} in {}ms",
                rsp.status(),
                elapsed.unwrap_or_default().as_millis()
            );

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

    /// Construct final URL for the resource given the service type and RestEndpoint
    fn rest_endpoint(
        &self,
        service_type: &ServiceType,
        endpoint: &str,
    ) -> Result<Url, api::ApiError<Self::Error>> {
        let service_url = self.get_service_endpoint(service_type)?.url;
        let mut work_endpoint = endpoint;
        if let Some(segments) = service_url.path_segments() {
            // Service catalog may point to /v2.1/ and target endpoint start
            // with v2.1/servers. The same may happen also for project_id being
            // used in the service catalog while rest endpoint also contain it.
            // In order to construct proper url look in the path elements of
            // the service catalog and for each entry ensure target url does
            // not start with that value.
            for part in segments {
                if !part.is_empty() && work_endpoint.starts_with(part) {
                    work_endpoint = work_endpoint
                        .get(part.len() + 1..)
                        .expect("Cannot remove prefix from url");
                }
            }
        }
        Ok(service_url.join(work_endpoint)?)
    }

    /// Get service endpoint from the catalog
    fn get_service_endpoint(
        &self,
        service_type: &ServiceType,
    ) -> Result<ServiceEndpoint, api::ApiError<Self::Error>> {
        self.catalog
            .get_service_endpoint(service_type)
            .ok_or_else(|| api::ApiError::endpoint(service_type))
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
