//! Imports (or generates) a keypair.
//!
//! Warning: Generating a keypair is no longer possible starting from
//!   version 2.92.
//!
//! Normal response codes: 200, 201
//! Note: The success status code was changed from 200 to 201 in version
//!   2.2
use derive_builder::Builder;
use http::{HeaderMap, HeaderName, HeaderValue};

use crate::api::common::CommaSeparatedList;
use crate::api::rest_endpoint_prelude::*;

/// Query for keypairs.post operation.
#[derive(Debug, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct Keypairs<'a> {
    /// A name for the keypair which will be used to reference it later.
    /// Note: Since microversion 2.92, allowed characters are ASCII letters
    /// [a-zA-Z], digits [0-9] and the following special characters: [@._- ].
    #[builder(default, setter(into))]
    name: Cow<'a, str>,

    /// The public ssh key to import. Was optional before microversion 2.92 :
    /// if you were omitting this value, a keypair was generated for you.
    #[builder(default, setter(into))]
    public_key: Cow<'a, str>,

    /// The type of the keypair. Allowed values are ssh or x509.
    /// New in version 2.2
    #[builder(default, setter(into))]
    xtype: Option<Cow<'a, str>>,

    /// The user_id for a keypair.
    #[builder(default, setter(into))]
    user_id: Option<Cow<'a, str>>,

    #[builder(setter(name = "_headers"), default, private)]
    _headers: Option<HeaderMap>,
}

impl<'a> Keypairs<'a> {
    /// Create a builder for the endpoint.
    pub fn builder() -> KeypairsBuilder<'a> {
        KeypairsBuilder::default()
    }
}

impl<'a> KeypairsBuilder<'a> {
    /// Add a single header to the Keypairs.
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

impl<'a> RestEndpoint for Keypairs<'a> {
    fn method(&self) -> http::Method {
        http::Method::POST
    }

    fn endpoint(&self) -> Cow<'static, str> {
        "os-keypairs".to_string().into()
    }

    fn parameters(&self) -> QueryParams {
        QueryParams::default()
    }

    fn body(&self) -> Result<Option<(&'static str, Vec<u8>)>, BodyError> {
        let mut params = JsonBodyParams::default();

        params.push("name", self.name.as_ref());
        params.push("public_key", self.public_key.as_ref());
        params.push_opt("type", self.xtype.as_ref());
        params.push_opt("user_id", self.user_id.as_ref());
        params.into_body_with_root_key("keypair")
    }

    fn service_type(&self) -> ServiceType {
        ServiceType::Compute
    }

    fn response_key(&self) -> Option<Cow<'static, str>> {
        Some("keypair".into())
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
            Keypairs::builder().build().unwrap().service_type(),
            ServiceType::Compute
        );
    }

    #[test]
    fn test_response_key() {
        assert_eq!(
            Keypairs::builder().build().unwrap().response_key().unwrap(),
            "keypair"
        );
    }

    #[test]
    fn endpoint() {
        let client = MockServerClient::new();
        let mock = client.server.mock(|when, then| {
            when.method(httpmock::Method::POST)
                .path(format!("/os-keypairs",));

            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({ "keypair": {} }));
        });

        let endpoint = Keypairs::builder().build().unwrap();
        let _: serde_json::Value = endpoint.query(&client).unwrap();
        mock.assert();
    }

    #[test]
    fn endpoint_headers() {
        let client = MockServerClient::new();
        let mock = client.server.mock(|when, then| {
            when.method(httpmock::Method::POST)
                .path(format!("/os-keypairs",))
                .header("foo", "bar")
                .header("not_foo", "not_bar");
            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({ "keypair": {} }));
        });

        let endpoint = Keypairs::builder()
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
        let endpoint = Keypairs::builder()
            .name("name")
            .public_key("public_key")
            .xtype("type")
            .user_id("user_id")
            .build()
            .unwrap();

        let (mime, body) = endpoint.body().unwrap().unwrap();
        assert_eq!(
            std::str::from_utf8(&body).unwrap(),
            json!({
              "keypair": {
                 "name": "name",
                 "public_key": "public_key",
                 "type": "type",
                 "user_id": "user_id",
             }
            })
            .to_string()
        );
    }
}
