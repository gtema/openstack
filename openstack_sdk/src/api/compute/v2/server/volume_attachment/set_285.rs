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

//! Update a volume attachment.
//!
//! Policy default role is ‘rule:system_admin_or_owner’, its scope is \[system,
//! project\], which allow project members or system admins to change the
//! fields of an attached volume of a server. Policy defaults enable only users
//! with the administrative role to change `volumeId` via this operation. Cloud
//! providers can change these permissions through the `policy.json` file.
//!
//! Updating, or what is commonly referred to as “swapping”, volume attachments
//! with volumes that have more than one read/write attachment, is not
//! supported.
//!
//! Normal response codes: 202
//!
//! Error response codes: badRequest(400), unauthorized(401), forbidden(403),
//! itemNotFound(404), conflict(409)
//!
use derive_builder::Builder;
use http::{HeaderMap, HeaderName, HeaderValue};

use crate::api::rest_endpoint_prelude::*;

use serde::Deserialize;
use serde::Serialize;
use std::borrow::Cow;

/// A dictionary representation of a volume attachment containing the field
/// `volumeId` which is the UUID of the replacement volume, and other fields to
/// update in the attachment.
///
#[derive(Builder, Debug, Deserialize, Clone, Serialize)]
#[builder(setter(strip_option))]
pub struct VolumeAttachment<'a> {
    /// A flag indicating if the attached volume will be deleted when the
    /// server is deleted.
    ///
    /// **New in version 2.85**
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub(crate) delete_on_termination: Option<bool>,

    /// Name of the device in the attachment object, such as, `/dev/vdb`.
    ///
    /// **New in version 2.85**
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) device: Option<Option<Cow<'a, str>>>,

    /// The UUID of the attachment.
    ///
    /// **New in version 2.85**
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) id: Option<Cow<'a, str>>,

    /// The UUID of the server.
    ///
    /// **New in version 2.85**
    ///
    #[serde(rename = "serverId", skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) server_id: Option<Cow<'a, str>>,

    /// The device tag applied to the volume block device or `null`.
    ///
    /// **New in version 2.85**
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) tag: Option<Cow<'a, str>>,

    /// The UUID of the volume to attach instead of the attached volume.
    ///
    #[serde(rename = "volumeId")]
    #[builder(setter(into))]
    pub(crate) volume_id: Cow<'a, str>,
}

#[derive(Builder, Debug, Clone)]
#[builder(setter(strip_option))]
pub struct Request<'a> {
    /// A dictionary representation of a volume attachment containing the field
    /// `volumeId` which is the UUID of the replacement volume, and other
    /// fields to update in the attachment.
    ///
    #[builder(setter(into))]
    pub(crate) volume_attachment: VolumeAttachment<'a>,

    /// id parameter for /v2.1/servers/{server_id}/os-volume_attachments/{id}
    /// API
    ///
    #[builder(default, setter(into))]
    id: Cow<'a, str>,

    /// server_id parameter for
    /// /v2.1/servers/{server_id}/os-volume_attachments/{id} API
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

impl<'a> RequestBuilder<'a> {
    /// Add a single header to the Volume_Attachment.
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
        format!(
            "v2.1/servers/{server_id}/os-volume_attachments/{id}",
            server_id = self.server_id.as_ref(),
            id = self.id.as_ref(),
        )
        .into()
    }

    fn parameters(&self) -> QueryParams {
        QueryParams::default()
    }

    fn body(&self) -> Result<Option<(&'static str, Vec<u8>)>, BodyError> {
        let mut params = JsonBodyParams::default();

        params.push(
            "volumeAttachment",
            serde_json::to_value(&self.volume_attachment)?,
        );

        params.into_body()
    }

    fn service_type(&self) -> ServiceType {
        ServiceType::Compute
    }

    fn response_key(&self) -> Option<Cow<'static, str>> {
        Some("volumeAttachment".into())
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
                .volume_attachment(
                    VolumeAttachmentBuilder::default()
                        .volume_id("foo")
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
                .volume_attachment(
                    VolumeAttachmentBuilder::default()
                        .volume_id("foo")
                        .build()
                        .unwrap()
                )
                .build()
                .unwrap()
                .response_key()
                .unwrap(),
            "volumeAttachment"
        );
    }

    #[test]
    fn endpoint() {
        let client = MockServerClient::new();
        let mock = client.server.mock(|when, then| {
            when.method(httpmock::Method::PUT).path(format!(
                "/v2.1/servers/{server_id}/os-volume_attachments/{id}",
                server_id = "server_id",
                id = "id",
            ));

            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({ "volumeAttachment": {} }));
        });

        let endpoint = Request::builder()
            .server_id("server_id")
            .id("id")
            .volume_attachment(
                VolumeAttachmentBuilder::default()
                    .volume_id("foo")
                    .build()
                    .unwrap(),
            )
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
                .path(format!(
                    "/v2.1/servers/{server_id}/os-volume_attachments/{id}",
                    server_id = "server_id",
                    id = "id",
                ))
                .header("foo", "bar")
                .header("not_foo", "not_bar");
            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({ "volumeAttachment": {} }));
        });

        let endpoint = Request::builder()
            .server_id("server_id")
            .id("id")
            .volume_attachment(
                VolumeAttachmentBuilder::default()
                    .volume_id("foo")
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
