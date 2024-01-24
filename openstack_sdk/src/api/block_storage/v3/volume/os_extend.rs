use derive_builder::Builder;
use http::{HeaderMap, HeaderName, HeaderValue};

use crate::api::rest_endpoint_prelude::*;
use serde::Serialize;

use serde::Deserialize;
use std::borrow::Cow;

#[derive(Builder, Debug, Deserialize, Clone, Serialize)]
#[builder(setter(strip_option))]
pub struct OsExtend {
    #[serde()]
    #[builder()]
    pub(crate) new_size: i32,
}

#[derive(Builder, Debug, Clone)]
#[builder(setter(strip_option))]
pub struct Request<'a> {
    #[builder(setter(into))]
    pub(crate) os_extend: OsExtend,

    /// id parameter for /v3/volumes/{id} API
    #[builder(default, setter(into))]
    id: Cow<'a, str>,

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
    /// Add a single header to the Volume.
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
        http::Method::POST
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("v3/volumes/{id}/action", id = self.id.as_ref(),).into()
    }

    fn parameters(&self) -> QueryParams {
        QueryParams::default()
    }

    fn body(&self) -> Result<Option<(&'static str, Vec<u8>)>, BodyError> {
        let mut params = JsonBodyParams::default();

        params.push("os-extend", serde_json::to_value(&self.os_extend)?);

        params.into_body()
    }

    fn service_type(&self) -> ServiceType {
        ServiceType::BlockStorage
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
    use serde::Serialize;
    use serde_json::json;

    #[test]
    fn test_service_type() {
        assert_eq!(
            Request::builder()
                .os_extend(OsExtendBuilder::default().new_size(123).build().unwrap())
                .build()
                .unwrap()
                .service_type(),
            ServiceType::BlockStorage
        );
    }

    #[test]
    fn test_response_key() {
        assert!(Request::builder()
            .os_extend(OsExtendBuilder::default().new_size(123).build().unwrap())
            .build()
            .unwrap()
            .response_key()
            .is_none())
    }

    #[test]
    fn endpoint() {
        let client = MockServerClient::new();
        let mock = client.server.mock(|when, then| {
            when.method(httpmock::Method::POST)
                .path(format!("/v3/volumes/{id}/action", id = "id",));

            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({ "dummy": {} }));
        });

        let endpoint = Request::builder()
            .id("id")
            .os_extend(OsExtendBuilder::default().new_size(123).build().unwrap())
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
                .path(format!("/v3/volumes/{id}/action", id = "id",))
                .header("foo", "bar")
                .header("not_foo", "not_bar");
            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({ "dummy": {} }));
        });

        let endpoint = Request::builder()
            .id("id")
            .os_extend(OsExtendBuilder::default().new_size(123).build().unwrap())
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
