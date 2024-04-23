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

//! Updates a L7 policy.
//!
//! If the request is valid, the service returns the `Accepted (202)` response
//! code. To confirm the update, check that the L7 policy provisioning status
//! is `ACTIVE`. If the status is `PENDING_UPDATE`, use a GET operation to poll
//! the L7 policy object for changes.
//!
//! This operation returns the updated L7 policy object with the `ACTIVE`,
//! `PENDING_UPDATE`, or `ERROR` provisioning status.
//!
//! If a policy is updated with a position that matches that of an existing
//! policy, then the updated policy is inserted at the given position.
//!
use derive_builder::Builder;
use http::{HeaderMap, HeaderName, HeaderValue};

use crate::api::rest_endpoint_prelude::*;

use serde::Deserialize;
use serde::Serialize;
use std::borrow::Cow;

#[derive(Debug, Deserialize, Clone, Serialize)]
pub enum Action {
    #[serde(rename = "REDIRECT_PREFIX")]
    RedirectPrefix,
    #[serde(rename = "REDIRECT_TO_POOL")]
    RedirectToPool,
    #[serde(rename = "REDIRECT_TO_URL")]
    RedirectToUrl,
    #[serde(rename = "REJECT")]
    Reject,
}

/// Defines attributes that are acceptable of a PUT request.
///
#[derive(Builder, Debug, Deserialize, Clone, Serialize)]
#[builder(setter(strip_option))]
pub struct L7policy<'a> {
    /// The L7 policy action. One of `REDIRECT_PREFIX`, `REDIRECT_TO_POOL`,
    /// `REDIRECT_TO_URL`, or `REJECT`.
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub(crate) action: Option<Action>,

    /// The administrative state of the resource, which is up (`true`) or down
    /// (`false`). Default is `true`.
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub(crate) admin_state_up: Option<bool>,

    /// A human-readable description for the resource.
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) description: Option<Cow<'a, str>>,

    /// Human-readable name of the resource.
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) name: Option<Cow<'a, str>>,

    /// The position of this policy on the listener. Positions start at 1.
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub(crate) position: Option<i32>,

    /// Requests matching this policy will be redirected to the specified URL
    /// or Prefix URL with the HTTP response code. Valid if `action` is
    /// `REDIRECT_TO_URL` or `REDIRECT_PREFIX`. Valid options are: 301, 302,
    /// 303, 307, or 308. Default is 302.
    ///
    /// **New in version 2.9**
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub(crate) redirect_http_code: Option<i32>,

    /// Requests matching this policy will be redirected to the pool with this
    /// ID. Only valid if `action` is `REDIRECT_TO_POOL`. The pool has some
    /// restrictions, See
    /// [Protocol Combinations (Listener/Pool)](#valid-protocol).
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) redirect_pool_id: Option<Cow<'a, str>>,

    /// Requests matching this policy will be redirected to this Prefix URL.
    /// Only valid if `action` is `REDIRECT_PREFIX`.
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) redirect_prefix: Option<Cow<'a, str>>,

    /// Requests matching this policy will be redirected to this URL. Only
    /// valid if `action` is `REDIRECT_TO_URL`.
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) redirect_url: Option<Cow<'a, str>>,

    /// A list of simple strings assigned to the resource.
    ///
    /// **New in version 2.5**
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) tags: Option<Vec<Cow<'a, str>>>,
}

#[derive(Builder, Debug, Clone)]
#[builder(setter(strip_option))]
pub struct Request<'a> {
    /// Defines attributes that are acceptable of a PUT request.
    ///
    #[builder(setter(into))]
    pub(crate) l7policy: L7policy<'a>,

    /// l7policy_id parameter for /v2/lbaas/l7policies/{l7policy_id} API
    ///
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

impl<'a> RequestBuilder<'a> {
    /// Add a single header to the L7Policy.
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
        http::Method::PUT
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("v2/lbaas/l7policies/{id}", id = self.id.as_ref(),).into()
    }

    fn parameters(&self) -> QueryParams {
        QueryParams::default()
    }

    fn body(&self) -> Result<Option<(&'static str, Vec<u8>)>, BodyError> {
        let mut params = JsonBodyParams::default();

        params.push("l7policy", serde_json::to_value(&self.l7policy)?);

        params.into_body()
    }

    fn service_type(&self) -> ServiceType {
        ServiceType::LoadBalancer
    }

    fn response_key(&self) -> Option<Cow<'static, str>> {
        Some("l7policy".into())
    }

    /// Returns headers to be set into the request
    fn request_headers(&self) -> Option<&HeaderMap> {
        self._headers.as_ref()
    }
}

#[cfg(test)]
mod tests {
    #![allow(unused_imports)]
    use super::*;
    use crate::api::Query;
    use crate::test::client::MockServerClient;
    use crate::types::ServiceType;
    use http::{HeaderName, HeaderValue};
    use serde_json::json;

    #[test]
    fn test_service_type() {
        assert_eq!(
            Request::builder()
                .l7policy(L7policyBuilder::default().build().unwrap())
                .build()
                .unwrap()
                .service_type(),
            ServiceType::LoadBalancer
        );
    }

    #[test]
    fn test_response_key() {
        assert_eq!(
            Request::builder()
                .l7policy(L7policyBuilder::default().build().unwrap())
                .build()
                .unwrap()
                .response_key()
                .unwrap(),
            "l7policy"
        );
    }

    #[test]
    fn endpoint() {
        let client = MockServerClient::new();
        let mock = client.server.mock(|when, then| {
            when.method(httpmock::Method::PUT)
                .path(format!("/v2/lbaas/l7policies/{id}", id = "id",));

            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({ "l7policy": {} }));
        });

        let endpoint = Request::builder()
            .id("id")
            .l7policy(L7policyBuilder::default().build().unwrap())
            .build()
            .unwrap();
        let _: serde_json::Value = endpoint.query(&client).unwrap();
        mock.assert();
    }

    #[test]
    fn endpoint_headers() {
        let client = MockServerClient::new();
        let mock = client.server.mock(|when, then| {
            when.method(httpmock::Method::PUT)
                .path(format!("/v2/lbaas/l7policies/{id}", id = "id",))
                .header("foo", "bar")
                .header("not_foo", "not_bar");
            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({ "l7policy": {} }));
        });

        let endpoint = Request::builder()
            .id("id")
            .l7policy(L7policyBuilder::default().build().unwrap())
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