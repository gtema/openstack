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

//! Creates a pool for a load balancer.
//!
//! The pool defines how requests should be balanced across the backend member
//! servers.
//!
//! This operation provisions a pool by using the configuration that you define
//! in the request object. After the API validates the request and starts the
//! provisioning process, the API returns a response object, which contains a
//! unique ID.
//!
//! In the response, the pool [provisioning status](#prov-status) is `ACTIVE`,
//! `PENDING_CREATE`, or `ERROR`.
//!
//! If the status is `PENDING_CREATE`, issue GET `/v2/lbaas/pools/{pool_id}` to
//! view the progress of the provisioning operation. When the pool status
//! changes to `ACTIVE`, the pool is successfully provisioned and is ready for
//! further configuration.
//!
//! At a minimum, you must specify these pool attributes:
//!
//! Some attributes receive default values if you omit them from the request:
//!
//! If the API cannot fulfill the request due to insufficient data or data that
//! is not valid, the service returns the HTTP `Bad Request (400)` response
//! code with information about the failure in the response body. Validation
//! errors require that you correct the error and submit the request again.
//!
//! Specifying a project_id is deprecated. The pool will inherit the project_id
//! of the parent load balancer.
//!
//! You can configure all documented features of the pool at creation time by
//! specifying the additional elements or attributes in the request.
//!
//! To create a pool, the parent load balancer must have an `ACTIVE`
//! provisioning status.
//!
//! `SOURCE_IP_PORT` algorithm is available from version 2.13.
//!
use derive_builder::Builder;
use http::{HeaderMap, HeaderName, HeaderValue};

use crate::api::rest_endpoint_prelude::*;

use serde::Deserialize;
use serde::Serialize;
use std::borrow::Cow;

#[derive(Debug, Deserialize, Clone, Serialize)]
#[serde(untagged)]
pub enum Protocol {
    #[serde(rename = "HTTP")]
    Http,
    #[serde(rename = "HTTPS")]
    Https,
    #[serde(rename = "PROXY")]
    Proxy,
    #[serde(rename = "PROXYV2")]
    Proxyv2,
    #[serde(rename = "SCTP")]
    Sctp,
    #[serde(rename = "TCP")]
    Tcp,
    #[serde(rename = "UDP")]
    Udp,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
#[serde(untagged)]
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
#[serde(untagged)]
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

    #[serde(rename = "type")]
    #[builder()]
    pub(crate) _type: Type,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
#[serde(untagged)]
pub enum HealthmonitorType {
    #[serde(rename = "HTTP")]
    Http,
    #[serde(rename = "HTTPS")]
    Https,
    #[serde(rename = "PING")]
    Ping,
    #[serde(rename = "SCTP")]
    Sctp,
    #[serde(rename = "TCP")]
    Tcp,
    #[serde(rename = "TLS-HELLO")]
    TlsHello,
    #[serde(rename = "UDP-CONNECT")]
    UdpConnect,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
#[serde(untagged)]
pub enum HttpMethod {
    #[serde(rename = "CONNECT")]
    Connect,
    #[serde(rename = "DELETE")]
    Delete,
    #[serde(rename = "GET")]
    Get,
    #[serde(rename = "HEAD")]
    Head,
    #[serde(rename = "OPTIONS")]
    Options,
    #[serde(rename = "PATCH")]
    Patch,
    #[serde(rename = "POST")]
    Post,
    #[serde(rename = "PUT")]
    Put,
    #[serde(rename = "TRACE")]
    Trace,
}

/// Defines mandatory and optional attributes of a POST request.
///
#[derive(Builder, Debug, Deserialize, Clone, Serialize)]
#[builder(setter(strip_option))]
pub struct Healthmonitor<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub(crate) admin_state_up: Option<bool>,

    #[serde()]
    #[builder()]
    pub(crate) delay: i32,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) domain_name: Option<Cow<'a, str>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) expected_codes: Option<Cow<'a, str>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub(crate) http_method: Option<HttpMethod>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub(crate) http_version: Option<f32>,

    #[serde()]
    #[builder()]
    pub(crate) max_retries: i32,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub(crate) max_retries_down: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) name: Option<Cow<'a, str>>,

    /// A list of simple strings assigned to the resource.
    ///
    /// **New in version 2.5**
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) tags: Option<Vec<Cow<'a, str>>>,

    #[serde()]
    #[builder()]
    pub(crate) timeout: i32,

    #[serde(rename = "type")]
    #[builder()]
    pub(crate) _type: HealthmonitorType,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) url_path: Option<Cow<'a, str>>,
}

/// Defines mandatory and optional attributes of a POST request.
///
#[derive(Builder, Debug, Deserialize, Clone, Serialize)]
#[builder(setter(strip_option))]
pub struct Members<'a> {
    #[serde()]
    #[builder(setter(into))]
    pub(crate) address: Cow<'a, str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub(crate) admin_state_up: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub(crate) backup: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) monitor_address: Option<Cow<'a, str>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub(crate) monitor_port: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) name: Option<Cow<'a, str>>,

    #[serde()]
    #[builder()]
    pub(crate) protocol_port: i32,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub(crate) request_sriov: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) subnet_id: Option<Cow<'a, str>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) tags: Option<Vec<Cow<'a, str>>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub(crate) weight: Option<i32>,
}

/// Defines mandatory and optional attributes of a POST request.
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

    /// Defines mandatory and optional attributes of a POST request.
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) healthmonitor: Option<Healthmonitor<'a>>,

    /// The load balancing algorithm for the pool. One of `LEAST_CONNECTIONS`,
    /// `ROUND_ROBIN`, `SOURCE_IP`, or `SOURCE_IP_PORT`.
    ///
    #[serde()]
    #[builder()]
    pub(crate) lb_algorithm: LbAlgorithm,

    /// The ID of the listener for the pool. Either `listener_id` or
    /// `loadbalancer_id` must be specified. The listener has some
    /// restrictions, See
    /// [Protocol Combinations (Listener/Pool)](#valid-protocol).
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) listener_id: Option<Cow<'a, str>>,

    /// The ID of the load balancer for the pool. Either `listener_id` or
    /// `loadbalancer_id` must be specified.
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) loadbalancer_id: Option<Cow<'a, str>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) members: Option<Vec<Members<'a>>>,

    /// Human-readable name of the resource.
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) name: Option<Cow<'a, str>>,

    /// The ID of the project owning this resource. (deprecated)
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) project_id: Option<Cow<'a, str>>,

    /// The protocol for the resource. One of `HTTP`, `HTTPS`, `PROXY`,
    /// `PROXYV2`, `SCTP`, `TCP`, or `UDP`.
    ///
    #[serde()]
    #[builder()]
    pub(crate) protocol: Protocol,

    /// A JSON object specifying the session persistence for the pool or `null`
    /// for no session persistence. See
    /// [Pool Session Persistence](#session-persistence). Default is `null`.
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) session_persistence: Option<SessionPersistence<'a>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) tags: Option<Vec<Cow<'a, str>>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) tenant_id: Option<Cow<'a, str>>,

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
    /// Defines mandatory and optional attributes of a POST request.
    ///
    #[builder(setter(into))]
    pub(crate) pool: Pool<'a>,

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

impl RestEndpoint for Request<'_> {
    fn method(&self) -> http::Method {
        http::Method::POST
    }

    fn endpoint(&self) -> Cow<'static, str> {
        "lbaas/pools".to_string().into()
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
                .pool(
                    PoolBuilder::default()
                        .lb_algorithm(LbAlgorithm::LeastConnections)
                        .protocol(Protocol::Http)
                        .build()
                        .unwrap()
                )
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
                .pool(
                    PoolBuilder::default()
                        .lb_algorithm(LbAlgorithm::LeastConnections)
                        .protocol(Protocol::Http)
                        .build()
                        .unwrap()
                )
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
        let server = MockServer::start();
        let client = FakeOpenStackClient::new(server.base_url());
        let mock = server.mock(|when, then| {
            when.method(httpmock::Method::POST)
                .path("/lbaas/pools".to_string());

            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({ "pool": {} }));
        });

        let endpoint = Request::builder()
            .pool(
                PoolBuilder::default()
                    .lb_algorithm(LbAlgorithm::LeastConnections)
                    .protocol(Protocol::Http)
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
                .path("/lbaas/pools".to_string())
                .header("foo", "bar")
                .header("not_foo", "not_bar");
            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({ "pool": {} }));
        });

        let endpoint = Request::builder()
            .pool(
                PoolBuilder::default()
                    .lb_algorithm(LbAlgorithm::LeastConnections)
                    .protocol(Protocol::Http)
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
