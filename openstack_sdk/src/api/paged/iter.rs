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

//! Paged endpoint iterator implements an iterator interface to lazily fetch pages when required.
//! This is similar to the python generator.

use std::sync::RwLock;

use async_trait::async_trait;
use bytes::Bytes;
use futures_util::Stream;
use http::request::Builder as RequestBuilder;
use http::{header, HeaderValue, Request, Response};
use serde::de::DeserializeOwned;
use url::Url;

use crate::api::paged::{next_page, Pageable, Paged, Pagination};
use crate::api::rest_endpoint::set_latest_microversion;
use crate::api::{query, ApiError, RestClient, RestEndpoint};
#[cfg(feature = "async")]
use crate::api::{AsyncClient, QueryAsync};
#[cfg(feature = "sync")]
use crate::api::{Client, Query};

#[cfg(feature = "sync")]
impl<E> Paged<E>
where
    E: RestEndpoint,
    E: Pageable,
{
    /// Create an iterator over the results of paginated results for with a client.
    pub fn iter<'a, C, T>(&'a self, client: &'a C) -> PagedIter<'a, E, C, T> {
        PagedIter::new(self, client)
    }
}

#[cfg(feature = "async")]
impl<E> Paged<E>
where
    E: RestEndpoint + Pageable + Sync,
{
    /// Create a stream over the results of paginated results for with a client.
    pub fn iter_async<'a, C, T>(
        &'a self,
        client: &'a C,
    ) -> impl Stream<Item = Result<T, ApiError<C::Error>>> + 'a
    where
        T: DeserializeOwned + 'static,
        C: AsyncClient + Sync,
    {
        let iter = PagedIter::new(self, client);
        futures_util::stream::unfold(iter, |mut iter| async move {
            iter.next_async().await.map(|item| (item, iter))
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum KeysetPage {
    First,
    Next(Url),
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Page {
    /// Pagination using page number.
    Number(u64),
    /// Pagination using KeySet data.
    Keyset(KeysetPage),
    /// Next page by the `marker` pointing to the last `id` of the previous page.
    Marker(Option<String>),
    /// Finished.
    Done,
}

impl Page {
    fn next_url(&self) -> Option<&Url> {
        if let Self::Keyset(KeysetPage::Next(url)) = self {
            Some(url)
        } else {
            None
        }
    }

    fn next_page(&mut self, next_url: Option<Url>, last_marker: Option<String>) {
        let next_page = match *self {
            Self::Marker(_) => Self::Marker(last_marker),
            Self::Number(page) => Self::Number(page + 1),
            Self::Keyset(_) => {
                if let Some(next_url) = next_url {
                    Self::Keyset(KeysetPage::Next(next_url))
                } else {
                    Self::Done
                }
            }
            Self::Done => Self::Done,
        };

        *self = next_page;
    }

    fn apply_to(&self, pairs: &mut url::form_urlencoded::Serializer<url::UrlQuery>) {
        match self {
            Self::Marker(marker) => {
                if let Some(marker) = &marker {
                    pairs.append_pair("marker", marker);
                }
            }
            Self::Number(page) => {
                let page_str = page.to_string();
                pairs.append_pair("page", &page_str);
            }
            Self::Keyset(_) => {}
            Self::Done => {
                unreachable!("The `Done` state should not be applied to any url")
            }
        }
    }
}

struct PageState {
    total_results: usize,
    next_page: Page,
}

struct PagedState<'a, E> {
    paged: &'a Paged<E>,
    page_state: RwLock<PageState>,
}

impl<'a, E> PagedState<'a, E>
where
    E: Pageable,
{
    fn new(paged: &'a Paged<E>) -> Self {
        let next_page = if paged.endpoint.use_keyset_pagination() {
            Page::Keyset(KeysetPage::First)
        } else {
            Page::Marker(None)
        };

        let page_state = PageState {
            total_results: 0,
            next_page,
        };

        Self {
            paged,
            page_state: RwLock::new(page_state),
        }
    }
}

impl<E> PagedState<'_, E> {
    fn next_page(&self, last_page_size: usize, next_url: Option<Url>, marker: Option<String>) {
        let mut page_state = self.page_state.write().expect("poisoned next_page");
        page_state.total_results += last_page_size;

        if self
            .paged
            .pagination
            .is_last_page(last_page_size, page_state.total_results)
        {
            page_state.next_page = Page::Done;
        } else {
            page_state.next_page.next_page(next_url, marker);
        }
    }
}

impl<E> PagedState<'_, E>
where
    E: RestEndpoint,
{
    fn page_url(&self, endpoint_url: Url) -> Option<Url> {
        let page_state = self.page_state.read().expect("poisoned next_page");
        let next_page = &page_state.next_page;

        if *next_page == Page::Done {
            return None;
        }

        let url = if let Some(next_url) = next_page.next_url() {
            next_url.clone()
        } else {
            let mut url = endpoint_url.clone();
            self.paged.endpoint.parameters().add_to_url(&mut url);

            let per_page = self.paged.pagination.page_limit();
            {
                let mut pairs = url.query_pairs_mut();
                if per_page < usize::MAX {
                    pairs.append_pair("limit", &per_page.to_string());
                }
                next_page.apply_to(&mut pairs);
            }

            url
        };

        Some(url)
    }

    fn build_request<C>(&self, url: Url) -> Result<(RequestBuilder, Vec<u8>), ApiError<C::Error>>
    where
        C: RestClient,
    {
        let body = self.paged.endpoint.body()?;

        let mut req = Request::builder()
            .method(self.paged.endpoint.method())
            .uri(query::url_to_http_uri(url))
            .header(header::ACCEPT, HeaderValue::from_static("application/json"));

        // Set endpoint headers
        if let Some(request_headers) = self.paged.endpoint.request_headers() {
            let headers = req.headers_mut().unwrap();
            for (k, v) in request_headers.iter() {
                headers.insert(k, v.clone());
            }
        }

        Ok(if let Some((mime, data)) = body.as_ref() {
            let req = req.header(header::CONTENT_TYPE, *mime);
            (req, data.clone())
        } else {
            (req, Vec::new())
        })
    }

    fn process_response<C, T>(
        &self,
        rsp: Response<Bytes>,
        base: Url,
    ) -> Result<Vec<T>, ApiError<C::Error>>
    where
        E: Pageable,
        T: DeserializeOwned,
        C: RestClient,
    {
        let status = rsp.status();
        let mut v = if let Ok(v) = serde_json::from_slice(rsp.body()) {
            v
        } else {
            return Err(ApiError::server_error(
                Some(query::url_to_http_uri(base)),
                status,
                rsp.body(),
            ));
        };
        if !status.is_success() {
            return Err(ApiError::from_openstack(
                Some(query::url_to_http_uri(base)),
                status,
                v,
            ));
        }

        let next_url = if self.paged.endpoint.use_keyset_pagination() {
            next_page::next_page_from_body(&v, &self.paged.endpoint.response_key(), base)?
        } else {
            next_page::next_page_from_headers(rsp.headers())?
        };

        if let Some(root_key) = self.paged.endpoint.response_key() {
            v = v[root_key.to_string()].take();
        }

        let mut marker: Option<String> = None;

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

        // List of items and every item is in additional container
        if let (Some(item_key), Some(array)) = (
            self.paged.endpoint.response_list_item_key(),
            v.as_array_mut(),
        ) {
            for elem in array {
                *elem = elem[item_key.to_string()].take();
            }
        }

        let mut page =
            serde_json::from_value::<Vec<T>>(v).map_err(ApiError::data_type::<Vec<T>>)?;

        if let Pagination::Limit(limit) = self.paged.pagination {
            // with total limit need to check whether the page contains more data then necessary
            let total_read_till_now = self
                .page_state
                .read()
                .expect("poisoned state")
                .total_results;
            if total_read_till_now + page.len() > limit {
                // Discard unnecessary data
                page.truncate(limit - total_read_till_now);
            }
        }
        self.next_page(page.len(), next_url, marker);

        Ok(page)
    }
}

#[cfg(feature = "sync")]
impl<E, T, C> Query<Vec<T>, C> for PagedState<'_, E>
where
    E: RestEndpoint,
    E: Pageable,
    T: DeserializeOwned,
    T: DeserializeOwned,
    C: Client,
{
    fn query(&self, client: &C) -> Result<Vec<T>, ApiError<C::Error>> {
        let ep = client.get_service_endpoint(
            &self.paged.endpoint.service_type(),
            self.paged.endpoint.api_version().as_ref(),
        )?;
        let url = if let Some(url) =
            self.page_url(ep.build_request_url(&self.paged.endpoint.endpoint())?)
        {
            url
        } else {
            // Just return empty data.
            return Ok(Vec::new());
        };
        let (mut req, data) = self.build_request::<C>(url.clone())?;
        set_latest_microversion(&mut req, ep, &self.paged.endpoint);
        let rsp = client.rest(req, data)?;
        self.process_response::<C, _>(rsp, url.clone())
    }
}

#[cfg(feature = "async")]
#[async_trait]
impl<E, T, C> QueryAsync<Vec<T>, C> for PagedState<'_, E>
where
    E: RestEndpoint + Pageable + Sync,
    T: DeserializeOwned + 'static,
    C: AsyncClient + Sync,
{
    async fn query_async(&self, client: &C) -> Result<Vec<T>, ApiError<C::Error>> {
        let ep = client.get_service_endpoint(
            &self.paged.endpoint.service_type(),
            self.paged.endpoint.api_version().as_ref(),
        )?;
        let url = if let Some(url) =
            self.page_url(ep.build_request_url(&self.paged.endpoint.endpoint())?)
        {
            url
        } else {
            // Just return empty data.
            return Ok(Vec::new());
        };
        let (mut req, data) = self.build_request::<C>(url.clone())?;
        set_latest_microversion(&mut req, ep, &self.paged.endpoint);
        let rsp = client.rest_async(req, data).await?;
        self.process_response::<C, _>(rsp, url.clone())
    }
}

/// An iterator which yields items from a paginated result.
///
/// The pages are fetched lazily, so endpoints not using keyset pagination may observe duplicate or
/// missing items (depending on sorting) if new objects are created or removed while iterating.
pub struct PagedIter<'a, E, C, T> {
    client: &'a C,
    state: PagedState<'a, E>,
    current_page: Vec<T>,
}

impl<'a, E, C, T> PagedIter<'a, E, C, T>
where
    E: RestEndpoint,
    E: Pageable,
{
    fn new(paged: &'a Paged<E>, client: &'a C) -> Self {
        let state = PagedState::new(paged);

        Self {
            client,
            state,
            current_page: Vec::new(),
        }
    }
}

#[cfg(feature = "sync")]
impl<E, C, T> Iterator for PagedIter<'_, E, C, T>
where
    E: RestEndpoint,
    E: Pageable,
    T: DeserializeOwned,
    C: Client,
{
    type Item = Result<T, ApiError<C::Error>>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_page.is_empty() {
            self.current_page = match self.state.query(self.client) {
                Ok(data) => data,
                Err(err) => return Some(Err(err)),
            };

            // Reverse the page order so that `.pop()` works.
            self.current_page.reverse();
        }

        self.current_page.pop().map(Ok)
    }
}

// Instead of implementing Stream directly, we implement this "async" next method and use it with
// `stream::unfold` to return an anonymous Stream impl.
#[cfg(feature = "async")]
impl<E, C, T> PagedIter<'_, E, C, T>
where
    E: RestEndpoint + Pageable + Sync,
    T: DeserializeOwned + 'static,
    C: AsyncClient + Sync,
{
    async fn next_async(&mut self) -> Option<Result<T, ApiError<C::Error>>> {
        if self.current_page.is_empty() {
            self.current_page = match self.state.query_async(self.client).await {
                Ok(data) => data,
                Err(err) => return Some(Err(err)),
            };

            // Reverse the page order so that `.pop()` works.
            self.current_page.reverse();
        }

        self.current_page.pop().map(Ok)
    }
}

#[cfg(test)]
mod tests {

    use futures_util::TryStreamExt;
    use http::StatusCode;
    use serde::{Deserialize, Serialize};
    use serde_json::json;

    use crate::api::rest_endpoint_prelude::*;
    use crate::api::{self, ApiError, Pagination};
    #[cfg(feature = "async")]
    use crate::test::client::MockAsyncServerClient;
    #[cfg(feature = "sync")]
    use crate::test::client::MockServerClient;
    use crate::test::client::{ExpectedUrl, PagedTestClient};

    #[derive(Debug, Default)]
    struct Dummy {
        with_keyset: bool,
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
    }

    impl Pageable for Dummy {}

    #[derive(Debug, Deserialize, Serialize)]
    struct DummyResult {
        value: u8,
    }

    #[cfg(feature = "sync")]
    #[test]
    fn test_non_json_response() {
        let client = MockServerClient::new();
        let mock = client.server.mock(|when, then| {
            when.method(httpmock::Method::GET).path("/paged_dummy");
            then.status(StatusCode::OK.into()).body("not json");
        });

        let res: Result<Vec<DummyResult>, _> = api::paged(Dummy::default(), Pagination::All)
            .iter(&client)
            .collect();
        let err = res.unwrap_err();
        if let ApiError::OpenStackService { status, .. } = err {
            assert_eq!(status, http::StatusCode::OK);
        } else {
            panic!("unexpected error: {}", err);
        }
        mock.assert();
    }

    #[cfg(feature = "async")]
    #[tokio::test]
    async fn test_non_json_response_async() {
        let client = MockAsyncServerClient::new().await;
        let mock = client.server.mock(|when, then| {
            when.method(httpmock::Method::GET).path("/paged_dummy");
            then.status(StatusCode::OK.into()).body("not json");
        });

        let res: Result<Vec<DummyResult>, _> = api::paged(Dummy::default(), Pagination::All)
            .iter_async(&client)
            .try_collect()
            .await;
        let err = res.unwrap_err();
        if let ApiError::OpenStackService { status, .. } = err {
            assert_eq!(status, http::StatusCode::OK);
        } else {
            panic!("unexpected error: {}", err);
        }
        mock.assert();
    }

    #[cfg(feature = "sync")]
    #[test]
    fn test_error_bad_json() {
        let client = MockServerClient::new();
        let mock = client.server.mock(|when, then| {
            when.method(httpmock::Method::GET).path("/paged_dummy");
            then.status(StatusCode::CONFLICT.into());
        });

        let res: Result<Vec<DummyResult>, _> = api::paged(Dummy::default(), Pagination::All)
            .iter(&client)
            .collect();
        let err = res.unwrap_err();
        if let ApiError::OpenStackService { status, .. } = err {
            assert_eq!(status, http::StatusCode::CONFLICT);
        } else {
            panic!("unexpected error: {}", err);
        }
        mock.assert();
    }

    #[cfg(feature = "async")]
    #[tokio::test]
    async fn test_error_bad_json_async() {
        let client = MockAsyncServerClient::new().await;
        let mock = client.server.mock(|when, then| {
            when.method(httpmock::Method::GET).path("/paged_dummy");
            then.status(StatusCode::CONFLICT.into());
        });

        let res: Result<Vec<DummyResult>, _> = api::paged(Dummy::default(), Pagination::All)
            .iter_async(&client)
            .try_collect()
            .await;
        let err = res.unwrap_err();
        if let ApiError::OpenStackService { status, .. } = err {
            assert_eq!(status, http::StatusCode::CONFLICT);
        } else {
            panic!("unexpected error: {}", err);
        }
        mock.assert();
    }

    #[cfg(feature = "sync")]
    #[test]
    fn test_error_detection() {
        let client = MockServerClient::new();
        let mock = client.server.mock(|when, then| {
            when.method(httpmock::Method::GET).path("/paged_dummy");
            then.status(StatusCode::CONFLICT.into())
                .json_body(json!({"message": "dummy error message"}));
        });
        let endpoint = Dummy::default();

        let res: Result<Vec<DummyResult>, _> = api::paged(endpoint, Pagination::All)
            .iter(&client)
            .collect();
        let err = res.unwrap_err();
        if let ApiError::OpenStack { msg, .. } = err {
            assert_eq!(msg, "dummy error message");
        } else {
            panic!("unexpected error: {}", err);
        }
        mock.assert();
    }

    #[cfg(feature = "async")]
    #[tokio::test]
    async fn test_error_detection_async() {
        let client = MockAsyncServerClient::new().await;
        let mock = client.server.mock(|when, then| {
            when.method(httpmock::Method::GET).path("/paged_dummy");
            then.status(StatusCode::CONFLICT.into())
                .json_body(json!({"message": "dummy error message"}));
        });
        let endpoint = Dummy::default();

        let res: Result<Vec<DummyResult>, _> = api::paged(endpoint, Pagination::All)
            .iter_async(&client)
            .try_collect()
            .await;
        let err = res.unwrap_err();
        if let ApiError::OpenStack { msg, .. } = err {
            assert_eq!(msg, "dummy error message");
        } else {
            panic!("unexpected error: {}", err);
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
        let query = Dummy { with_keyset: false };

        let res: Vec<DummyResult> = api::paged(query, Pagination::Limit(25))
            .iter(&client)
            .collect::<Result<Vec<_>, _>>()
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
        let query = Dummy { with_keyset: false };

        let res: Vec<DummyResult> = api::paged(query, Pagination::Limit(25))
            .iter_async(&client)
            .try_collect()
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

        let res: Vec<DummyResult> = api::paged(query, Pagination::All)
            .iter(&client)
            .collect::<Result<Vec<_>, _>>()
            .unwrap();
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
            .iter_async(&client)
            .try_collect()
            .await
            .unwrap();
        assert_eq!(res.len(), 256);
        for (i, value) in res.iter().enumerate() {
            assert_eq!(value.value, i as u8);
        }
    }
}
