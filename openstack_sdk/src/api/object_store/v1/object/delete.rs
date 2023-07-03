//! Permanently deletes an object from the object store.
//! Object deletion occurs immediately at request time. Any subsequent GET,
//! HEAD, POST, or DELETE operations will return a 404 Not Found error code.
//! For static large object manifests, you can add the ?multipart-
//! manifest=delete query parameter. This operation deletes the segment objects
//! and, if all deletions succeed, this operation deletes the manifest object.
//! A DELETE request made to a symlink path will delete the symlink rather than
//! the target object.
//! An alternative to using the DELETE operation is to use the POST operation
//! with the bulk-delete query parameter.
use derive_builder::Builder;
use http::{HeaderMap, HeaderName, HeaderValue};
use std::collections::BTreeSet;

use crate::api::common::CommaSeparatedList;
use crate::api::rest_endpoint_prelude::*;

/// Query for object.delete operation.
#[derive(Debug, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct Object<'a> {
    /// The unique name for the account. An account is also known as the
    /// project or tenant.
    #[builder(default, setter(into))]
    container: Cow<'a, str>,

    /// The unique name for the object.
    #[builder(default, setter(into))]
    object: Cow<'a, str>,

    /// If you include the multipart-manifest=get query parameter and the
    /// object is a large object, the object contents are not returned.
    /// Instead, the manifest is returned in the X-Object-Manifest response
    /// header for dynamic large objects or in the response body for static
    /// large objects.
    #[builder(default, setter(into))]
    multipart_manifest: Option<Cow<'a, str>>,

    #[builder(setter(name = "_headers"), default, private)]
    _headers: Option<HeaderMap>,
}

impl<'a> Object<'a> {
    /// Create a builder for the endpoint.
    pub fn builder() -> ObjectBuilder<'a> {
        ObjectBuilder::default()
    }
}

impl<'a> ObjectBuilder<'a> {
    /// Add a single header to the Object.
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

impl<'a> RestEndpoint for Object<'a> {
    fn method(&self) -> Method {
        Method::DELETE
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!(
            "{container}/{object}",
            container = self.container.as_ref(),
            object = self.object.as_ref(),
        )
        .into()
    }

    fn parameters(&self) -> QueryParams {
        let mut params = QueryParams::default();
        params.push_opt("multipart-manifest", self.multipart_manifest.as_ref());

        params
    }

    fn service_type(&self) -> Cow<'static, str> {
        "object-store".into()
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
        assert_eq!(
            Object::builder().build().unwrap().service_type(),
            "object-store"
        );
    }

    #[test]
    fn test_response_key() {
        assert!(Object::builder().build().unwrap().response_key().is_none())
    }

    #[test]
    fn endpoint() {
        let client = MockServerClient::new();
        let mock = client.server.mock(|when, then| {
            when.method(httpmock::Method::DELETE).path(format!(
                "/{container}/{object}",
                container = "container",
                object = "object",
            ));

            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({ "dummy": {} }));
        });

        let endpoint = Object::builder()
            .container("container")
            .object("object")
            .build()
            .unwrap();
        let _: serde_json::Value = endpoint.query(&client).unwrap();
        mock.assert();
    }

    #[test]
    fn endpoint_headers() {
        let client = MockServerClient::new();
        let mock = client.server.mock(|when, then| {
            when.method(httpmock::Method::DELETE)
                .path(format!(
                    "/{container}/{object}",
                    container = "container",
                    object = "object",
                ))
                .header("foo", "bar")
                .header("not_foo", "not_bar");
            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({ "dummy": {} }));
        });

        let endpoint = Object::builder()
            .container("container")
            .object("object")
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
