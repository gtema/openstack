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

//! Creates a server group.
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

#[derive(Debug, Deserialize, Clone, Serialize)]
pub enum Policy {
    #[serde(rename = "affinity")]
    Affinity,
    #[serde(rename = "anti-affinity")]
    AntiAffinity,
    #[serde(rename = "soft-affinity")]
    SoftAffinity,
    #[serde(rename = "soft-anti-affinity")]
    SoftAntiAffinity,
}

/// The `rules` field, which is a dict, can be applied to the policy.
/// Currently, only the `max_server_per_host` rule is supported for the
/// `anti-affinity` policy. The `max_server_per_host` rule allows specifying
/// how many members of the anti-affinity group can reside on the same compute
/// host. If not specified, only one member from the same anti-affinity group
/// can reside on a given host. Requesting policy rules with any other policy
/// than `anti-affinity` will be 400.
///
/// **New in version 2.64**
///
#[derive(Builder, Debug, Deserialize, Clone, Serialize)]
#[builder(setter(strip_option))]
pub struct Rules<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) max_server_per_host: Option<Cow<'a, str>>,
}

/// The server group object.
///
#[derive(Builder, Debug, Deserialize, Clone, Serialize)]
#[builder(setter(strip_option))]
pub struct ServerGroup<'a> {
    /// The name of the server group.
    ///
    #[serde()]
    #[builder(setter(into))]
    pub(crate) name: Cow<'a, str>,

    /// The `policy` field represents the name of the policy. The current valid
    /// policy names are:
    ///
    /// - `anti-affinity` - servers in this group must be scheduled to
    ///   different hosts.
    /// - `affinity` - servers in this group must be scheduled to the same
    ///   host.
    /// - `soft-anti-affinity` - servers in this group should be scheduled to
    ///   different hosts if possible, but if not possible then they should
    ///   still be scheduled instead of resulting in a build failure.
    /// - `soft-affinity` - servers in this group should be scheduled to the
    ///   same host if possible, but if not possible then they should still be
    ///   scheduled instead of resulting in a build failure.
    ///
    /// **New in version 2.64**
    ///
    #[serde()]
    #[builder()]
    pub(crate) policy: Policy,

    /// The `rules` field, which is a dict, can be applied to the policy.
    /// Currently, only the `max_server_per_host` rule is supported for the
    /// `anti-affinity` policy. The `max_server_per_host` rule allows
    /// specifying how many members of the anti-affinity group can reside on
    /// the same compute host. If not specified, only one member from the same
    /// anti-affinity group can reside on a given host. Requesting policy rules
    /// with any other policy than `anti-affinity` will be 400.
    ///
    /// **New in version 2.64**
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) rules: Option<Rules<'a>>,
}

#[derive(Builder, Debug, Clone)]
#[builder(setter(strip_option))]
pub struct Request<'a> {
    /// The server group object.
    ///
    #[builder(setter(into))]
    pub(crate) server_group: ServerGroup<'a>,

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
    /// Add a single header to the Server_Group.
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
        http::Method::POST
    }

    fn endpoint(&self) -> Cow<'static, str> {
        "os-server-groups".to_string().into()
    }

    fn parameters(&self) -> QueryParams {
        QueryParams::default()
    }

    fn body(&self) -> Result<Option<(&'static str, Vec<u8>)>, BodyError> {
        let mut params = JsonBodyParams::default();

        params.push("server_group", serde_json::to_value(&self.server_group)?);

        params.into_body()
    }

    fn service_type(&self) -> ServiceType {
        ServiceType::Compute
    }

    fn response_key(&self) -> Option<Cow<'static, str>> {
        Some("server_group".into())
    }

    /// Returns headers to be set into the request
    fn request_headers(&self) -> Option<&HeaderMap> {
        self._headers.as_ref()
    }

    /// Returns required API version
    fn api_version(&self) -> Option<ApiVersion> {
        Some(ApiVersion::new(2, 64))
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
                .server_group(
                    ServerGroupBuilder::default()
                        .name("foo")
                        .policy(Policy::Affinity)
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
                .server_group(
                    ServerGroupBuilder::default()
                        .name("foo")
                        .policy(Policy::Affinity)
                        .build()
                        .unwrap()
                )
                .build()
                .unwrap()
                .response_key()
                .unwrap(),
            "server_group"
        );
    }

    #[cfg(feature = "sync")]
    #[test]
    fn endpoint() {
        let client = MockServerClient::new();
        let mock = client.server.mock(|when, then| {
            when.method(httpmock::Method::POST)
                .path("/os-server-groups".to_string());

            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({ "server_group": {} }));
        });

        let endpoint = Request::builder()
            .server_group(
                ServerGroupBuilder::default()
                    .name("foo")
                    .policy(Policy::Affinity)
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
        let client = MockServerClient::new();
        let mock = client.server.mock(|when, then| {
            when.method(httpmock::Method::POST)
                .path("/os-server-groups".to_string())
                .header("foo", "bar")
                .header("not_foo", "not_bar");
            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({ "server_group": {} }));
        });

        let endpoint = Request::builder()
            .server_group(
                ServerGroupBuilder::default()
                    .name("foo")
                    .policy(Policy::Affinity)
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
