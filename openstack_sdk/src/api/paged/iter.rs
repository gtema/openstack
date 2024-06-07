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
    // whether necessary
    Number(u64),
    Keyset(KeysetPage),
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

    fn next_page(&mut self, next_url: Option<Url>) {
        let next_page = match *self {
            // whether necessary
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
            // whether necessary
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
            Page::Number(1)
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

impl<'a, E> PagedState<'a, E> {
    fn next_page(&self, last_page_size: usize, next_url: Option<Url>) {
        let mut page_state = self.page_state.write().expect("poisoned next_page");
        page_state.total_results += last_page_size;

        if self
            .paged
            .pagination
            .is_last_page(last_page_size, page_state.total_results)
        {
            page_state.next_page = Page::Done;
        } else {
            page_state.next_page.next_page(next_url);
        }
    }
}

impl<'a, E> PagedState<'a, E>
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
            if per_page < usize::MAX {
                let per_page_str = per_page.to_string();

                {
                    let mut pairs = url.query_pairs_mut();
                    pairs.append_pair("limit", &per_page_str);

                    next_page.apply_to(&mut pairs);
                }
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
            None
        };

        if let Some(root_key) = self.paged.endpoint.response_key() {
            v = v[root_key.to_string()].take();
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
        self.next_page(page.len(), next_url);

        Ok(page)
    }
}

#[cfg(feature = "sync")]
impl<'a, E, T, C> Query<Vec<T>, C> for PagedState<'a, E>
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
            // XXX: Return a new kind of PaginationError here?
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
impl<'a, E, T, C> QueryAsync<Vec<T>, C> for PagedState<'a, E>
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
            // XXX: Return a new kind of PaginationError here?
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
impl<'a, E, C, T> Iterator for PagedIter<'a, E, C, T>
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
impl<'a, E, C, T> PagedIter<'a, E, C, T>
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
