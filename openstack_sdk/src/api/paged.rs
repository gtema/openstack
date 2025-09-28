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

mod iter;
mod next_page;
mod pagination;

use http::{header, HeaderValue, Request};
use tracing::{debug, trace};

#[cfg(feature = "async")]
use async_trait::async_trait;
use std::sync::{Arc, Mutex};

use serde::de::DeserializeOwned;

pub use self::pagination::{Pagination, PaginationError};

use crate::api::rest_endpoint::set_latest_microversion;
use crate::api::{query, ApiError, RestEndpoint};

#[cfg(feature = "async")]
use crate::api::{AsyncClient, QueryAsync};
#[cfg(feature = "sync")]
use crate::api::{Client, Query};

/// A trait to indicate that an endpoint is pageable.
pub trait Pageable {
    /// Whether the endpoint uses keyset pagination or not.
    /// Keyset pagination (or advanced keyset) is limit + offset
    fn use_keyset_pagination(&self) -> bool {
        true
    }
}

/// A query modifier that paginates an endpoint.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Paged<E> {
    pub(in crate::api::paged) endpoint: E,
    pub(in crate::api::paged) pagination: Pagination,
}

/// Collect data from a paged endpoint.
// TODO: maybe introduce page_size modifier
pub fn paged<E>(endpoint: E, pagination: Pagination) -> Paged<E> {
    Paged {
        endpoint,
        pagination,
    }
}

#[cfg(feature = "sync")]
impl<E, T, C> Query<Vec<T>, C> for Paged<E>
where
    E: RestEndpoint,
    E: Pageable,
    T: DeserializeOwned,
    C: Client,
{
    fn query(&self, client: &C) -> Result<Vec<T>, ApiError<C::Error>> {
        trace!("Query for paginated resource");
        // Consume iterator and fetch all requested data.
        self.iter(client).collect()
    }
}

#[cfg(feature = "async")]
#[async_trait]
impl<E, T, C> QueryAsync<Vec<T>, C> for Paged<E>
where
    E: RestEndpoint + Sync + Send,
    E: Pageable,
    T: DeserializeOwned + Send + 'static,
    C: AsyncClient + Sync,
{
    async fn query_async(&self, client: &C) -> Result<Vec<T>, ApiError<C::Error>> {
        debug!("Async Query for paginated resource");
        // Consume iterator and fetch all requested data.

        let ep = client.get_service_endpoint(
            &self.endpoint.service_type(),
            self.endpoint.api_version().as_ref(),
        )?;
        let url = {
            let mut url = ep.build_request_url(&self.endpoint.endpoint())?;
            self.endpoint.parameters().add_to_url(&mut url);
            url
        };

        let mut page_num = 1;
        let per_page = self.pagination.page_limit();
        let per_page_str = per_page.to_string();

        let results = Arc::new(Mutex::new(Vec::new()));
        let mut next_url = None;
        let use_keyset_pagination = self.endpoint.use_keyset_pagination();
        let mut marker: Option<String> = None;

        let body = self.endpoint.body()?;

        loop {
            let page_url = if let Some(url) = next_url.take() {
                url
            } else {
                let _page_str = page_num.to_string();
                let mut page_url = url.clone();

                {
                    let mut pairs = page_url.query_pairs_mut();
                    if per_page < usize::MAX {
                        pairs.append_pair("limit", &per_page_str);
                    }
                    if let Some(ref m) = marker {
                        pairs.append_pair("marker", m.as_str());
                    }
                }

                page_url
            };

            let mut req = Request::builder()
                .method(self.endpoint.method())
                .uri(query::url_to_http_uri(page_url.clone()))
                .header(header::ACCEPT, HeaderValue::from_static("application/json"));
            set_latest_microversion(&mut req, ep, &self.endpoint);
            // Set endpoint headers
            if let Some(request_headers) = self.endpoint.request_headers() {
                let headers = req.headers_mut().unwrap();
                for (k, v) in request_headers.iter() {
                    headers.insert(k, v.clone());
                }
            }

            let (req, data) = if let Some((mime, data)) = body.as_ref() {
                let req = req.header(header::CONTENT_TYPE, *mime);
                (req, data.clone())
            } else {
                (req, Vec::new())
            };
            let query_uri = req.uri_ref().cloned();
            let rsp = client.rest_async(req, data).await?;
            let status = rsp.status();

            let mut v = if let Ok(v) = serde_json::from_slice(rsp.body()) {
                v
            } else {
                return Err(ApiError::server_error(query_uri, &rsp, rsp.body()));
            };
            if !status.is_success() {
                return Err(ApiError::from_openstack(query_uri, &rsp, v));
            }

            if use_keyset_pagination {
                next_url = next_page::next_page_from_body(
                    &v,
                    &self.endpoint.response_key(),
                    page_url.clone(),
                )?;
                debug!("data = {:?}", v.clone());
            } else {
                next_url = next_page::next_page_from_headers(rsp.headers())?;
            }

            debug!("raw data = {:?}", v.clone());
            if let Some(root_key) = self.endpoint.response_key() {
                v = v[root_key.as_ref()].take();
            }

            if next_url.is_none() {
                // In swift we do not have next_page coming from anywhere.
                // There is a header with total amount of records, but before
                // even checking that we should also calculate the marker for
                // the next page
                if let Some(data) = v.as_array() {
                    if let Some(last_page_element) = data.last() {
                        if let Some(id) = last_page_element.get("id") {
                            if let Some(val) = id.as_str() {
                                marker = Some(val.into());
                            }
                        } else if let Some(id) = last_page_element.get("name") {
                            if let Some(val) = id.as_str() {
                                marker = Some(val.into());
                            }
                        }
                    }
                }
            }

            if let (Some(item_key), Some(array)) =
                (self.endpoint.response_list_item_key(), v.as_array_mut())
            {
                for elem in array {
                    *elem = elem[item_key.as_ref()].take();
                }
            }
            trace!("items data = {:?}", v.clone());

            let mut page =
                serde_json::from_value::<Vec<T>>(v).map_err(ApiError::data_type::<Vec<T>>)?;
            let page_len = page.len();
            let is_last_page = {
                let mut locked_results = results.lock().expect("poisoned results");
                if let Pagination::Limit(limit) = self.pagination {
                    // with total limit need to check whether the page contains more data then necessary
                    let total_read_till_now = locked_results.len();
                    if total_read_till_now + page.len() > limit {
                        // Discard unnecessary data
                        page.truncate(limit - total_read_till_now);
                    }
                }

                locked_results.extend(page);
                self.pagination.is_last_page(page_len, locked_results.len())
            };
            if is_last_page {
                break;
            }

            if use_keyset_pagination {
                if next_url.is_none() {
                    break;
                }
            } else {
                page_num += 1;
            }
        }

        let mut locked_results = results.lock().expect("poisoned results");
        Ok(std::mem::take(&mut locked_results))
    }
}

#[cfg(test)]
mod tests {
    use http::{HeaderMap, HeaderName, HeaderValue, StatusCode};
    use httpmock::MockServer;
    use serde::{Deserialize, Serialize};
    use serde_json::json;

    use crate::api::rest_endpoint_prelude::*;
    #[cfg(feature = "sync")]
    use crate::api::Query;
    #[cfg(feature = "async")]
    use crate::api::QueryAsync;
    use crate::api::{self, ApiError, Pagination};
    use crate::test::client::FakeOpenStackClient;
    use crate::test::internal::{ExpectedUrl, PagedTestClient};

    struct Dummy {
        with_keyset: bool,

        _headers: Option<HeaderMap>,
    }

    impl RestEndpoint for Dummy {
        fn method(&self) -> http::Method {
            http::Method::GET
        }

        fn endpoint(&self) -> Cow<'static, str> {
            "paged_dummy".into()
        }
        fn service_type(&self) -> ServiceType {
            ServiceType::Compute
        }
        fn response_key(&self) -> Option<Cow<'static, str>> {
            Some("resources".into())
        }
        /// Returns headers to be set into the request
        fn request_headers(&self) -> Option<&HeaderMap> {
            self._headers.as_ref()
        }
    }

    impl Default for Dummy {
        fn default() -> Self {
            Dummy {
                with_keyset: true,
                _headers: Some(HeaderMap::new()),
            }
        }
    }

    impl Pageable for Dummy {}

    #[derive(Debug, Deserialize, Serialize)]
    struct DummyResult {
        value: u8,
    }

    #[cfg(feature = "sync")]
    #[test]
    fn test_non_json_response() {
        let server = MockServer::start();
        let client = FakeOpenStackClient::new(server.base_url());
        let mock = server.mock(|when, then| {
            when.method(httpmock::Method::GET).path("/paged_dummy");
            then.status(StatusCode::OK).body("not json");
        });

        let res: Result<Vec<DummyResult>, _> =
            api::paged(Dummy::default(), Pagination::All).query(&client);
        let err = res.unwrap_err();
        if let ApiError::OpenStackService { status, .. } = err {
            assert_eq!(status, http::StatusCode::OK);
        } else {
            panic!("unexpected error: {err}");
        }
        mock.assert();
    }

    #[cfg(feature = "async")]
    #[tokio::test]
    async fn test_non_json_response_async() {
        let server = MockServer::start_async().await;
        let client = FakeOpenStackClient::new(server.base_url());
        let mock = server.mock(|when, then| {
            when.method(httpmock::Method::GET).path("/paged_dummy");
            then.status(StatusCode::OK).body("not json");
        });

        let res: Result<Vec<DummyResult>, _> = api::paged(Dummy::default(), Pagination::All)
            .query_async(&client)
            .await;
        let err = res.unwrap_err();
        if let ApiError::OpenStackService { status, .. } = err {
            assert_eq!(status, http::StatusCode::OK);
        } else {
            panic!("unexpected error: {err}");
        }
        mock.assert();
    }

    #[cfg(feature = "sync")]
    #[test]
    fn test_error_bad_json() {
        let server = MockServer::start();
        let client = FakeOpenStackClient::new(server.base_url());
        let mock = server.mock(|when, then| {
            when.method(httpmock::Method::GET).path("/paged_dummy");
            then.status(StatusCode::CONFLICT);
        });

        let res: Result<Vec<DummyResult>, _> =
            api::paged(Dummy::default(), Pagination::All).query(&client);
        let err = res.unwrap_err();
        if let ApiError::OpenStackService { status, .. } = err {
            assert_eq!(status, http::StatusCode::CONFLICT);
        } else {
            panic!("unexpected error: {err}");
        }
        mock.assert();
    }

    #[cfg(feature = "async")]
    #[tokio::test]
    async fn test_error_bad_json_async() {
        let server = MockServer::start_async().await;
        let client = FakeOpenStackClient::new(server.base_url());
        let mock = server.mock(|when, then| {
            when.method(httpmock::Method::GET).path("/paged_dummy");
            then.status(StatusCode::CONFLICT);
        });

        let res: Result<Vec<DummyResult>, _> = api::paged(Dummy::default(), Pagination::All)
            .query_async(&client)
            .await;
        let err = res.unwrap_err();
        if let ApiError::OpenStackService { status, .. } = err {
            assert_eq!(status, http::StatusCode::CONFLICT);
        } else {
            panic!("unexpected error: {err}");
        }
        mock.assert();
    }

    #[cfg(feature = "sync")]
    #[test]
    fn test_error_detection() {
        let server = MockServer::start();
        let client = FakeOpenStackClient::new(server.base_url());
        let mock = server.mock(|when, then| {
            when.method(httpmock::Method::GET).path("/paged_dummy");
            then.status(StatusCode::CONFLICT)
                .json_body(json!({"message": "dummy error message"}));
        });
        let endpoint = Dummy::default();

        let res: Result<Vec<DummyResult>, _> = api::paged(endpoint, Pagination::All).query(&client);
        let err = res.unwrap_err();
        if let ApiError::OpenStack { msg, .. } = err {
            assert_eq!(msg, "dummy error message");
        } else {
            panic!("unexpected error: {err}");
        }
        mock.assert();
    }

    #[cfg(feature = "async")]
    #[tokio::test]
    async fn test_error_detection_async() {
        let server = MockServer::start_async().await;
        let client = FakeOpenStackClient::new(server.base_url());
        let mock = server.mock(|when, then| {
            when.method(httpmock::Method::GET).path("/paged_dummy");
            then.status(StatusCode::CONFLICT)
                .json_body(json!({"message": "dummy error message"}));
        });
        let endpoint = Dummy::default();

        let res: Result<Vec<DummyResult>, _> = api::paged(endpoint, Pagination::All)
            .query_async(&client)
            .await;
        let err = res.unwrap_err();
        if let ApiError::OpenStack { msg, .. } = err {
            assert_eq!(msg, "dummy error message");
        } else {
            panic!("unexpected error: {err}");
        }
        mock.assert();
    }

    #[cfg(feature = "sync")]
    #[test]
    fn test_pagination_limit() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("paged_dummy")
            .paginated(true)
            .build()
            .unwrap();
        let client =
            PagedTestClient::new_raw(endpoint, (0..=255).map(|value| DummyResult { value }));
        let query = Dummy {
            with_keyset: false,
            _headers: None,
        };

        let res: Vec<DummyResult> = api::paged(query, Pagination::Limit(25))
            .query(&client)
            .unwrap();
        assert_eq!(res.len(), 25);
        for (i, value) in res.iter().enumerate() {
            assert_eq!(value.value, i as u8);
        }
    }

    #[cfg(feature = "async")]
    #[tokio::test]
    async fn test_pagination_limit_async() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("paged_dummy")
            .paginated(true)
            .build()
            .unwrap();
        let client =
            PagedTestClient::new_raw(endpoint, (0..=255).map(|value| DummyResult { value }));
        let query = Dummy {
            with_keyset: false,
            _headers: None,
        };

        let res: Vec<DummyResult> = api::paged(query, Pagination::Limit(25))
            .query_async(&client)
            .await
            .unwrap();
        assert_eq!(res.len(), 25);
        for (i, value) in res.iter().enumerate() {
            assert_eq!(value.value, i as u8);
        }
    }

    #[cfg(feature = "sync")]
    #[test]
    fn test_pagination_all() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("paged_dummy")
            .paginated(true)
            .build()
            .unwrap();
        let client =
            PagedTestClient::new_raw(endpoint, (0..=255).map(|value| DummyResult { value }));
        let query = Dummy::default();

        let res: Vec<DummyResult> = api::paged(query, Pagination::All).query(&client).unwrap();
        assert_eq!(res.len(), 256);
        for (i, value) in res.iter().enumerate() {
            assert_eq!(value.value, i as u8);
        }
    }

    #[cfg(feature = "async")]
    #[tokio::test]
    async fn test_pagination_all_async() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("paged_dummy")
            .paginated(true)
            .build()
            .unwrap();
        let client =
            PagedTestClient::new_raw(endpoint, (0..=255).map(|value| DummyResult { value }));
        let query = Dummy::default();

        let res: Vec<DummyResult> = api::paged(query, Pagination::All)
            .query_async(&client)
            .await
            .unwrap();
        assert_eq!(res.len(), 256);
        for (i, value) in res.iter().enumerate() {
            assert_eq!(value.value, i as u8);
        }
    }

    #[cfg(feature = "sync")]
    #[test]
    fn test_keyset_pagination_limit() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("paged_dummy")
            .paginated(true)
            .build()
            .unwrap();
        let client =
            PagedTestClient::new_raw(endpoint, (0..=255).map(|value| DummyResult { value }));
        let query = Dummy {
            with_keyset: true,
            _headers: None,
        };

        let res: Vec<DummyResult> = api::paged(query, Pagination::Limit(25))
            .query(&client)
            .unwrap();
        assert_eq!(res.len(), 25);
        for (i, value) in res.iter().enumerate() {
            assert_eq!(value.value, i as u8);
        }
    }

    #[cfg(feature = "async")]
    #[tokio::test]
    async fn test_keyset_pagination_limit_async() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("paged_dummy")
            .paginated(true)
            .build()
            .unwrap();
        let client =
            PagedTestClient::new_raw(endpoint, (0..=255).map(|value| DummyResult { value }));
        let query = Dummy {
            with_keyset: true,
            _headers: None,
        };

        let res: Vec<DummyResult> = api::paged(query, Pagination::Limit(25))
            .query_async(&client)
            .await
            .unwrap();
        assert_eq!(res.len(), 25);
        for (i, value) in res.iter().enumerate() {
            assert_eq!(value.value, i as u8);
        }
    }

    #[cfg(feature = "sync")]
    #[test]
    fn test_keyset_pagination_all() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("paged_dummy")
            .paginated(true)
            .build()
            .unwrap();
        let client =
            PagedTestClient::new_raw(endpoint, (0..=255).map(|value| DummyResult { value }));
        let query = Dummy {
            with_keyset: true,
            _headers: None,
        };

        let res: Vec<DummyResult> = api::paged(query, Pagination::All).query(&client).unwrap();
        assert_eq!(res.len(), 256);
        for (i, value) in res.iter().enumerate() {
            assert_eq!(value.value, i as u8);
        }
    }

    #[cfg(feature = "async")]
    #[tokio::test]
    async fn test_keyset_pagination_all_async() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("paged_dummy")
            .paginated(true)
            .build()
            .unwrap();
        let client =
            PagedTestClient::new_raw(endpoint, (0..=255).map(|value| DummyResult { value }));
        let query = Dummy {
            with_keyset: true,
            _headers: None,
        };

        let res: Vec<DummyResult> = api::paged(query, Pagination::All)
            .query_async(&client)
            .await
            .unwrap();
        assert_eq!(res.len(), 256);
        for (i, value) in res.iter().enumerate() {
            assert_eq!(value.value, i as u8);
        }
    }

    #[cfg(feature = "sync")]
    #[test]
    fn test_pagination_headers() {
        let server = MockServer::start();
        let client = FakeOpenStackClient::new(server.base_url());
        let mock_data: Vec<DummyResult> = (0..=255).map(|value| DummyResult { value }).collect();
        let _mock = server.mock(|when, then| {
            when.method(httpmock::Method::GET)
                .path("/paged_dummy")
                .header("foo", "bar");
            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({"resources": mock_data}));
        });

        let mut query = Dummy::default();
        query._headers.get_or_insert_with(HeaderMap::new).insert(
            HeaderName::from_static("foo"),
            HeaderValue::from_static("bar"),
        );

        let res: Vec<DummyResult> = api::paged(query, Pagination::All).query(&client).unwrap();
        assert_eq!(res.len(), 256);
    }

    #[cfg(feature = "async")]
    #[tokio::test]
    async fn test_pagination_headers_async() {
        let server = MockServer::start_async().await;
        let client = FakeOpenStackClient::new(server.base_url());
        let mock_data: Vec<DummyResult> = (0..=255).map(|value| DummyResult { value }).collect();
        let _mock = server.mock(|when, then| {
            when.method(httpmock::Method::GET)
                .path("/paged_dummy")
                .header("foo", "bar");
            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({"resources": mock_data}));
        });

        let mut query = Dummy::default();
        query._headers.get_or_insert_with(HeaderMap::new).insert(
            HeaderName::from_static("foo"),
            HeaderValue::from_static("bar"),
        );

        let res: Vec<DummyResult> = api::paged(query, Pagination::All)
            .query_async(&client)
            .await
            .unwrap();
        assert_eq!(res.len(), 256);
    }
}
