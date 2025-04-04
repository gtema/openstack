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

//! Lists a summary of all OpenStack Networking security group rules that are
//! used for every newly created Security Group.
//!
//! The list provides the ID for each security group default rule.
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
    /// description query parameter for /v2.0/default-security-group-rules API
    ///
    #[builder(default, setter(into))]
    description: Option<Cow<'a, str>>,

    /// direction query parameter for /v2.0/default-security-group-rules API
    ///
    #[builder(default, setter(into))]
    direction: Option<Cow<'a, str>>,

    /// ethertype query parameter for /v2.0/default-security-group-rules API
    ///
    #[builder(default, setter(into))]
    ethertype: Option<Cow<'a, str>>,

    /// id query parameter for /v2.0/default-security-group-rules API
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

    /// port_range_max query parameter for /v2.0/default-security-group-rules
    /// API
    ///
    #[builder(default)]
    port_range_max: Option<i32>,

    /// port_range_min query parameter for /v2.0/default-security-group-rules
    /// API
    ///
    #[builder(default)]
    port_range_min: Option<i32>,

    /// protocol query parameter for /v2.0/default-security-group-rules API
    ///
    #[builder(default, setter(into))]
    protocol: Option<Cow<'a, str>>,

    /// remote_address_group_id query parameter for
    /// /v2.0/default-security-group-rules API
    ///
    #[builder(default, setter(into))]
    remote_address_group_id: Option<Cow<'a, str>>,

    /// Filter the security group rule list result by the ID of the remote
    /// group that associates with this security group rule. This field can
    /// contains uuid of the security group or special word `PARENT` which
    /// means that in the real rule created from this template, uuid of the
    /// owner Security Group will be put as `remote_group_id`.
    ///
    #[builder(default, setter(into))]
    remote_group_id: Option<Cow<'a, str>>,

    /// remote_ip_prefix query parameter for /v2.0/default-security-group-rules
    /// API
    ///
    #[builder(default, setter(into))]
    remote_ip_prefix: Option<Cow<'a, str>>,

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

    /// used_in_default_sg query parameter for
    /// /v2.0/default-security-group-rules API
    ///
    #[builder(default)]
    used_in_default_sg: Option<bool>,

    /// used_in_non_default_sg query parameter for
    /// /v2.0/default-security-group-rules API
    ///
    #[builder(default)]
    used_in_non_default_sg: Option<bool>,

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

    /// Add a single header to the Default_Security_Group_Rule.
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
        "default-security-group-rules".to_string().into()
    }

    fn parameters(&self) -> QueryParams {
        let mut params = QueryParams::default();
        params.push_opt("id", self.id.as_ref());
        params.push_opt("description", self.description.as_ref());
        params.push_opt("remote_group_id", self.remote_group_id.as_ref());
        params.push_opt(
            "remote_address_group_id",
            self.remote_address_group_id.as_ref(),
        );
        params.push_opt("direction", self.direction.as_ref());
        params.push_opt("protocol", self.protocol.as_ref());
        params.push_opt("port_range_min", self.port_range_min);
        params.push_opt("port_range_max", self.port_range_max);
        params.push_opt("ethertype", self.ethertype.as_ref());
        params.push_opt("remote_ip_prefix", self.remote_ip_prefix.as_ref());
        params.push_opt("used_in_default_sg", self.used_in_default_sg);
        params.push_opt("used_in_non_default_sg", self.used_in_non_default_sg);
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
        Some("default_security_group_rules".into())
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
            "default_security_group_rules"
        );
    }

    #[cfg(feature = "sync")]
    #[test]
    fn endpoint() {
        let server = MockServer::start();
        let client = FakeOpenStackClient::new(server.base_url());
        let mock = server.mock(|when, then| {
            when.method(httpmock::Method::GET)
                .path("/default-security-group-rules".to_string());

            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({ "default_security_group_rules": {} }));
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
                .path("/default-security-group-rules".to_string())
                .header("foo", "bar")
                .header("not_foo", "not_bar");
            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({ "default_security_group_rules": {} }));
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
