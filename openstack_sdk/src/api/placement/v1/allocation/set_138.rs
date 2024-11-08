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
pub struct Allocations<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub(crate) generation: Option<i32>,

    #[serde()]
    #[builder(private, setter(name = "_resources"))]
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
    /// A dictionary of resource allocations keyed by resource provider uuid.
    /// If this is an empty object, allocations for this consumer will be
    /// removed.
    ///
    #[builder(private, setter(name = "_allocations"))]
    pub(crate) allocations: BTreeMap<Cow<'a, str>, Allocations<'a>>,

    /// The generation of the consumer. Should be set to `null` when indicating
    /// that the caller expects the consumer does not yet exist.
    ///
    /// **New in version 1.28**
    ///
    #[builder(setter(into))]
    pub(crate) consumer_generation: Option<i32>,

    /// A string that consists of numbers, `A-Z`, and `_` describing what kind
    /// of consumer is creating, or has created, allocations using a quantity
    /// of inventory. The string is determined by the client when writing
    /// allocations and it is up to the client to ensure correct choices
    /// amongst collaborating services. For example, the compute service may
    /// choose to type some consumers ‘INSTANCE’ and others ‘MIGRATION’.
    ///
    /// **New in version 1.38**
    ///
    #[builder(setter(into))]
    pub(crate) consumer_type: Cow<'a, str>,

    /// A dictionary associating request group suffixes with a list of uuids
    /// identifying the resource providers that satisfied each group. The empty
    /// string and `[a-zA-Z0-9_-]+` are valid suffixes. This field may be sent
    /// when writing allocations back to the server but will be ignored; this
    /// preserves symmetry between read and write representations.
    ///
    /// **New in version 1.34**
    ///
    #[builder(default, private, setter(name = "_mappings"))]
    pub(crate) mappings: Option<BTreeMap<Cow<'a, str>, Vec<Cow<'a, str>>>>,

    /// The uuid of a project.
    ///
    #[builder(setter(into))]
    pub(crate) project_id: Cow<'a, str>,

    /// The uuid of a user.
    ///
    #[builder(setter(into))]
    pub(crate) user_id: Cow<'a, str>,

    /// consumer_uuid parameter for /allocations/{consumer_uuid} API
    ///
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

impl<'a> RequestBuilder<'a> {
    /// A dictionary of resource allocations keyed by resource provider uuid.
    /// If this is an empty object, allocations for this consumer will be
    /// removed.
    ///
    pub fn allocations<I, K, V>(&mut self, iter: I) -> &mut Self
    where
        I: Iterator<Item = (K, V)>,
        K: Into<Cow<'a, str>>,
        V: Into<Allocations<'a>>,
    {
        self.allocations
            .get_or_insert_with(BTreeMap::new)
            .extend(iter.map(|(k, v)| (k.into(), v.into())));
        self
    }

    /// A dictionary associating request group suffixes with a list of uuids
    /// identifying the resource providers that satisfied each group. The empty
    /// string and `[a-zA-Z0-9_-]+` are valid suffixes. This field may be sent
    /// when writing allocations back to the server but will be ignored; this
    /// preserves symmetry between read and write representations.
    ///
    /// **New in version 1.34**
    ///
    pub fn mappings<I, K, V>(&mut self, iter: I) -> &mut Self
    where
        I: Iterator<Item = (K, V)>,
        K: Into<Cow<'a, str>>,
        V: Into<Vec<Cow<'a, str>>>,
    {
        self.mappings
            .get_or_insert(None)
            .get_or_insert_with(BTreeMap::new)
            .extend(iter.map(|(k, v)| (k.into(), v.into())));
        self
    }

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

impl<'a> RestEndpoint for Request<'a> {
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
        params.push(
            "consumer_generation",
            serde_json::to_value(self.consumer_generation)?,
        );
        if let Some(val) = &self.mappings {
            params.push("mappings", serde_json::to_value(val)?);
        }
        params.push("consumer_type", serde_json::to_value(&self.consumer_type)?);

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
        Some(ApiVersion::new(1, 38))
    }
}

#[cfg(test)]
mod tests {
    #![allow(unused_imports)]
    use super::*;
    #[cfg(feature = "sync")]
    use crate::api::Query;
    #[cfg(feature = "sync")]
    use crate::test::client::MockServerClient;
    use crate::types::ServiceType;
    use http::{HeaderName, HeaderValue};
    use serde_json::json;

    #[test]
    fn test_service_type() {
        assert_eq!(
            Request::builder()
                .allocations(BTreeMap::<String, Allocations<'_>>::new().into_iter())
                .project_id("foo")
                .user_id("foo")
                .consumer_generation(123)
                .consumer_type("foo")
                .build()
                .unwrap()
                .service_type(),
            ServiceType::Placement
        );
    }

    #[test]
    fn test_response_key() {
        assert!(Request::builder()
            .allocations(BTreeMap::<String, Allocations<'_>>::new().into_iter())
            .project_id("foo")
            .user_id("foo")
            .consumer_generation(123)
            .consumer_type("foo")
            .build()
            .unwrap()
            .response_key()
            .is_none())
    }

    #[cfg(feature = "sync")]
    #[test]
    fn endpoint() {
        let client = MockServerClient::new();
        let mock = client.server.mock(|when, then| {
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
            .allocations(BTreeMap::<String, Allocations<'_>>::new().into_iter())
            .project_id("foo")
            .user_id("foo")
            .consumer_generation(123)
            .consumer_type("foo")
            .build()
            .unwrap();
        let _: serde_json::Value = endpoint.query(&client).unwrap();
        mock.assert();
    }

    #[cfg(feature = "sync")]
    #[test]
    fn endpoint_headers() {
        let client = MockServerClient::new();
        let mock = client.server.mock(|when, then| {
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
            .allocations(BTreeMap::<String, Allocations<'_>>::new().into_iter())
            .project_id("foo")
            .user_id("foo")
            .consumer_generation(123)
            .consumer_type("foo")
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
