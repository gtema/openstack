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

//! Create an attachment.
//!
//! This method can be used to create an empty attachment (reserve) or to
//! create and initialize a volume attachment based on the provided input
//! parameters.
//!
//! If the caller does not yet have the connector information but needs to
//! reserve an attachment for the volume (ie Nova BootFromVolume) the create
//! can be called with just the volume-uuid and the server identifier. This
//! will reserve an attachment, mark the volume as reserved and prevent any new
//! attachment_create calls from being made until the attachment is updated
//! (completed).
//!
//! The alternative is that the connection can be reserved and initialized all
//! at once with a single call if the caller has all of the required
//! information (connector data) at the time of the call.
//!
//! NOTE: In Nova terms server == instance, the server_id parameter referenced
//! below is the UUID of the Instance, for non-nova consumers this can be a
//! server UUID or some other arbitrary unique identifier.
//!
//! Starting from microversion 3.54, we can pass the attach mode as argument in
//! the request body.
//!
//! Expected format of the input parameter 'body':
//!
//! ```text
//!
//! {
//!     "attachment":
//!     {
//!         "volume_uuid": "volume-uuid",
//!         "instance_uuid": "null|nova-server-uuid",
//!         "connector": "null|<connector-object>",
//!         "mode": "null|rw|ro"
//!     }
//! }
//!
//! ```
//!
//! Example connector:
//!
//! ```text
//!
//! {
//!     "connector":
//!     {
//!         "initiator": "iqn.1993-08.org.debian:01:cad181614cec",
//!         "ip": "192.168.1.20",
//!         "platform": "x86_64",
//!         "host": "tempest-1",
//!         "os_type": "linux2",
//!         "multipath": false,
//!         "mountpoint": "/dev/vdb",
//!         "mode": "null|rw|ro"
//!     }
//! }
//!
//! ```
//!
//! NOTE all that's required for a reserve is volume_uuid and an instance_uuid.
//!
//! returns: A summary view of the attachment object
//!
use derive_builder::Builder;
use http::{HeaderMap, HeaderName, HeaderValue};

use crate::api::rest_endpoint_prelude::*;

use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;
use std::borrow::Cow;
use std::collections::BTreeMap;

#[derive(Debug, Deserialize, Clone, Serialize)]
pub enum Mode {
    #[serde(rename = "ro")]
    Ro,
    #[serde(rename = "rw")]
    Rw,
}

/// An attachment object.
///
#[derive(Builder, Debug, Deserialize, Clone, Serialize)]
#[builder(setter(strip_option))]
pub struct Attachment<'a> {
    /// The `connector` object.
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, private, setter(name = "_connector"))]
    pub(crate) connector: Option<Option<BTreeMap<Cow<'a, str>, Value>>>,

    /// The UUID of the volume which the attachment belongs to.
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) instance_uuid: Option<Cow<'a, str>>,

    /// The attach mode of attachment, acceptable values are read-only (‘ro’)
    /// and read-and-write (‘rw’).
    ///
    /// **New in version 3.54**
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub(crate) mode: Option<Mode>,

    /// The UUID of the volume which the attachment belongs to.
    ///
    #[serde()]
    #[builder(setter(into))]
    pub(crate) volume_uuid: Cow<'a, str>,
}

impl<'a> AttachmentBuilder<'a> {
    /// The `connector` object.
    ///
    pub fn connector<I, K, V>(&mut self, iter: I) -> &mut Self
    where
        I: Iterator<Item = (K, V)>,
        K: Into<Cow<'a, str>>,
        V: Into<Value>,
    {
        self.connector
            .get_or_insert(None)
            .get_or_insert(None)
            .get_or_insert_with(BTreeMap::new)
            .extend(iter.map(|(k, v)| (k.into(), v.into())));
        self
    }
}

#[derive(Builder, Debug, Clone)]
#[builder(setter(strip_option))]
pub struct Request<'a> {
    /// An attachment object.
    ///
    #[builder(setter(into))]
    pub(crate) attachment: Attachment<'a>,

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
    /// Add a single header to the Attachment.
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
        "attachments".to_string().into()
    }

    fn parameters(&self) -> QueryParams {
        QueryParams::default()
    }

    fn body(&self) -> Result<Option<(&'static str, Vec<u8>)>, BodyError> {
        let mut params = JsonBodyParams::default();

        params.push("attachment", serde_json::to_value(&self.attachment)?);

        params.into_body()
    }

    fn service_type(&self) -> ServiceType {
        ServiceType::BlockStorage
    }

    fn response_key(&self) -> Option<Cow<'static, str>> {
        Some("attachment".into())
    }

    /// Returns headers to be set into the request
    fn request_headers(&self) -> Option<&HeaderMap> {
        self._headers.as_ref()
    }

    /// Returns required API version
    fn api_version(&self) -> Option<ApiVersion> {
        Some(ApiVersion::new(3, 54))
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
                .attachment(
                    AttachmentBuilder::default()
                        .volume_uuid("foo")
                        .build()
                        .unwrap()
                )
                .build()
                .unwrap()
                .service_type(),
            ServiceType::BlockStorage
        );
    }

    #[test]
    fn test_response_key() {
        assert_eq!(
            Request::builder()
                .attachment(
                    AttachmentBuilder::default()
                        .volume_uuid("foo")
                        .build()
                        .unwrap()
                )
                .build()
                .unwrap()
                .response_key()
                .unwrap(),
            "attachment"
        );
    }

    #[cfg(feature = "sync")]
    #[test]
    fn endpoint() {
        let client = MockServerClient::new();
        let mock = client.server.mock(|when, then| {
            when.method(httpmock::Method::POST)
                .path("/attachments".to_string());

            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({ "attachment": {} }));
        });

        let endpoint = Request::builder()
            .attachment(
                AttachmentBuilder::default()
                    .volume_uuid("foo")
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
        let client = MockServerClient::new();
        let mock = client.server.mock(|when, then| {
            when.method(httpmock::Method::POST)
                .path("/attachments".to_string())
                .header("foo", "bar")
                .header("not_foo", "not_bar");
            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({ "attachment": {} }));
        });

        let endpoint = Request::builder()
            .attachment(
                AttachmentBuilder::default()
                    .volume_uuid("foo")
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
