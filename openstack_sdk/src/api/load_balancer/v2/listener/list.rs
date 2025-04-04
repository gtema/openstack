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

//! Lists all listeners for the project.
//!
//! Use the `fields` query parameter to control which fields are returned in
//! the response body. Additionally, you can filter results by using query
//! string parameters. For information, see
//! [Filtering and column selection](#filtering).
//!
//! Administrative users can specify a project ID that is different than their
//! own to list listeners for other projects.
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
    /// The administrative state of the resource
    ///
    #[builder(default)]
    admin_state_up: Option<bool>,

    /// A list of ALPN protocols. Available protocols: http/1.0, http/1.1, h2
    ///
    #[builder(default, setter(into))]
    alpn_protocols: Option<Cow<'a, str>>,

    /// The maximum number of connections permitted for this listener. Default
    /// value is -1 which represents infinite connections or a default value
    /// defined by the provider driver.
    ///
    #[builder(default, setter(into))]
    connection_limit: Option<Cow<'a, str>>,

    /// The UTC date and timestamp when the resource was created.
    ///
    #[builder(default, setter(into))]
    created_at: Option<Cow<'a, str>>,

    /// The ID of the pool used by the listener if no L7 policies match.
    ///
    #[builder(default, setter(into))]
    default_pool_id: Option<Cow<'a, str>>,

    /// A human-readable description for the resource.
    ///
    #[builder(default, setter(into))]
    description: Option<Cow<'a, str>>,

    /// Defines whether the includeSubDomains directive should be added to the
    /// Strict-Transport-Security HTTP response header.
    ///
    #[builder(default)]
    hsts_include_subdomains: Option<bool>,

    /// The value of the max_age directive for the Strict-Transport-Security
    /// HTTP response header.
    ///
    #[builder(default)]
    hsts_max_age: Option<i32>,

    /// Defines whether the preload directive should be added to the
    /// Strict-Transport-Security HTTP response header.
    ///
    #[builder(default)]
    hsts_preload: Option<bool>,

    /// The ID of the resource
    ///
    #[builder(default, setter(into))]
    id: Option<Cow<'a, str>>,

    /// Page size
    ///
    #[builder(default)]
    limit: Option<i32>,

    /// Load balancer ID
    ///
    #[builder(default, setter(into))]
    load_balancer_id: Option<Cow<'a, str>>,

    /// ID of the last item in the previous list
    ///
    #[builder(default, setter(into))]
    marker: Option<Cow<'a, str>>,

    /// Human-readable name of the resource.
    ///
    #[builder(default, setter(into))]
    name: Option<Cow<'a, str>>,

    /// Return the list of entities that do not have one or more of the given
    /// tags.
    ///
    #[builder(default, setter(into))]
    not_tags: Option<Cow<'a, str>>,

    /// Return the list of entities that do not have at least one of the given
    /// tags.
    ///
    #[builder(default, setter(into))]
    not_tags_any: Option<Cow<'a, str>>,

    /// The operating status of the resource.
    ///
    #[builder(default, setter(into))]
    operating_status: Option<Cow<'a, str>>,

    /// The page direction.
    ///
    #[builder(default)]
    page_reverse: Option<bool>,

    /// The ID of the project owning this resource.
    ///
    #[builder(default, setter(into))]
    project_id: Option<Cow<'a, str>>,

    /// The protocol for the resource.
    ///
    #[builder(default, setter(into))]
    protocol: Option<Cow<'a, str>>,

    /// The protocol port number for the resource.
    ///
    #[builder(default)]
    protocol_port: Option<i32>,

    /// The provisioning status of the resource.
    ///
    #[builder(default, setter(into))]
    provisioning_status: Option<Cow<'a, str>>,

    /// Return the list of entities that have this tag or tags.
    ///
    #[builder(default, setter(into))]
    tags: Option<Cow<'a, str>>,

    /// Return the list of entities that have one or more of the given tags.
    ///
    #[builder(default, setter(into))]
    tags_any: Option<Cow<'a, str>>,

    /// Frontend client inactivity timeout in milliseconds.
    ///
    #[builder(default)]
    timeout_client_data: Option<i32>,

    /// Backend member connection timeout in milliseconds.
    ///
    #[builder(default)]
    timeout_member_connect: Option<i32>,

    /// Backend member inactivity timeout in milliseconds.
    ///
    #[builder(default)]
    timeout_member_data: Option<i32>,

    /// Time, in milliseconds, to wait for additional TCP packets for content
    /// inspection.
    ///
    #[builder(default)]
    timeout_tcp_inspect: Option<i32>,

    /// List of ciphers in OpenSSL format
    ///
    #[builder(default, setter(into))]
    tls_ciphers: Option<Cow<'a, str>>,

    /// A list of TLS protocol versions.
    ///
    #[builder(default, setter(into))]
    tls_versions: Option<Cow<'a, str>>,

    /// The UTC date and timestamp when the resource was last updated.
    ///
    #[builder(default, setter(into))]
    updated_at: Option<Cow<'a, str>>,

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
    /// Add a single header to the Listener.
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
        "lbaas/listeners".to_string().into()
    }

    fn parameters(&self) -> QueryParams {
        let mut params = QueryParams::default();
        params.push_opt("admin_state_up", self.admin_state_up);
        params.push_opt("alpn_protocols", self.alpn_protocols.as_ref());
        params.push_opt("connection_limit", self.connection_limit.as_ref());
        params.push_opt("created_at", self.created_at.as_ref());
        params.push_opt("default_pool_id", self.default_pool_id.as_ref());
        params.push_opt("description", self.description.as_ref());
        params.push_opt("hsts_include_subdomains", self.hsts_include_subdomains);
        params.push_opt("hsts_max_age", self.hsts_max_age);
        params.push_opt("hsts_preload", self.hsts_preload);
        params.push_opt("id", self.id.as_ref());
        params.push_opt("limit", self.limit);
        params.push_opt("load_balancer_id", self.load_balancer_id.as_ref());
        params.push_opt("marker", self.marker.as_ref());
        params.push_opt("name", self.name.as_ref());
        params.push_opt("page_reverse", self.page_reverse);
        params.push_opt("project_id", self.project_id.as_ref());
        params.push_opt("protocol", self.protocol.as_ref());
        params.push_opt("protocol_port", self.protocol_port);
        params.push_opt("timeout_client_data", self.timeout_client_data);
        params.push_opt("timeout_member_connect", self.timeout_member_connect);
        params.push_opt("timeout_member_data", self.timeout_member_data);
        params.push_opt("timeout_tcp_inspect", self.timeout_tcp_inspect);
        params.push_opt("tls_ciphers", self.tls_ciphers.as_ref());
        params.push_opt("tls_versions", self.tls_versions.as_ref());
        params.push_opt("updated_at", self.updated_at.as_ref());
        params.push_opt("provisioning_status", self.provisioning_status.as_ref());
        params.push_opt("operating_status", self.operating_status.as_ref());
        params.push_opt("tags", self.tags.as_ref());
        params.push_opt("tags-any", self.tags_any.as_ref());
        params.push_opt("not-tags", self.not_tags.as_ref());
        params.push_opt("not-tags-any", self.not_tags_any.as_ref());

        params
    }

    fn service_type(&self) -> ServiceType {
        ServiceType::LoadBalancer
    }

    fn response_key(&self) -> Option<Cow<'static, str>> {
        Some("listeners".into())
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
            "listeners"
        );
    }

    #[cfg(feature = "sync")]
    #[test]
    fn endpoint() {
        let server = MockServer::start();
        let client = FakeOpenStackClient::new(server.base_url());
        let mock = server.mock(|when, then| {
            when.method(httpmock::Method::GET)
                .path("/lbaas/listeners".to_string());

            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({ "listeners": {} }));
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
                .path("/lbaas/listeners".to_string())
                .header("foo", "bar")
                .header("not_foo", "not_bar");
            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({ "listeners": {} }));
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
