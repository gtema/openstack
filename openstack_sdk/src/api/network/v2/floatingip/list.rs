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

//! Lists floating IPs visible to the user.
//!
//! Default policy settings return only the floating IPs owned by the user’s
//! project, unless the user has admin role.
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
//! This example request lists floating IPs in JSON format:
//!
//! Normal response codes: 200
//!
//! Error response codes: 401
//!
use derive_builder::Builder;
use http::{HeaderMap, HeaderName, HeaderValue};

use crate::api::rest_endpoint_prelude::*;

use crate::api::common::CommaSeparatedList;
use std::borrow::Cow;

use crate::api::Pageable;
#[derive(Builder, Debug, Clone)]
#[builder(setter(strip_option))]
pub struct Request<'a> {
    /// description query parameter for /v2.0/floatingips API
    ///
    #[builder(default, setter(into))]
    description: Option<Cow<'a, str>>,

    /// fixed_ip_address query parameter for /v2.0/floatingips API
    ///
    #[builder(default, setter(into))]
    fixed_ip_address: Option<Cow<'a, str>>,

    /// floating_ip_address query parameter for /v2.0/floatingips API
    ///
    #[builder(default, setter(into))]
    floating_ip_address: Option<Cow<'a, str>>,

    /// floating_network_id query parameter for /v2.0/floatingips API
    ///
    #[builder(default, setter(into))]
    floating_network_id: Option<Cow<'a, str>>,

    /// id query parameter for /v2.0/floatingips API
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

    /// not-tags query parameter for /v2.0/floatingips API
    ///
    #[builder(default, private, setter(name = "_not_tags"))]
    not_tags: Option<CommaSeparatedList<Cow<'a, str>>>,

    /// not-tags-any query parameter for /v2.0/floatingips API
    ///
    #[builder(default, private, setter(name = "_not_tags_any"))]
    not_tags_any: Option<CommaSeparatedList<Cow<'a, str>>>,

    /// Reverse the page direction
    ///
    #[builder(default)]
    page_reverse: Option<bool>,

    /// port_id query parameter for /v2.0/floatingips API
    ///
    #[builder(default, setter(into))]
    port_id: Option<Cow<'a, str>>,

    /// revision_number query parameter for /v2.0/floatingips API
    ///
    #[builder(default, setter(into))]
    revision_number: Option<Cow<'a, str>>,

    /// router_id query parameter for /v2.0/floatingips API
    ///
    #[builder(default, setter(into))]
    router_id: Option<Cow<'a, str>>,

    /// Sort direction. This is an optional feature and may be silently ignored
    /// by the server.
    ///
    #[builder(default, private, setter(name = "_sort_dir"))]
    sort_dir: Option<Vec<Cow<'a, str>>>,

    /// Sort results by the attribute. This is an optional feature and may be
    /// silently ignored by the server.
    ///
    #[builder(default, private, setter(name = "_sort_key"))]
    sort_key: Option<Vec<Cow<'a, str>>>,

    /// status query parameter for /v2.0/floatingips API
    ///
    #[builder(default, setter(into))]
    status: Option<Cow<'a, str>>,

    /// tags query parameter for /v2.0/floatingips API
    ///
    #[builder(default, private, setter(name = "_tags"))]
    tags: Option<CommaSeparatedList<Cow<'a, str>>>,

    /// tags-any query parameter for /v2.0/floatingips API
    ///
    #[builder(default, private, setter(name = "_tags_any"))]
    tags_any: Option<CommaSeparatedList<Cow<'a, str>>>,

    /// tenant_id query parameter for /v2.0/floatingips API
    ///
    #[builder(default, setter(into))]
    tenant_id: Option<Cow<'a, str>>,

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
    /// tags query parameter for /v2.0/floatingips API
    ///
    pub fn tags<I, T>(&mut self, iter: I) -> &mut Self
    where
        I: Iterator<Item = T>,
        T: Into<Cow<'a, str>>,
    {
        self.tags
            .get_or_insert(None)
            .get_or_insert_with(CommaSeparatedList::new)
            .extend(iter.map(Into::into));
        self
    }

    /// tags-any query parameter for /v2.0/floatingips API
    ///
    pub fn tags_any<I, T>(&mut self, iter: I) -> &mut Self
    where
        I: Iterator<Item = T>,
        T: Into<Cow<'a, str>>,
    {
        self.tags_any
            .get_or_insert(None)
            .get_or_insert_with(CommaSeparatedList::new)
            .extend(iter.map(Into::into));
        self
    }

    /// not-tags query parameter for /v2.0/floatingips API
    ///
    pub fn not_tags<I, T>(&mut self, iter: I) -> &mut Self
    where
        I: Iterator<Item = T>,
        T: Into<Cow<'a, str>>,
    {
        self.not_tags
            .get_or_insert(None)
            .get_or_insert_with(CommaSeparatedList::new)
            .extend(iter.map(Into::into));
        self
    }

    /// not-tags-any query parameter for /v2.0/floatingips API
    ///
    pub fn not_tags_any<I, T>(&mut self, iter: I) -> &mut Self
    where
        I: Iterator<Item = T>,
        T: Into<Cow<'a, str>>,
    {
        self.not_tags_any
            .get_or_insert(None)
            .get_or_insert_with(CommaSeparatedList::new)
            .extend(iter.map(Into::into));
        self
    }

    /// Sort results by the attribute. This is an optional feature and may be
    /// silently ignored by the server.
    ///
    pub fn sort_key<I, T>(&mut self, iter: I) -> &mut Self
    where
        I: Iterator<Item = T>,
        T: Into<Cow<'a, str>>,
    {
        self.sort_key
            .get_or_insert(None)
            .get_or_insert_with(Vec::new)
            .extend(iter.map(Into::into));
        self
    }

    /// Sort direction. This is an optional feature and may be silently ignored
    /// by the server.
    ///
    pub fn sort_dir<I, T>(&mut self, iter: I) -> &mut Self
    where
        I: Iterator<Item = T>,
        T: Into<Cow<'a, str>>,
    {
        self.sort_dir
            .get_or_insert(None)
            .get_or_insert_with(Vec::new)
            .extend(iter.map(Into::into));
        self
    }

    /// Add a single header to the Floatingip.
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
        "floatingips".to_string().into()
    }

    fn parameters(&self) -> QueryParams {
        let mut params = QueryParams::default();
        params.push_opt("id", self.id.as_ref());
        params.push_opt("floating_ip_address", self.floating_ip_address.as_ref());
        params.push_opt("floating_network_id", self.floating_network_id.as_ref());
        params.push_opt("router_id", self.router_id.as_ref());
        params.push_opt("port_id", self.port_id.as_ref());
        params.push_opt("fixed_ip_address", self.fixed_ip_address.as_ref());
        params.push_opt("tenant_id", self.tenant_id.as_ref());
        params.push_opt("status", self.status.as_ref());
        params.push_opt("revision_number", self.revision_number.as_ref());
        params.push_opt("tags", self.tags.as_ref());
        params.push_opt("tags-any", self.tags_any.as_ref());
        params.push_opt("not-tags", self.not_tags.as_ref());
        params.push_opt("not-tags-any", self.not_tags_any.as_ref());
        params.push_opt("description", self.description.as_ref());
        if let Some(val) = &self.sort_key {
            params.extend(val.iter().map(|value| ("sort_key", value)));
        }
        if let Some(val) = &self.sort_dir {
            params.extend(val.iter().map(|value| ("sort_dir", value)));
        }
        params.push_opt("limit", self.limit);
        params.push_opt("marker", self.marker.as_ref());
        params.push_opt("page_reverse", self.page_reverse);

        params
    }

    fn service_type(&self) -> ServiceType {
        ServiceType::Network
    }

    fn response_key(&self) -> Option<Cow<'static, str>> {
        Some("floatingips".into())
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
            ServiceType::Network
        );
    }

    #[test]
    fn test_response_key() {
        assert_eq!(
            Request::builder().build().unwrap().response_key().unwrap(),
            "floatingips"
        );
    }

    #[cfg(feature = "sync")]
    #[test]
    fn endpoint() {
        let server = MockServer::start();
        let client = FakeOpenStackClient::new(server.base_url());
        let mock = server.mock(|when, then| {
            when.method(httpmock::Method::GET)
                .path("/floatingips".to_string());

            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({ "floatingips": {} }));
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
                .path("/floatingips".to_string())
                .header("foo", "bar")
                .header("not_foo", "not_bar");
            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({ "floatingips": {} }));
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
