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

use std::borrow::Cow;
use std::cmp;
use std::collections::HashMap;
use std::ops::Range;

use async_trait::async_trait;

use bytes::Bytes;
use derive_builder::Builder;
use http::request::Builder as RequestBuilder;
use http::{header, Method, Response, StatusCode};
use http::{HeaderMap, Response as HttpResponse};
use reqwest::blocking::Client as HttpClient;
use reqwest::Client as AsyncHttpClient;
use serde::ser::Serialize;
use thiserror::Error;
use url::Url;

use serde_json::json;

use crate::api::{ApiError, AsyncClient, Client, RestClient};
use crate::catalog::ServiceEndpoint;
use crate::types::identity::v3::Project;
use crate::types::{BoxedAsyncRead, ServiceType};
use crate::RestError;

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
}

const KEYSET_QUERY_PARAM: &str = "marker";
const DEFAULT_PAGE_SIZE: usize = 20;

impl<T> PagedTestClient<T> {
    pub fn new_raw<I>(expected: ExpectedUrl, data: I) -> Self
    where
        I: IntoIterator<Item = T>,
    {
        Self {
            expected,
            data: data.into_iter().collect(),
        }
    }
}

impl<T> RestClient for PagedTestClient<T> {
    type Error = TestClientError;

    fn rest_endpoint(
        &self,
        service_type: &ServiceType,
        endpoint: &str,
    ) -> Result<Url, ApiError<Self::Error>> {
        Ok(Url::parse(&format!("{}/{}", CLIENT_STUB, endpoint))?)
    }

    fn get_service_endpoint(
        &self,
        service_type: &ServiceType,
    ) -> Result<ServiceEndpoint, ApiError<Self::Error>> {
        Ok(ServiceEndpoint {
            url: Url::parse(CLIENT_STUB)?,
            discovered: true,
            versions: Vec::new(),
            current_version: None,
        })
    }

    fn get_current_project(&self) -> Option<Project> {
        None
    }
}

impl<T> Client for PagedTestClient<T>
where
    T: Serialize,
{
    fn rest(
        &self,
        request: RequestBuilder,
        body: Vec<u8>,
    ) -> Result<Response<Bytes>, ApiError<Self::Error>> {
        let url = Url::parse(&format!("{}", request.uri_ref().unwrap())).unwrap();

        self.expected
            .check(request.method_ref().unwrap().clone(), &url);
        assert_eq!(
            &body,
            &self.expected.body,
            "\nbody is not the same:\nactual  : {}\nexpected: {}\n",
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

        let mut pagination = true;
        let mut keyset: Option<usize> = None;

        let mut page: Option<usize> = None;
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
                number: page.unwrap_or(1),
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
        <Self as Client>::rest(self, request, body)
    }

    async fn rest_read_body_async(
        &self,
        request: RequestBuilder,
        body: BoxedAsyncRead,
    ) -> Result<Response<Bytes>, ApiError<Self::Error>> {
        todo!();
    }

    async fn download_async(
        &self,
        request: RequestBuilder,
        body: Vec<u8>,
    ) -> Result<(HeaderMap, BoxedAsyncRead), ApiError<Self::Error>> {
        todo!();
    }
}

/// Mock Test client
pub struct MockServerClient {
    pub server: MockServer,
    pub client: HttpClient,
}

impl MockServerClient {
    pub fn new() -> Self {
        let server = MockServer::start();
        let client = HttpClient::new();
        Self { server, client }
    }
}

impl RestClient for MockServerClient {
    type Error = RestError;

    fn rest_endpoint(
        &self,
        service_type: &ServiceType,
        endpoint: &str,
    ) -> Result<Url, ApiError<Self::Error>> {
        Ok(Url::parse(&format!(
            "http://127.0.0.1:{}/{}",
            self.server.port(),
            endpoint
        ))?)
    }

    fn get_service_endpoint(
        &self,
        service_type: &ServiceType,
    ) -> Result<ServiceEndpoint, ApiError<Self::Error>> {
        Ok(ServiceEndpoint {
            url: Url::parse(&self.server.base_url().to_string())?,
            discovered: true,
            versions: Vec::new(),
            current_version: None,
        })
    }

    fn get_current_project(&self) -> Option<Project> {
        None
    }
}

impl Client for MockServerClient {
    fn rest(
        &self,
        mut request: RequestBuilder,
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

pub struct MockAsyncServerClient {
    pub server: MockServer,
    pub client: AsyncHttpClient,
}

impl MockAsyncServerClient {
    pub async fn new() -> Self {
        let server = MockServer::start_async().await;
        let client = AsyncHttpClient::new();
        Self { server, client }
    }
}

impl RestClient for MockAsyncServerClient {
    type Error = RestError;

    fn rest_endpoint(
        &self,
        service_type: &ServiceType,
        endpoint: &str,
    ) -> Result<Url, ApiError<Self::Error>> {
        Ok(Url::parse(&format!(
            "{}/{}",
            self.server.base_url(),
            endpoint
        ))?)
    }

    fn get_service_endpoint(
        &self,
        service_type: &ServiceType,
    ) -> Result<ServiceEndpoint, ApiError<Self::Error>> {
        Ok(ServiceEndpoint {
            url: Url::parse(&self.server.base_url().to_string())?,
            discovered: true,
            versions: Vec::new(),
            current_version: None,
        })
    }

    fn get_current_project(&self) -> Option<Project> {
        None
    }
}

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
        request: RequestBuilder,
        body: BoxedAsyncRead,
    ) -> Result<Response<Bytes>, ApiError<Self::Error>> {
        todo!();
    }

    async fn download_async(
        &self,
        request: RequestBuilder,
        body: Vec<u8>,
    ) -> Result<(HeaderMap, BoxedAsyncRead), ApiError<Self::Error>> {
        todo!();
    }
}
