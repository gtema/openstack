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

#[cfg(feature = "async")]
use async_trait::async_trait;
use bytes::Bytes;
use derive_builder::Builder;
use http::request::Builder as RequestBuilder;
use http::{HeaderMap, Response as HttpResponse};
use http::{Method, Response, StatusCode, header};
#[cfg(feature = "async")]
use reqwest::Client as AsyncHttpClient;
#[cfg(feature = "sync")]
use reqwest::blocking::Client as HttpClient;
use serde::ser::Serialize;
use serde_json::json;
use std::borrow::Cow;
use std::cmp;
use std::collections::HashMap;
use std::ops::Range;
use thiserror::Error;
use url::Url;

#[cfg(feature = "async")]
use crate::api::AsyncClient;
#[cfg(feature = "sync")]
use crate::api::Client;
use crate::api::{ApiError, RestClient};

use crate::RestError;
use crate::catalog::{CatalogError, ServiceEndpoint};
use crate::types::identity::v3::Project;
use crate::types::{ApiVersion, BoxedAsyncRead, ServiceType};

use httpmock::prelude::*;

#[derive(Debug, Builder)]
pub struct ExpectedUrl {
    #[builder(default = "Method::GET")]
    pub method: Method,
    pub endpoint: &'static str,
    #[builder(default)]
    pub query: Vec<(Cow<'static, str>, Cow<'static, str>)>,
    #[builder(setter(strip_option, into), default)]
    pub content_type: Option<String>,
    #[builder(default)]
    pub body: Vec<u8>,
    #[builder(default = "StatusCode::OK")]
    pub status: StatusCode,

    #[builder(default = "false")]
    pub paginated: bool,
}

impl ExpectedUrlBuilder {
    pub fn add_query_params(&mut self, pairs: &[(&'static str, &'static str)]) -> &mut Self {
        self.query
            .get_or_insert_with(Vec::new)
            .extend(pairs.iter().cloned().map(|(k, v)| (k.into(), v.into())));
        self
    }

    pub fn body_str(&mut self, body: &str) -> &mut Self {
        self.body = Some(body.bytes().collect());
        self
    }
}

impl ExpectedUrl {
    pub fn builder() -> ExpectedUrlBuilder {
        ExpectedUrlBuilder::default()
    }

    fn check(&self, method: Method, url: &Url) {
        // Test that the method is as expected.
        assert_eq!(method, self.method);

        // Ensure that the URL was not tampered with in the meantime.
        assert_eq!(url.scheme(), "https");
        assert_eq!(url.username(), "");
        assert_eq!(url.password(), None);
        assert_eq!(url.host_str().unwrap(), "openstack.host.invalid");
        assert_eq!(url.port(), None);
        assert_eq!(url.path(), format!("/{}", self.endpoint));
        let mut count = 0;
        for (ref key, ref value) in url.query_pairs() {
            if self.paginated && Self::is_pagination_key(key) {
                continue;
            }

            let found = self.query.iter().any(|(expected_key, expected_value)| {
                key == expected_key && value == expected_value
            });

            if !found {
                panic!("unexpected query parameter `{}={}`", key, value);
            }
            count += 1;
        }
        assert_eq!(count, self.query.len());
        assert_eq!(url.fragment(), None);
    }

    fn is_pagination_key(key: &str) -> bool {
        key == "pagination" || key == "__test_keyset" || key == "marker" || key == "limit"
    }
}

const CLIENT_STUB: &str = "https://openstack.host.invalid";

#[derive(Debug, Error)]
#[error("test client error")]
pub enum TestClientError {
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
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Page {
    ByNumber { number: usize, size: usize },
    ByKeyset { start: usize, size: usize },
}

impl Page {
    fn range(self) -> Range<usize> {
        match self {
            Page::ByNumber { number, size } => {
                assert_ne!(number, 0);
                let start = size * (number - 1);
                start..start + size
            }
            Page::ByKeyset { start, size } => start..start + size,
        }
    }
}

pub struct PagedTestClient<T> {
    expected: ExpectedUrl,
    data: Vec<T>,
    endpoint: ServiceEndpoint,
}

const KEYSET_QUERY_PARAM: &str = "marker";
const DEFAULT_PAGE_SIZE: usize = 20;

impl<T> PagedTestClient<T>
where
    T: Serialize,
{
    pub fn new_raw<I>(expected: ExpectedUrl, data: I) -> Self
    where
        I: IntoIterator<Item = T>,
    {
        Self {
            expected,
            data: data.into_iter().collect(),
            endpoint: ServiceEndpoint::new(Url::parse(CLIENT_STUB).unwrap(), ApiVersion::new(0, 0)),
        }
    }

    fn _query(
        &self,
        request: RequestBuilder,
        body: Vec<u8>,
    ) -> Result<Response<Bytes>, ApiError<TestClientError>> {
        let url = Url::parse(&format!("{}", request.uri_ref().unwrap())).unwrap();

        self.expected
            .check(request.method_ref().unwrap().clone(), &url);
        assert_eq!(
            &body,
            &self.expected.body,
            "\nbody is not the same:\nactual: {}\nexpected: {}\n",
            String::from_utf8_lossy(&body),
            String::from_utf8_lossy(&self.expected.body),
        );
        let headers = request.headers_ref().unwrap();
        let content_type = headers
            .get_all(header::CONTENT_TYPE)
            .iter()
            .map(|value| value.to_str().unwrap());
        if let Some(expected_content_type) = self.expected.content_type.as_ref() {
            itertools::assert_equal(content_type, [expected_content_type].iter().cloned());
        } else {
            assert_eq!(content_type.count(), 0);
        }

        let pagination = true;
        let mut keyset: Option<usize> = None;

        let mut per_page = DEFAULT_PAGE_SIZE;

        for (ref key, ref value) in url.query_pairs() {
            match key.as_ref() {
                KEYSET_QUERY_PARAM => {
                    keyset = Some(value.parse().unwrap());
                }
                "limit" => {
                    per_page = value.parse().unwrap();
                }
                _ => (),
            }
        }

        let page = if pagination {
            Page::ByKeyset {
                start: keyset.unwrap_or(0),
                size: per_page,
            }
        } else {
            Page::ByNumber {
                number: 1,
                size: per_page,
            }
        };
        let range = {
            // Limit the range to the amount of data actually available.
            let mut range = page.range();
            range.end = cmp::min(range.end, self.data.len());
            range
        };

        let request = request.body(body).unwrap();
        assert_eq!(*request.method(), Method::GET);

        let response = Response::builder().status(self.expected.status);
        let mut next_url = None;
        if pagination && range.end + 1 < self.data.len() {
            // Generate the URL for the next page.
            next_url = {
                let mut next_url = url.clone();
                next_url
                    .query_pairs_mut()
                    .clear()
                    .extend_pairs(
                        url.query_pairs()
                            .filter(|(key, _)| key != KEYSET_QUERY_PARAM),
                    )
                    .append_pair(KEYSET_QUERY_PARAM, &format!("{}", range.end));
                Some(next_url)
            };
        };

        let mut data_page = json!({"resources": &self.data[range]});
        if let Some(next) = next_url {
            data_page["links"] = json!([{"rel": "next", "href": String::from(next)}]);
        }

        Ok(response
            .body(serde_json::to_vec(&data_page).unwrap())
            .unwrap()
            .map(Into::into))
    }
}

impl<T> RestClient for PagedTestClient<T> {
    type Error = TestClientError;

    fn get_service_endpoint(
        &self,
        _service_type: &ServiceType,
        _version: Option<&ApiVersion>,
    ) -> Result<&ServiceEndpoint, ApiError<Self::Error>> {
        Ok(&self.endpoint)
    }

    fn get_current_project(&self) -> Option<Project> {
        None
    }
}

#[cfg(feature = "sync")]
impl<T> Client for PagedTestClient<T>
where
    T: Serialize,
{
    fn rest(
        &self,
        request: RequestBuilder,
        body: Vec<u8>,
    ) -> Result<Response<Bytes>, ApiError<Self::Error>> {
        self._query(request, body)
    }
}

#[cfg(feature = "async")]
#[async_trait]
impl<T> AsyncClient for PagedTestClient<T>
where
    T: Serialize + Send + Sync,
{
    async fn rest_async(
        &self,
        request: RequestBuilder,
        body: Vec<u8>,
    ) -> Result<Response<Bytes>, ApiError<<Self as RestClient>::Error>> {
        Ok(self._query(request, body)?)
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

/// Mock Test client
#[cfg(feature = "sync")]
#[deprecated]
pub struct MockServerClient {
    pub server: MockServer,
    pub client: HttpClient,
    endpoint: ServiceEndpoint,
}

#[cfg(feature = "sync")]
impl MockServerClient {
    pub fn new() -> Self {
        let server = MockServer::start();
        let client = HttpClient::new();
        let base_url = Url::parse(server.base_url().as_ref()).unwrap();
        Self {
            server,
            client,
            endpoint: ServiceEndpoint::new(base_url, ApiVersion::new(0, 0)),
        }
    }
}

#[cfg(feature = "sync")]
impl RestClient for MockServerClient {
    type Error = RestError;

    fn get_service_endpoint(
        &self,
        _service_type: &ServiceType,
        _version: Option<&ApiVersion>,
    ) -> Result<&ServiceEndpoint, ApiError<Self::Error>> {
        Ok(&self.endpoint)
    }

    fn get_current_project(&self) -> Option<Project> {
        None
    }
}

#[cfg(feature = "sync")]
impl Client for MockServerClient {
    fn rest(
        &self,
        request: RequestBuilder,
        body: Vec<u8>,
    ) -> Result<Response<Bytes>, ApiError<Self::Error>> {
        let call = || -> Result<_, Self::Error> {
            let http_request = request.body(body.clone())?;
            let request = http_request.try_into()?;

            let rsp = self.client.execute(request)?;

            let mut http_rsp = HttpResponse::builder()
                .status(rsp.status())
                .version(rsp.version());

            let headers = http_rsp.headers_mut().unwrap();
            for (key, value) in rsp.headers() {
                headers.insert(key, value.clone());
            }

            Ok(http_rsp.body(rsp.bytes()?)?)
        };
        call().map_err(ApiError::client)
    }
}

#[cfg(feature = "async")]
#[deprecated]
pub struct MockAsyncServerClient {
    pub server: MockServer,
    pub client: AsyncHttpClient,
    endpoint: ServiceEndpoint,
}

#[cfg(feature = "async")]
impl MockAsyncServerClient {
    pub async fn new() -> Self {
        let server = MockServer::start_async().await;
        let client = AsyncHttpClient::new();
        let base_url = Url::parse(server.base_url().as_ref()).unwrap();
        Self {
            server,
            client,
            endpoint: ServiceEndpoint::new(base_url, ApiVersion::new(0, 0)),
        }
    }
}

#[cfg(feature = "async")]
impl RestClient for MockAsyncServerClient {
    type Error = RestError;

    fn get_service_endpoint(
        &self,
        _service_type: &ServiceType,
        _version: Option<&ApiVersion>,
    ) -> Result<&ServiceEndpoint, ApiError<Self::Error>> {
        Ok(&self.endpoint)
    }

    fn get_current_project(&self) -> Option<Project> {
        None
    }
}

#[cfg(feature = "async")]
#[async_trait]
impl AsyncClient for MockAsyncServerClient {
    async fn rest_async(
        &self,
        request: http::request::Builder,
        body: Vec<u8>,
    ) -> Result<HttpResponse<Bytes>, ApiError<Self::Error>> {
        use futures_util::TryFutureExt;
        let call = || async {
            let http_request = request.body(body)?;
            let request = http_request.try_into()?;

            let rsp = self.client.execute(request).await?;

            let mut http_rsp = HttpResponse::builder()
                .status(rsp.status())
                .version(rsp.version());
            let headers = http_rsp.headers_mut().unwrap();
            for (key, value) in rsp.headers() {
                headers.insert(key, value.clone());
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

/// Fake (test) OpenStack client
///
/// A client for a faked OpenStack server. Can be used together with mock servers to verify
/// RestEndpoint instances behavior. It may be also used together with a more sophisticated mock
/// server to simulate OpenStack.
///
/// A mock server is explicitly left out to give possibility to use the one fitting to the preciese
/// requirements.
///
/// ```
/// use httpmock::MockServer;
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
///
/// ```
pub struct FakeOpenStackClient {
    /// Known endpoints used by the client
    endpoints: HashMap<String, ServiceEndpoint>,
}

impl FakeOpenStackClient {
    /// Instantiate new Fake OpenStack Client with a url pointing to the base url of a server (i.e.
    /// `http://localhost:1234`)
    pub fn new<S: AsRef<str>>(url: S) -> Self {
        let mut slf = Self {
            endpoints: HashMap::new(),
        };
        slf.add_endpoint("default", Url::parse(url.as_ref()).unwrap());
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
        request: RequestBuilder,
        body: Vec<u8>,
    ) -> Result<Response<Bytes>, ApiError<Self::Error>> {
        let call = || -> Result<_, Self::Error> {
            let http_request = request.body(body.clone())?;
            let request = http_request.try_into()?;

            let client = HttpClient::new();
            let rsp = client.execute(request)?;

            let mut http_rsp = HttpResponse::builder()
                .status(rsp.status())
                .version(rsp.version());

            let headers = http_rsp.headers_mut().unwrap();
            for (key, value) in rsp.headers() {
                headers.insert(key, value.clone());
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
        request: http::request::Builder,
        body: Vec<u8>,
    ) -> Result<HttpResponse<Bytes>, ApiError<Self::Error>> {
        use futures_util::TryFutureExt;
        let call = || async {
            let http_request = request.body(body)?;
            let request = http_request.try_into()?;

            let client = AsyncHttpClient::new();
            let rsp = client.execute(request).await?;

            let mut http_rsp = HttpResponse::builder()
                .status(rsp.status())
                .version(rsp.version());
            let headers = http_rsp.headers_mut().unwrap();
            for (key, value) in rsp.headers() {
                headers.insert(key, value.clone());
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
