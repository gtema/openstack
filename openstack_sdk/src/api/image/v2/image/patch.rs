//! Updates an image.
//! *(Since Image API v2.0)*
//!
//! Conceptually, you update an image record by patching the JSON
//! representation of
//! the image, passing a request body conforming to one of the following media
//! types:
//!
//! Attempting to make a PATCH call using some other media type will provoke a
//! response code of 415 (Unsupported media type).
//!
//! The `application/openstack-images-v2.1-json-patch` media type provides a
//! useful and compatible subset of the functionality defined in JavaScript
//! Object
//! Notation (JSON) Patch [RFC6902](http://tools.ietf.org/html/rfc6902), which
//! defines the `application/json-patch+json` media type.
//!
//! For information about the PATCH method and the available media types, see
//! [Image API v2 HTTP PATCH media
//! types](http://specs.openstack.org/openstack/glance-specs/specs/api/v2/http-
//! patch-image-api-v2.html).
//!
//! Attempting to modify some image properties will cause the entire request to
//! fail with a 403 (Forbidden) response code:
//!
//! Attempting to add a location path to an image that is not in `queued` or
//! `active` state will result in a 409 (Conflict) response code
//! *(since Image API v2.4)*.
//!
//! Normal response codes: 200
//!
//! Error response codes: 400, 401, 403, 404, 409, 413, 415
//!
use derive_builder::Builder;
use http::{HeaderMap, HeaderName, HeaderValue};

use crate::api::rest_endpoint_prelude::*;
use serde::Serialize;

use std::borrow::Cow;

use json_patch::Patch;
#[derive(Builder, Debug, Clone)]
#[builder(setter(strip_option))]
pub struct Request<'a> {
    /// image_id parameter for /v2/images/{image_id}/members/{member_id} API
    #[builder(default, setter(into))]
    id: Cow<'a, str>,

    /// Patch data
    #[builder()]
    patch: Patch,

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

impl<'a> RestEndpoint for Request<'a> {
    fn method(&self) -> http::Method {
        http::Method::PATCH
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("v2/images/{id}", id = self.id.as_ref(),).into()
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

    fn service_type(&self) -> ServiceType {
        ServiceType::Image
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
    use crate::types::ServiceType;
    use http::{HeaderName, HeaderValue};
    use json_patch::Patch;
    use serde::Deserialize;
    use serde::Serialize;
    use serde_json::from_value;
    use serde_json::json;

    #[test]
    fn test_service_type() {
        assert_eq!(
            Request::builder()
                .patch(from_value::<Patch>(json!([])).unwrap())
                .build()
                .unwrap()
                .service_type(),
            ServiceType::Image
        );
    }

    #[test]
    fn test_response_key() {
        assert!(Request::builder()
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
                .path(format!("/v2/images/{id}", id = "id",));

            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({ "dummy": {} }));
        });

        let endpoint = Request::builder()
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
                .path(format!("/v2/images/{id}", id = "id",))
                .header("foo", "bar")
                .header("not_foo", "not_bar");
            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({ "dummy": {} }));
        });

        let endpoint = Request::builder()
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
