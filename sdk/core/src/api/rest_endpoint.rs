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

use tracing::{instrument, trace};
use url::Url;

use http::{
    self, HeaderMap, HeaderValue, Method, Request, Response, Uri, header, request::Builder,
};
use serde::de::DeserializeOwned;
//use serde_bytes::ByteBuf;

use serde_json::json;

use crate::api::{ApiError, BodyError, QueryParams, RestClient, query};
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
    fn parameters(&self) -> QueryParams<'_> {
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

    /// Returns the minimum microversion this endpoint variant requires for
    /// the `OpenStack-API-Version` header.
    ///
    /// - `None` is interpreted as default version
    /// - `ApiVersion {0, 0}` is interpreted as explicitly unversioned API
    ///
    /// Defaults to [`RestEndpoint::api_version`] for backwards compatibility.
    fn min_version(&self) -> Option<ApiVersion> {
        self.api_version()
    }

    /// Returns the maximum microversion this endpoint variant supports, if
    /// bounded.
    ///
    /// Generated code emits a separate struct per microversion break (e.g.
    /// `create_20`, `create_276`, ...); each variant's `min_version()` is the
    /// version it was introduced at. `max_version()` is the version of the
    /// *next* variant minus one, when known. `None` means this variant is
    /// valid for every version the cloud supports at or above
    /// `min_version()`.
    fn max_version(&self) -> Option<ApiVersion> {
        None
    }
}

/// Compute the microversion to send for this endpoint against a cloud's
/// discovered range, without touching any request.
///
/// - `endpoint.min_version()` is a floor, not a hard requirement: codegen's
///   `min-ver` just marks when a struct variant was introduced, so a cloud
///   whose discovered `min_version` is higher doesn't make the variant
///   unsupported — the sent version is bumped up to the cloud's floor
///   instead (`max(endpoint.min_version(), cloud.min_version())`).
/// - `endpoint.max_version()`, when set, is a genuine ceiling: the variant
///   stops applying once the cloud's floor moves past it.
///
/// Returns `Ok(None)` for unversioned endpoints (nothing to negotiate).
/// Returns `Err` when no version can satisfy both the endpoint's and the
/// cloud's constraints.
///
/// Exposed as `pub` so callers that already hold `client` + the endpoint
/// (e.g. to pick a version-appropriate response schema) can learn the
/// negotiated version via `client.get_service_endpoint(...)` without
/// duplicating this bounds-check logic.
pub fn negotiate_microversion<C, E>(
    service_endpoint: &ServiceEndpoint,
    endpoint: &E,
) -> Result<Option<ApiVersion>, ApiError<C::Error>>
where
    C: RestClient,
    E: RestEndpoint,
{
    let ep_min = match endpoint.min_version() {
        Some(v) if v.major != 0 => v,
        _ => return Ok(None),
    };
    let ep_max = endpoint.max_version();

    let cloud_min: Option<ApiVersion> = service_endpoint
        .min_version()
        .as_deref()
        .and_then(|s| ApiVersion::from_apiver_str(s, false).ok());
    let cloud_max: Option<ApiVersion> = service_endpoint
        .max_version()
        .as_deref()
        .and_then(|s| ApiVersion::from_apiver_str(s, false).ok());

    let incompatible = |required: ApiVersion| {
        ApiError::microversion_incompatible(
            required.major,
            required.minor,
            service_endpoint.min_version().clone(),
            service_endpoint.max_version().clone(),
        )
    };

    // The endpoint's floor is newer than anything this cloud will ever
    // support.
    if let Some(cmax) = cloud_max
        && ep_min > cmax
    {
        return Err(incompatible(ep_min));
    }

    // The variant's ceiling is older than what this cloud's floor requires
    // — it's been superseded by a newer variant on this cloud.
    if let Some(emax) = ep_max
        && let Some(cmin) = cloud_min
        && emax < cmin
    {
        return Err(incompatible(emax));
    }

    // Send the highest version both sides agree is valid: raise the
    // endpoint's floor to the cloud's floor when the cloud requires newer.
    let req_ver = match cloud_min {
        Some(cmin) if cmin > ep_min => cmin,
        _ => ep_min,
    };

    Ok(Some(req_ver))
}

/// Validate the endpoint's microversion against the cloud's discovered
/// range and set the `OpenStack-API-Version` header.
///
/// See [`negotiate_microversion`] for how the sent version is picked.
/// Returns `Ok(())` when the header was set (or skipped for unversioned
/// endpoints, or an unrecognized service type). Returns `Err` when no
/// version can satisfy both the endpoint's and the cloud's constraints.
pub fn set_request_microversion_header<C, E>(
    request: &mut Builder,
    service_endpoint: &ServiceEndpoint,
    endpoint: &E,
) -> Result<(), ApiError<C::Error>>
where
    C: RestClient,
    E: RestEndpoint,
{
    let Some(req_ver) = negotiate_microversion::<C, E>(service_endpoint, endpoint)? else {
        return Ok(());
    };

    let Some(st) = (match endpoint.service_type() {
        ServiceType::BlockStorage => Some("volume"),
        ServiceType::Compute => Some("compute"),
        ServiceType::ContainerInfrastructureManagement => Some("container-infra"),
        ServiceType::Placement => Some("placement"),
        _ => None,
    }) else {
        return Ok(());
    };

    if let Some(hdrs) = request.headers_mut()
        && let Ok(val) = HeaderValue::from_str(format!("{} {}", st, req_ver).as_str())
    {
        hdrs.insert("OpenStack-API-Version", val);
    }
    Ok(())
}

pub fn prepare_request<C, E>(
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
        .uri(query::url_to_http_uri(url)?)
        .header(header::ACCEPT, HeaderValue::from_static("application/json"));
    set_request_microversion_header::<C, E>(&mut req, service_endpoint, endpoint)?;
    if let Some(request_headers) = endpoint.request_headers()
        && let Some(headers) = req.headers_mut()
    {
        headers.extend(request_headers.clone())
    }
    if let Some((mime, data)) = endpoint.body()? {
        let req = req.header(header::CONTENT_TYPE, mime);
        Ok((req, data))
    } else {
        Ok((req, Vec::new()))
    }
}

/// Cast response to Json Value
pub fn get_json<C>(
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
        return Err(ApiError::server_error(uri, rsp, rsp.body()));
    };
    if !status.is_success() {
        return Err(ApiError::from_openstack(uri, rsp, v));
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
            return Err(ApiError::server_error(uri, rsp, rsp.body()));
        };
        return Err(ApiError::from_openstack(uri, rsp, v));
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
        let (req, data) = prepare_request::<C, E>(&ep, url, self)?;

        let query_uri = req.uri_ref().cloned();
        let rsp = client.rest(req, data)?;
        let mut v = get_json::<C>(&rsp, query_uri)?;
        //.with_context(|| format!("Request to `{}` failed", url))?;

        if let Some(root_key) = self.response_key() {
            v = v[root_key.as_ref()].take();
        }
        if let (Some(item_key), Some(array)) = (self.response_list_item_key(), v.as_array_mut()) {
            for elem in array {
                *elem = elem[item_key.as_ref()].take();
            }
        }

        let headers = rsp.headers();
        // Process headers which endpoint wants to capture
        for (header_key, target_val) in self.response_headers().iter() {
            if let Some(val) = headers.get(*header_key) {
                trace!("Registered Header {} was found", header_key);
                match val.to_str() {
                    Ok(header_str) => v[*target_val] = json!(header_str),
                    Err(e) => {
                        return Err(ApiError::InvalidHeader {
                            header: header_key.to_string(),
                            message: e.to_string(),
                        });
                    }
                };
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
        let ep = client
            .get_service_endpoint(&self.service_type(), self.api_version().as_ref())
            .await?;
        let (req, data) =
            prepare_request::<C, E>(&ep, ep.build_request_url(&self.endpoint())?, self)?;

        let query_uri = req.uri_ref().cloned();
        let rsp = client.rest_async(req, data).await?;
        let mut v = get_json::<C>(&rsp, query_uri)?;

        if let Some(root_key) = self.response_key() {
            v = v[root_key.as_ref()].take();
        }

        if let (Some(item_key), Some(array)) = (self.response_list_item_key(), v.as_array_mut()) {
            for elem in array {
                *elem = elem[item_key.as_ref()].take();
            }
        }

        let headers = rsp.headers();
        // Process headers which endpoint wants to capture
        for (header_key, target_val) in self.response_headers().iter() {
            if let Some(val) = headers.get(*header_key) {
                trace!("Registered Header {} was found", header_key);
                match val.to_str() {
                    Ok(header_str) => v[*target_val] = json!(header_str),
                    Err(e) => {
                        return Err(ApiError::InvalidHeader {
                            header: header_key.to_string(),
                            message: e.to_string(),
                        });
                    }
                };
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
            prepare_request::<C, E>(&ep, ep.build_request_url(&self.endpoint())?, self)?;

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
        let ep = client
            .get_service_endpoint(&self.service_type(), self.api_version().as_ref())
            .await?;
        let (req, data) =
            prepare_request::<C, E>(&ep, ep.build_request_url(&self.endpoint())?, self)?;

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
        let ep = client
            .get_service_endpoint(&self.service_type(), self.api_version().as_ref())
            .await?;
        let mut url = ep.build_request_url(&self.endpoint())?;
        self.parameters().add_to_url(&mut url);
        let mut req = Request::builder()
            .method(self.method())
            .uri(query::url_to_http_uri(url)?);
        set_request_microversion_header::<C, E>(&mut req, &ep, self)?;
        if let Some(request_headers) = self.request_headers()
            && let Some(headers) = req.headers_mut()
        {
            headers.extend(request_headers.clone())
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
        let ep = client
            .get_service_endpoint(&self.service_type(), self.api_version().as_ref())
            .await?;
        let (req, data) =
            prepare_request::<C, E>(&ep, ep.build_request_url(&self.endpoint())?, self)?;

        let rsp = client.download_async(req, data).await?;

        Ok(rsp)
    }
}

#[cfg(test)]
mod tests {
    use http::StatusCode;
    use httpmock::MockServer;
    use serde::Deserialize;
    use serde_json::json;

    use crate::api::ApiError;
    #[cfg(feature = "sync")]
    use crate::api::Query;
    #[cfg(feature = "async")]
    use crate::api::QueryAsync;
    use crate::api::rest_endpoint_prelude::*;
    use crate::test::client::FakeOpenStackClient;
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
            ServiceType::from("dummy")
        }
    }

    #[derive(Debug, Deserialize, PartialEq)]
    struct DummyResult {
        value: u8,
    }

    #[cfg(feature = "sync")]
    #[test]
    fn test_non_json_response() {
        let server = MockServer::start();
        let client = FakeOpenStackClient::new(server.base_url());
        let mock = server.mock(|when, then| {
            when.method(httpmock::Method::GET).path("/dummy");
            then.status(200).body("not json");
        });

        let res: Result<DummyResult, _> = Dummy.query(&client);
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
    fn test_empty_response() {
        let server = MockServer::start();
        let client = FakeOpenStackClient::new(server.base_url());
        let mock = server.mock(|when, then| {
            when.method(httpmock::Method::GET).path("/dummy");
            then.status(200);
        });

        let res: Result<DummyResult, _> = Dummy.query(&client);
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
    fn test_error_not_found() {
        let server = MockServer::start();
        let client = FakeOpenStackClient::new(server.base_url());
        let mock = server.mock(|when, then| {
            when.method(httpmock::Method::GET).path("/dummy");
            then.status(404);
        });
        let res: Result<DummyResult, _> = Dummy.query(&client);
        let err = res.unwrap_err();
        if let ApiError::OpenStack { status, .. } = err {
            assert_eq!(status, http::StatusCode::NOT_FOUND);
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
            when.method(httpmock::Method::GET).path("/dummy");
            then.status(http::StatusCode::CONFLICT);
        });

        let res: Result<DummyResult, _> = Dummy.query(&client);
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
            when.method(httpmock::Method::GET).path("/dummy");
            then.status(http::StatusCode::CONFLICT)
                .json_body(json!({"message": "dummy error message"}));
        });

        let res: Result<DummyResult, _> = Dummy.query(&client);
        let err = res.unwrap_err();
        if let ApiError::OpenStack {
            status: _,
            uri: _,
            msg,
            ..
        } = err
        {
            assert_eq!(msg, "dummy error message");
        } else {
            panic!("unexpected error: {err}");
        }
        mock.assert();
    }

    #[cfg(feature = "sync")]
    #[test]
    fn test_error_detection_unknown() {
        let server = MockServer::start();
        let client = FakeOpenStackClient::new(server.base_url());
        let err_obj = json!({"bogus": "dummy error message"});
        let mock = server.mock(|when, then| {
            when.method(httpmock::Method::GET).path("/dummy");
            then.status(StatusCode::CONFLICT).json_body(err_obj.clone());
        });

        let res: Result<DummyResult, _> = Dummy.query(&client);
        let err = res.unwrap_err();
        if let ApiError::OpenStackUnrecognized {
            status: _,
            uri: _,
            obj,
            ..
        } = err
        {
            assert_eq!(obj, err_obj);
        } else {
            panic!("unexpected error: {err}");
        }
        mock.assert();
    }

    #[cfg(feature = "sync")]
    #[test]
    fn test_bad_deserialization() {
        let server = MockServer::start();
        let client = FakeOpenStackClient::new(server.base_url());
        let mock = server.mock(|when, then| {
            when.method(httpmock::Method::GET).path("/dummy");
            then.status(200).json_body(json!({"not_value": 0}));
        });

        let res: Result<DummyResult, _> = Dummy.query(&client);
        let err = res.unwrap_err();
        if let ApiError::DataType { source, typename } = err {
            assert_eq!(source.to_string(), "missing field `value`");
            assert_eq!(
                typename,
                "openstack_sdk_core::api::rest_endpoint::tests::DummyResult"
            );
        } else {
            panic!("unexpected error: {err}");
        }
        mock.assert();
    }

    #[cfg(feature = "sync")]
    #[test]
    fn test_good_deserialization() {
        let server = MockServer::start();
        let client = FakeOpenStackClient::new(server.base_url());
        let mock = server.mock(|when, then| {
            when.method(httpmock::Method::GET).path("/dummy");
            then.status(200).json_body(json!({"value": 0}));
        });

        let res: Result<DummyResult, _> = Dummy.query(&client);
        assert_eq!(res.unwrap().value, 0);
        mock.assert();
    }

    struct DummyLi;
    impl RestEndpoint for DummyLi {
        fn method(&self) -> http::Method {
            http::Method::GET
        }

        fn endpoint(&self) -> Cow<'static, str> {
            "dummy".into()
        }

        fn service_type(&self) -> ServiceType {
            ServiceType::from("dummy")
        }
        fn response_key(&self) -> Option<Cow<'static, str>> {
            Some("container".into())
        }

        fn response_list_item_key(&self) -> Option<Cow<'static, str>> {
            Some("data".into())
        }
    }

    #[cfg(feature = "sync")]
    #[test]
    fn test_resource_with_list_inside() {
        let server = MockServer::start();
        let client = FakeOpenStackClient::new(server.base_url());
        let mock = server.mock(|when, then| {
            when.method(httpmock::Method::GET).path("/dummy");
            then.status(200).json_body(json!({
                "container": [
                    {"data": {"value": 0}},
                    {"data": {"value": 2}}
            ]}));
        });

        let res: Vec<DummyResult> = DummyLi.query(&client).unwrap();
        assert!(res.contains(&DummyResult { value: 0 }));
        assert!(res.contains(&DummyResult { value: 2 }));
        mock.assert();
    }

    #[cfg(feature = "async")]
    #[tokio::test]
    async fn test_resource_with_list_inside_async() {
        let server = MockServer::start_async().await;
        let client = FakeOpenStackClient::new(server.base_url());
        let mock = server.mock(|when, then| {
            when.method(httpmock::Method::GET).path("/dummy");
            then.status(200).json_body(json!({
            "container": [
                {"data": {"value": 0}},
                {"data": {"value": 1}}
            ]}));
        });

        let res: Vec<DummyResult> = DummyLi.query_async(&client).await.unwrap();
        assert!(res.contains(&DummyResult { value: 0 }), "{:?}", res);
        assert!(res.contains(&DummyResult { value: 1 }), "{:?}", res);
        mock.assert();
    }

    mod microversion {
        use super::*;
        use crate::api::rest_endpoint::{negotiate_microversion, set_request_microversion_header};
        use crate::catalog::ServiceEndpoint;
        use crate::test::client::FakeOpenStackClient;
        use url::Url;

        struct VersionedEndpoint {
            version: Option<ApiVersion>,
            max_version: Option<ApiVersion>,
            service_type: ServiceType,
        }

        impl RestEndpoint for VersionedEndpoint {
            fn method(&self) -> http::Method {
                http::Method::GET
            }
            fn endpoint(&self) -> Cow<'static, str> {
                "test".into()
            }
            fn service_type(&self) -> ServiceType {
                self.service_type.clone()
            }
            fn api_version(&self) -> Option<ApiVersion> {
                self.version
            }
            fn max_version(&self) -> Option<ApiVersion> {
                self.max_version
            }
        }

        fn make_service_endpoint(min: Option<&str>, max: Option<&str>) -> ServiceEndpoint {
            let mut sep = ServiceEndpoint::new(
                Url::parse("http://test.com/v2.1/").unwrap(),
                ApiVersion::new(2, 1),
            );
            if let Some(m) = min {
                sep.set_min_version(Some(m));
            }
            if let Some(mx) = max {
                sep.set_max_version(Some(mx));
            }
            sep.to_owned()
        }

        fn make_ep(ver: Option<ApiVersion>, st: ServiceType) -> VersionedEndpoint {
            VersionedEndpoint {
                version: ver,
                max_version: None,
                service_type: st,
            }
        }

        fn make_ep_bounded(
            ver: Option<ApiVersion>,
            max_ver: Option<ApiVersion>,
            st: ServiceType,
        ) -> VersionedEndpoint {
            VersionedEndpoint {
                version: ver,
                max_version: max_ver,
                service_type: st,
            }
        }

        fn header(req: &http::request::Builder) -> Option<String> {
            req.headers_ref()
                .and_then(|h| h.get("OpenStack-API-Version"))
                .map(|v| v.to_str().unwrap().to_string())
        }

        // ── core happy path ──────────────────────────────────────────

        #[test]
        fn within_range_sets_header() {
            let ep = make_ep(Some(ApiVersion::new(2, 50)), ServiceType::Compute);
            let sep = make_service_endpoint(Some("2.1"), Some("2.60"));
            let mut req = http::Request::builder();
            set_request_microversion_header::<FakeOpenStackClient, _>(&mut req, &sep, &ep).unwrap();
            assert_eq!(header(&req), Some("compute 2.50".into()));
        }

        // ── service type → header prefix mapping ─────────────────────

        #[test]
        fn block_storage_uses_volume_prefix() {
            let ep = make_ep(Some(ApiVersion::new(3, 50)), ServiceType::BlockStorage);
            let sep = make_service_endpoint(Some("3.0"), Some("3.60"));
            let mut req = http::Request::builder();
            set_request_microversion_header::<FakeOpenStackClient, _>(&mut req, &sep, &ep).unwrap();
            assert_eq!(header(&req), Some("volume 3.50".into()));
        }

        #[test]
        fn container_infra_uses_short_prefix() {
            let ep = make_ep(
                Some(ApiVersion::new(1, 15)),
                ServiceType::ContainerInfrastructureManagement,
            );
            let sep = make_service_endpoint(Some("1.0"), Some("1.20"));
            let mut req = http::Request::builder();
            set_request_microversion_header::<FakeOpenStackClient, _>(&mut req, &sep, &ep).unwrap();
            assert_eq!(header(&req), Some("container-infra 1.15".into()));
        }

        #[test]
        fn placement_uses_placement_prefix() {
            let ep = make_ep(Some(ApiVersion::new(1, 10)), ServiceType::Placement);
            let sep = make_service_endpoint(Some("1.0"), Some("1.20"));
            let mut req = http::Request::builder();
            set_request_microversion_header::<FakeOpenStackClient, _>(&mut req, &sep, &ep).unwrap();
            assert_eq!(header(&req), Some("placement 1.10".into()));
        }

        #[test]
        fn unsupported_service_type_skips_header() {
            let ep = make_ep(Some(ApiVersion::new(2, 30)), ServiceType::Network);
            let sep = make_service_endpoint(Some("2.1"), Some("2.60"));
            let mut req = http::Request::builder();
            set_request_microversion_header::<FakeOpenStackClient, _>(&mut req, &sep, &ep).unwrap();
            assert!(header(&req).is_none());
        }

        #[test]
        fn other_service_type_skips_header() {
            let ep = make_ep(
                Some(ApiVersion::new(2, 30)),
                ServiceType::Other("custom-service".into()),
            );
            let sep = make_service_endpoint(Some("2.1"), Some("2.60"));
            let mut req = http::Request::builder();
            set_request_microversion_header::<FakeOpenStackClient, _>(&mut req, &sep, &ep).unwrap();
            assert!(header(&req).is_none());
        }

        // ── bounds validation ────────────────────────────────────────

        #[test]
        fn below_min_is_clamped_up() {
            // endpoint.min_version() below cloud_min is fine: `min-ver` on a
            // generated variant means "introduced at this version", not a
            // hard floor the cloud must match. Compute's legacy 2.0 variants
            // must work against clouds whose discovered min_version is
            // 2.1+ — the sent version is bumped up to the cloud's floor.
            let ep = make_ep(Some(ApiVersion::new(2, 5)), ServiceType::Compute);
            let sep = make_service_endpoint(Some("2.10"), Some("2.60"));
            let mut req = http::Request::builder();
            set_request_microversion_header::<FakeOpenStackClient, _>(&mut req, &sep, &ep).unwrap();
            assert_eq!(header(&req), Some("compute 2.10".into()));
        }

        #[test]
        fn above_max_fails() {
            let ep = make_ep(Some(ApiVersion::new(2, 99)), ServiceType::Compute);
            let sep = make_service_endpoint(Some("2.1"), Some("2.60"));
            let mut req = http::Request::builder();
            let err =
                set_request_microversion_header::<FakeOpenStackClient, _>(&mut req, &sep, &ep)
                    .unwrap_err();
            assert!(matches!(err, ApiError::MicroversionIncompatible { .. }));
        }

        #[test]
        fn exact_bound_min_ok() {
            let ep = make_ep(Some(ApiVersion::new(2, 10)), ServiceType::Compute);
            let sep = make_service_endpoint(Some("2.10"), Some("2.60"));
            let mut req = http::Request::builder();
            set_request_microversion_header::<FakeOpenStackClient, _>(&mut req, &sep, &ep).unwrap();
            assert_eq!(header(&req), Some("compute 2.10".into()));
        }

        #[test]
        fn exact_bound_max_ok() {
            let ep = make_ep(Some(ApiVersion::new(2, 60)), ServiceType::Compute);
            let sep = make_service_endpoint(Some("2.1"), Some("2.60"));
            let mut req = http::Request::builder();
            set_request_microversion_header::<FakeOpenStackClient, _>(&mut req, &sep, &ep).unwrap();
            assert_eq!(header(&req), Some("compute 2.60".into()));
        }

        // ── single-bound (one side missing) ──────────────────────────

        #[test]
        fn only_min_set_clamps_up() {
            let ep = make_ep(Some(ApiVersion::new(2, 5)), ServiceType::Compute);
            let sep = make_service_endpoint(Some("2.10"), None);
            let mut req = http::Request::builder();
            set_request_microversion_header::<FakeOpenStackClient, _>(&mut req, &sep, &ep).unwrap();
            assert_eq!(header(&req), Some("compute 2.10".into()));
        }

        #[test]
        fn only_min_set_passes_when_above() {
            let ep = make_ep(Some(ApiVersion::new(2, 99)), ServiceType::Compute);
            let sep = make_service_endpoint(Some("2.10"), None);
            let mut req = http::Request::builder();
            set_request_microversion_header::<FakeOpenStackClient, _>(&mut req, &sep, &ep).unwrap();
            assert_eq!(header(&req), Some("compute 2.99".into()));
        }

        #[test]
        fn only_max_set_validates_max_only() {
            let ep = make_ep(Some(ApiVersion::new(2, 99)), ServiceType::Compute);
            let sep = make_service_endpoint(None, Some("2.60"));
            let mut req = http::Request::builder();
            let err =
                set_request_microversion_header::<FakeOpenStackClient, _>(&mut req, &sep, &ep)
                    .unwrap_err();
            assert!(matches!(err, ApiError::MicroversionIncompatible { .. }));
        }

        #[test]
        fn only_max_set_passes_when_below() {
            let ep = make_ep(Some(ApiVersion::new(2, 5)), ServiceType::Compute);
            let sep = make_service_endpoint(None, Some("2.60"));
            let mut req = http::Request::builder();
            set_request_microversion_header::<FakeOpenStackClient, _>(&mut req, &sep, &ep).unwrap();
            assert_eq!(header(&req), Some("compute 2.5".into()));
        }

        #[test]
        fn no_cloud_range_skips_validation() {
            let ep = make_ep(Some(ApiVersion::new(2, 99)), ServiceType::Compute);
            let sep = make_service_endpoint(None, None);
            let mut req = http::Request::builder();
            set_request_microversion_header::<FakeOpenStackClient, _>(&mut req, &sep, &ep).unwrap();
            assert_eq!(header(&req), Some("compute 2.99".into()));
        }

        // ── major version mismatch ───────────────────────────────────

        #[test]
        fn major_version_below_cloud_min_is_clamped_up() {
            let ep = make_ep(Some(ApiVersion::new(1, 99)), ServiceType::Compute);
            let sep = make_service_endpoint(Some("2.1"), Some("2.60"));
            let mut req = http::Request::builder();
            set_request_microversion_header::<FakeOpenStackClient, _>(&mut req, &sep, &ep).unwrap();
            assert_eq!(header(&req), Some("compute 2.1".into()));
        }

        #[test]
        fn major_version_above_cloud_max_fails() {
            let ep = make_ep(Some(ApiVersion::new(3, 0)), ServiceType::Compute);
            let sep = make_service_endpoint(Some("2.1"), Some("2.60"));
            let mut req = http::Request::builder();
            let err =
                set_request_microversion_header::<FakeOpenStackClient, _>(&mut req, &sep, &ep)
                    .unwrap_err();
            assert!(matches!(err, ApiError::MicroversionIncompatible { .. }));
        }

        // ── unversioned / zero-version endpoints ─────────────────────

        #[test]
        fn unversioned_skips_header() {
            let ep = make_ep(None, ServiceType::Compute);
            let sep = make_service_endpoint(Some("2.1"), Some("2.60"));
            let mut req = http::Request::builder();
            set_request_microversion_header::<FakeOpenStackClient, _>(&mut req, &sep, &ep).unwrap();
            assert!(header(&req).is_none());
        }

        #[test]
        fn major_zero_skips_header() {
            let ep = make_ep(Some(ApiVersion::new(0, 0)), ServiceType::Compute);
            let sep = make_service_endpoint(Some("2.1"), Some("2.60"));
            let mut req = http::Request::builder();
            set_request_microversion_header::<FakeOpenStackClient, _>(&mut req, &sep, &ep).unwrap();
            assert!(header(&req).is_none());
        }

        #[test]
        fn minor_zero_valid() {
            let ep = make_ep(Some(ApiVersion::new(2, 0)), ServiceType::Compute);
            let sep = make_service_endpoint(Some("1.0"), Some("2.60"));
            let mut req = http::Request::builder();
            set_request_microversion_header::<FakeOpenStackClient, _>(&mut req, &sep, &ep).unwrap();
            assert_eq!(header(&req), Some("compute 2.0".into()));
        }

        // ── cloud range version string formats ───────────────────────

        #[test]
        fn cloud_min_major_only_parsing() {
            let ep = make_ep(Some(ApiVersion::new(2, 50)), ServiceType::Compute);
            let sep = make_service_endpoint(Some("2"), Some("3"));
            let mut req = http::Request::builder();
            set_request_microversion_header::<FakeOpenStackClient, _>(&mut req, &sep, &ep).unwrap();
            assert_eq!(header(&req), Some("compute 2.50".into()));
        }

        #[test]
        fn malformed_cloud_range_parses_gracefully() {
            let ep = make_ep(Some(ApiVersion::new(2, 50)), ServiceType::Compute);
            let sep = make_service_endpoint(Some("abc"), Some("def"));
            let mut req = http::Request::builder();
            set_request_microversion_header::<FakeOpenStackClient, _>(&mut req, &sep, &ep).unwrap();
            assert_eq!(header(&req), Some("compute 2.50".into()));
        }

        #[test]
        fn malformed_cloud_min_silently_ignored() {
            let ep = make_ep(Some(ApiVersion::new(2, 50)), ServiceType::Compute);
            let sep = make_service_endpoint(Some("bad"), Some("2.99"));
            let mut req = http::Request::builder();
            set_request_microversion_header::<FakeOpenStackClient, _>(&mut req, &sep, &ep).unwrap();
            assert_eq!(header(&req), Some("compute 2.50".into()));
        }

        // ── error variant contents ───────────────────────────────────

        #[test]
        fn error_contains_required_version_and_cloud_range() {
            let ep = make_ep(Some(ApiVersion::new(2, 99)), ServiceType::Compute);
            let sep = make_service_endpoint(Some("2.1"), Some("2.60"));
            let mut req = http::Request::builder();
            let err =
                set_request_microversion_header::<FakeOpenStackClient, _>(&mut req, &sep, &ep)
                    .unwrap_err();
            match err {
                ApiError::MicroversionIncompatible {
                    required_major,
                    required_minor,
                    cloud_min,
                    cloud_max,
                } => {
                    assert_eq!(required_major, 2);
                    assert_eq!(required_minor, 99);
                    assert_eq!(cloud_min, Some("2.1".into()));
                    assert_eq!(cloud_max, Some("2.60".into()));
                }
                other => panic!("expected MicroversionIncompatible, got {other:?}"),
            }
        }

        #[test]
        fn error_display_shows_readable_range() {
            let ep = make_ep(Some(ApiVersion::new(2, 99)), ServiceType::Compute);
            let sep = make_service_endpoint(Some("2.1"), Some("2.60"));
            let mut req = http::Request::builder();
            let err =
                set_request_microversion_header::<FakeOpenStackClient, _>(&mut req, &sep, &ep)
                    .unwrap_err();
            let msg = err.to_string();
            assert!(
                msg.contains("2.99"),
                "error message should contain required version, got: {msg}"
            );
            assert!(
                msg.contains("2.1..=2.60"),
                "error message should contain cloud range, got: {msg}"
            );
        }

        // ── endpoint max_version (bounded variant) ────────────────────

        #[test]
        fn bounded_variant_within_cloud_range_uses_cloud_min() {
            // Endpoint valid for [2.0, 2.75]; cloud floor is 2.30 — send 2.30.
            let ep = make_ep_bounded(
                Some(ApiVersion::new(2, 0)),
                Some(ApiVersion::new(2, 75)),
                ServiceType::Compute,
            );
            let sep = make_service_endpoint(Some("2.30"), Some("2.90"));
            let mut req = http::Request::builder();
            set_request_microversion_header::<FakeOpenStackClient, _>(&mut req, &sep, &ep).unwrap();
            assert_eq!(header(&req), Some("compute 2.30".into()));
        }

        #[test]
        fn bounded_variant_obsoleted_by_cloud_floor_fails() {
            // Endpoint only valid up to 2.75; cloud's floor already moved past
            // it to 2.80 — a newer variant should have been picked instead.
            let ep = make_ep_bounded(
                Some(ApiVersion::new(2, 0)),
                Some(ApiVersion::new(2, 75)),
                ServiceType::Compute,
            );
            let sep = make_service_endpoint(Some("2.80"), Some("2.90"));
            let mut req = http::Request::builder();
            let err =
                set_request_microversion_header::<FakeOpenStackClient, _>(&mut req, &sep, &ep)
                    .unwrap_err();
            assert!(matches!(err, ApiError::MicroversionIncompatible { .. }));
        }

        #[test]
        fn bounded_variant_exact_ceiling_equals_cloud_floor_ok() {
            // Boundary: endpoint max == cloud min is still compatible.
            let ep = make_ep_bounded(
                Some(ApiVersion::new(2, 0)),
                Some(ApiVersion::new(2, 75)),
                ServiceType::Compute,
            );
            let sep = make_service_endpoint(Some("2.75"), Some("2.90"));
            let mut req = http::Request::builder();
            set_request_microversion_header::<FakeOpenStackClient, _>(&mut req, &sep, &ep).unwrap();
            assert_eq!(header(&req), Some("compute 2.75".into()));
        }

        #[test]
        fn bounded_variant_without_cloud_min_skips_ceiling_check() {
            // No cloud_min reported at all — nothing to compare max_version
            // against, so it's not validated (only cloud_max still applies).
            let ep = make_ep_bounded(
                Some(ApiVersion::new(2, 0)),
                Some(ApiVersion::new(2, 75)),
                ServiceType::Compute,
            );
            let sep = make_service_endpoint(None, Some("2.90"));
            let mut req = http::Request::builder();
            set_request_microversion_header::<FakeOpenStackClient, _>(&mut req, &sep, &ep).unwrap();
            assert_eq!(header(&req), Some("compute 2.0".into()));
        }

        #[test]
        fn unbounded_variant_ignores_ceiling_check() {
            // max_version() unset (default None) — no ceiling to violate,
            // regardless of how high the cloud's floor is.
            let ep = make_ep(Some(ApiVersion::new(2, 0)), ServiceType::Compute);
            let sep = make_service_endpoint(Some("2.95"), Some("2.99"));
            let mut req = http::Request::builder();
            set_request_microversion_header::<FakeOpenStackClient, _>(&mut req, &sep, &ep).unwrap();
            assert_eq!(header(&req), Some("compute 2.95".into()));
        }

        // ── negotiate_microversion (standalone, no request/header) ────

        #[test]
        fn negotiate_clamps_up_to_cloud_min() {
            let ep = make_ep(Some(ApiVersion::new(2, 5)), ServiceType::Compute);
            let sep = make_service_endpoint(Some("2.10"), Some("2.60"));
            let negotiated = negotiate_microversion::<FakeOpenStackClient, _>(&sep, &ep).unwrap();
            assert_eq!(negotiated, Some(ApiVersion::new(2, 10)));
        }

        #[test]
        fn negotiate_bounded_variant_obsoleted_by_cloud_floor_fails() {
            let ep = make_ep_bounded(
                Some(ApiVersion::new(2, 0)),
                Some(ApiVersion::new(2, 75)),
                ServiceType::Compute,
            );
            let sep = make_service_endpoint(Some("2.80"), Some("2.90"));
            let err = negotiate_microversion::<FakeOpenStackClient, _>(&sep, &ep).unwrap_err();
            assert!(matches!(err, ApiError::MicroversionIncompatible { .. }));
        }

        #[test]
        fn negotiate_unversioned_returns_none() {
            let ep = make_ep(None, ServiceType::Compute);
            let sep = make_service_endpoint(Some("2.1"), Some("2.60"));
            let negotiated = negotiate_microversion::<FakeOpenStackClient, _>(&sep, &ep).unwrap();
            assert_eq!(negotiated, None);
        }
    }
}
