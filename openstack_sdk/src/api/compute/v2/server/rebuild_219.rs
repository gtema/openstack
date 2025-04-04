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

use derive_builder::Builder;
use http::{HeaderMap, HeaderName, HeaderValue};

use crate::api::rest_endpoint_prelude::*;

use serde::Deserialize;
use serde::Serialize;
use std::borrow::Cow;
use std::collections::BTreeMap;

#[derive(Debug, Deserialize, Clone, Serialize)]
pub enum OsDcfDiskConfig {
    #[serde(rename = "AUTO")]
    Auto,
    #[serde(rename = "MANUAL")]
    Manual,
}

#[derive(Builder, Debug, Deserialize, Clone, Serialize)]
#[builder(setter(strip_option))]
pub struct Personality<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) contents: Option<Cow<'a, str>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) path: Option<Cow<'a, str>>,
}

/// The action to rebuild a server.
///
#[derive(Builder, Debug, Deserialize, Clone, Serialize)]
#[builder(setter(strip_option))]
pub struct Rebuild<'a> {
    /// IPv4 address that should be used to access this server.
    ///
    #[serde(rename = "accessIPv4", skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) access_ipv4: Option<Cow<'a, str>>,

    /// IPv6 address that should be used to access this server.
    ///
    #[serde(rename = "accessIPv6", skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) access_ipv6: Option<Cow<'a, str>>,

    /// The administrative password of the server. If you omit this parameter,
    /// the operation generates a new password.
    ///
    #[serde(rename = "adminPass", skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) admin_pass: Option<Cow<'a, str>>,

    /// A free form description of the server. Limited to 255 characters in
    /// length. Before microversion 2.19 this was set to the server name.
    ///
    /// **New in version 2.19**
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) description: Option<Option<Cow<'a, str>>>,

    /// The UUID of the image to rebuild for your server instance. It must be a
    /// valid UUID otherwise API will return 400. To rebuild a volume-backed
    /// server with a new image, at least microversion 2.93 needs to be
    /// provided in the request else the request will fall back to old
    /// behaviour i.e. the API will return 400 (for an image different from the
    /// image used when creating the volume). For non-volume-backed servers,
    /// specifying a new image will result in validating that the image is
    /// acceptable for the current compute host on which the server exists. If
    /// the new image is not valid, the server will go into `ERROR` status.
    ///
    #[serde(rename = "imageRef")]
    #[builder(setter(into))]
    pub(crate) image_ref: Cow<'a, str>,

    /// Metadata key and value pairs. The maximum size of the metadata key and
    /// value is 255 bytes each.
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, private, setter(name = "_metadata"))]
    pub(crate) metadata: Option<BTreeMap<Cow<'a, str>, Cow<'a, str>>>,

    /// The server name.
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) name: Option<Cow<'a, str>>,

    /// Controls how the API partitions the disk when you create, rebuild, or
    /// resize servers. A server inherits the `OS-DCF:diskConfig` value from
    /// the image from which it was created, and an image inherits the
    /// `OS-DCF:diskConfig` value from the server from which it was created. To
    /// override the inherited setting, you can include this attribute in the
    /// request body of a server create, rebuild, or resize request. If the
    /// `OS-DCF:diskConfig` value for an image is `MANUAL`, you cannot create a
    /// server from that image and set its `OS-DCF:diskConfig` value to `AUTO`.
    /// A valid value is:
    ///
    /// - `AUTO`. The API builds the server with a single partition the size of
    ///   the target flavor disk. The API automatically adjusts the file system
    ///   to fit the entire partition.
    /// - `MANUAL`. The API builds the server by using whatever partition
    ///   scheme and file system is in the source image. If the target flavor
    ///   disk is larger, the API does not partition the remaining disk space.
    ///
    #[serde(rename = "OS-DCF:diskConfig", skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub(crate) os_dcf_disk_config: Option<OsDcfDiskConfig>,

    /// The file path and contents, text only, to inject into the server at
    /// launch. The maximum size of the file path data is 255 bytes. The
    /// maximum limit is the number of allowed bytes in the decoded, rather
    /// than encoded, data.
    ///
    /// **Available until version 2.56**
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) personality: Option<Vec<Personality<'a>>>,

    /// Indicates whether the server is rebuilt with the preservation of the
    /// ephemeral partition (`true`).
    ///
    /// Note
    ///
    /// This only works with baremetal servers provided by Ironic. Passing it
    /// to any other server instance results in a fault and will prevent the
    /// rebuild from happening.
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub(crate) preserve_ephemeral: Option<bool>,
}

impl<'a> RebuildBuilder<'a> {
    /// Metadata key and value pairs. The maximum size of the metadata key and
    /// value is 255 bytes each.
    ///
    pub fn metadata<I, K, V>(&mut self, iter: I) -> &mut Self
    where
        I: Iterator<Item = (K, V)>,
        K: Into<Cow<'a, str>>,
        V: Into<Cow<'a, str>>,
    {
        self.metadata
            .get_or_insert(None)
            .get_or_insert_with(BTreeMap::new)
            .extend(iter.map(|(k, v)| (k.into(), v.into())));
        self
    }
}

#[derive(Builder, Debug, Clone)]
#[builder(setter(strip_option))]
pub struct Request<'a> {
    /// The action to rebuild a server.
    ///
    #[builder(setter(into))]
    pub(crate) rebuild: Rebuild<'a>,

    /// id parameter for /v2.1/servers/{id}/action API
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

impl RequestBuilder<'_> {
    /// Add a single header to the Server.
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
        format!("servers/{id}/action", id = self.id.as_ref(),).into()
    }

    fn parameters(&self) -> QueryParams {
        QueryParams::default()
    }

    fn body(&self) -> Result<Option<(&'static str, Vec<u8>)>, BodyError> {
        let mut params = JsonBodyParams::default();

        params.push("rebuild", serde_json::to_value(&self.rebuild)?);

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
        Some(ApiVersion::new(2, 19))
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
                .rebuild(RebuildBuilder::default().image_ref("foo").build().unwrap())
                .build()
                .unwrap()
                .service_type(),
            ServiceType::Compute
        );
    }

    #[test]
    fn test_response_key() {
        assert!(Request::builder()
            .rebuild(RebuildBuilder::default().image_ref("foo").build().unwrap())
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
                .path(format!("/servers/{id}/action", id = "id",));

            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({ "dummy": {} }));
        });

        let endpoint = Request::builder()
            .id("id")
            .rebuild(RebuildBuilder::default().image_ref("foo").build().unwrap())
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
                .path(format!("/servers/{id}/action", id = "id",))
                .header("foo", "bar")
                .header("not_foo", "not_bar");
            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({ "dummy": {} }));
        });

        let endpoint = Request::builder()
            .id("id")
            .rebuild(RebuildBuilder::default().image_ref("foo").build().unwrap())
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
