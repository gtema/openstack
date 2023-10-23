// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use async_trait::async_trait;
use bytes::Bytes;
//use futures_core::stream::Stream;
//use futures::io::AsyncRead;
//use tokio::io::AsyncRead;
use std::borrow::Cow;
use std::collections::HashMap;
use std::error::Error;
use std::time::SystemTime;
use tracing::{debug, info, span, trace, Level};
use url::Url;

use http::{self, header, request::Builder, HeaderMap, HeaderValue, Method, Request, Response};
use serde::de::DeserializeOwned;
//use serde_bytes::ByteBuf;
use reqwest::Body;
use serde_json::json;

use crate::api::{
    query, ApiError, AsyncClient, BodyError, Client, Query, QueryAsync, QueryParams, RawQuery,
    RawQueryAsync, RestClient,
};
use crate::types::BoxedAsyncRead;
use crate::types::ServiceType;
//use crate::types::identity::v3::ResourceWithHeaders;

/// A trait for providing the necessary information for a single REST API endpoint.
pub trait RestEndpoint {
    /// The HTTP method to use for the endpoint.
    fn method(&self) -> Method;
    /// The path to the endpoint.
    fn endpoint(&self) -> Cow<'static, str>;
    /// The endpoint service type.
    fn service_type(&self) -> ServiceType;

    /// Query parameters for the endpoint.
    fn parameters(&self) -> QueryParams {
        QueryParams::default()
    }

    /// The body for the endpoint.
    ///
    /// Returns the `Content-Type` header for the data as well as the data itself.
    fn body(&self) -> Result<Option<(&'static str, Vec<u8>)>, BodyError> {
        Ok(None)
    }

    /// Returns response key under which the data is expected
    fn response_key(&self) -> Option<Cow<'static, str>> {
        None
    }

    /// Returns map of headers to capture from the endpoint response to the names in the target result struct.
    fn response_headers(&self) -> HashMap<&str, &str> {
        HashMap::new()
    }

    /// Returns headers to be set into the request
    fn request_headers(&self) -> Option<&HeaderMap> {
        None
    }
}

pub(crate) fn prepare_request<E>(
    mut url: Url,
    endpoint: &E,
) -> Result<(Builder, Vec<u8>), BodyError>
where
    E: RestEndpoint,
{
    endpoint.parameters().add_to_url(&mut url);
    let mut req = Request::builder()
        .method(endpoint.method())
        .uri(query::url_to_http_uri(url))
        .header(header::ACCEPT, HeaderValue::from_static("application/json"));
    if let Some(request_headers) = endpoint.request_headers() {
        let headers = req.headers_mut().unwrap();
        for (k, v) in request_headers.iter() {
            headers.append(k, v.clone());
        }
    }
    if let Some((mime, data)) = endpoint.body()? {
        let req = req.header(header::CONTENT_TYPE, mime);
        Ok((req, data))
    } else {
        Ok((req, Vec::new()))
    }
}

pub(super) fn get_json<C>(rsp: &Response<Bytes>) -> Result<serde_json::Value, ApiError<C::Error>>
where
    C: RestClient,
{
    let status = rsp.status();
    let mut v = if let Ok(v) = serde_json::from_slice(rsp.body()) {
        v
    } else {
        return Err(ApiError::server_error(status, rsp.body()));
    };
    if !status.is_success() {
        return Err(ApiError::from_openstack(status, v));
    }
    Ok(v)
}

impl<E, T, C> Query<T, C> for E
where
    E: RestEndpoint,
    T: DeserializeOwned,
    C: Client,
{
    fn query(&self, client: &C) -> Result<T, ApiError<C::Error>> {
        let span = span!(Level::DEBUG, "Query span");
        let _enter = span.enter();

        let mut url = client.rest_endpoint(&self.service_type(), &self.endpoint())?;
        let (req, data) = prepare_request::<E>(url, self)?;

        let rsp = client.rest(req, data)?;

        let mut v = get_json::<C>(&rsp)?;

        if let Some(root_key) = self.response_key() {
            v = v[root_key.to_string()].take();
        }

        let headers = rsp.headers();
        // Process headers which endpoint wants to capture
        for (header_key, target_val) in self.response_headers().iter() {
            if let Some(val) = headers.get(*header_key) {
                trace!("Registered Header {} was found", header_key);
                v[*target_val] = json!(val.to_str().unwrap());
            }
        }
        match serde_json::from_value::<T>(v) {
            Ok(mut r) => Ok(r),
            Err(e) => Err(ApiError::data_type::<T>(e)),
        }
    }
}

#[async_trait]
impl<E, T, C> QueryAsync<T, C> for E
where
    E: RestEndpoint + Sync,
    C: AsyncClient + Sync,
    T: DeserializeOwned + 'static,
{
    async fn query_async(&self, client: &C) -> Result<T, ApiError<C::Error>> {
        let span = span!(Level::DEBUG, "Query span");
        let _enter = span.enter();

        //let rsp = self.raw_query_async(client).await?;

        let mut url = client.rest_endpoint(&self.service_type(), &self.endpoint())?;
        let (req, data) = prepare_request::<E>(url, self)?;

        let rsp = client.rest_async(req, data).await?;
        let mut v = get_json::<C>(&rsp)?;

        if let Some(root_key) = self.response_key() {
            v = v[root_key.to_string()].take();
        }

        let headers = rsp.headers();
        // Process headers which endpoint wants to capture
        for (header_key, target_val) in self.response_headers().iter() {
            if let Some(val) = headers.get(*header_key) {
                trace!("Registered Header {} was found", header_key);
                v[*target_val] = json!(val.to_str().unwrap());
            }
        }
        match serde_json::from_value::<T>(v) {
            Ok(mut r) => Ok(r),
            Err(e) => Err(ApiError::data_type::<T>(e)),
        }
    }
}

/// Raw Query implementation
impl<E, C> RawQuery<C> for E
where
    E: RestEndpoint,
    C: Client,
{
    fn raw_query(&self, client: &C) -> Result<Response<Bytes>, ApiError<C::Error>> {
        let span = span!(Level::DEBUG, "Query span");
        let _enter = span.enter();

        let mut url = client.rest_endpoint(&self.service_type(), &self.endpoint())?;
        let (req, data) = prepare_request::<E>(url, self)?;

        let rsp = client.rest(req, data)?;

        Ok(rsp)
    }
}

/// Raw Query Async implementation
#[async_trait]
impl<E, C> RawQueryAsync<C> for E
where
    E: RestEndpoint + Sync,
    C: AsyncClient + Sync,
{
    async fn raw_query_async(&self, client: &C) -> Result<Response<Bytes>, ApiError<C::Error>> {
        let span = span!(Level::DEBUG, "Query span");
        let _enter = span.enter();

        let mut url = client.rest_endpoint(&self.service_type(), &self.endpoint())?;
        let (req, data) = prepare_request::<E>(url, self)?;

        let rsp = client.rest_async(req, data).await?;

        let status = rsp.status();
        if !status.is_success() {
            let mut v = if let Ok(v) = serde_json::from_slice(rsp.body()) {
                v
            } else {
                return Err(ApiError::server_error(status, rsp.body()));
            };
            return Err(ApiError::from_openstack(status, v));
        }

        Ok(rsp)
    }

    async fn raw_query_read_body_async(
        &self,
        client: &C,
        data: BoxedAsyncRead,
    ) -> Result<Response<Bytes>, ApiError<C::Error>> {
        let span = span!(Level::DEBUG, "Query span");
        let _enter = span.enter();

        let mut url = client.rest_endpoint(&self.service_type(), &self.endpoint())?;
        self.parameters().add_to_url(&mut url);
        let mut req = Request::builder()
            .method(self.method())
            .uri(query::url_to_http_uri(url));
        if let Some(request_headers) = self.request_headers() {
            let headers = req.headers_mut().unwrap();
            for (k, v) in request_headers.iter() {
                headers.append(k, v.clone());
            }
        }

        let rsp = client.rest_read_body_async(req, data).await?;

        let status = rsp.status();
        if !status.is_success() {
            let mut v = if let Ok(v) = serde_json::from_slice(rsp.body()) {
                v
            } else {
                return Err(ApiError::server_error(status, rsp.body()));
            };
            return Err(ApiError::from_openstack(status, v));
        }

        Ok(rsp)
    }

    /// Perform a download API call (returning AsyncRead or the body)
    async fn download_async(
        &self,
        client: &C,
    ) -> Result<(HeaderMap, BoxedAsyncRead), ApiError<C::Error>> {
        let span = span!(Level::DEBUG, "Query span");
        let _enter = span.enter();

        let mut url = client.rest_endpoint(&self.service_type(), &self.endpoint())?;
        let (req, data) = prepare_request::<E>(url, self)?;

        let rsp = client.download_async(req, data).await?;

        Ok(rsp)
    }
}

#[cfg(test)]
mod tests {
    use http::HeaderMap;
    use http::StatusCode;
    use serde::Deserialize;
    use serde_json::json;

    use crate::api::rest_endpoint_prelude::*;
    use crate::api::{ApiError, Query};
    use crate::test::client::{MockAsyncServerClient, MockServerClient};
    use crate::types::ServiceType;

    struct Dummy;

    impl RestEndpoint for Dummy {
        fn method(&self) -> Method {
            Method::GET
        }

        fn endpoint(&self) -> Cow<'static, str> {
            "dummy".into()
        }

        fn service_type(&self) -> ServiceType {
            ServiceType::Other("dummy".to_string())
        }
    }

    #[derive(Debug, Deserialize)]
    struct DummyResult {
        value: u8,
    }

    #[test]
    fn test_non_json_response() {
        let client = MockServerClient::new();
        let mock = client.server.mock(|when, then| {
            when.method(httpmock::Method::GET).path("/dummy");
            then.status(200).body("not json");
        });

        let res: Result<DummyResult, _> = Dummy.query(&client);
        let err = res.unwrap_err();
        if let ApiError::OpenStackService { status, .. } = err {
            assert_eq!(status, http::StatusCode::OK);
        } else {
            panic!("unexpected error: {}", err);
        }
        mock.assert();
    }

    #[test]
    fn test_empty_response() {
        let client = MockServerClient::new();
        let mock = client.server.mock(|when, then| {
            when.method(httpmock::Method::GET).path("/dummy");
            then.status(200);
        });

        let res: Result<DummyResult, _> = Dummy.query(&client);
        let err = res.unwrap_err();
        if let ApiError::OpenStackService { status, .. } = err {
            assert_eq!(status, http::StatusCode::OK);
        } else {
            panic!("unexpected error: {}", err);
        }
        mock.assert();
    }

    #[test]
    fn test_error_not_found() {
        let client = MockServerClient::new();
        let mock = client.server.mock(|when, then| {
            when.method(httpmock::Method::GET).path("/dummy");
            then.status(404);
        });
        let res: Result<DummyResult, _> = Dummy.query(&client);
        match res.unwrap_err() {
            ApiError::ResourceNotFound {} => {}
            err => {
                panic!("unexpected error: {}", err);
            }
        }
        mock.assert();
    }

    #[test]
    fn test_error_bad_json() {
        let client = MockServerClient::new();
        let mock = client.server.mock(|when, then| {
            when.method(httpmock::Method::GET).path("/dummy");
            then.status(http::StatusCode::CONFLICT.into());
        });

        let res: Result<DummyResult, _> = Dummy.query(&client);
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
            when.method(httpmock::Method::GET).path("/dummy");
            then.status(http::StatusCode::CONFLICT.into())
                .json_body(json!({"message": "dummy error message"}));
        });

        let res: Result<DummyResult, _> = Dummy.query(&client);
        let err = res.unwrap_err();
        if let ApiError::OpenStack { status, msg } = err {
            assert_eq!(msg, "dummy error message");
        } else {
            panic!("unexpected error: {}", err);
        }
        mock.assert();
    }

    #[test]
    fn test_error_detection_unknown() {
        let client = MockServerClient::new();
        let err_obj = json!({"bogus": "dummy error message"});
        let mock = client.server.mock(|when, then| {
            when.method(httpmock::Method::GET).path("/dummy");
            then.status(StatusCode::CONFLICT.into())
                .json_body(err_obj.clone());
        });

        let res: Result<DummyResult, _> = Dummy.query(&client);
        let err = res.unwrap_err();
        if let ApiError::OpenStackUnrecognized { status, obj } = err {
            assert_eq!(obj, err_obj);
        } else {
            panic!("unexpected error: {}", err);
        }
        mock.assert();
    }

    #[test]
    fn test_bad_deserialization() {
        let client = MockServerClient::new();
        let mock = client.server.mock(|when, then| {
            when.method(httpmock::Method::GET).path("/dummy");
            then.status(200).json_body(json!({"not_value": 0}));
        });

        let res: Result<DummyResult, _> = Dummy.query(&client);
        let err = res.unwrap_err();
        if let ApiError::DataType { source, typename } = err {
            assert_eq!(source.to_string(), "missing field `value`");
            assert_eq!(
                typename,
                "openstack_sdk::api::rest_endpoint::tests::DummyResult"
            );
        } else {
            panic!("unexpected error: {}", err);
        }
        mock.assert();
    }

    #[test]
    fn test_good_deserialization() {
        let client = MockServerClient::new();
        let mock = client.server.mock(|when, then| {
            when.method(httpmock::Method::GET).path("/dummy");
            then.status(200).json_body(json!({"value": 0}));
        });

        let res: Result<DummyResult, _> = Dummy.query(&client);
        assert_eq!(res.unwrap().value, 0);
        mock.assert();
    }
}
