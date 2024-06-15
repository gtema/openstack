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

//! Update an existing pool.
//!
//! If the request is valid, the service returns the `Accepted (202)` response
//! code. To confirm the update, check that the pool provisioning status is
//! `ACTIVE`. If the status is `PENDING_UPDATE`, use a GET operation to poll
//! the pool object for changes.
//!
//! This operation returns the updated pool object with the `ACTIVE`,
//! `PENDING_UPDATE`, or `ERROR` provisioning status.
//!
use derive_builder::Builder;
use http::{HeaderMap, HeaderName, HeaderValue};

use crate::api::rest_endpoint_prelude::*;

use serde::Deserialize;
use serde::Serialize;
use std::borrow::Cow;

#[derive(Debug, Deserialize, Clone, Serialize)]
pub enum LbAlgorithm {
    #[serde(rename = "LEAST_CONNECTIONS")]
    LeastConnections,
    #[serde(rename = "ROUND_ROBIN")]
    RoundRobin,
    #[serde(rename = "SOURCE_IP")]
    SourceIp,
    #[serde(rename = "SOURCE_IP_PORT")]
    SourceIpPort,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub enum Type {
    #[serde(rename = "APP_COOKIE")]
    AppCookie,
    #[serde(rename = "HTTP_COOKIE")]
    HttpCookie,
    #[serde(rename = "SOURCE_IP")]
    SourceIp,
}

/// A JSON object specifying the session persistence for the pool or `null` for
/// no session persistence. See
/// [Pool Session Persistence](#session-persistence). Default is `null`.
///
#[derive(Builder, Debug, Deserialize, Clone, Serialize)]
#[builder(setter(strip_option))]
pub struct SessionPersistence<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) cookie_name: Option<Cow<'a, str>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) persistence_granularity: Option<Cow<'a, str>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub(crate) persistence_timeout: Option<i32>,

    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub(crate) _type: Option<Type>,
}

/// Defines attributes that are acceptable of a PUT request.
///
#[derive(Builder, Debug, Deserialize, Clone, Serialize)]
#[builder(setter(strip_option))]
pub struct Pool<'a> {
    /// The administrative state of the resource, which is up (`true`) or down
    /// (`false`). Default is `true`.
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub(crate) admin_state_up: Option<bool>,

    /// A list of ALPN protocols. Available protocols: http/1.0, http/1.1, h2
    ///
    /// **New in version 2.24**
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) alpn_protocols: Option<Vec<Cow<'a, str>>>,

    /// The reference of the
    /// [key manager service](https://docs.openstack.org/castellan/latest/)
    /// secret containing a PEM format CA certificate bundle for `tls_enabled`
    /// pools.
    ///
    /// **New in version 2.8**
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) ca_tls_container_ref: Option<Cow<'a, str>>,

    /// The reference of the
    /// [key manager service](https://docs.openstack.org/castellan/latest/)
    /// secret containing a PEM format CA revocation list file for
    /// `tls_enabled` pools.
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) crl_container_ref: Option<Cow<'a, str>>,

    /// A human-readable description for the resource.
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) description: Option<Cow<'a, str>>,

    /// The load balancing algorithm for the pool. One of `LEAST_CONNECTIONS`,
    /// `ROUND_ROBIN`, or `SOURCE_IP`.
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub(crate) lb_algorithm: Option<LbAlgorithm>,

    /// Human-readable name of the resource.
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) name: Option<Cow<'a, str>>,

    /// A JSON object specifying the session persistence for the pool or `null`
    /// for no session persistence. See
    /// [Pool Session Persistence](#session-persistence). Default is `null`.
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) session_persistence: Option<SessionPersistence<'a>>,

    /// A list of simple strings assigned to the resource.
    ///
    /// **New in version 2.5**
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) tags: Option<Vec<Cow<'a, str>>>,

    /// List of ciphers in OpenSSL format (colon-separated). See
    /// <https://www.openssl.org/docs/man1.1.1/man1/ciphers.html>
    ///
    /// **New in version 2.15**
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) tls_ciphers: Option<Cow<'a, str>>,

    /// The reference to the
    /// [key manager service](https://docs.openstack.org/castellan/latest/)
    /// secret containing a PKCS12 format certificate/key bundle for
    /// `tls_enabled` pools for TLS client authentication to the member
    /// servers.
    ///
    /// **New in version 2.8**
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) tls_container_ref: Option<Cow<'a, str>>,

    /// When `true` connections to backend member servers will use TLS
    /// encryption. Default is `false`.
    ///
    /// **New in version 2.8**
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub(crate) tls_enabled: Option<bool>,

    /// A list of TLS protocol versions. Available versions: SSLv3, TLSv1,
    /// TLSv1.1, TLSv1.2, TLSv1.3
    ///
    /// **New in version 2.17**
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) tls_versions: Option<Vec<Cow<'a, str>>>,
}

#[derive(Builder, Debug, Clone)]
#[builder(setter(strip_option))]
pub struct Request<'a> {
    /// Defines attributes that are acceptable of a PUT request.
    ///
    #[builder(setter(into))]
    pub(crate) pool: Pool<'a>,

    /// pool_id parameter for /v2/lbaas/pools/{pool_id} API
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
    /// Add a single header to the Pool.
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
        format!("lbaas/pools/{id}", id = self.id.as_ref(),).into()
    }

    fn parameters(&self) -> QueryParams {
        QueryParams::default()
    }

    fn body(&self) -> Result<Option<(&'static str, Vec<u8>)>, BodyError> {
        let mut params = JsonBodyParams::default();

        params.push("pool", serde_json::to_value(&self.pool)?);

        params.into_body()
    }

    fn service_type(&self) -> ServiceType {
        ServiceType::LoadBalancer
    }

    fn response_key(&self) -> Option<Cow<'static, str>> {
        Some("pool".into())
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
                .pool(PoolBuilder::default().build().unwrap())
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
                .pool(PoolBuilder::default().build().unwrap())
                .build()
                .unwrap()
                .response_key()
                .unwrap(),
            "pool"
        );
    }

    #[cfg(feature = "sync")]
    #[test]
    fn endpoint() {
        let client = MockServerClient::new();
        let mock = client.server.mock(|when, then| {
            when.method(httpmock::Method::PUT)
                .path(format!("/lbaas/pools/{id}", id = "id",));

            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({ "pool": {} }));
        });

        let endpoint = Request::builder()
            .id("id")
            .pool(PoolBuilder::default().build().unwrap())
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
            when.method(httpmock::Method::PUT)
                .path(format!("/lbaas/pools/{id}", id = "id",))
                .header("foo", "bar")
                .header("not_foo", "not_bar");
            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({ "pool": {} }));
        });

        let endpoint = Request::builder()
            .id("id")
            .pool(PoolBuilder::default().build().unwrap())
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
