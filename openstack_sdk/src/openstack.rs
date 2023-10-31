use std::any;
use std::borrow::Cow;
use std::collections::HashMap;
use std::convert::TryInto;
use std::fmt::{self, Debug};
use std::pin::Pin;
use std::time::SystemTime;

use tracing::{debug, error, info, span, trace, Level};

use async_trait::async_trait;
use bytes::Bytes;
use futures::io::{AsyncRead, Error as IoError, ErrorKind as IoErrorKind};
use futures::stream::TryStreamExt;
use http::{request::Builder, HeaderMap, Method, Response as HttpResponse, StatusCode, Uri};

//use http::{self, request::Builder, Method, Request, Response};
use itertools::Itertools;
use tokio_util::codec;
use tokio_util::compat::FuturesAsyncReadCompatExt;

use reqwest::{blocking::Client, Body, Client as AsyncClient, Request, Response};
use serde::{de::DeserializeOwned, Deserialize};
use thiserror::Error;
use url::Url;

use crate::config::CloudConfig;

use crate::api;
use crate::api::query;
use crate::api::query::{Query, QueryAsync, RawQuery, RawQueryAsync};
use crate::api::rest_endpoint_prelude::RestEndpoint;
use crate::auth::{Auth, AuthError};
use crate::config::ConfigError;
use crate::types::identity::v3::{AuthResponse, AuthToken, Project, ServiceEndpoints};
use crate::types::{BoxedAsyncRead, ServiceType, SupportedServiceTypes};

use crate::api::identity::v3::auth_tokens::create as token_v3;

use crate::catalog::{Catalog, CatalogError, EndpointVersion, EndpointVersions, ServiceEndpoint};

use crate::utils;

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum RestError {
    #[error("error setting auth header: {}", source)]
    AuthError {
        #[from]
        source: AuthError,
    },

    #[error("communication with openstack: {}", source)]
    Communication {
        #[from]
        source: reqwest::Error,
    },

    #[error("`http` error: {}", source)]
    Http {
        #[from]
        source: http::Error,
    },
    #[error("`IO` error: {}", source)]
    IO {
        #[from]
        source: IoError,
    },
    #[error("`Catalog` error: {}", source)]
    Catalog {
        #[from]
        source: CatalogError,
    },
}

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum OpenStackError {
    #[error("failed to parse url: {}", source)]
    UrlParse {
        #[from]
        source: url::ParseError,
    },

    #[error("error setting auth header: {}", source)]
    AuthError {
        #[from]
        source: AuthError,
    },

    #[error("communication with cloud: {}", source)]
    Communication {
        #[from]
        source: reqwest::Error,
    },

    #[error("openstack HTTP error: {}", status)]
    Http { status: reqwest::StatusCode },

    #[error("no response from API")]
    NoResponse {},

    #[error("could not parse {} data from JSON: {}", typename, source)]
    DataType {
        #[source]
        source: serde_json::Error,
        typename: &'static str,
    },

    #[error("api error: {}", source)]
    Api {
        #[from]
        source: api::ApiError<RestError>,
    },
    #[error("config error: {}", msg)]
    Config { msg: String },

    #[error("service_catalog error: {}", source)]
    Catalog {
        #[from]
        source: CatalogError,
    },

    #[error("configuration error: {}", source)]
    ConfigError {
        #[from]
        source: ConfigError,
    },

    #[error("Endpoint version discovery error: {}", msg)]
    Discovery { msg: String },
}

impl OpenStackError {
    pub fn http(status: reqwest::StatusCode) -> Self {
        OpenStackError::Http { status }
    }

    pub fn no_response() -> Self {
        OpenStackError::NoResponse {}
    }

    pub fn data_type<T>(source: serde_json::Error) -> Self {
        OpenStackError::DataType {
            source,
            typename: any::type_name::<T>(),
        }
    }
}

pub type OpenStackResult<T> = Result<T, OpenStackError>;

// Private enum that enables the parsing of the cert bytes to be
// delayed until the client is built rather than when they're passed
// to a builder.
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
    config: CloudConfig,
    // /// The base URL to use for API calls.
    // rest_url: Url,
    /// The authentication information to use when communicating with OpenStack.
    auth: Auth,
    /// Authorization information as received from the server
    token: Option<AuthToken>,

    catalog: Catalog,
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
#[derive(Debug, Clone)]
enum CertPolicy {
    Default,
    Insecure,
}

impl OpenStack {
    /// Create a new OpenStack API session from CloudConfig
    pub fn new(config: &CloudConfig) -> OpenStackResult<Self> {
        trace!("Building new session");
        let span = span!(Level::TRACE, "Session span");
        let _enter = span.enter();
        let client = Client::new();

        let mut session = OpenStack {
            client,
            config: config.clone(),
            auth: Auth::None,
            token: None,
            catalog: Catalog::default(),
        };

        // TODO: no readable error from urlparse
        let identity_service_url = Url::parse(
            session
                .config
                .auth
                .as_ref()
                .expect("Auth is missing")
                .auth_url
                .as_ref()
                .expect("Auth_url is missing")
                .as_str(),
        )?;

        session.catalog.service_endpoints.insert(
            "identity".to_string(),
            ServiceEndpoint {
                url: identity_service_url,
                discovered: false,
                versions: Vec::new(),
                current_version: None,
            },
        );

        let auth_data = token_v3::AuthData::try_from(config)?;
        let scope = token_v3::Scope::try_from(config)?;
        let auth: token_v3::Auth = match auth_data {
            token_v3::AuthData::Password(d) => token_v3::Auth::builder()
                .with_user(d.user)
                .scope(scope)
                .build()
                .unwrap(),
            token_v3::AuthData::Token(d) => token_v3::Auth::builder()
                .with_token(d)
                .scope(scope)
                .build()
                .unwrap(),
            token_v3::AuthData::None => {
                return Err(OpenStackError::Config {
                    msg: "No auth possible".into(),
                });
            }
        };

        debug!("Session: {:?}", session);

        session.authorize(auth)?;

        debug!("Config is {:?}", config);

        for (name, val) in config.options.iter() {
            info!("Option {}={}", name, val);
        }

        Ok(session)
    }

    /// Authorize against the cloud using provided credentials and get the session token
    pub fn authorize(
        &mut self,
        auth: crate::api::identity::Auth<'_>,
    ) -> Result<(), OpenStackError> {
        let auth_endpoint = crate::api::identity::CreateAuthToken::builder()
            .auth(auth)
            .build()
            .unwrap();

        let rsp: HttpResponse<Bytes> = auth_endpoint.raw_query(self).unwrap();
        let data: AuthResponse = serde_json::from_slice(rsp.body()).unwrap();
        self.auth = Auth::Token(
            rsp.headers()
                .get("x-subject-token")
                .unwrap()
                .to_str()
                .unwrap()
                .to_string(),
        );
        if let Some(endpoints) = data.token.catalog {
            self.catalog
                .process_catalog_endpoints(&endpoints, Some("public"))?;
        } else {
            error!("No catalog information");
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
                "Request completed whith status {} in {}ms",
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
        if let Some(project) = self.get_current_project() {
            // We are in the project scope
            if service_url.as_str().contains(&project.id) && endpoint.starts_with(&project.id) {
                // Catalog endpoint contains project_id and suffix contains same project_id -> deduplicate
                trace!(
                    "Preventing double project_id in url for {:?}: {:?}",
                    service_type,
                    endpoint
                );
                return Ok(service_url.join(
                    endpoint
                        .get(project.id.len() + 1..)
                        .expect("Endpoint contains project_id"),
                )?);
            }
        }
        Ok(service_url.join(endpoint)?)
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
        self.token.clone().and_then(|x| x.project)
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

#[derive(Clone)]
pub struct AsyncOpenStack {
    /// The client to use for API calls.
    client: reqwest::Client,
    /// Cloud configuration
    config: CloudConfig,
    /// The authentication information to use when communicating with OpenStack.
    auth: Auth,
    /// Authorization information as received from the server
    token: Option<AuthToken>,
    /// Endpoints catalog
    catalog: Catalog,
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

    /// Construct final URL for the resource given the service type and RestEndpoint
    fn rest_endpoint(
        &self,
        service_type: &ServiceType,
        endpoint: &str,
    ) -> Result<Url, api::ApiError<Self::Error>> {
        let service_url = self.get_service_endpoint(service_type)?.url;
        if let Some(project) = self.get_current_project() {
            // We are in the project scope
            if service_url.as_str().contains(&project.id) && endpoint.starts_with(&project.id) {
                // Catalog endpoint contains project_id and suffix contains same project_id -> deduplicate
                trace!(
                    "Preventing double project_id in url for {:?}: {:?}",
                    service_type,
                    endpoint
                );
                return Ok(service_url.join(
                    endpoint
                        .get(project.id.len() + 1..)
                        .expect("Endpoint contains project_id"),
                )?);
            }
        }
        Ok(service_url.join(endpoint)?)
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
        self.token.clone().and_then(|x| x.project)
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
    /// Create a new OpenStack API session from CloudConfig
    pub async fn new(config: &CloudConfig) -> OpenStackResult<Self> {
        let span = span!(Level::DEBUG, "Session span");
        let _enter = span.enter();
        debug!("Building new session");
        let client = AsyncClient::new();

        let mut session = AsyncOpenStack {
            client,
            config: config.clone(),
            auth: Auth::None,
            token: None,
            catalog: Catalog::default(),
        };

        let auth = session
            .config
            .auth
            .as_ref()
            .ok_or(AuthError::MissingAuthData)?;

        let identity_service_url = auth.auth_url.as_ref().ok_or(AuthError::MissingAuthUrl)?;

        session
            .catalog
            .add_service_endpoint("identity", identity_service_url)?;

        let auth_data = token_v3::AuthData::try_from(config)?;
        let scope = token_v3::Scope::try_from(config)?;
        let auth: token_v3::Auth = match auth_data {
            token_v3::AuthData::Password(d) => token_v3::Auth::builder()
                .with_user(d.user)
                .scope(scope)
                .build()
                .unwrap(),
            token_v3::AuthData::Token(d) => token_v3::Auth::builder()
                .with_token(d)
                .scope(scope)
                .build()
                .unwrap(),
            token_v3::AuthData::None => {
                return Err(OpenStackError::Config {
                    msg: "No auth possible".into(),
                });
            }
        };

        debug!("Session: {:?}", session);

        session.authorize(auth).await?;

        for (name, val) in config.options.iter() {
            // Consume endpoint overrides
            if name.ends_with("_endpoint_override") {
                let len = name.len();
                let srv_type = &name[..(len - 18)];
                session
                    .catalog
                    .add_service_endpoint(&srv_type.replace('_', "-"), &val.to_string())?;
            }
        }

        Ok(session)
    }

    /// Authorize against the cloud using provided credentials and get the session token
    pub async fn authorize(
        &mut self,
        auth: crate::api::identity::Auth<'_>,
    ) -> Result<(), OpenStackError> {
        let auth_endpoint = crate::api::identity::CreateAuthToken::builder()
            .auth(auth)
            .build()
            .unwrap();

        self.discover_service_endpoint(&ServiceType::Identity)
            .await?;

        let rsp: HttpResponse<Bytes> = auth_endpoint.raw_query_async(self).await?;
        debug!("Auth response is {:?}", rsp);
        let data: AuthResponse = serde_json::from_slice(rsp.body()).unwrap();
        info!("Auth token is {:?}", data);
        self.auth = Auth::Token(
            rsp.headers()
                .get("x-subject-token")
                .unwrap()
                .to_str()
                .unwrap()
                .to_string(),
        );
        self.token = Some(data.token.clone());

        if let Some(endpoints) = data.token.catalog {
            self.catalog
                .process_catalog_endpoints(&endpoints, Some("public"))?;
        } else {
            error!("No catalog information");
        }
        Ok(())
    }

    pub async fn discover_service_endpoint(
        &mut self,
        service_type: &ServiceType,
    ) -> Result<(), OpenStackError> {
        for cat_type in service_type.get_supported_catalog_types() {
            if let Some(mut ep) = self.catalog.service_endpoints.get(&cat_type.to_string()) {
                if !ep.discovered {
                    info!("Performing `{}` endpoint version discovery", service_type);

                    let mut try_url = ep.url.clone();
                    let mut max_depth = 10;
                    loop {
                        let req = http::Request::builder()
                            .method(http::Method::GET)
                            .uri(query::url_to_http_uri(try_url.clone()));

                        let rsp = self
                            .rest_with_auth_async(req, Vec::new(), &self.auth)
                            .await?;
                        if rsp.status() != StatusCode::NOT_FOUND {
                            let dt = self
                                .catalog
                                .service_endpoints
                                .get_mut(&cat_type.to_string())
                                .unwrap();

                            if dt.process_discovery(rsp.body()).is_ok() {
                                // We have found a valid version document. Exit.
                                return Ok(());
                            }
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
        }
        Ok(())
    }

    pub async fn find_version_document(
        &self,
        url: Url,
    ) -> Result<HttpResponse<Bytes>, OpenStackError> {
        let mut try_url = url.clone();
        let mut max_depth = 10;
        loop {
            let req = http::Request::builder()
                .method(http::Method::GET)
                .uri(query::url_to_http_uri(try_url.clone()));

            let rsp = self
                .rest_with_auth_async(req, Vec::new(), &self.auth)
                .await?;
            if rsp.status() == StatusCode::NOT_FOUND {
                if try_url.path() != "/" {
                    try_url = try_url.join("../")?;
                } else {
                    break;
                }
            } else {
                return Ok(rsp);
            }
            max_depth -= 1;
            if max_depth == 0 {
                break;
            }
        }
        Err(OpenStackError::Discovery {
            msg: "Unknown".to_string(),
        })
    }

    pub fn get_token_catalog(&self) -> Option<Vec<ServiceEndpoints>> {
        self.catalog.get_token_catalog()
    }

    /// Perform HTTP request with given request and return raw response.
    async fn execute_request(&self, request: Request) -> Result<Response, reqwest::Error> {
        info!("Sending request {:?}", request);
        // Body may contain sensitive info.
        //if let Some(body) = request.body() {
        //  debug!("Body: {:?}", std::str::from_utf8(body.as_bytes().unwrap()));
        //}
        let start = SystemTime::now();
        let rsp = self.client.execute(request).await?;
        let elapsed = SystemTime::now().duration_since(start);
        info!(
            "Request completed whith status {} in {}ms",
            rsp.status(),
            elapsed.unwrap_or_default().as_millis()
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
