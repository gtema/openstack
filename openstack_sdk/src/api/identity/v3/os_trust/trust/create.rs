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

//! Create a new trust.
//!
//! The User creating the trust must be the trustor.
//!
//! POST /v3/OS-TRUST/trusts
//!
use derive_builder::Builder;
use http::{HeaderMap, HeaderName, HeaderValue};

use crate::api::rest_endpoint_prelude::*;

use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;
use std::borrow::Cow;
use std::collections::BTreeMap;

#[derive(Builder, Debug, Deserialize, Clone, Serialize)]
#[builder(setter(strip_option))]
pub struct Roles<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) id: Option<Cow<'a, str>>,

    /// The resource name.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) name: Option<Cow<'a, str>>,
}

#[derive(Builder, Debug, Deserialize, Clone, Serialize)]
#[builder(setter(strip_option))]
pub struct Trust<'a> {
    /// If set to true then a trust between a trustor and any third-party user
    /// may be issued by the trustee just like a regular trust. If set to
    /// false, stops further redelegation. False by default.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) allow_redelegation: Option<Option<bool>>,

    /// Specifies the expiration time of the trust. A trust may be revoked
    /// ahead of expiration. If the value represents a time in the past, the
    /// trust is deactivated. In the redelegation case it must not exceed the
    /// value of the corresponding expires_at field of the redelegated trust or
    /// it may be omitted, then the expires_at value is copied from the
    /// redelegated trust.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) expires_at: Option<Option<Cow<'a, str>>>,

    /// If set to true, then the user attribute of tokens generated based on
    /// the trust will represent that of the trustor rather than the trustee,
    /// thus allowing the trustee to impersonate the trustor. If impersonation
    /// if set to false, then the token's user attribute will represent that of
    /// the trustee.
    #[serde()]
    #[builder(setter(into))]
    pub(crate) impersonation: bool,

    /// Identifies the project upon which the trustor is delegating
    /// authorization.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) project_id: Option<Option<Cow<'a, str>>>,

    /// Returned with redelegated trust provides information about the
    /// predecessor in the trust chain.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) redelegated_trust_id: Option<Option<Cow<'a, str>>>,

    /// Specifies the maximum remaining depth of the redelegated trust chain.
    /// Each subsequent trust has this field decremented by 1 automatically.
    /// The initial trustor issuing new trust that can be redelegated, must set
    /// allow_redelegation to true and may set redelegation_count to an integer
    /// value less than or equal to max_redelegation_count configuration
    /// parameter in order to limit the possible length of derived trust
    /// chains. The trust issued by the trustor using a project-scoped token
    /// (not redelegating), in which allow_redelegation is set to true (the new
    /// trust is redelegatable), will be populated with the value specified in
    /// the max_redelegation_count configuration parameter if
    /// redelegation_count is not set or set to null. If allow_redelegation is
    /// set to false then redelegation_count will be set to 0 in the trust. If
    /// the trust is being issued by the trustee of a redelegatable
    /// trust-scoped token (redelegation case) then redelegation_count should
    /// not be set, as it will automatically be set to the value in the
    /// redelegatable trust-scoped token decremented by 1. Note, if the
    /// resulting value is 0, this means that the new trust will not be
    /// redelegatable, regardless of the value of allow_redelegation.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) redelegation_count: Option<Option<i32>>,

    /// Specifies how many times the trust can be used to obtain a token. This
    /// value is decreased each time a token is issued through the trust. Once
    /// it reaches 0, no further tokens will be issued through the trust. The
    /// default value is null, meaning there is no limit on the number of
    /// tokens issued through the trust. If redelegation is enabled it must not
    /// be set.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) remaining_uses: Option<Option<i32>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) roles: Option<Vec<Roles<'a>>>,

    /// Represents the user who is capable of consuming the trust.
    #[serde()]
    #[builder(setter(into))]
    pub(crate) trustee_user_id: Cow<'a, str>,

    /// Represents the user who created the trust, and who's authorization is
    /// being delegated.
    #[serde()]
    #[builder(setter(into))]
    pub(crate) trustor_user_id: Cow<'a, str>,

    #[builder(setter(name = "_properties"), default, private)]
    #[serde(flatten)]
    _properties: BTreeMap<Cow<'a, str>, Value>,
}

impl<'a> TrustBuilder<'a> {
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
    #[builder(setter(into))]
    pub(crate) trust: Trust<'a>,

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
    /// Add a single header to the Trust.
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
        http::Method::POST
    }

    fn endpoint(&self) -> Cow<'static, str> {
        "OS-TRUST/trusts".to_string().into()
    }

    fn parameters(&self) -> QueryParams {
        QueryParams::default()
    }

    fn body(&self) -> Result<Option<(&'static str, Vec<u8>)>, BodyError> {
        let mut params = JsonBodyParams::default();

        params.push("trust", serde_json::to_value(&self.trust)?);

        params.into_body()
    }

    fn service_type(&self) -> ServiceType {
        ServiceType::Identity
    }

    fn response_key(&self) -> Option<Cow<'static, str>> {
        Some("trust".into())
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
                .trust(
                    TrustBuilder::default()
                        .impersonation(false)
                        .trustee_user_id("foo")
                        .trustor_user_id("foo")
                        .build()
                        .unwrap()
                )
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
                .trust(
                    TrustBuilder::default()
                        .impersonation(false)
                        .trustee_user_id("foo")
                        .trustor_user_id("foo")
                        .build()
                        .unwrap()
                )
                .build()
                .unwrap()
                .response_key()
                .unwrap(),
            "trust"
        );
    }

    #[cfg(feature = "sync")]
    #[test]
    fn endpoint() {
        let server = MockServer::start();
        let client = FakeOpenStackClient::new(server.base_url());
        let mock = server.mock(|when, then| {
            when.method(httpmock::Method::POST)
                .path("/OS-TRUST/trusts".to_string());

            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({ "trust": {} }));
        });

        let endpoint = Request::builder()
            .trust(
                TrustBuilder::default()
                    .impersonation(false)
                    .trustee_user_id("foo")
                    .trustor_user_id("foo")
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
        let server = MockServer::start();
        let client = FakeOpenStackClient::new(server.base_url());
        let mock = server.mock(|when, then| {
            when.method(httpmock::Method::POST)
                .path("/OS-TRUST/trusts".to_string())
                .header("foo", "bar")
                .header("not_foo", "not_bar");
            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({ "trust": {} }));
        });

        let endpoint = Request::builder()
            .trust(
                TrustBuilder::default()
                    .impersonation(false)
                    .trustee_user_id("foo")
                    .trustor_user_id("foo")
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
