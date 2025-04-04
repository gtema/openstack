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

//! Lists all amphora for the project.
//!
//! If you are not an administrative user, the service returns the HTTP
//! `Forbidden (403)` response code.
//!
//! Use the `fields` query parameter to control which fields are returned in
//! the response body. Additionally, you can filter results by using query
//! string parameters. For information, see
//! [Filtering and column selection](#filtering).
//!
//! The list might be empty.
//!
use derive_builder::Builder;
use http::{HeaderMap, HeaderName, HeaderValue};

use crate::api::rest_endpoint_prelude::*;

use std::borrow::Cow;

use crate::api::Pageable;
#[derive(Builder, Debug, Clone)]
#[builder(setter(strip_option))]
pub struct Request<'a> {
    #[builder(default, setter(into))]
    cached_zone: Option<Cow<'a, str>>,

    #[builder(default, setter(into))]
    cert_busy: Option<Cow<'a, str>>,

    #[builder(default, setter(into))]
    cert_expiration: Option<Cow<'a, str>>,

    #[builder(default, setter(into))]
    compute_flavor: Option<Cow<'a, str>>,

    #[builder(default, setter(into))]
    compute_id: Option<Cow<'a, str>>,

    #[builder(default, setter(into))]
    created_at: Option<Cow<'a, str>>,

    #[builder(default, setter(into))]
    ha_ip: Option<Cow<'a, str>>,

    #[builder(default, setter(into))]
    ha_port_id: Option<Cow<'a, str>>,

    #[builder(default, setter(into))]
    id: Option<Cow<'a, str>>,

    #[builder(default, setter(into))]
    image_id: Option<Cow<'a, str>>,

    #[builder(default, setter(into))]
    lb_network_ip: Option<Cow<'a, str>>,

    /// Page size
    ///
    #[builder(default)]
    limit: Option<i32>,

    #[builder(default, setter(into))]
    loadbalancer_id: Option<Cow<'a, str>>,

    /// ID of the last item in the previous list
    ///
    #[builder(default, setter(into))]
    marker: Option<Cow<'a, str>>,

    /// The page direction.
    ///
    #[builder(default)]
    page_reverse: Option<bool>,

    #[builder(default, setter(into))]
    role: Option<Cow<'a, str>>,

    #[builder(default, setter(into))]
    status: Option<Cow<'a, str>>,

    #[builder(default, setter(into))]
    updated_at: Option<Cow<'a, str>>,

    #[builder(default, setter(into))]
    vrrp_id: Option<Cow<'a, str>>,

    #[builder(default, setter(into))]
    vrrp_interface: Option<Cow<'a, str>>,

    #[builder(default, setter(into))]
    vrrp_ip: Option<Cow<'a, str>>,

    #[builder(default, setter(into))]
    vrrp_port_id: Option<Cow<'a, str>>,

    #[builder(default, setter(into))]
    vrrp_priority: Option<Cow<'a, str>>,

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
    /// Add a single header to the Amphorae.
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
        "octavia/amphorae".to_string().into()
    }

    fn parameters(&self) -> QueryParams {
        let mut params = QueryParams::default();
        params.push_opt("cached_zone", self.cached_zone.as_ref());
        params.push_opt("cert_busy", self.cert_busy.as_ref());
        params.push_opt("cert_expiration", self.cert_expiration.as_ref());
        params.push_opt("compute_id", self.compute_id.as_ref());
        params.push_opt("compute_flavor", self.compute_flavor.as_ref());
        params.push_opt("created_at", self.created_at.as_ref());
        params.push_opt("ha_ip", self.ha_ip.as_ref());
        params.push_opt("ha_port_id", self.ha_port_id.as_ref());
        params.push_opt("id", self.id.as_ref());
        params.push_opt("image_id", self.image_id.as_ref());
        params.push_opt("lb_network_ip", self.lb_network_ip.as_ref());
        params.push_opt("limit", self.limit);
        params.push_opt("loadbalancer_id", self.loadbalancer_id.as_ref());
        params.push_opt("marker", self.marker.as_ref());
        params.push_opt("page_reverse", self.page_reverse);
        params.push_opt("role", self.role.as_ref());
        params.push_opt("status", self.status.as_ref());
        params.push_opt("updated_at", self.updated_at.as_ref());
        params.push_opt("vrrp_ip", self.vrrp_ip.as_ref());
        params.push_opt("vrrp_port_id", self.vrrp_port_id.as_ref());
        params.push_opt("vrrp_interface", self.vrrp_interface.as_ref());
        params.push_opt("vrrp_id", self.vrrp_id.as_ref());
        params.push_opt("vrrp_priority", self.vrrp_priority.as_ref());

        params
    }

    fn service_type(&self) -> ServiceType {
        ServiceType::LoadBalancer
    }

    fn response_key(&self) -> Option<Cow<'static, str>> {
        Some("amphorae".into())
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
            ServiceType::LoadBalancer
        );
    }

    #[test]
    fn test_response_key() {
        assert_eq!(
            Request::builder().build().unwrap().response_key().unwrap(),
            "amphorae"
        );
    }

    #[cfg(feature = "sync")]
    #[test]
    fn endpoint() {
        let server = MockServer::start();
        let client = FakeOpenStackClient::new(server.base_url());
        let mock = server.mock(|when, then| {
            when.method(httpmock::Method::GET)
                .path("/octavia/amphorae".to_string());

            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({ "amphorae": {} }));
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
                .path("/octavia/amphorae".to_string())
                .header("foo", "bar")
                .header("not_foo", "not_bar");
            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({ "amphorae": {} }));
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
