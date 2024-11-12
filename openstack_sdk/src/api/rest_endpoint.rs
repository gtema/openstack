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

use async_trait::async_trait;
use bytes::Bytes;
//use futures_core::stream::Stream;
//use futures::io::AsyncRead;
//use tokio::io::AsyncRead;
use std::borrow::Cow;
use std::collections::HashMap;

use tracing::{instrument, trace, Level};
use url::Url;

use http::{
    self, header, request::Builder, HeaderMap, HeaderValue, Method, Request, Response, Uri,
};
use serde::de::DeserializeOwned;
//use serde_bytes::ByteBuf;

use serde_json::json;

use crate::api::{query, ApiError, BodyError, QueryParams, RestClient};
#[cfg(feature = "async")]
use crate::api::{AsyncClient, QueryAsync, RawQueryAsync};
#[cfg(feature = "sync")]
use crate::api::{Client, Query, RawQuery};
use crate::catalog::ServiceEndpoint;
use crate::types::ApiVersion;
use crate::types::BoxedAsyncRead;
use crate::types::ServiceType;

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

    /// Returns response key under which the list item data is expected (i.e. `{"list": ["item":{},  "item": {}}`
    fn response_list_item_key(&self) -> Option<Cow<'static, str>> {
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

    /// Returns required API version
    ///
    /// - `None` is interpreted as default version
    /// - `ApiVersion {0, 0}` is interpreted as explicitly unversioned API
    ///
    /// Default is to try to determine version from the url (first segment)
    fn api_version(&self) -> Option<ApiVersion> {
        ApiVersion::from_endpoint_url(self.endpoint())
    }
}

/// Set latest microversion information into the request
/// for services that support that.
pub(crate) fn set_latest_microversion<E>(
    request: &mut Builder,
    service_endpoint: &ServiceEndpoint,
    endpoint: &E,
) where
    E: RestEndpoint,
{
    let mh_service_type = match endpoint.service_type() {
        ServiceType::BlockStorage => Some("volume"),
        ServiceType::Compute => Some("compute"),
        ServiceType::Placement => Some("placement"),
        _ => None,
    };
    if let Some(st) = mh_service_type {
        // TODO(gtema) switch to `get_api_version` method since version may be missing
        if let Some(hdrs) = request.headers_mut() {
            let ver = service_endpoint.version();
            if ver.major == 0 {
                return;
            }
            if let Ok(val) =
                HeaderValue::from_str(format!("{} {}.{}", st, ver.major, ver.minor).as_str())
            {
                hdrs.insert("Openstack-API-Version", val);
            }
        }
    }
}

pub(crate) fn prepare_request<C, E>(
    service_endpoint: &ServiceEndpoint,
    mut url: Url,
    endpoint: &E,
) -> Result<(Builder, Vec<u8>), ApiError<C::Error>>
where
    E: RestEndpoint,
    C: RestClient,
{
    endpoint.parameters().add_to_url(&mut url);
    let mut req = Request::builder()
        .method(endpoint.method())
        .uri(query::url_to_http_uri(url))
        .header(header::ACCEPT, HeaderValue::from_static("application/json"));
    set_latest_microversion(&mut req, service_endpoint, endpoint);
    if let Some(request_headers) = endpoint.request_headers() {
        let headers = req.headers_mut().unwrap();
        for (k, v) in request_headers.iter() {
            headers.insert(k, v.clone());
        }
    }
    if let Some((mime, data)) = endpoint.body()? {
        let req = req.header(header::CONTENT_TYPE, mime);
        Ok((req, data))
    } else {
        Ok((req, Vec::new()))
    }
}

/// Cast response to Json Value
pub(super) fn get_json<C>(
    rsp: &Response<Bytes>,
    uri: Option<Uri>,
) -> Result<serde_json::Value, ApiError<C::Error>>
where
    C: RestClient,
{
    let status = rsp.status();
    let v = if let Ok(v) = serde_json::from_slice(rsp.body()) {
        v
    } else {
        return Err(ApiError::server_error(uri, status, rsp.body()));
    };
    if !status.is_success() {
        return Err(ApiError::from_openstack(uri, status, v));
    }
    Ok(v)
}

/// Check for possible error in the response
pub fn check_response_error<C>(
    rsp: &Response<Bytes>,
    uri: Option<Uri>,
) -> Result<(), ApiError<C::Error>>
where
    C: RestClient,
{
    let status = rsp.status();
    if !status.is_success() {
        let v = if let Ok(v) = serde_json::from_slice(rsp.body()) {
            v
        } else {
            return Err(ApiError::server_error(uri, status, rsp.body()));
        };
        return Err(ApiError::from_openstack(uri, status, v));
    }
    Ok(())
}

#[cfg(feature = "sync")]
impl<E, T, C> Query<T, C> for E
where
    E: RestEndpoint,
    T: DeserializeOwned,
    C: Client,
{
    #[instrument(name = "query", level = "debug", skip_all)]
    fn query(&self, client: &C) -> Result<T, ApiError<C::Error>> {
        let ep = client.get_service_endpoint(&self.service_type(), self.api_version().as_ref())?;
        let url = ep.build_request_url(&self.endpoint())?;
        let (req, data) = prepare_request::<C, E>(ep, url, self)?;

        let query_uri = req.uri_ref().cloned();
        let rsp = client.rest(req, data)?;
        let mut v = get_json::<C>(&rsp, query_uri)?;
        //.with_context(|| format!("Request to `{}` failed", url))?;

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
            Ok(r) => Ok(r),
            Err(e) => Err(ApiError::data_type::<T>(e)),
        }
    }
}

#[cfg(feature = "async")]
#[async_trait]
impl<E, T, C> QueryAsync<T, C> for E
where
    E: RestEndpoint + Sync,
    C: AsyncClient + Sync,
    T: DeserializeOwned + 'static,
{
    #[instrument(name = "query", level = "debug", skip_all)]
    async fn query_async(&self, client: &C) -> Result<T, ApiError<C::Error>> {
        let ep = client.get_service_endpoint(&self.service_type(), self.api_version().as_ref())?;
        let (req, data) =
            prepare_request::<C, E>(ep, ep.build_request_url(&self.endpoint())?, self)?;

        let query_uri = req.uri_ref().cloned();
        let rsp = client.rest_async(req, data).await?;
        let mut v = get_json::<C>(&rsp, query_uri)?;

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
            Ok(r) => Ok(r),
            Err(e) => Err(ApiError::data_type::<T>(e)),
        }
    }
}

#[cfg(feature = "sync")]
/// Raw Query implementation
impl<E, C> RawQuery<C> for E
where
    E: RestEndpoint,
    C: Client,
{
    #[instrument(name = "query", level = "debug", skip_all)]
    fn raw_query(&self, client: &C) -> Result<Response<Bytes>, ApiError<C::Error>> {
        let ep = client.get_service_endpoint(&self.service_type(), self.api_version().as_ref())?;
        let (req, data) =
            prepare_request::<C, E>(ep, ep.build_request_url(&self.endpoint())?, self)?;

        let rsp = client.rest(req, data)?;

        Ok(rsp)
    }
}

#[cfg(feature = "async")]
/// Raw Query Async implementation
#[async_trait]
impl<E, C> RawQueryAsync<C> for E
where
    E: RestEndpoint + Sync,
    C: AsyncClient + Sync,
{
    #[instrument(name = "query", level = "debug", skip_all)]
    async fn raw_query_async_ll(
        &self,
        client: &C,
        inspect_error: Option<bool>,
    ) -> Result<Response<Bytes>, ApiError<C::Error>> {
        let ep = client.get_service_endpoint(&self.service_type(), self.api_version().as_ref())?;
        let (req, data) =
            prepare_request::<C, E>(ep, ep.build_request_url(&self.endpoint())?, self)?;

        let query_uri = req.uri_ref().cloned();
        let rsp = client.rest_async(req, data).await?;

        if inspect_error.unwrap_or(true) {
            check_response_error::<C>(&rsp, query_uri)?;
        }
        Ok(rsp)
    }

    async fn raw_query_async(&self, client: &C) -> Result<Response<Bytes>, ApiError<C::Error>> {
        self.raw_query_async_ll(client, Some(true)).await
    }

    #[instrument(name = "query", level = "debug", skip_all)]
    async fn raw_query_read_body_async(
        &self,
        client: &C,
        data: BoxedAsyncRead,
    ) -> Result<Response<Bytes>, ApiError<C::Error>> {
        let ep = client.get_service_endpoint(&self.service_type(), self.api_version().as_ref())?;
        let mut url = ep.build_request_url(&self.endpoint())?;
        self.parameters().add_to_url(&mut url);
        let mut req = Request::builder()
            .method(self.method())
            .uri(query::url_to_http_uri(url));
        set_latest_microversion(&mut req, ep, self);
        if let Some(request_headers) = self.request_headers() {
            let headers = req.headers_mut().unwrap();
            for (k, v) in request_headers.iter() {
                headers.insert(k, v.clone());
            }
        }

        let query_uri = req.uri_ref().cloned();
        let rsp = client.rest_read_body_async(req, data).await?;

        check_response_error::<C>(&rsp, query_uri)?;

        Ok(rsp)
    }

    /// Perform a download API call (returning AsyncRead or the body)
    #[instrument(name = "query", level = "debug", skip_all)]
    async fn download_async(
        &self,
        client: &C,
    ) -> Result<(HeaderMap, BoxedAsyncRead), ApiError<C::Error>> {
        let ep = client.get_service_endpoint(&self.service_type(), self.api_version().as_ref())?;
        let (req, data) =
            prepare_request::<C, E>(ep, ep.build_request_url(&self.endpoint())?, self)?;

        let rsp = client.download_async(req, data).await?;

        Ok(rsp)
    }
}

#[cfg(feature = "sync")]
#[cfg(test)]
mod tests {

    use http::StatusCode;
    use serde::Deserialize;
    use serde_json::json;

    use crate::api::rest_endpoint_prelude::*;
    use crate::api::{ApiError, Query};
    use crate::test::client::MockServerClient;
    use crate::types::ServiceType;

    struct Dummy;

    impl RestEndpoint for Dummy {
        fn method(&self) -> http::Method {
            http::Method::GET
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
        let err = res.unwrap_err();
        if let ApiError::OpenStack { status, .. } = err {
            assert_eq!(status, http::StatusCode::NOT_FOUND);
        } else {
            panic!("unexpected error: {}", err);
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
        if let ApiError::OpenStack {
            status: _,
            uri: _,
            msg,
        } = err
        {
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
        if let ApiError::OpenStackUnrecognized {
            status: _,
            uri: _,
            obj,
        } = err
        {
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
