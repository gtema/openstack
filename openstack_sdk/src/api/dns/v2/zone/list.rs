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

//! List all zones
//!
use derive_builder::Builder;
use http::{HeaderMap, HeaderName, HeaderValue};

use crate::api::rest_endpoint_prelude::*;

use std::borrow::Cow;

use crate::api::Pageable;
#[derive(Builder, Debug, Clone)]
#[builder(setter(strip_option))]
pub struct Request<'a> {
    /// Filter results to only show zones that have a type matching the filter
    ///
    #[builder(default, setter(into))]
    _type: Option<Cow<'a, str>>,

    /// Filter results to only show zones that have a description matching the
    /// filter
    ///
    #[builder(default, setter(into))]
    description: Option<Cow<'a, str>>,

    /// Filter results to only show zones that have an email matching the
    /// filter
    ///
    #[builder(default, setter(into))]
    email: Option<Cow<'a, str>>,

    /// Requests a page size of items. Returns a number of items up to a limit
    /// value. Use the limit parameter to make an initial limited request and
    /// use the ID of the last-seen item from the response as the marker
    /// parameter value in a subsequent limited request.
    ///
    #[builder(default)]
    limit: Option<i32>,

    /// The ID of the last-seen item. Use the limit parameter to make an
    /// initial limited request and use the ID of the last-seen item from the
    /// response as the marker parameter value in a subsequent limited request.
    ///
    #[builder(default, setter(into))]
    market: Option<Cow<'a, str>>,

    /// Filter results to only show zones that have a name matching the filter
    ///
    #[builder(default, setter(into))]
    name: Option<Cow<'a, str>>,

    /// Sorts the response by the requested sort direction. A valid value is
    /// asc (ascending) or desc (descending). Default is asc. You can specify
    /// multiple pairs of sort key and sort direction query parameters. If you
    /// omit the sort direction in a pair, the API uses the natural sorting
    /// direction of the server attribute that is provided as the sort_key.
    ///
    #[builder(default, setter(into))]
    sort_dir: Option<Cow<'a, str>>,

    /// Sorts the response by the this attribute value. Default is id. You can
    /// specify multiple pairs of sort key and sort direction query parameters.
    /// If you omit the sort direction in a pair, the API uses the natural
    /// sorting direction of the server attribute that is provided as the
    /// sort_key.
    ///
    #[builder(default, setter(into))]
    sort_key: Option<Cow<'a, str>>,

    /// Filter results to only show zones that have a status matching the
    /// filter
    ///
    #[builder(default, setter(into))]
    status: Option<Cow<'a, str>>,

    /// Filter results to only show zones that have a ttl matching the filter
    ///
    #[builder(default)]
    ttl: Option<i32>,

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
    /// Add a single header to the Zone.
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
        http::Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        "zones".to_string().into()
    }

    fn parameters(&self) -> QueryParams {
        let mut params = QueryParams::default();
        params.push_opt("limit", self.limit);
        params.push_opt("market", self.market.as_ref());
        params.push_opt("sort_dir", self.sort_dir.as_ref());
        params.push_opt("sort_key", self.sort_key.as_ref());
        params.push_opt("name", self.name.as_ref());
        params.push_opt("description", self.description.as_ref());
        params.push_opt("type", self._type.as_ref());
        params.push_opt("email", self.email.as_ref());
        params.push_opt("status", self.status.as_ref());
        params.push_opt("ttl", self.ttl);

        params
    }

    fn service_type(&self) -> ServiceType {
        ServiceType::Dns
    }

    fn response_key(&self) -> Option<Cow<'static, str>> {
        Some("zones".into())
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
impl Pageable for Request<'_> {}

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
            Request::builder().build().unwrap().service_type(),
            ServiceType::Dns
        );
    }

    #[test]
    fn test_response_key() {
        assert_eq!(
            Request::builder().build().unwrap().response_key().unwrap(),
            "zones"
        );
    }

    #[cfg(feature = "sync")]
    #[test]
    fn endpoint() {
        let client = MockServerClient::new();
        let mock = client.server.mock(|when, then| {
            when.method(httpmock::Method::GET)
                .path("/zones".to_string());

            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({ "zones": {} }));
        });

        let endpoint = Request::builder().build().unwrap();
        let _: serde_json::Value = endpoint.query(&client).unwrap();
        mock.assert();
    }

    #[cfg(feature = "sync")]
    #[test]
    fn endpoint_headers() {
        let client = MockServerClient::new();
        let mock = client.server.mock(|when, then| {
            when.method(httpmock::Method::GET)
                .path("/zones".to_string())
                .header("foo", "bar")
                .header("not_foo", "not_bar");
            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({ "zones": {} }));
        });

        let endpoint = Request::builder()
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
