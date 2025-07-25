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

//! Return a report of usage information for resources associated with the
//! project identified by project_id and user identified by user_id. The value
//! is a dictionary of resource classes paired with the sum of the allocations
//! of that resource class for provided parameters.
//!
//! Normal Response Codes: 200
//!
//! Error response codes: badRequest(400)
//!
use derive_builder::Builder;
use http::{HeaderMap, HeaderName, HeaderValue};

use crate::api::rest_endpoint_prelude::*;

use std::borrow::Cow;

#[derive(Builder, Debug, Clone)]
#[builder(setter(strip_option))]
pub struct Request<'a> {
    /// A string that consists of numbers, A-Z, and _ describing the consumer
    /// type by which to filter usage results. For example, to retrieve only
    /// usage information for ‘INSTANCE’ type consumers a parameter of
    /// consumer_type=INSTANCE should be provided. The all query parameter may
    /// be specified to group all results under one key, all. The unknown query
    /// parameter may be specified to group all results under one key, unknown.
    #[builder(default, setter(into))]
    consumer_type: Option<Cow<'a, str>>,

    /// The uuid of a project.
    #[builder(default, setter(into))]
    project_id: Cow<'a, str>,

    /// The uuid of a user.
    #[builder(default, setter(into))]
    user_id: Option<Cow<'a, str>>,

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
    /// Add a single header to the Usage.
    pub fn header<K, V>(&mut self, header_name: K, header_value: V) -> &mut Self
    where
        K: Into<HeaderName>,
        V: Into<HeaderValue>,
    {
        self._headers
            .get_or_insert(None)
            .get_or_insert_with(HeaderMap::new)
            .insert(header_name.into(), header_value.into());
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
        http::Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        "usages".to_string().into()
    }

    fn parameters(&self) -> QueryParams {
        let mut params = QueryParams::default();
        params.push_opt("consumer_type", self.consumer_type.as_ref());
        params.push("project_id", self.project_id.as_ref());
        params.push_opt("user_id", self.user_id.as_ref());

        params
    }

    fn service_type(&self) -> ServiceType {
        ServiceType::Placement
    }

    fn response_key(&self) -> Option<Cow<'static, str>> {
        Some("usages".into())
    }

    /// Returns headers to be set into the request
    fn request_headers(&self) -> Option<&HeaderMap> {
        self._headers.as_ref()
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
            Request::builder().build().unwrap().service_type(),
            ServiceType::Placement
        );
    }

    #[test]
    fn test_response_key() {
        assert_eq!(
            Request::builder().build().unwrap().response_key().unwrap(),
            "usages"
        );
    }

    #[cfg(feature = "sync")]
    #[test]
    fn endpoint() {
        let server = MockServer::start();
        let client = FakeOpenStackClient::new(server.base_url());
        let mock = server.mock(|when, then| {
            when.method(httpmock::Method::GET)
                .path("/usages".to_string());

            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({ "usages": {} }));
        });

        let endpoint = Request::builder().build().unwrap();
        let _: serde_json::Value = endpoint.query(&client).unwrap();
        mock.assert();
    }

    #[cfg(feature = "sync")]
    #[test]
    fn endpoint_headers() {
        let server = MockServer::start();
        let client = FakeOpenStackClient::new(server.base_url());
        let mock = server.mock(|when, then| {
            when.method(httpmock::Method::GET)
                .path("/usages".to_string())
                .header("foo", "bar")
                .header("not_foo", "not_bar");
            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({ "usages": {} }));
        });

        let endpoint = Request::builder()
            .headers(
                [(
                    Some(HeaderName::from_static("foo")),
                    HeaderValue::from_static("bar"),
                )]
                .into_iter(),
            )
            .header(
                HeaderName::from_static("not_foo"),
                HeaderValue::from_static("not_bar"),
            )
            .build()
            .unwrap();
        let _: serde_json::Value = endpoint.query(&client).unwrap();
        mock.assert();
    }
}
