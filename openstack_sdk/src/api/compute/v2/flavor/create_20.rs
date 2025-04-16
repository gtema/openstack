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
//
// WARNING: This file is automatically generated from OpenAPI schema using
// `openstack-codegenerator`.

//! Creates a flavor.
//!
//! Creating a flavor is typically only available to administrators of a cloud
//! because this has implications for scheduling efficiently in the cloud.
//!
//! Normal response codes: 200
//!
//! Error response codes: badRequest(400), unauthorized(401), forbidden(403),
//! conflict(409)
//!
use derive_builder::Builder;
use http::{HeaderMap, HeaderName, HeaderValue};

use crate::api::rest_endpoint_prelude::*;

use serde::Deserialize;
use serde::Serialize;
use std::borrow::Cow;

/// The ID and links for the flavor for your server instance. A flavor is a
/// combination of memory, disk size, and CPUs.
#[derive(Builder, Debug, Deserialize, Clone, Serialize)]
#[builder(setter(strip_option))]
pub struct Flavor<'a> {
    /// The size of a dedicated swap disk that will be allocated, in MiB. If 0
    /// (the default), no dedicated swap disk will be created.
    #[serde()]
    #[builder(setter(into))]
    pub(crate) disk: i32,

    /// Only alphanumeric characters with hyphen ‘-’, underscore ‘\_’, spaces
    /// and dots ‘.’ are permitted. If an ID is not provided, then a default
    /// UUID will be assigned.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) id: Option<Option<Cow<'a, str>>>,

    /// The display name of a flavor.
    #[serde()]
    #[builder(setter(into))]
    pub(crate) name: Cow<'a, str>,

    /// Whether the flavor is public (available to all projects) or scoped to a
    /// set of projects. Default is True if not specified.
    #[serde(
        rename = "os-flavor-access:is_public",
        skip_serializing_if = "Option::is_none"
    )]
    #[builder(default, setter(into))]
    pub(crate) os_flavor_access_is_public: Option<bool>,

    /// The size of a dedicated swap disk that will be allocated, in MiB. If 0
    /// (the default), no dedicated swap disk will be created.
    #[serde(
        rename = "OS-FLV-EXT-DATA:ephemeral",
        skip_serializing_if = "Option::is_none"
    )]
    #[builder(default, setter(into))]
    pub(crate) os_flv_ext_data_ephemeral: Option<i32>,

    /// The number of virtual CPUs that will be allocated to the server.
    #[serde()]
    #[builder(setter(into))]
    pub(crate) ram: i32,

    /// The receive / transmit factor (as a float) that will be set on ports if
    /// the network backend supports the QOS extension. Otherwise it will be
    /// ignored. It defaults to 1.0.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) rxtx_factor: Option<Cow<'a, str>>,

    /// The size of a dedicated swap disk that will be allocated, in MiB. If 0
    /// (the default), no dedicated swap disk will be created.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) swap: Option<i32>,

    /// The number of virtual CPUs that will be allocated to the server.
    #[serde()]
    #[builder(setter(into))]
    pub(crate) vcpus: i32,
}

#[derive(Builder, Debug, Clone)]
#[builder(setter(strip_option))]
pub struct Request<'a> {
    /// The ID and links for the flavor for your server instance. A flavor is a
    /// combination of memory, disk size, and CPUs.
    #[builder(setter(into))]
    pub(crate) flavor: Flavor<'a>,

    #[builder(setter(name = "_headers"), default, private)]
    _headers: Option<HeaderMap>,
}
impl<'a> Request<'a> {
    /// Create a builder for the endpoint.
    pub fn builder() -> RequestBuilder<'a> {
        RequestBuilder::default()
    }
}

impl RequestBuilder<'_> {
    /// Add a single header to the Flavor.
    pub fn header(&mut self, header_name: &'static str, header_value: &'static str) -> &mut Self
where {
        self._headers
            .get_or_insert(None)
            .get_or_insert_with(HeaderMap::new)
            .insert(header_name, HeaderValue::from_static(header_value));
        self
    }

    /// Add multiple headers.
    pub fn headers<I, T>(&mut self, iter: I) -> &mut Self
    where
        I: Iterator<Item = T>,
        T: Into<(Option<HeaderName>, HeaderValue)>,
    {
        self._headers
            .get_or_insert(None)
            .get_or_insert_with(HeaderMap::new)
            .extend(iter.map(Into::into));
        self
    }
}

impl RestEndpoint for Request<'_> {
    fn method(&self) -> http::Method {
        http::Method::POST
    }

    fn endpoint(&self) -> Cow<'static, str> {
        "flavors".to_string().into()
    }

    fn parameters(&self) -> QueryParams {
        QueryParams::default()
    }

    fn body(&self) -> Result<Option<(&'static str, Vec<u8>)>, BodyError> {
        let mut params = JsonBodyParams::default();

        params.push("flavor", serde_json::to_value(&self.flavor)?);

        params.into_body()
    }

    fn service_type(&self) -> ServiceType {
        ServiceType::Compute
    }

    fn response_key(&self) -> Option<Cow<'static, str>> {
        Some("flavor".into())
    }

    /// Returns headers to be set into the request
    fn request_headers(&self) -> Option<&HeaderMap> {
        self._headers.as_ref()
    }

    /// Returns required API version
    fn api_version(&self) -> Option<ApiVersion> {
        Some(ApiVersion::new(2, 0))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[cfg(feature = "sync")]
    use crate::api::Query;
    use crate::test::client::FakeOpenStackClient;
    use crate::types::ServiceType;
    use http::{HeaderName, HeaderValue};
    use httpmock::MockServer;
    use serde_json::json;

    #[test]
    fn test_service_type() {
        assert_eq!(
            Request::builder()
                .flavor(
                    FlavorBuilder::default()
                        .disk(123)
                        .name("foo")
                        .ram(123)
                        .vcpus(123)
                        .build()
                        .unwrap()
                )
                .build()
                .unwrap()
                .service_type(),
            ServiceType::Compute
        );
    }

    #[test]
    fn test_response_key() {
        assert_eq!(
            Request::builder()
                .flavor(
                    FlavorBuilder::default()
                        .disk(123)
                        .name("foo")
                        .ram(123)
                        .vcpus(123)
                        .build()
                        .unwrap()
                )
                .build()
                .unwrap()
                .response_key()
                .unwrap(),
            "flavor"
        );
    }

    #[cfg(feature = "sync")]
    #[test]
    fn endpoint() {
        let server = MockServer::start();
        let client = FakeOpenStackClient::new(server.base_url());
        let mock = server.mock(|when, then| {
            when.method(httpmock::Method::POST)
                .path("/flavors".to_string());

            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({ "flavor": {} }));
        });

        let endpoint = Request::builder()
            .flavor(
                FlavorBuilder::default()
                    .disk(123)
                    .name("foo")
                    .ram(123)
                    .vcpus(123)
                    .build()
                    .unwrap(),
            )
            .build()
            .unwrap();
        let _: serde_json::Value = endpoint.query(&client).unwrap();
        mock.assert();
    }

    #[cfg(feature = "sync")]
    #[test]
    fn endpoint_headers() {
        let server = MockServer::start();
        let client = FakeOpenStackClient::new(server.base_url());
        let mock = server.mock(|when, then| {
            when.method(httpmock::Method::POST)
                .path("/flavors".to_string())
                .header("foo", "bar")
                .header("not_foo", "not_bar");
            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({ "flavor": {} }));
        });

        let endpoint = Request::builder()
            .flavor(
                FlavorBuilder::default()
                    .disk(123)
                    .name("foo")
                    .ram(123)
                    .vcpus(123)
                    .build()
                    .unwrap(),
            )
            .headers(
                [(
                    Some(HeaderName::from_static("foo")),
                    HeaderValue::from_static("bar"),
                )]
                .into_iter(),
            )
            .header("not_foo", "not_bar")
            .build()
            .unwrap();
        let _: serde_json::Value = endpoint.query(&client).unwrap();
        mock.assert();
    }
}
