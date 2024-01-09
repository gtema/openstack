//! Imports (or generates) a keypair.
//!
//! Normal response codes: 200, 201
//!
//! Error response codes: badRequest(400), unauthorized(401), forbidden(403),
//! conflict(409)
//!
use derive_builder::Builder;
use http::{HeaderMap, HeaderName, HeaderValue};

use crate::api::rest_endpoint_prelude::*;
use serde::Serialize;

use serde::Deserialize;
use std::borrow::Cow;

#[derive(Debug, Deserialize, Clone, Serialize)]
#[serde(untagged)]
pub enum Type {
    #[serde(alias = "ssh")]
    Ssh,
    #[serde(alias = "x509")]
    X509,
}

/// Keypair object
#[derive(Builder, Debug, Deserialize, Clone, Serialize)]
#[builder(setter(strip_option))]
pub struct Keypair<'a> {
    /// A name for the keypair which will be used to reference it later.
    ///
    ///
    ///
    /// Note
    ///
    ///
    /// Since microversion 2.92, allowed characters are ASCII letters
    /// `[a-zA-Z]`, digits `[0-9]` and the following special
    /// characters: `[@.\_- ]`.
    #[serde()]
    #[builder(setter(into))]
    name: Cow<'a, str>,

    /// The type of the keypair. Allowed values are `ssh` or `x509`.
    ///
    ///
    /// **New in version 2.2**
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    _type: Option<Type>,

    /// The public ssh key to import.
    /// Was optional before microversion 2.92 : if you were omitting this
    /// value, a
    /// keypair was generated for you.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    public_key: Option<Cow<'a, str>>,

    /// The user\_id for a keypair. This allows administrative users to
    /// upload keys for other users than themselves.
    ///
    ///
    /// **New in version 2.10**
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    user_id: Option<Cow<'a, str>>,
}

#[derive(Builder, Debug, Clone)]
#[builder(setter(strip_option))]
pub struct Request<'a> {
    /// Keypair object
    #[builder(setter(into))]
    keypair: Keypair<'a>,

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
    /// Add a single header to the Os_Keypair.
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
    fn method(&self) -> Method {
        Method::POST
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("v2.1/os-keypairs",).into()
    }

    fn parameters(&self) -> QueryParams {
        QueryParams::default()
    }

    fn body(&self) -> Result<Option<(&'static str, Vec<u8>)>, BodyError> {
        let mut params = JsonBodyParams::default();

        params.push("keypair", serde_json::to_value(&self.keypair)?);

        params.into_body()
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
    use serde::Serialize;
    use serde_json::json;

    #[test]
    fn test_service_type() {
        assert_eq!(
            Request::builder()
                .keypair(KeypairBuilder::default().name("foo").build().unwrap())
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
                .keypair(KeypairBuilder::default().name("foo").build().unwrap())
                .build()
                .unwrap()
                .response_key()
                .unwrap(),
            "keypair"
        );
    }

    #[test]
    fn endpoint() {
        let client = MockServerClient::new();
        let mock = client.server.mock(|when, then| {
            when.method(httpmock::Method::POST)
                .path(format!("/v2.1/os-keypairs",));

            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({ "keypair": {} }));
        });

        let endpoint = Request::builder()
            .keypair(KeypairBuilder::default().name("foo").build().unwrap())
            .build()
            .unwrap();
        let _: serde_json::Value = endpoint.query(&client).unwrap();
        mock.assert();
    }

    #[test]
    fn endpoint_headers() {
        let client = MockServerClient::new();
        let mock = client.server.mock(|when, then| {
            when.method(httpmock::Method::POST)
                .path(format!("/v2.1/os-keypairs",))
                .header("foo", "bar")
                .header("not_foo", "not_bar");
            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({ "keypair": {} }));
        });

        let endpoint = Request::builder()
            .keypair(KeypairBuilder::default().name("foo").build().unwrap())
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
