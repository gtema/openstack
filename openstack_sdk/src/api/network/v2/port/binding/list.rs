//! Normal response codes: 200
//!
//! Error response codes: 401, 404
//!
use derive_builder::Builder;
use http::{HeaderMap, HeaderName, HeaderValue};

use crate::api::rest_endpoint_prelude::*;
use serde::Serialize;

use std::borrow::Cow;

use crate::api::Pageable;
#[derive(Builder, Debug, Clone)]
#[builder(setter(strip_option))]
pub struct Request<'a> {
    /// port_id parameter for /v2.0/ports/{port_id}/add_allowed_address_pairs
    /// API
    #[builder(setter(into), default)]
    port_id: Cow<'a, str>,

    /// host query parameter for /v2.0/ports/{port_id}/bindings API
    #[builder(setter(into), default)]
    host: Option<Cow<'a, str>>,

    /// vif_type query parameter for /v2.0/ports/{port_id}/bindings API
    #[builder(setter(into), default)]
    vif_type: Option<Cow<'a, str>>,

    /// vnic_type query parameter for /v2.0/ports/{port_id}/bindings API
    #[builder(setter(into), default)]
    vnic_type: Option<Cow<'a, str>>,

    /// status query parameter for /v2.0/ports/{port_id}/bindings API
    #[builder(setter(into), default)]
    status: Option<Cow<'a, str>>,

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
    /// Add a single header to the Binding.
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
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!(
            "v2.0/ports/{port_id}/bindings",
            port_id = self.port_id.as_ref(),
        )
        .into()
    }

    fn parameters(&self) -> QueryParams {
        let mut params = QueryParams::default();
        params.push_opt("host", self.host.as_ref());
        params.push_opt("vif_type", self.vif_type.as_ref());
        params.push_opt("vnic_type", self.vnic_type.as_ref());
        params.push_opt("status", self.status.as_ref());

        params
    }

    fn service_type(&self) -> ServiceType {
        ServiceType::Network
    }

    fn response_key(&self) -> Option<Cow<'static, str>> {
        Some("bindings".into())
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
            Request::builder().build().unwrap().service_type(),
            ServiceType::Network
        );
    }

    #[test]
    fn test_response_key() {
        assert_eq!(
            Request::builder().build().unwrap().response_key().unwrap(),
            "bindings"
        );
    }

    #[test]
    fn endpoint() {
        let client = MockServerClient::new();
        let mock = client.server.mock(|when, then| {
            when.method(httpmock::Method::GET).path(format!(
                "/v2.0/ports/{port_id}/bindings",
                port_id = "port_id",
            ));

            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({ "bindings": {} }));
        });

        let endpoint = Request::builder().port_id("port_id").build().unwrap();
        let _: serde_json::Value = endpoint.query(&client).unwrap();
        mock.assert();
    }

    #[test]
    fn endpoint_headers() {
        let client = MockServerClient::new();
        let mock = client.server.mock(|when, then| {
            when.method(httpmock::Method::GET)
                .path(format!(
                    "/v2.0/ports/{port_id}/bindings",
                    port_id = "port_id",
                ))
                .header("foo", "bar")
                .header("not_foo", "not_bar");
            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({ "bindings": {} }));
        });

        let endpoint = Request::builder()
            .port_id("port_id")
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
