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

//! Validates a token.
//!
//! This call is similar to `GET /auth/tokens` but no response body is provided
//! even in the `X-Subject-Token` header.
//!
//! The Identity API returns the same response as when the subject token was
//! issued by `POST /auth/tokens` even if an error occurs because the token is
//! not valid. An HTTP `204` response code indicates that the `X-Subject-Token`
//! is valid.
//!
//! Relationship:
//! `https://docs.openstack.org/api/openstack-identity/3/rel/auth_tokens`
//!
use derive_builder::Builder;
use http::{HeaderMap, HeaderName, HeaderValue};

use crate::api::rest_endpoint_prelude::*;

#[derive(Builder, Debug, Clone)]
#[builder(setter(strip_option))]
pub struct Request {
    #[builder(setter(name = "_headers"), default, private)]
    _headers: Option<HeaderMap>,
}
impl Request {
    /// Create a builder for the endpoint.
    pub fn builder() -> RequestBuilder {
        RequestBuilder::default()
    }
}

impl RequestBuilder {
    /// Add a single header to the Token.
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

impl RestEndpoint for Request {
    fn method(&self) -> http::Method {
        http::Method::HEAD
    }

    fn endpoint(&self) -> Cow<'static, str> {
        "auth/tokens".to_string().into()
    }

    fn parameters(&self) -> QueryParams {
        QueryParams::default()
    }

    fn service_type(&self) -> ServiceType {
        ServiceType::Identity
    }

    fn response_key(&self) -> Option<Cow<'static, str>> {
        None
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
    #![allow(unused_imports)]
    use super::*;
    #[cfg(feature = "sync")]
    use crate::api::RawQuery;
    #[cfg(feature = "sync")]
    use crate::test::client::MockServerClient;
    use crate::types::ServiceType;
    use http::{HeaderName, HeaderValue};

    #[test]
    fn test_service_type() {
        assert_eq!(
            Request::builder().build().unwrap().service_type(),
            ServiceType::Identity
        );
    }

    #[test]
    fn test_response_key() {
        assert!(Request::builder().build().unwrap().response_key().is_none())
    }

    #[cfg(feature = "sync")]
    #[test]
    fn endpoint() {
        let client = MockServerClient::new();
        let mock = client.server.mock(|when, then| {
            when.method(httpmock::Method::HEAD)
                .path("/auth/tokens".to_string());

            then.status(200).header("content-type", "application/json");
        });

        let endpoint = Request::builder().build().unwrap();
        let _ = endpoint.raw_query(&client).unwrap();
        mock.assert();
    }

    #[cfg(feature = "sync")]
    #[test]
    fn endpoint_headers() {
        let client = MockServerClient::new();
        let mock = client.server.mock(|when, then| {
            when.method(httpmock::Method::HEAD)
                .path("/auth/tokens".to_string())
                .header("foo", "bar")
                .header("not_foo", "not_bar");
            then.status(200).header("content-type", "application/json");
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
        let _ = endpoint.raw_query(&client).unwrap();
        mock.assert();
    }
}
