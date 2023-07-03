//! Updates an image. (Since Image API v2.0)
//! Conceptually, you update an image record by patching the JSON
//! representation of the image, passing a request body conforming to one of
//! the following media types:
//!
//!   - application/openstack-images-v2.0-json-patch (deprecated)
//!
//!   - application/openstack-images-v2.1-json-patch (since Image API v2.1)
//!
//! Attempting to make a PATCH call using some other media type will provoke a
//! response code of 415 (Unsupported media type).
//! The application/openstack-images-v2.1-json-patch media type provides a
//! useful and compatible subset of the functionality defined in JavaScript
//! Object Notation (JSON) Patch RFC6902, which defines the application/json-
//! patch+json media type.
//! For information about the PATCH method and the available media types, see
//! Image API v2 HTTP PATCH media types.
//! Attempting to modify some image properties will cause the entire request to
//! fail with a 403 (Forbidden) response code:
//!
//!   - An attempt to modify any of the “base” image properties that are
//!     managed by the Image Service. These are the properties specified as
//!     read only in the Image Schema.
//!
//!   - An attempt to create or modify image properties for which you do
//!     not have permission to do so (since Image API v2.2). This depends
//!     upon how property protections are configured in the OpenStack cloud
//!     in which you are making the call. Consult your cloud’s
//!     documentation for details.
//!
//!   - An attempt to delete the only image location, or to replace the
//!     image locations with an empty list (since Image API v2.4).
//!
//!   - An attempt to set or modify a property with a reserved name, such as
//!     anything prefixed with the os_glance namespace.
//!
//! Attempting to add a location path to an image that is not in queued or
//! active state will result in a 409 (Conflict) response code (since Image API
//! v2.4).
use derive_builder::Builder;
use http::{HeaderMap, HeaderName, HeaderValue};
use std::collections::BTreeSet;

use crate::api::common::CommaSeparatedList;
use crate::api::rest_endpoint_prelude::*;

use json_patch::Patch;

/// Query for image.patch operation.
#[derive(Debug, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct Image<'a> {
    /// Image ID
    #[builder(default, setter(into))]
    id: Cow<'a, str>,

    /// Patch data
    #[builder()]
    patch: Patch,

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
        Method::PATCH
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("images/{id}", id = self.id.as_ref(),).into()
    }

    fn parameters(&self) -> QueryParams {
        QueryParams::default()
    }

    fn body(&self) -> Result<Option<(&'static str, Vec<u8>)>, BodyError> {
        Ok(Some((
            "application/openstack-images-v2.1-json-patch",
            serde_json::to_string(&self.patch)?.into_bytes(),
        )))
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
    use json_patch::Patch;
    use serde::Deserialize;
    use serde_json::from_value;
    use serde_json::json;

    #[test]
    fn test_service_type() {
        assert_eq!(
            Image::builder()
                .patch(from_value::<Patch>(json!([])).unwrap())
                .build()
                .unwrap()
                .service_type(),
            "image"
        );
    }

    #[test]
    fn test_response_key() {
        assert!(Image::builder()
            .patch(from_value::<Patch>(json!([])).unwrap())
            .build()
            .unwrap()
            .response_key()
            .is_none())
    }

    #[test]
    fn endpoint() {
        let client = MockServerClient::new();
        let mock = client.server.mock(|when, then| {
            when.method(httpmock::Method::PATCH)
                .path(format!("/images/{id}", id = "id",));

            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({ "dummy": {} }));
        });

        let endpoint = Image::builder()
            .id("id")
            .patch(from_value::<Patch>(json!([])).unwrap())
            .build()
            .unwrap();
        let _: serde_json::Value = endpoint.query(&client).unwrap();
        mock.assert();
    }

    #[test]
    fn endpoint_headers() {
        let client = MockServerClient::new();
        let mock = client.server.mock(|when, then| {
            when.method(httpmock::Method::PATCH)
                .path(format!("/images/{id}", id = "id",))
                .header("foo", "bar")
                .header("not_foo", "not_bar");
            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({ "dummy": {} }));
        });

        let endpoint = Image::builder()
            .id("id")
            .patch(from_value::<Patch>(json!([])).unwrap())
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
