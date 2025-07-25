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

//! Creates a health monitor on a pool.
//!
//! Health monitors define how the load balancer monitors backend servers to
//! determine if they are available to service requests.
//!
//! This operation provisions a new health monitor by using the configuration
//! that you define in the request object. After the API validates the request
//! and starts the provisioning process, the API returns a response object that
//! contains a unique ID and the status of provisioning the health monitor.
//!
//! In the response, the health monitor [provisioning status](#prov-status) is
//! `ACTIVE`, `PENDING_CREATE`, or `ERROR`.
//!
//! If the status is `PENDING_CREATE`, issue GET
//! `/v2/lbaas/healthmonitors/{healthmonitor_id}` to view the progress of the
//! provisioning operation. When the health monitor status changes to `ACTIVE`,
//! the health monitor is successfully provisioned and is ready for further
//! configuration.
//!
//! If the API cannot fulfill the request due to insufficient data or data that
//! is not valid, the service returns the HTTP `Bad Request (400)` response
//! code with information about the failure in the response body. Validation
//! errors require that you correct the error and submit the request again.
//!
//! Specifying a project_id is deprecated. The health monitor will inherit the
//! project_id of the parent load balancer.
//!
//! At a minimum, you must specify these health monitor attributes:
//!
//! Some attributes receive default values if you omit them from the request:
//!
//! To create a health monitor, the parent load balancer must have an `ACTIVE`
//! provisioning status.
//!
use derive_builder::Builder;
use http::{HeaderMap, HeaderName, HeaderValue};

use crate::api::rest_endpoint_prelude::*;

use serde::Deserialize;
use serde::Serialize;
use std::borrow::Cow;

#[derive(Debug, Deserialize, Clone, Serialize)]
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

#[derive(Debug, Deserialize, Clone, Serialize)]
pub enum Type {
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

/// Defines mandatory and optional attributes of a POST request.
#[derive(Builder, Debug, Deserialize, Clone, Serialize)]
#[builder(setter(strip_option))]
pub struct Healthmonitor<'a> {
    /// The administrative state of the resource, which is up (`true`) or down
    /// (`false`). Default is `true`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) admin_state_up: Option<bool>,

    /// The time, in seconds, between sending probes to members.
    #[serde()]
    #[builder(setter(into))]
    pub(crate) delay: i32,

    /// The domain name, which be injected into the HTTP Host Header to the
    /// backend server for HTTP health check.
    ///
    /// **New in version 2.10**
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) domain_name: Option<Cow<'a, str>>,

    /// The list of HTTP status codes expected in response from the member to
    /// declare it healthy. Specify one of the following values:
    ///
    /// - A single value, such as `200`
    /// - A list, such as `200, 202`
    /// - A range, such as `200-204`
    ///
    /// The default is 200.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) expected_codes: Option<Cow<'a, str>>,

    /// The HTTP method that the health monitor uses for requests. One of
    /// `CONNECT`, `DELETE`, `GET`, `HEAD`, `OPTIONS`, `PATCH`, `POST`, `PUT`,
    /// or `TRACE`. The default is `GET`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub(crate) http_method: Option<HttpMethod>,

    /// The HTTP version. One of `1.0` or `1.1`. The default is `1.0`.
    ///
    /// **New in version 2.10**
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) http_version: Option<f32>,

    /// The number of successful checks before changing the `operating status`
    /// of the member to `ONLINE`. A valid value is from `1` to `10`.
    #[serde()]
    #[builder(setter(into))]
    pub(crate) max_retries: i32,

    /// The number of allowed check failures before changing the
    /// `operating status` of the member to `ERROR`. A valid value is from `1`
    /// to `10`. The default is `3`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) max_retries_down: Option<i32>,

    /// Human-readable name of the resource.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) name: Option<Cow<'a, str>>,

    /// The ID of the pool.
    #[serde()]
    #[builder(setter(into))]
    pub(crate) pool_id: Cow<'a, str>,

    /// The ID of the project owning this resource. (deprecated)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) project_id: Option<Cow<'a, str>>,

    /// A list of simple strings assigned to the resource.
    ///
    /// **New in version 2.5**
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) tags: Option<Vec<Cow<'a, str>>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) tenant_id: Option<Cow<'a, str>>,

    /// The maximum time, in seconds, that a monitor waits to connect before it
    /// times out. This value must be less than the delay value.
    #[serde()]
    #[builder(setter(into))]
    pub(crate) timeout: i32,

    /// The type of health monitor. One of `HTTP`, `HTTPS`, `PING`, `SCTP`,
    /// `TCP`, `TLS-HELLO`, or `UDP-CONNECT`.
    #[serde(rename = "type")]
    #[builder()]
    pub(crate) _type: Type,

    /// The HTTP URL path of the request sent by the monitor to test the health
    /// of a backend member. Must be a string that begins with a forward slash
    /// (`/`). The default URL path is `/`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) url_path: Option<Cow<'a, str>>,
}

#[derive(Builder, Debug, Clone)]
#[builder(setter(strip_option))]
pub struct Request<'a> {
    /// Defines mandatory and optional attributes of a POST request.
    #[builder(setter(into))]
    pub(crate) healthmonitor: Healthmonitor<'a>,

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
    /// Add a single header to the Healthmonitor.
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
        "lbaas/healthmonitors".to_string().into()
    }

    fn parameters(&self) -> QueryParams {
        QueryParams::default()
    }

    fn body(&self) -> Result<Option<(&'static str, Vec<u8>)>, BodyError> {
        let mut params = JsonBodyParams::default();

        params.push("healthmonitor", serde_json::to_value(&self.healthmonitor)?);

        params.into_body()
    }

    fn service_type(&self) -> ServiceType {
        ServiceType::LoadBalancer
    }

    fn response_key(&self) -> Option<Cow<'static, str>> {
        Some("healthmonitor".into())
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
                .healthmonitor(
                    HealthmonitorBuilder::default()
                        ._type(Type::Http)
                        .delay(123)
                        .max_retries(123)
                        .pool_id("foo")
                        .timeout(123)
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
                .healthmonitor(
                    HealthmonitorBuilder::default()
                        ._type(Type::Http)
                        .delay(123)
                        .max_retries(123)
                        .pool_id("foo")
                        .timeout(123)
                        .build()
                        .unwrap()
                )
                .build()
                .unwrap()
                .response_key()
                .unwrap(),
            "healthmonitor"
        );
    }

    #[cfg(feature = "sync")]
    #[test]
    fn endpoint() {
        let server = MockServer::start();
        let client = FakeOpenStackClient::new(server.base_url());
        let mock = server.mock(|when, then| {
            when.method(httpmock::Method::POST)
                .path("/lbaas/healthmonitors".to_string());

            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({ "healthmonitor": {} }));
        });

        let endpoint = Request::builder()
            .healthmonitor(
                HealthmonitorBuilder::default()
                    ._type(Type::Http)
                    .delay(123)
                    .max_retries(123)
                    .pool_id("foo")
                    .timeout(123)
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
                .path("/lbaas/healthmonitors".to_string())
                .header("foo", "bar")
                .header("not_foo", "not_bar");
            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({ "healthmonitor": {} }));
        });

        let endpoint = Request::builder()
            .healthmonitor(
                HealthmonitorBuilder::default()
                    ._type(Type::Http)
                    .delay(123)
                    .max_retries(123)
                    .pool_id("foo")
                    .timeout(123)
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
