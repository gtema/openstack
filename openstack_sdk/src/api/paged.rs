mod iter;
mod next_page;
mod pagination;

use http::{header, HeaderMap, HeaderName, HeaderValue, Request};
use tracing::{debug, info, trace};
use url::Url;

use async_trait::async_trait;
use std::sync::{Arc, Mutex};

use serde::de::DeserializeOwned;

pub use self::pagination::{Pagination, PaginationError};

use crate::api::{query, ApiError, AsyncClient, Client, Query, QueryAsync, RestEndpoint};

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
        let url = {
            let mut url =
                client.rest_endpoint(&self.endpoint.service_type(), &self.endpoint.endpoint())?;
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
                let page_str = page_num.to_string();
                let mut page_url = url.clone();

                {
                    let mut pairs = page_url.query_pairs_mut();
                    pairs.append_pair("limit", &per_page_str);
                    if let Some(ref m) = marker {
                        pairs.append_pair("marker", m.as_str());
                    }

                    // if use_keyset_pagination {
                    //     pairs.append_pair("pagination", "keyset");
                    // } else {
                    //     pairs.append_pair("page", &page_str);
                    // }
                }

                page_url
            };

            let mut req = Request::builder()
                .method(self.endpoint.method())
                .uri(query::url_to_http_uri(page_url.clone()))
                .header(header::ACCEPT, HeaderValue::from_static("application/json"));
            // Set endpoint headers
            if let Some(request_headers) = self.endpoint.request_headers() {
                let headers = req.headers_mut().unwrap();
                for (k, v) in request_headers.iter() {
                    headers.append(k, v.clone());
                }
            }

            let (req, data) = if let Some((mime, data)) = body.as_ref() {
                let req = req.header(header::CONTENT_TYPE, *mime);
                (req, data.clone())
            } else {
                (req, Vec::new())
            };
            let rsp = client.rest_async(req, data).await?;
            let status = rsp.status();

            let mut v = if let Ok(v) = serde_json::from_slice(rsp.body()) {
                v
            } else {
                return Err(ApiError::server_error(status, rsp.body()));
            };
            if !status.is_success() {
                return Err(ApiError::from_openstack(status, v));
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

            debug!("data = {:?}", v.clone());
            if let Some(root_key) = self.endpoint.response_key() {
                v = v[root_key.to_string()].take();
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
                                marker = Some(String::from(val));
                            }
                        } else if let Some(id) = last_page_element.get("name") {
                            if let Some(val) = id.as_str() {
                                marker = Some(String::from(val));
                            }
                        }
                    }
                }
            }
            debug!("data = {:?}", v.clone());

            let page =
                serde_json::from_value::<Vec<T>>(v).map_err(ApiError::data_type::<Vec<T>>)?;
            let page_len = page.len();
            let is_last_page = {
                let mut locked_results = results.lock().expect("poisoned results");
                locked_results.extend(page);
                self.pagination.is_last_page(page_len, locked_results.len())
            };
            if is_last_page {
                break;
            }

            if use_keyset_pagination {
                if next_url.is_none() && marker.is_none() {
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
    use http::StatusCode;
    use http::{HeaderMap, HeaderName, HeaderValue};
    use serde::{Deserialize, Serialize};
    use serde_json::json;

    use crate::api::rest_endpoint_prelude::*;
    use crate::api::{self, ApiError, Pagination, Query, QueryAsync};
    use crate::test::client::{
        ExpectedUrl, MockAsyncServerClient, MockServerClient, PagedTestClient,
    };

    #[derive(Debug)]
    struct Dummy {
        with_keyset: bool,

        _headers: Option<HeaderMap>,
    }

    impl RestEndpoint for Dummy {
        fn method(&self) -> Method {
            Method::GET
        }

        fn endpoint(&self) -> Cow<'static, str> {
            "paged_dummy".into()
        }
        fn service_type(&self) -> Cow<'static, str> {
            "dummy".into()
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

    #[test]
    fn test_non_json_response() {
        let client = MockServerClient::new();
        let mock = client.server.mock(|when, then| {
            when.method(httpmock::Method::GET)
                .path("/paged_dummy")
                .query_param("limit", "100");
            then.status(200).body("not json");
        });

        let res: Result<Vec<DummyResult>, _> =
            api::paged(Dummy::default(), Pagination::All).query(&client);
        let err = res.unwrap_err();
        if let ApiError::OpenStackService { status, .. } = err {
            assert_eq!(status, http::StatusCode::OK);
        } else {
            panic!("unexpected error: {}", err);
        }
        mock.assert();
    }

    #[test]
    fn test_error_bad_json() {
        let client = MockServerClient::new();
        let mock = client.server.mock(|when, then| {
            when.method(httpmock::Method::GET)
                .path("/paged_dummy")
                .query_param("limit", "100");
            then.status(StatusCode::CONFLICT.into());
        });

        let res: Result<Vec<DummyResult>, _> =
            api::paged(Dummy::default(), Pagination::All).query(&client);
        let err = res.unwrap_err();
        if let ApiError::OpenStackService { status, .. } = err {
            assert_eq!(status, http::StatusCode::CONFLICT);
        } else {
            panic!("unexpected error: {}", err);
        }
        mock.assert();
    }

    #[test]
    fn test_error_detection() {
        let client = MockServerClient::new();
        let mock = client.server.mock(|when, then| {
            when.method(httpmock::Method::GET)
                .path("/paged_dummy")
                .query_param("limit", "100");
            then.status(StatusCode::CONFLICT.into())
                .json_body(json!({"message": "dummy error message"}));
        });
        let endpoint = Dummy::default();

        let res: Result<Vec<DummyResult>, _> = api::paged(endpoint, Pagination::All).query(&client);
        let err = res.unwrap_err();
        if let ApiError::OpenStack { status, msg } = err {
            assert_eq!(msg, "dummy error message");
        } else {
            panic!("unexpected error: {}", err);
        }
        mock.assert();
    }

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

    #[test]
    fn test_pagination_headers() {
        let client = MockServerClient::new();
        let mock_data: Vec<DummyResult> = (0..=255).map(|value| DummyResult { value }).collect();
        let mock = client.server.mock(|when, then| {
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

    #[tokio::test]
    async fn test_pagination_headers_async() {
        let client = MockAsyncServerClient::new().await;
        let mock_data: Vec<DummyResult> = (0..=255).map(|value| DummyResult { value }).collect();
        let mock = client.server.mock(|when, then| {
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
