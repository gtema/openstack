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

//! Creates an IKE policy.
//!
//! The IKE policy is used for phases one and two negotiation of the VPN
//! connection. You can specify both the authentication and encryption
//! algorithms for connections.
//!
//! Normal response codes: 201
//!
//! Error response codes: 400, 401
//!
use derive_builder::Builder;
use http::{HeaderMap, HeaderName, HeaderValue};

use crate::api::rest_endpoint_prelude::*;

use serde::Deserialize;
use serde::Serialize;
use std::borrow::Cow;

#[derive(Debug, Deserialize, Clone, Serialize)]
pub enum AuthAlgorithm {
    #[serde(rename = "aes-cmac")]
    AesCmac,
    #[serde(rename = "aes-xcbc")]
    AesXcbc,
    #[serde(rename = "sha1")]
    Sha1,
    #[serde(rename = "sha256")]
    Sha256,
    #[serde(rename = "sha384")]
    Sha384,
    #[serde(rename = "sha512")]
    Sha512,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub enum EncryptionAlgorithm {
    #[serde(rename = "3des")]
    _3des,
    #[serde(rename = "aes-128")]
    Aes128,
    #[serde(rename = "aes-128-ccm-12")]
    Aes128Ccm12,
    #[serde(rename = "aes-128-ccm-16")]
    Aes128Ccm16,
    #[serde(rename = "aes-128-ccm-8")]
    Aes128Ccm8,
    #[serde(rename = "aes-128-gcm-12")]
    Aes128Gcm12,
    #[serde(rename = "aes-128-gcm-16")]
    Aes128Gcm16,
    #[serde(rename = "aes-128-gcm-8")]
    Aes128Gcm8,
    #[serde(rename = "aes-192")]
    Aes192,
    #[serde(rename = "aes-192-ccm-12")]
    Aes192Ccm12,
    #[serde(rename = "aes-192-ccm-16")]
    Aes192Ccm16,
    #[serde(rename = "aes-192-ccm-8")]
    Aes192Ccm8,
    #[serde(rename = "aes-192-gcm-12")]
    Aes192Gcm12,
    #[serde(rename = "aes-192-gcm-16")]
    Aes192Gcm16,
    #[serde(rename = "aes-192-gcm-8")]
    Aes192Gcm8,
    #[serde(rename = "aes-256")]
    Aes256,
    #[serde(rename = "aes-256-ccm-12")]
    Aes256Ccm12,
    #[serde(rename = "aes-256-ccm-16")]
    Aes256Ccm16,
    #[serde(rename = "aes-256-ccm-8")]
    Aes256Ccm8,
    #[serde(rename = "aes-256-gcm-12")]
    Aes256Gcm12,
    #[serde(rename = "aes-256-gcm-16")]
    Aes256Gcm16,
    #[serde(rename = "aes-256-gcm-8")]
    Aes256Gcm8,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub enum Phase1NegotiationMode {
    #[serde(rename = "aggressive")]
    Aggressive,
    #[serde(rename = "main")]
    Main,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub enum IkeVersion {
    #[serde(rename = "v1")]
    V1,
    #[serde(rename = "v2")]
    V2,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub enum Pfs {
    #[serde(rename = "group14")]
    Group14,
    #[serde(rename = "group15")]
    Group15,
    #[serde(rename = "group16")]
    Group16,
    #[serde(rename = "group17")]
    Group17,
    #[serde(rename = "group18")]
    Group18,
    #[serde(rename = "group19")]
    Group19,
    #[serde(rename = "group2")]
    Group2,
    #[serde(rename = "group20")]
    Group20,
    #[serde(rename = "group21")]
    Group21,
    #[serde(rename = "group22")]
    Group22,
    #[serde(rename = "group23")]
    Group23,
    #[serde(rename = "group24")]
    Group24,
    #[serde(rename = "group25")]
    Group25,
    #[serde(rename = "group26")]
    Group26,
    #[serde(rename = "group27")]
    Group27,
    #[serde(rename = "group28")]
    Group28,
    #[serde(rename = "group29")]
    Group29,
    #[serde(rename = "group30")]
    Group30,
    #[serde(rename = "group31")]
    Group31,
    #[serde(rename = "group5")]
    Group5,
}

/// An `ikepolicy` object.
///
#[derive(Builder, Debug, Deserialize, Clone, Serialize)]
#[builder(setter(strip_option))]
pub struct Ikepolicy<'a> {
    /// The authentication hash algorithm. Valid values are `sha1`, `sha256`,
    /// `sha384`, `sha512`, `aes-xcbc`, `aes-cmac`. The default is `sha1`.
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub(crate) auth_algorithm: Option<AuthAlgorithm>,

    /// A human-readable description for the resource. Default is an empty
    /// string.
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) description: Option<Cow<'a, str>>,

    /// The encryption algorithm. A valid value is `3des`, `aes-128`,
    /// `aes-192`, `aes-256`. Additional values for AES CCM and GCM modes are
    /// defined (e.g. `aes-256-ccm-16`, `aes-256-gcm-16`) for all combinations
    /// of key length 128, 192, 256 bits and ICV length 8, 12, 16 octets.
    /// Default is `aes-128`.
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub(crate) encryption_algorithm: Option<EncryptionAlgorithm>,

    /// The IKE version. A valid value is `v1` or `v2`. Default is `v1`.
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub(crate) ike_version: Option<IkeVersion>,

    /// The lifetime of the security association. The lifetime consists of a
    /// unit and integer value. You can omit either the unit or value portion
    /// of the lifetime. Default unit is seconds and default value is 3600.
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) lifetime: Option<Cow<'a, str>>,

    /// Human-readable name of the resource. Default is an empty string.
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) name: Option<Cow<'a, str>>,

    /// Perfect forward secrecy (PFS). A valid value is `Group2`, `Group5`,
    /// `Group14` to `Group31`. Default is `Group5`.
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub(crate) pfs: Option<Pfs>,

    /// The IKE mode. A valid value is `main`, which is the default.
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub(crate) phase1_negotiation_mode: Option<Phase1NegotiationMode>,

    /// The ID of the project.
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) tenant_id: Option<Cow<'a, str>>,
}

#[derive(Builder, Debug, Clone)]
#[builder(setter(strip_option))]
pub struct Request<'a> {
    /// An `ikepolicy` object.
    ///
    #[builder(setter(into))]
    pub(crate) ikepolicy: Ikepolicy<'a>,

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
    /// Add a single header to the Ikepolicy.
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
        "vpn/ikepolicies".to_string().into()
    }

    fn parameters(&self) -> QueryParams {
        QueryParams::default()
    }

    fn body(&self) -> Result<Option<(&'static str, Vec<u8>)>, BodyError> {
        let mut params = JsonBodyParams::default();

        params.push("ikepolicy", serde_json::to_value(&self.ikepolicy)?);

        params.into_body()
    }

    fn service_type(&self) -> ServiceType {
        ServiceType::Network
    }

    fn response_key(&self) -> Option<Cow<'static, str>> {
        Some("ikepolicy".into())
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
                .ikepolicy(IkepolicyBuilder::default().build().unwrap())
                .build()
                .unwrap()
                .service_type(),
            ServiceType::Network
        );
    }

    #[test]
    fn test_response_key() {
        assert_eq!(
            Request::builder()
                .ikepolicy(IkepolicyBuilder::default().build().unwrap())
                .build()
                .unwrap()
                .response_key()
                .unwrap(),
            "ikepolicy"
        );
    }

    #[cfg(feature = "sync")]
    #[test]
    fn endpoint() {
        let client = MockServerClient::new();
        let mock = client.server.mock(|when, then| {
            when.method(httpmock::Method::POST)
                .path("/vpn/ikepolicies".to_string());

            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({ "ikepolicy": {} }));
        });

        let endpoint = Request::builder()
            .ikepolicy(IkepolicyBuilder::default().build().unwrap())
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
                .path("/vpn/ikepolicies".to_string())
                .header("foo", "bar")
                .header("not_foo", "not_bar");
            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({ "ikepolicy": {} }));
        });

        let endpoint = Request::builder()
            .ikepolicy(IkepolicyBuilder::default().build().unwrap())
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