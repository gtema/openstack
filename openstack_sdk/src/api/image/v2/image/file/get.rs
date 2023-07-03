//! Downloads binary image data. (Since Image API v2.0)
//! Example call: `curl -i -X GET -H "X-Auth-Token: $token"
//!   $image_url/v2/images/{image_id}/file`
//!
//! The response body contains the raw binary data that represents the actual
//! virtual disk. The Content-Type header contains the application/octet-stream
//! value. The Content-MD5 header contains an MD5 checksum of the image data.
//! Use this checksum to verify the integrity of the image data.
//! Preconditions:
//!
//!   - The image must exist.
//!
//! Synchronous Postconditions:
//!
//!   - You can download the binary image data in your machine if the image
//!     has image data.
//!
//!   - If image data exists, the call returns the HTTP 200 response code
//!     for a full image download request.
//!
//!   - If image data exists, the call returns the HTTP 206 response code
//!     for a partial download request.
//!
//!   - If no image data exists, the call returns the HTTP 204 (No Content)
//!     response code.
//!
//!   - If no image record exists, the call returns the HTTP 404 response
//!     code for an attempted full image download request.
//!
//!   - For an unsatisfiable partial download request, the call returns the
//!     HTTP 416 response code.
use derive_builder::Builder;
use http::{HeaderMap, HeaderName, HeaderValue};
use std::collections::BTreeSet;

use crate::api::common::CommaSeparatedList;
use crate::api::rest_endpoint_prelude::*;

/// Query for image.download operation.
#[derive(Debug, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct Image<'a> {
    /// Image ID
    #[builder(default, setter(into))]
    id: Cow<'a, str>,

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
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("images/{id}/file", id = self.id.as_ref(),).into()
    }

    fn parameters(&self) -> QueryParams {
        QueryParams::default()
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
            when.method(httpmock::Method::GET)
                .path(format!("/images/{id}/file", id = "id",));

            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({ "dummy": {} }));
        });

        let endpoint = Image::builder().id("id").build().unwrap();
        let _: serde_json::Value = endpoint.query(&client).unwrap();
        mock.assert();
    }

    #[test]
    fn endpoint_headers() {
        let client = MockServerClient::new();
        let mock = client.server.mock(|when, then| {
            when.method(httpmock::Method::GET)
                .path(format!("/images/{id}/file", id = "id",))
                .header("foo", "bar")
                .header("not_foo", "not_bar");
            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({ "dummy": {} }));
        });

        let endpoint = Image::builder()
            .id("id")
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
}
