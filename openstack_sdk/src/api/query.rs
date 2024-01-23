use http::Uri;
use tracing::{span, trace, Level};
use url::Url;

use async_trait::async_trait;
use bytes::Bytes;
//use futures::io::AsyncRead;
use tokio::io::AsyncRead;

use crate::api::{ApiError, AsyncClient, Client, RestClient};
use crate::types::BoxedAsyncRead;
use http::{HeaderMap, Response};
use serde::de::DeserializeOwned;
use serde_json::Value;

pub fn url_to_http_uri(url: Url) -> Uri {
    url.as_str()
        .parse::<Uri>()
        .expect("failed to parse a url::Url as an http::Uri")
}

/// A trait which represents a query which may be made to a OpenStack
/// service API client trat returns deserializable data. It does know
/// nothing about required authorization, which is handled by the client.
pub trait Query<T, C>
where
    C: Client,
{
    /// Perform the query against the client.
    fn query(&self, client: &C) -> Result<T, ApiError<C::Error>>;
}

/// A trait which represents an asynchronous query which may be made to a
/// OpenStack service API client that returns deserializable data. It does know
/// nothing about required authorization, which is handled by the client.
#[async_trait]
pub trait QueryAsync<T, C>
where
    C: AsyncClient,
{
    /// Perform the query asynchronously against the client.
    async fn query_async(&self, client: &C) -> Result<T, ApiError<C::Error>>;
}

/// A trait which represents a synchronous query which may be made to a
/// OpenStack service API client and return http response. It does know
/// nothing about required authorization, which is handled by the client. It
/// can be used for special cases where headers must be captured, response
/// is not json, etc.
pub trait RawQuery<C>
where
    C: Client,
{
    /// Perform the query against the client.
    fn raw_query(&self, client: &C) -> Result<Response<Bytes>, ApiError<C::Error>>;
}

/// A trait which represents an asynchronous query which may be made to a
/// OpenStack service API client and return http response. It does know
/// nothing about required authorization, which is handled by the client.
#[async_trait]
pub trait RawQueryAsync<C>
where
    C: AsyncClient,
{
    /// Perform the query asynchronously against the client.
    async fn raw_query_async(&self, client: &C) -> Result<Response<Bytes>, ApiError<C::Error>>;

    /// Perform the low level query asynchronously against
    /// the client.
    async fn raw_query_async_ll(
        &self,
        client: &C,
        inspect_error: Option<bool>,
    ) -> Result<Response<Bytes>, ApiError<C::Error>>;

    /// Perform the query asynchronously against the client.
    async fn raw_query_read_body_async(
        &self,
        client: &C,
        data: BoxedAsyncRead,
    ) -> Result<Response<Bytes>, ApiError<C::Error>>;

    /// Perform async call and return response headers
    /// with AsyncRead of the body
    async fn download_async(
        &self,
        client: &C,
    ) -> Result<(HeaderMap, BoxedAsyncRead), ApiError<C::Error>>;
}
