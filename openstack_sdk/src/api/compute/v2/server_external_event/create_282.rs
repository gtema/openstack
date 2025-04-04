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

//! Creates one or more external events, which the API dispatches to the host a
//! server is assigned to. If the server is not currently assigned to a host
//! the event will not be delivered.
//!
//! You will receive back the list of events that you submitted, with an
//! updated `code` and `status` indicating their level of success.
//!
//! Normal response codes: 200, 207
//!
//! A 200 will be returned if all events succeeded, 207 will be returned if any
//! events could not be processed. The `code` attribute for the event will
//! explain further what went wrong.
//!
//! Error response codes: badRequest(400), unauthorized(401), forbidden(403)
//!
use derive_builder::Builder;
use http::{HeaderMap, HeaderName, HeaderValue};

use crate::api::rest_endpoint_prelude::*;

use serde::Deserialize;
use serde::Serialize;
use std::borrow::Cow;

#[derive(Debug, Deserialize, Clone, Serialize)]
pub enum Name {
    #[serde(rename = "accelerator-request-bound")]
    AcceleratorRequestBound,
    #[serde(rename = "network-changed")]
    NetworkChanged,
    #[serde(rename = "network-vif-deleted")]
    NetworkVifDeleted,
    #[serde(rename = "network-vif-plugged")]
    NetworkVifPlugged,
    #[serde(rename = "network-vif-unplugged")]
    NetworkVifUnplugged,
    #[serde(rename = "power-update")]
    PowerUpdate,
    #[serde(rename = "volume-extended")]
    VolumeExtended,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub enum Status {
    #[serde(rename = "completed")]
    Completed,
    #[serde(rename = "failed")]
    Failed,
    #[serde(rename = "in-progress")]
    InProgress,
}

#[derive(Builder, Debug, Deserialize, Clone, Serialize)]
#[builder(setter(strip_option))]
pub struct Events<'a> {
    /// The event name. A valid value is:
    ///
    /// - `network-changed`
    /// - `network-vif-plugged`
    /// - `network-vif-unplugged`
    /// - `network-vif-deleted`
    /// - `volume-extended` (since microversion `2.51`)
    /// - `power-update` (since microversion `2.76`)
    /// - `accelerator-request-bound` (since microversion `2.82`)
    ///
    #[serde()]
    #[builder()]
    pub(crate) name: Name,

    /// The UUID of the server instance to which the API dispatches the event.
    /// You must assign this instance to a host. Otherwise, this call does not
    /// dispatch the event to the instance.
    ///
    #[serde()]
    #[builder(setter(into))]
    pub(crate) server_uuid: Cow<'a, str>,

    /// The event status. A valid value is `failed`, `completed`, or
    /// `in-progress`. Default is `completed`.
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub(crate) status: Option<Status>,

    /// A string value that identifies the event. Certain types of events
    /// require specific tags:
    ///
    /// - For the `accelerator-request-bound` event, the tag must be the
    ///   accelerator request UUID.
    /// - For the `power-update` event the tag must be either be `POWER_ON` or
    ///   `POWER_OFF`.
    /// - For the `volume-extended` event the tag must be the volume id.
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) tag: Option<Cow<'a, str>>,
}

#[derive(Builder, Debug, Clone)]
#[builder(setter(strip_option))]
pub struct Request<'a> {
    /// List of external events to process.
    ///
    #[builder(setter(into))]
    pub(crate) events: Vec<Events<'a>>,

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
    /// Add a single header to the Server_External_Event.
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
        "os-server-external-events".to_string().into()
    }

    fn parameters(&self) -> QueryParams {
        QueryParams::default()
    }

    fn body(&self) -> Result<Option<(&'static str, Vec<u8>)>, BodyError> {
        let mut params = JsonBodyParams::default();

        params.push("events", serde_json::to_value(&self.events)?);

        params.into_body()
    }

    fn service_type(&self) -> ServiceType {
        ServiceType::Compute
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
        Some(ApiVersion::new(2, 82))
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
                .events(Vec::from([EventsBuilder::default()
                    .name(Name::AcceleratorRequestBound)
                    .server_uuid("foo")
                    .build()
                    .unwrap()]))
                .build()
                .unwrap()
                .service_type(),
            ServiceType::Compute
        );
    }

    #[test]
    fn test_response_key() {
        assert!(Request::builder()
            .events(Vec::from([EventsBuilder::default()
                .name(Name::AcceleratorRequestBound)
                .server_uuid("foo")
                .build()
                .unwrap()]))
            .build()
            .unwrap()
            .response_key()
            .is_none())
    }

    #[cfg(feature = "sync")]
    #[test]
    fn endpoint() {
        let server = MockServer::start();
        let client = FakeOpenStackClient::new(server.base_url());
        let mock = server.mock(|when, then| {
            when.method(httpmock::Method::POST)
                .path("/os-server-external-events".to_string());

            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({ "dummy": {} }));
        });

        let endpoint = Request::builder()
            .events(Vec::from([EventsBuilder::default()
                .name(Name::AcceleratorRequestBound)
                .server_uuid("foo")
                .build()
                .unwrap()]))
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
                .path("/os-server-external-events".to_string())
                .header("foo", "bar")
                .header("not_foo", "not_bar");
            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({ "dummy": {} }));
        });

        let endpoint = Request::builder()
            .events(Vec::from([EventsBuilder::default()
                .name(Name::AcceleratorRequestBound)
                .server_uuid("foo")
                .build()
                .unwrap()]))
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
