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

//! Updates a role.
//!
//! Relationship:
//! `https://docs.openstack.org/api/openstack-identity/3/rel/role`
//!
use derive_builder::Builder;
use http::{HeaderMap, HeaderName, HeaderValue};

use crate::api::rest_endpoint_prelude::*;

use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;
use std::borrow::Cow;
use std::collections::BTreeMap;

/// The resource options for the role. Available resource options are
/// `immutable`.
#[derive(Builder, Debug, Deserialize, Clone, Serialize)]
#[builder(setter(strip_option))]
pub struct Options {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) immutable: Option<bool>,
}

/// A `role` object
#[derive(Builder, Debug, Deserialize, Clone, Serialize)]
#[builder(setter(strip_option))]
pub struct Role<'a> {
    /// The new role description.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) description: Option<Option<Cow<'a, str>>>,

    /// The ID of the domain.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) domain_id: Option<Option<Cow<'a, str>>>,

    /// The new role name.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) name: Option<Cow<'a, str>>,

    /// The resource options for the role. Available resource options are
    /// `immutable`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) options: Option<Options>,

    #[builder(setter(name = "_properties"), default, private)]
    #[serde(flatten)]
    _properties: BTreeMap<Cow<'a, str>, Value>,
}

impl<'a> RoleBuilder<'a> {
    pub fn properties<I, K, V>(&mut self, iter: I) -> &mut Self
    where
        I: Iterator<Item = (K, V)>,
        K: Into<Cow<'a, str>>,
        V: Into<Value>,
    {
        self._properties
            .get_or_insert_with(BTreeMap::new)
            .extend(iter.map(|(k, v)| (k.into(), v.into())));
        self
    }
}

#[derive(Builder, Debug, Clone)]
#[builder(setter(strip_option))]
pub struct Request<'a> {
    /// A `role` object
    #[builder(setter(into))]
    pub(crate) role: Role<'a>,

    /// role_id parameter for /v3/roles/{role_id} API
    #[builder(default, setter(into))]
    id: Cow<'a, str>,

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
    /// Add a single header to the Role.
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
        http::Method::PATCH
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("roles/{id}", id = self.id.as_ref(),).into()
    }

    fn parameters(&self) -> QueryParams {
        QueryParams::default()
    }

    fn body(&self) -> Result<Option<(&'static str, Vec<u8>)>, BodyError> {
        let mut params = JsonBodyParams::default();

        params.push("role", serde_json::to_value(&self.role)?);

        params.into_body()
    }

    fn service_type(&self) -> ServiceType {
        ServiceType::Identity
    }

    fn response_key(&self) -> Option<Cow<'static, str>> {
        Some("role".into())
    }

    /// Returns headers to be set into the request
    fn request_headers(&self) -> Option<&HeaderMap> {
        self._headers.as_ref()
    }

    /// Returns required API version
    fn api_version(&self) -> Option<ApiVersion> {
        Some(ApiVersion::new(3, 0))
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
                .role(RoleBuilder::default().build().unwrap())
                .build()
                .unwrap()
                .service_type(),
            ServiceType::Identity
        );
    }

    #[test]
    fn test_response_key() {
        assert_eq!(
            Request::builder()
                .role(RoleBuilder::default().build().unwrap())
                .build()
                .unwrap()
                .response_key()
                .unwrap(),
            "role"
        );
    }

    #[cfg(feature = "sync")]
    #[test]
    fn endpoint() {
        let server = MockServer::start();
        let client = FakeOpenStackClient::new(server.base_url());
        let mock = server.mock(|when, then| {
            when.method(httpmock::Method::PATCH)
                .path(format!("/roles/{id}", id = "id",));

            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({ "role": {} }));
        });

        let endpoint = Request::builder()
            .id("id")
            .role(RoleBuilder::default().build().unwrap())
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
            when.method(httpmock::Method::PATCH)
                .path(format!("/roles/{id}", id = "id",))
                .header("foo", "bar")
                .header("not_foo", "not_bar");
            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({ "role": {} }));
        });

        let endpoint = Request::builder()
            .id("id")
            .role(RoleBuilder::default().build().unwrap())
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
