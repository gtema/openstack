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

//! The API provides a unified request for creating a remote console. The user
//! can get a URL to connect the console from this API. The URL includes the
//! token which is used to get permission to access the console. Servers may
//! support different console protocols. To return a remote console using a
//! specific protocol, such as VNC, set the `protocol` parameter to `vnc`.
//!
//! Normal response codes: 200
//!
//! Error response codes: badRequest(400), unauthorized(401), forbidden(403),
//! itemNotFound(404), conflict(409), notImplemented(501)
//!
use derive_builder::Builder;
use http::{HeaderMap, HeaderName, HeaderValue};

use crate::api::rest_endpoint_prelude::*;

use serde::Deserialize;
use serde::Serialize;
use std::borrow::Cow;

#[derive(Debug, Deserialize, Clone, Serialize)]
pub enum Protocol {
    #[serde(rename = "serial")]
    Serial,
    #[serde(rename = "spice")]
    Spice,
    #[serde(rename = "vnc")]
    Vnc,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub enum Type {
    #[serde(rename = "novnc")]
    Novnc,
    #[serde(rename = "serial")]
    Serial,
    #[serde(rename = "spice-html5")]
    SpiceHtml5,
    #[serde(rename = "xvpvnc")]
    Xvpvnc,
}

/// The remote console object.
///
#[derive(Builder, Debug, Deserialize, Clone, Serialize)]
#[builder(setter(strip_option))]
pub struct RemoteConsole {
    /// The protocol of remote console. The valid values are `vnc`, `spice`,
    /// `serial` and `mks`. The protocol `mks` is added since Microversion
    /// `2.8`.
    ///
    #[serde()]
    #[builder()]
    pub(crate) protocol: Protocol,

    /// The type of remote console. The valid values are `novnc`,
    /// `spice-html5`, `spice-direct`, `serial`, and `webmks`. The type
    /// `webmks` was added in Microversion `2.8`, and the type `spice-direct`
    /// was added in Microversion `2.99`.
    ///
    #[serde(rename = "type")]
    #[builder()]
    pub(crate) _type: Type,
}

#[derive(Builder, Debug, Clone)]
#[builder(setter(strip_option))]
pub struct Request<'a> {
    /// The remote console object.
    ///
    #[builder(setter(into))]
    pub(crate) remote_console: RemoteConsole,

    /// server_id parameter for /v2.1/servers/{server_id}/remote-consoles API
    ///
    #[builder(default, setter(into))]
    server_id: Cow<'a, str>,

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
    /// Add a single header to the Remote_Console.
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
        format!(
            "servers/{server_id}/remote-consoles",
            server_id = self.server_id.as_ref(),
        )
        .into()
    }

    fn parameters(&self) -> QueryParams {
        QueryParams::default()
    }

    fn body(&self) -> Result<Option<(&'static str, Vec<u8>)>, BodyError> {
        let mut params = JsonBodyParams::default();

        params.push(
            "remote_console",
            serde_json::to_value(&self.remote_console)?,
        );

        params.into_body()
    }

    fn service_type(&self) -> ServiceType {
        ServiceType::Compute
    }

    fn response_key(&self) -> Option<Cow<'static, str>> {
        Some("remote_console".into())
    }

    /// Returns headers to be set into the request
    fn request_headers(&self) -> Option<&HeaderMap> {
        self._headers.as_ref()
    }

    /// Returns required API version
    fn api_version(&self) -> Option<ApiVersion> {
        Some(ApiVersion::new(2, 6))
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
                .remote_console(
                    RemoteConsoleBuilder::default()
                        ._type(Type::Novnc)
                        .protocol(Protocol::Serial)
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
                .remote_console(
                    RemoteConsoleBuilder::default()
                        ._type(Type::Novnc)
                        .protocol(Protocol::Serial)
                        .build()
                        .unwrap()
                )
                .build()
                .unwrap()
                .response_key()
                .unwrap(),
            "remote_console"
        );
    }

    #[cfg(feature = "sync")]
    #[test]
    fn endpoint() {
        let server = MockServer::start();
        let client = FakeOpenStackClient::new(server.base_url());
        let mock = server.mock(|when, then| {
            when.method(httpmock::Method::POST).path(format!(
                "/servers/{server_id}/remote-consoles",
                server_id = "server_id",
            ));

            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({ "remote_console": {} }));
        });

        let endpoint = Request::builder()
            .server_id("server_id")
            .remote_console(
                RemoteConsoleBuilder::default()
                    ._type(Type::Novnc)
                    .protocol(Protocol::Serial)
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
                .path(format!(
                    "/servers/{server_id}/remote-consoles",
                    server_id = "server_id",
                ))
                .header("foo", "bar")
                .header("not_foo", "not_bar");
            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({ "remote_console": {} }));
        });

        let endpoint = Request::builder()
            .server_id("server_id")
            .remote_console(
                RemoteConsoleBuilder::default()
                    ._type(Type::Novnc)
                    .protocol(Protocol::Serial)
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
