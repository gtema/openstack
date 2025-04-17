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

//! Create or update one or more allocation records representing the
//! consumption of one or more classes of resources from one or more resource
//! providers by the consumer identified by {consumer_uuid}. If allocations
//! already exist for this consumer, they are replaced.
//!
//! Normal Response Codes: 204
//!
//! Error response codes: badRequest(400), itemNotFound(404), conflict(409)
//!
use derive_builder::Builder;
use http::{HeaderMap, HeaderName, HeaderValue};

use crate::api::rest_endpoint_prelude::*;

use serde::Deserialize;
use serde::Serialize;
use std::borrow::Cow;
use std::collections::BTreeMap;

#[derive(Builder, Debug, Deserialize, Clone, Serialize)]
#[builder(setter(strip_option))]
pub struct ResourceProvider<'a> {
    #[serde()]
    #[builder(setter(into))]
    pub(crate) uuid: Cow<'a, str>,
}

#[derive(Builder, Debug, Deserialize, Clone, Serialize)]
#[builder(setter(strip_option))]
pub struct Allocations<'a> {
    #[serde()]
    #[builder(setter(into))]
    pub(crate) resource_provider: ResourceProvider<'a>,

    #[serde()]
    #[builder(private, setter(into, name = "_resources"))]
    pub(crate) resources: BTreeMap<Cow<'a, str>, i32>,
}

impl<'a> AllocationsBuilder<'a> {
    pub fn resources<I, K, V>(&mut self, iter: I) -> &mut Self
    where
        I: Iterator<Item = (K, V)>,
        K: Into<Cow<'a, str>>,
        V: Into<i32>,
    {
        self.resources
            .get_or_insert_with(BTreeMap::new)
            .extend(iter.map(|(k, v)| (k.into(), v.into())));
        self
    }
}

#[derive(Builder, Debug, Clone)]
#[builder(setter(strip_option))]
pub struct Request<'a> {
    #[builder(setter(into))]
    pub(crate) allocations: Vec<Allocations<'a>>,

    #[builder(setter(into))]
    pub(crate) project_id: Cow<'a, str>,

    #[builder(setter(into))]
    pub(crate) user_id: Cow<'a, str>,

    /// consumer_uuid parameter for /allocations/{consumer_uuid} API
    #[builder(default, setter(into))]
    consumer_uuid: Cow<'a, str>,

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
    /// Add a single header to the Allocation.
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
        http::Method::PUT
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!(
            "allocations/{consumer_uuid}",
            consumer_uuid = self.consumer_uuid.as_ref(),
        )
        .into()
    }

    fn parameters(&self) -> QueryParams {
        QueryParams::default()
    }

    fn body(&self) -> Result<Option<(&'static str, Vec<u8>)>, BodyError> {
        let mut params = JsonBodyParams::default();

        params.push("allocations", serde_json::to_value(&self.allocations)?);
        params.push("project_id", serde_json::to_value(&self.project_id)?);
        params.push("user_id", serde_json::to_value(&self.user_id)?);

        params.into_body()
    }

    fn service_type(&self) -> ServiceType {
        ServiceType::Placement
    }

    fn response_key(&self) -> Option<Cow<'static, str>> {
        None
    }

    /// Returns headers to be set into the request
    fn request_headers(&self) -> Option<&HeaderMap> {
        self._headers.as_ref()
    }

    /// Returns required API version
    fn api_version(&self) -> Option<ApiVersion> {
        Some(ApiVersion::new(1, 8))
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
                .allocations(Vec::from([AllocationsBuilder::default()
                    .resource_provider(
                        ResourceProviderBuilder::default()
                            .uuid("foo")
                            .build()
                            .unwrap()
                    )
                    .resources(BTreeMap::<String, i32>::new().into_iter())
                    .build()
                    .unwrap()]))
                .project_id("foo")
                .user_id("foo")
                .build()
                .unwrap()
                .service_type(),
            ServiceType::Placement
        );
    }

    #[test]
    fn test_response_key() {
        assert!(Request::builder()
            .allocations(Vec::from([AllocationsBuilder::default()
                .resource_provider(
                    ResourceProviderBuilder::default()
                        .uuid("foo")
                        .build()
                        .unwrap()
                )
                .resources(BTreeMap::<String, i32>::new().into_iter())
                .build()
                .unwrap()]))
            .project_id("foo")
            .user_id("foo")
            .build()
            .unwrap()
            .response_key()
            .is_none())
    }

    #[cfg(feature = "sync")]
    #[test]
    fn endpoint() {
        let server = MockServer::start();
        let client = FakeOpenStackClient::new(server.base_url());
        let mock = server.mock(|when, then| {
            when.method(httpmock::Method::PUT).path(format!(
                "/allocations/{consumer_uuid}",
                consumer_uuid = "consumer_uuid",
            ));

            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({ "dummy": {} }));
        });

        let endpoint = Request::builder()
            .consumer_uuid("consumer_uuid")
            .allocations(Vec::from([AllocationsBuilder::default()
                .resource_provider(
                    ResourceProviderBuilder::default()
                        .uuid("foo")
                        .build()
                        .unwrap(),
                )
                .resources(BTreeMap::<String, i32>::new().into_iter())
                .build()
                .unwrap()]))
            .project_id("foo")
            .user_id("foo")
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
            when.method(httpmock::Method::PUT)
                .path(format!(
                    "/allocations/{consumer_uuid}",
                    consumer_uuid = "consumer_uuid",
                ))
                .header("foo", "bar")
                .header("not_foo", "not_bar");
            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({ "dummy": {} }));
        });

        let endpoint = Request::builder()
            .consumer_uuid("consumer_uuid")
            .allocations(Vec::from([AllocationsBuilder::default()
                .resource_provider(
                    ResourceProviderBuilder::default()
                        .uuid("foo")
                        .build()
                        .unwrap(),
                )
                .resources(BTreeMap::<String, i32>::new().into_iter())
                .build()
                .unwrap()]))
            .project_id("foo")
            .user_id("foo")
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
