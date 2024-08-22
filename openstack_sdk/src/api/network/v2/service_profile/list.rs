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

//! Lists all service profiles visible for the tenant account.
//!
//! The list can be empty.
//!
//! Standard query parameters are supported on the URI. For more information,
//! see [Filtering and Column Selection](#filtering).
//!
//! Use the `fields` query parameter to control which fields are returned in
//! the response body. For more information, see [Fields](#fields).
//!
//! Pagination query parameters are supported if Neutron configuration supports
//! it by overriding `allow_pagination=false`. For more information, see
//! [Pagination](#pagination).
//!
//! Sorting query parameters are supported if Neutron configuration supports it
//! with `allow_sorting=true`. For more information, see [Sorting](#sorting).
//!
//! Normal response codes: 200
//!
//! Error response codes: 401
//!
use derive_builder::Builder;
use http::{HeaderMap, HeaderName, HeaderValue};

use crate::api::rest_endpoint_prelude::*;

use std::borrow::Cow;

use crate::api::Pageable;
#[derive(Builder, Debug, Clone)]
#[builder(setter(strip_option))]
pub struct Request<'a> {
    /// description query parameter for /v2.0/service_profiles API
    ///
    #[builder(default, setter(into))]
    description: Option<Cow<'a, str>>,

    /// driver query parameter for /v2.0/service_profiles API
    ///
    #[builder(default, setter(into))]
    driver: Option<Cow<'a, str>>,

    /// enabled query parameter for /v2.0/service_profiles API
    ///
    #[builder(default)]
    enabled: Option<bool>,

    /// id query parameter for /v2.0/service_profiles API
    ///
    #[builder(default, setter(into))]
    id: Option<Cow<'a, str>>,

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
    marker: Option<Cow<'a, str>>,

    /// Reverse the page direction
    ///
    #[builder(default)]
    page_reverse: Option<bool>,

    /// Sort direction. This is an optional feature and may be silently ignored
    /// by the server.
    ///
    #[builder(default, setter(into))]
    sort_dir: Option<Cow<'a, str>>,

    /// Sort results by the attribute. This is an optional feature and may be
    /// silently ignored by the server.
    ///
    #[builder(default, setter(into))]
    sort_key: Option<Cow<'a, str>>,

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
    /// Add a single header to the Service_Profile.
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
        http::Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        "service_profiles".to_string().into()
    }

    fn parameters(&self) -> QueryParams {
        let mut params = QueryParams::default();
        params.push_opt("id", self.id.as_ref());
        params.push_opt("description", self.description.as_ref());
        params.push_opt("driver", self.driver.as_ref());
        params.push_opt("enabled", self.enabled);
        params.push_opt("sort_key", self.sort_key.as_ref());
        params.push_opt("sort_dir", self.sort_dir.as_ref());
        params.push_opt("limit", self.limit);
        params.push_opt("marker", self.marker.as_ref());
        params.push_opt("page_reverse", self.page_reverse);

        params
    }

    fn service_type(&self) -> ServiceType {
        ServiceType::Network
    }

    fn response_key(&self) -> Option<Cow<'static, str>> {
        Some("service_profiles".into())
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
impl<'a> Pageable for Request<'a> {}

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
            ServiceType::Network
        );
    }

    #[test]
    fn test_response_key() {
        assert_eq!(
            Request::builder().build().unwrap().response_key().unwrap(),
            "service_profiles"
        );
    }

    #[cfg(feature = "sync")]
    #[test]
    fn endpoint() {
        let client = MockServerClient::new();
        let mock = client.server.mock(|when, then| {
            when.method(httpmock::Method::GET)
                .path("/service_profiles".to_string());

            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({ "service_profiles": {} }));
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
                .path("/service_profiles".to_string())
                .header("foo", "bar")
                .header("not_foo", "not_bar");
            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({ "service_profiles": {} }));
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
