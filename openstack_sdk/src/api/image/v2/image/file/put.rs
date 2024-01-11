//! Uploads binary image data. (Since Image API v2.0)
//! Set the Content-Type request header to application/octet-stream.
//! A multiple store backend support is introduced in the Rocky release as a
//! part of the EXPERIMENTAL Image API v2.8.
//! Beginning with API version 2.8, an optional X-Image-Meta-Store header may
//! be added to the request. When present, the image data will be placed into
//! the backing store whose identifier is the value of this header. If the
//! store identifier specified is not recognized, a 400 (Bad Request) response
//! is returned. When the header is not present, the image data is placed into
//! the default backing store.
//! Store identifiers are site-specific. Use the Store Discovery call to
//! determine what stores are available in a particular cloud.
//! The default store may be determined from the Store Discovery response.
//! A default store is always defined, so if you do not have a need to use a
//! particular store, simply omit this header and the default store will be
//! used.
//! For API versions before version 2.8, this header is silently ignored.
//! Preconditions
//! Before you can store binary image data, you must meet the following
//! preconditions:
//!
//!   - The image must exist.
//!
//!   - You must set the disk and container formats in the image.
//!
//!   - The image status must be queued.
//!
//!   - Your image storage quota must be sufficient.
//!
//!   - The size of the data that you want to store must not exceed the size
//! that
//!     the OpenStack Image service allows.
//!
//! Synchronous Postconditions:
//!
//!   - With correct permissions, you can see the image status as active
//! through
//!     API calls.
//!
//!   - With correct access, you can see the stored data in the storage system
//!     that the OpenStack Image Service manages.
//!
//! Troubleshooting
//!
//!   - If you cannot store the data, either your request lacks required
//!     information or you exceeded your allotted quota. Ensure that you meet
//! the
//!     preconditions and run the request again. If the request fails again,
//!     review your API request.
//!
//!   - The storage back ends for storing the data must have enough free
//! storage
//!     space to accommodate the size of the data.
use derive_builder::Builder;
use http::{HeaderMap, HeaderName, HeaderValue};

use crate::api::common::CommaSeparatedList;
use crate::api::rest_endpoint_prelude::*;

/// Query for image.upload operation.
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
    fn method(&self) -> http::Method {
        http::Method::PUT
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("images/{id}/file", id = self.id.as_ref(),).into()
    }

    fn parameters(&self) -> QueryParams {
        QueryParams::default()
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
    use serde::Deserialize;
    use serde_json::json;

    #[test]
    fn test_service_type() {
        assert_eq!(
            Image::builder().build().unwrap().service_type(),
            ServiceType::Image
        );
    }

    #[test]
    fn test_response_key() {
        assert!(Image::builder().build().unwrap().response_key().is_none())
    }

    #[test]
    fn endpoint() {
        let client = MockServerClient::new();
        let mock = client.server.mock(|when, then| {
            when.method(httpmock::Method::PUT)
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
            when.method(httpmock::Method::PUT)
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
