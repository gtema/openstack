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

//! Raw query wrappers that implement [Query] / [QueryAsync] and return the
//! undecorated HTTP response. Use these instead of the deprecated
//! [RawQuery] / [RawQueryAsync] traits.

use crate::api::rest_endpoint::{check_response_error, prepare_request};
use crate::api::{ApiError, RestEndpoint};
use crate::types::BoxedAsyncRead;

#[cfg(feature = "async")]
use crate::api::{AsyncClient, QueryAsync};
#[cfg(feature = "sync")]
use crate::api::{Client, Query};

use async_trait::async_trait;
use bytes::Bytes;
use http::{HeaderMap, Response};
use std::sync::{Arc, Mutex};
use tracing::instrument;

/// A query modifier that returns the raw [Response] from an endpoint.
///
/// By default the wrapper checks the response for errors. To obtain the raw
/// response regardless of status code, chain `.skip_error_check(true)`.
pub struct Raw<E> {
    endpoint: E,
    /// Stored in Arc<Mutex<Option>> so that `QueryAsync::query_async(&self)`
    /// can consume the body despite taking `&self`.
    body: Option<Arc<Mutex<Option<BoxedAsyncRead>>>>,
    skip_error_check: bool,
}

/// A query modifier that streams the response body as [BoxedAsyncRead].
///
/// Returns a tuple of (response headers, async-readable body stream). Useful
/// for downloading large objects without buffering the entire body into
/// memory.
pub struct Download<E> {
    endpoint: E,
}

/// Create a raw query wrapper for an endpoint.
///
/// This is the replacement for the deprecated [RawQueryAsync::raw_query_async].
///
/// ```
/// use openstack_sdk_core::api::{raw, QueryAsync};
/// // let rsp: Response<Bytes> = raw(ep).query_async(&client).await?;
/// ```
pub fn raw<E>(endpoint: E) -> Raw<E> {
    Raw {
        endpoint,
        body: None,
        skip_error_check: false,
    }
}

/// Create a raw query wrapper that sends a streamed body.
///
/// This is the replacement for the deprecated
/// [RawQueryAsync::raw_query_read_body_async].
///
/// ```
/// use openstack_sdk_core::api::{raw_with_body, QueryAsync};
/// use openstack_sdk_core::types::BoxedAsyncRead;
/// // let body: BoxedAsyncRead = /* ... */;
/// // let rsp = raw_with_body(ep, body).query_async(&client).await?;
/// ```
pub fn raw_with_body<E>(endpoint: E, body: BoxedAsyncRead) -> Raw<E> {
    Raw {
        endpoint,
        body: Some(Arc::new(Mutex::new(Some(body)))),
        skip_error_check: false,
    }
}

/// Create a streaming download wrapper for an endpoint.
///
/// This is the replacement for the deprecated
/// [RawQueryAsync::download_async].
///
/// ```
/// use openstack_sdk_core::api::{download, QueryAsync};
/// // let (headers, stream) = download(ep).query_async(&client).await?;
/// ```
pub fn download<E>(endpoint: E) -> Download<E> {
    Download { endpoint }
}

impl<E> Raw<E> {
    /// Configure whether to check the response for errors.
    ///
    /// When `false` (default), error status codes are returned as [ApiError].
    /// When `true`, the raw response is returned regardless of status code.
    ///
    /// This is the replacement for the deprecated
    /// [RawQueryAsync::raw_query_async_ll].
    ///
    /// ```
    /// use openstack_sdk_core::api::{raw, QueryAsync};
    /// // let rsp: Response<Bytes> = raw(ep)
    /// //     .skip_error_check(true)
    /// //     .query_async(&client)
    /// //     .await?;
    /// ```
    pub fn skip_error_check(mut self, v: bool) -> Self {
        self.skip_error_check = v;
        self
    }
}

#[cfg(feature = "sync")]
impl<E, C> Query<Response<Bytes>, C> for Raw<E>
where
    E: RestEndpoint,
    C: Client,
{
    fn query(&self, client: &C) -> Result<Response<Bytes>, ApiError<C::Error>> {
        let ep = client.get_service_endpoint(
            &self.endpoint.service_type(),
            self.endpoint.api_version().as_ref(),
        )?;
        let (req, data) = prepare_request::<C, E>(
            &ep,
            ep.build_request_url(&self.endpoint.endpoint())?,
            &self.endpoint,
        )?;

        let query_uri = req.uri_ref().cloned();
        let rsp = client.rest(req, data)?;

        if !self.skip_error_check {
            check_response_error::<C>(&rsp, query_uri)?;
        }

        Ok(rsp)
    }
}

#[cfg(feature = "async")]
#[async_trait]
impl<E, C> QueryAsync<Response<Bytes>, C> for Raw<E>
where
    E: RestEndpoint + Sync,
    C: AsyncClient + Sync,
{
    #[instrument(name = "raw_query", level = "debug", skip_all)]
    async fn query_async(&self, client: &C) -> Result<Response<Bytes>, ApiError<C::Error>> {
        let ep = client
            .get_service_endpoint(
                &self.endpoint.service_type(),
                self.endpoint.api_version().as_ref(),
            )
            .await?;
        let (req, data) = prepare_request::<C, E>(
            &ep,
            ep.build_request_url(&self.endpoint.endpoint())?,
            &self.endpoint,
        )?;

        let query_uri = req.uri_ref().cloned();
        let rsp = if let Some(ref body_arc) = self.body {
            let body = body_arc
                .lock()
                .map_err(|_| ApiError::poisoned_lock("locking body"))?
                .take()
                .ok_or_else(|| ApiError::Session {
                    msg: "raw_with_body endpoint reused after body consumed".into(),
                })?;
            client.rest_read_body_async(req, body).await?
        } else {
            client.rest_async(req, data).await?
        };

        if !self.skip_error_check {
            check_response_error::<C>(&rsp, query_uri)?;
        }

        Ok(rsp)
    }
}

#[cfg(feature = "async")]
#[async_trait]
impl<E, C> QueryAsync<(HeaderMap, BoxedAsyncRead), C> for Download<E>
where
    E: RestEndpoint + Sync,
    C: AsyncClient + Sync,
{
    #[instrument(name = "download", level = "debug", skip_all)]
    async fn query_async(
        &self,
        client: &C,
    ) -> Result<(HeaderMap, BoxedAsyncRead), ApiError<C::Error>> {
        let ep = client
            .get_service_endpoint(
                &self.endpoint.service_type(),
                self.endpoint.api_version().as_ref(),
            )
            .await?;
        let (req, data) = prepare_request::<C, E>(
            &ep,
            ep.build_request_url(&self.endpoint.endpoint())?,
            &self.endpoint,
        )?;

        client.download_async(req, data).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::rest_endpoint_prelude::*;
    use crate::test::client::FakeOpenStackClient;
    use crate::types::ServiceType;
    use http::StatusCode;
    use httpmock::MockServer;
    use serde_json::json;

    struct Dummy;

    impl RestEndpoint for Dummy {
        fn method(&self) -> http::Method {
            http::Method::GET
        }

        fn endpoint(&self) -> Cow<'static, str> {
            "dummy".into()
        }

        fn service_type(&self) -> ServiceType {
            ServiceType::from("dummy")
        }
    }

    #[cfg(feature = "async")]
    #[tokio::test]
    async fn test_raw_ok_async() {
        let server = MockServer::start_async().await;
        let client = FakeOpenStackClient::new(server.base_url());
        server.mock(|when, then| {
            when.method(httpmock::Method::GET).path("/dummy");
            then.status(StatusCode::OK).body("hello");
        });

        let rsp: Response<Bytes> = crate::api::raw(Dummy).query_async(&client).await.unwrap();
        assert!(rsp.status().is_success());
        assert_eq!(rsp.body(), &Bytes::from_static(b"hello"));
    }

    #[cfg(feature = "async")]
    #[tokio::test]
    async fn test_raw_error_converted_async() {
        let server = MockServer::start_async().await;
        let client = FakeOpenStackClient::new(server.base_url());
        server.mock(|when, then| {
            when.method(httpmock::Method::GET).path("/dummy");
            then.status(StatusCode::CONFLICT)
                .json_body(json!({"message": "err"}));
        });

        let err = crate::api::raw(Dummy)
            .query_async(&client)
            .await
            .unwrap_err();
        if let ApiError::OpenStack { msg, .. } = err {
            assert_eq!(msg, "err");
        } else {
            panic!("unexpected error: {err}");
        }
    }

    #[cfg(feature = "async")]
    #[tokio::test]
    async fn test_raw_skip_error_check_async() {
        let server = MockServer::start_async().await;
        let client = FakeOpenStackClient::new(server.base_url());
        server.mock(|when, then| {
            when.method(httpmock::Method::GET).path("/dummy");
            then.status(StatusCode::FORBIDDEN)
                .json_body(json!({"message": "denied"}));
        });

        let rsp: Response<Bytes> = crate::api::raw(Dummy)
            .skip_error_check(true)
            .query_async(&client)
            .await
            .unwrap();
        assert_eq!(rsp.status(), StatusCode::FORBIDDEN);
    }

    #[cfg(feature = "async")]
    #[tokio::test]
    async fn test_raw_headers_async() {
        let server = MockServer::start_async().await;
        let client = FakeOpenStackClient::new(server.base_url());
        server.mock(|when, then| {
            when.method(httpmock::Method::GET).path("/dummy");
            then.status(StatusCode::OK)
                .header("x-custom", "value123")
                .body("hello");
        });

        let rsp: Response<Bytes> = crate::api::raw(Dummy).query_async(&client).await.unwrap();
        assert_eq!(
            rsp.headers().get("x-custom").unwrap().to_str().unwrap(),
            "value123"
        );
    }
}
