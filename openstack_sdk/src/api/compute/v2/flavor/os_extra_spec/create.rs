//! Creates extra specs for a flavor, by ID.
//!
//! Normal response codes: 200
//!
//! Error response codes: unauthorized(401), forbidden(403), itemNotFound(404),
//! conflict(409)
//!
use derive_builder::Builder;
use http::{HeaderMap, HeaderName, HeaderValue};

use crate::api::rest_endpoint_prelude::*;
use serde::Serialize;

use serde::Deserialize;
use std::borrow::Cow;
use std::collections::BTreeMap;

#[derive(Builder, Debug, Clone)]
#[builder(setter(strip_option))]
pub struct Request<'a> {
    /// A dictionary of the flavor’s extra-specs key-and-value pairs. It
    /// appears
    /// in the os-extra-specs’ “create” REQUEST body, as well as the
    /// os-extra-specs’ “create” and “list” RESPONSE body.
    #[builder(private, setter(name = "_extra_specs"))]
    extra_specs: BTreeMap<Cow<'a, str>, Cow<'a, str>>,

    /// flavor_id parameter for /v2.1/flavors/{flavor_id}/os-flavor-access API
    #[builder(default, setter(into))]
    flavor_id: Cow<'a, str>,

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
    /// A dictionary of the flavor’s extra-specs key-and-value pairs. It
    /// appears
    /// in the os-extra-specs’ “create” REQUEST body, as well as the
    /// os-extra-specs’ “create” and “list” RESPONSE body.
    pub fn extra_specs<I, K, V>(&mut self, iter: I) -> &mut Self
    where
        I: Iterator<Item = (K, V)>,
        K: Into<Cow<'a, str>>,
        V: Into<Cow<'a, str>>,
    {
        self.extra_specs
            .get_or_insert_with(BTreeMap::new)
            .extend(iter.map(|(k, v)| (k.into(), v.into())));
        self
    }

    /// Add a single header to the Os_Extra_Spec.
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
        format!(
            "v2.1/flavors/{flavor_id}/os-extra_specs",
            flavor_id = self.flavor_id.as_ref(),
        )
        .into()
    }

    fn parameters(&self) -> QueryParams {
        let mut params = QueryParams::default();

        params
    }

    fn body(&self) -> Result<Option<(&'static str, Vec<u8>)>, BodyError> {
        let mut params = JsonBodyParams::default();

        params.push("extra_specs", serde_json::to_value(&self.extra_specs)?);

        params.into_body()
    }

    fn service_type(&self) -> ServiceType {
        ServiceType::Compute
    }

    fn response_key(&self) -> Option<Cow<'static, str>> {
        Some("extra_specs".into())
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
                .extra_specs(BTreeMap::<String, String>::new().into_iter())
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
                .extra_specs(BTreeMap::<String, String>::new().into_iter())
                .build()
                .unwrap()
                .response_key()
                .unwrap(),
            "extra_specs"
        );
    }

    #[test]
    fn endpoint() {
        let client = MockServerClient::new();
        let mock = client.server.mock(|when, then| {
            when.method(httpmock::Method::POST).path(format!(
                "/v2.1/flavors/{flavor_id}/os-extra_specs",
                flavor_id = "flavor_id",
            ));

            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({ "extra_specs": {} }));
        });

        let endpoint = Request::builder()
            .flavor_id("flavor_id")
            .extra_specs(BTreeMap::<String, String>::new().into_iter())
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
                .path(format!(
                    "/v2.1/flavors/{flavor_id}/os-extra_specs",
                    flavor_id = "flavor_id",
                ))
                .header("foo", "bar")
                .header("not_foo", "not_bar");
            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({ "extra_specs": {} }));
        });

        let endpoint = Request::builder()
            .flavor_id("flavor_id")
            .extra_specs(BTreeMap::<String, String>::new().into_iter())
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
