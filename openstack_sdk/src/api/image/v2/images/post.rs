//! Creates a catalog record for an operating system disk image. (Since Image
//! API v2.0)
//! The Location response header contains the URI for the image.
//! A multiple store backend support is introduced in the Rocky release as a
//! part of the EXPERIMENTAL Image API v2.8. Since Image API v2.8 a new header
//! OpenStack-image-store-ids which contains the list of available stores will
//! be included in response. This header is only included if multiple backend
//! stores are supported.
//! The response body contains the new image entity.
//! Synchronous Postconditions
//! With correct permissions, you can see the image status as queued through
//! API calls.
use derive_builder::Builder;
use http::{HeaderMap, HeaderName, HeaderValue};
use std::collections::BTreeSet;

use crate::api::common::CommaSeparatedList;
use crate::api::rest_endpoint_prelude::*;

/// Query for image.post operation.
#[derive(Debug, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct Image<'a> {
    /// The container format refers to whether the VM image is in a file format
    /// that also contains metadata about the actual VM. Container formats
    /// include OVF and Amazon AMI. In addition, a VM image might not have a
    /// container format - instead, the image is just a blob of unstructured
    /// data.
    #[builder(default, setter(into))]
    container_format: Option<Cow<'a, str>>,

    /// The format of the disk.
    /// Values may vary based on the configuration available in a particular
    /// OpenStack cloud. See the Image Schema response from the cloud itself
    /// for the valid values available.
    /// Example formats are: ami, ari, aki, vhd, vhdx, vmdk, raw, qcow2,
    ///   vdi, ploop or iso.
    ///
    /// The value might be null (JSON null data type).
    /// Newton changes: The vhdx disk format is a supported value. Ocata
    /// changes: The ploop disk format is a supported value.
    #[builder(default, setter(into))]
    disk_format: Option<Cow<'a, str>>,

    /// Amount of disk space in GB that is required to boot the image.
    #[builder(default)]
    min_disk: Option<u32>,

    /// Amount of RAM in MB that is required to boot the image.
    #[builder(default)]
    min_ram: Option<u32>,

    /// The name of the image.
    #[builder(default, setter(into))]
    name: Option<Cow<'a, str>>,

    /// Image protection for deletion. Valid value is true or false. Default is
    /// false.
    #[builder(default)]
    is_protected: Option<bool>,

    /// List of tags for this image. Each tag is a string of at most 255 chars.
    /// The maximum number of tags allowed on an image is set by the operator.
    #[builder(default, private, setter(name = "_tags"))]
    tags: BTreeSet<Cow<'a, str>>,

    /// Visibility for this image. Valid value is one of: ``public``,
    /// ``private``, ``shared``, or ``community``. At most sites, only an
    /// administrator can make an image public. Some sites may restrict what
    /// users can make an image community. Some sites may restrict what users
    /// can perform member operations on a shared image. Since the Image API
    /// v2.5, the default value is ``shared``.
    #[builder(default, setter(into))]
    visibility: Option<Cow<'a, str>>,

    #[builder(setter(name = "_headers"), default, private)]
    _headers: Option<HeaderMap>,
}

impl<'a> Image<'a> {
    /// Create a builder for the endpoint.
    pub fn builder() -> ImageBuilder<'a> {
        ImageBuilder::default()
    }
}

impl<'a> ImageBuilder<'a> {
    /// List of tags for this image. Each tag is a string of at most 255 chars.
    /// The maximum number of tags allowed on an image is set by the operator.
    pub fn tags<I, T>(&mut self, iter: I) -> &mut Self
    where
        I: Iterator<Item = T>,
        T: Into<Cow<'a, str>>,
    {
        self.tags
            .get_or_insert_with(BTreeSet::new)
            .extend(iter.map(Into::into));
        self
    }

    /// Add a single header to the Image.
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

impl<'a> RestEndpoint for Image<'a> {
    fn method(&self) -> Method {
        Method::POST
    }

    fn endpoint(&self) -> Cow<'static, str> {
        "images".to_string().into()
    }

    fn parameters(&self) -> QueryParams {
        QueryParams::default()
    }

    fn body(&self) -> Result<Option<(&'static str, Vec<u8>)>, BodyError> {
        let mut params = JsonBodyParams::default();

        params.push_opt("container_format", self.container_format.as_ref());
        params.push_opt("disk_format", self.disk_format.as_ref());
        params.push_opt("min_disk", self.min_disk);
        params.push_opt("min_ram", self.min_ram);
        params.push_opt("name", self.name.as_ref());
        params.push_opt("protected", self.is_protected);
        params.push("tags", &self.tags);
        params.push_opt("visibility", self.visibility.as_ref());

        params.into_body()
    }

    fn service_type(&self) -> Cow<'static, str> {
        "image".into()
    }

    fn response_key(&self) -> Option<Cow<'static, str>> {
        None
    }

    /// Returns headers to be set into the request
    fn request_headers(&self) -> Option<&HeaderMap> {
        self._headers.as_ref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::{self, Query, RawQuery};
    use crate::test::client::MockServerClient;
    use http::{HeaderName, HeaderValue};
    use serde::Deserialize;
    use serde_json::json;

    #[test]
    fn test_service_type() {
        assert_eq!(Image::builder().build().unwrap().service_type(), "image");
    }

    #[test]
    fn test_response_key() {
        assert!(Image::builder().build().unwrap().response_key().is_none())
    }

    #[test]
    fn endpoint() {
        let client = MockServerClient::new();
        let mock = client.server.mock(|when, then| {
            when.method(httpmock::Method::POST)
                .path(format!("/images",));

            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({ "dummy": {} }));
        });

        let endpoint = Image::builder().build().unwrap();
        let _: serde_json::Value = endpoint.query(&client).unwrap();
        mock.assert();
    }

    #[test]
    fn endpoint_headers() {
        let client = MockServerClient::new();
        let mock = client.server.mock(|when, then| {
            when.method(httpmock::Method::POST)
                .path(format!("/images",))
                .header("foo", "bar")
                .header("not_foo", "not_bar");
            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({ "dummy": {} }));
        });

        let endpoint = Image::builder()
            .headers(
                [(
                    Some(HeaderName::from_static("foo")),
                    HeaderValue::from_static("bar"),
                )]
                .iter()
                .cloned(),
            )
            .header("not_foo", "not_bar")
            .build()
            .unwrap();
        let _: serde_json::Value = endpoint.query(&client).unwrap();
        mock.assert();
    }

    #[test]
    fn endpoint_body() {
        let endpoint = Image::builder()
            .container_format("container_format")
            .disk_format("disk_format")
            .name("name")
            .tags(["tags"].iter().cloned())
            .visibility("visibility")
            .build()
            .unwrap();

        let (mime, body) = endpoint.body().unwrap().unwrap();
        assert_eq!(
            std::str::from_utf8(&body).unwrap(),
            json!({
                 "container_format": "container_format",
                 "disk_format": "disk_format",
                 "name": "name",
                 "tags": ["tags"],
                 "visibility": "visibility",
            })
            .to_string()
        );
    }
}
