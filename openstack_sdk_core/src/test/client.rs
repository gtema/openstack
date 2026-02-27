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
//! Faked OpenStack client
//!
//! FakeOpenStackClient can be used mostly with the same interface as the normal OpenStackClient
//! with the exception that there is no authentication handling, no service catalog processing,
//! etc. It is intended to be used together with some mock server.

#[cfg(feature = "async")]
use async_trait::async_trait;
use bytes::Bytes;
use http::Response;
use http::request::Builder as RequestBuilder;
use http::{HeaderMap, Response as HttpResponse};
#[cfg(feature = "async")]
use reqwest::Client as AsyncHttpClient;
#[cfg(feature = "sync")]
use reqwest::blocking::Client as HttpClient;
use std::collections::HashMap;
use url::Url;

#[cfg(feature = "async")]
use crate::api::AsyncClient;
#[cfg(feature = "sync")]
use crate::api::Client;
use crate::api::{ApiError, RestClient};
use crate::auth::Auth;

use crate::RestError;
use crate::catalog::{CatalogError, ServiceEndpoint};
use crate::types::identity::v3::Project;
use crate::types::{ApiVersion, BoxedAsyncRead, ServiceType};

/// Fake (test) OpenStack client
///
/// A client for a faked OpenStack server. Can be used together with mock servers to verify
/// RestEndpoint instances behavior. It may be also used together with a more sophisticated mock
/// server to simulate OpenStack.
///
/// A mock server implementation itself is explicitly left out to give possibility to use the one
/// fitting to the preciese requirements.
///
/// The interface might slightly change shortly due to experiments with building tests with faked
/// OpenStack using the OpenAPI spec file.
///
/// ```no_run
/// use httpmock::MockServer;
/// use std::borrow::Cow;
/// use openstack_sdk_core::{api::RestEndpoint, types::ServiceType};
///
/// struct Dummy;
///
/// impl RestEndpoint for Dummy {
///     fn method(&self) -> http::Method {
///         http::Method::GET
///     }
///
///     fn endpoint(&self) -> Cow<'static, str> {
///         "dummy".into()
///     }
///
///     fn service_type(&self) -> ServiceType {
///         ServiceType::from("dummy")
///     }
/// }
///
/// #[test]
/// fn test_non_json_response() {
///     let server = MockServer::start();
///     let client = FakeOpenStackClient::new(server.base_url());
///     let mock = server.mock(|when, then| {
///         when.method(httpmock::Method::GET).path("/dummy");
///         then.status(200).body("not json");
///     });
///
///     let res: Result<DummyResult, _> = Dummy.query(&client);
///     let err = res.unwrap_err();
///     if let ApiError::OpenStackService { status, .. } = err {
///         assert_eq!(status, http::StatusCode::OK);
///     } else {
///         panic!("unexpected error: {}", err);
///     }
///     mock.assert();
/// }
/// ```
pub struct FakeOpenStackClient {
    /// Known endpoints used by the client
    endpoints: HashMap<String, ServiceEndpoint>,

    /// Optional auth information to be used
    auth: Option<Auth>,
}

impl FakeOpenStackClient {
    /// Instantiate new Fake OpenStack Client with a url pointing to the base url of a server (i.e.
    /// `http://localhost:1234`).
    #[allow(clippy::expect_used)]
    pub fn new<S: AsRef<str>>(url: S) -> Self {
        let mut slf = Self {
            endpoints: HashMap::new(),
            auth: None,
        };
        slf.add_endpoint(
            "default",
            Url::parse(url.as_ref()).expect("valid test url is used"),
        );
        slf
    }

    /// Register dedicated endpoint for a specific service_type. When no dedicated endpoint is
    /// present a `default` one (used in the client initialization) is used.
    pub fn add_endpoint<S: AsRef<str>>(&mut self, service_type: S, url: Url) -> &mut Self {
        self.endpoints.insert(
            service_type.as_ref().into(),
            ServiceEndpoint::new(url, ApiVersion::new(0, 0)),
        );
        self
    }

    /// Set auth information
    pub fn set_auth(&mut self, auth: Option<Auth>) -> &mut Self {
        self.auth = auth;
        self
    }
}

impl RestClient for FakeOpenStackClient {
    type Error = RestError;

    fn get_service_endpoint(
        &self,
        service_type: &ServiceType,
        _version: Option<&ApiVersion>,
    ) -> Result<&ServiceEndpoint, ApiError<Self::Error>> {
        self.endpoints
            .get(&service_type.to_string())
            .or(self.endpoints.get("default"))
            .ok_or(ApiError::catalog(CatalogError::ServiceNotConfigured(
                service_type.to_string(),
            )))
    }

    fn get_current_project(&self) -> Option<Project> {
        None
    }
}

#[cfg(feature = "sync")]
impl Client for FakeOpenStackClient {
    fn rest(
        &self,
        mut request: RequestBuilder,
        body: Vec<u8>,
    ) -> Result<Response<Bytes>, ApiError<Self::Error>> {
        let call = || -> Result<_, Self::Error> {
            if let Some(auth) = &self.auth {
                if let Some(headers) = request.headers_mut() {
                    auth.set_header(headers)?;
                }
            }
            let http_request = request.body(body.clone())?;
            let request = http_request.try_into()?;

            let client = HttpClient::new();
            let rsp = client.execute(request)?;

            let mut http_rsp = HttpResponse::builder()
                .status(rsp.status())
                .version(rsp.version());

            if let Some(headers) = http_rsp.headers_mut() {
                headers.extend(rsp.headers().clone())
            }

            Ok(http_rsp.body(rsp.bytes()?)?)
        };
        call().map_err(ApiError::client)
    }
}

#[cfg(feature = "async")]
#[async_trait]
impl AsyncClient for FakeOpenStackClient {
    async fn rest_async(
        &self,
        mut request: http::request::Builder,
        body: Vec<u8>,
    ) -> Result<HttpResponse<Bytes>, ApiError<Self::Error>> {
        use futures_util::TryFutureExt;
        let call = || async {
            if let Some(auth) = &self.auth {
                if let Some(headers) = request.headers_mut() {
                    auth.set_header(headers)?;
                }
            }
            let http_request = request.body(body)?;
            let request = http_request.try_into()?;

            let client = AsyncHttpClient::new();
            let rsp = client.execute(request).await?;

            let mut http_rsp = HttpResponse::builder()
                .status(rsp.status())
                .version(rsp.version());

            if let Some(headers) = http_rsp.headers_mut() {
                headers.extend(rsp.headers().clone())
            }

            Ok(http_rsp.body(rsp.bytes().await?)?)
        };
        call().map_err(ApiError::client).await
    }

    async fn rest_read_body_async(
        &self,
        _request: RequestBuilder,
        _body: BoxedAsyncRead,
    ) -> Result<Response<Bytes>, ApiError<Self::Error>> {
        todo!();
    }

    async fn download_async(
        &self,
        _request: RequestBuilder,
        _body: Vec<u8>,
    ) -> Result<(HeaderMap, BoxedAsyncRead), ApiError<Self::Error>> {
        todo!();
    }
}
