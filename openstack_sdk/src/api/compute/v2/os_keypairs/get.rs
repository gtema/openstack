//! Lists keypairs that are associated with the account.
use derive_builder::Builder;
use http::{HeaderMap, HeaderName, HeaderValue};

use crate::api::common::CommaSeparatedList;
use crate::api::rest_endpoint_prelude::*;

use crate::api::Pageable;

/// Query for keypairs.get operation.
#[derive(Debug, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct Keypairs<'a> {
    /// This allows administrative users to operate key-pairs of specified user
    /// ID.
    /// New in version 2.10
    #[builder(default, setter(into))]
    user_id: Option<Cow<'a, str>>,

    /// Requests a page size of items. Returns a number of items up to a limit
    /// value. Use the limit parameter to make an initial limited request and
    /// use the last-seen item from the response as the marker parameter value
    /// in a subsequent limited request.
    /// New in version 2.35
    #[builder(default, setter(into))]
    limit: Option<Cow<'a, str>>,

    /// The last-seen item. Use the limit parameter to make an initial limited
    /// request and use the last-seen item from the response as the marker
    /// parameter value in a subsequent limited request.
    /// New in version 2.35
    #[builder(default, setter(into))]
    marker: Option<Cow<'a, str>>,

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
        http::Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        "os-keypairs".to_string().into()
    }

    fn parameters(&self) -> QueryParams {
        let mut params = QueryParams::default();
        params.push_opt("user_id", self.user_id.as_ref());
        params.push_opt("limit", self.limit.as_ref());
        params.push_opt("marker", self.marker.as_ref());

        params
    }

    fn service_type(&self) -> ServiceType {
        ServiceType::Compute
    }

    fn response_key(&self) -> Option<Cow<'static, str>> {
        Some("keypairs".into())
    }

    fn response_list_item_key(&self) -> Option<Cow<'static, str>> {
        Some("keypair".into())
    }

    /// Returns headers to be set into the request
    fn request_headers(&self) -> Option<&HeaderMap> {
        self._headers.as_ref()
    }
}
impl<'a> Pageable for Keypairs<'a> {}

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
            "keypairs"
        );
    }

    #[test]
    fn endpoint() {
        let client = MockServerClient::new();
        let mock = client.server.mock(|when, then| {
            when.method(httpmock::Method::GET)
                .path(format!("/os-keypairs",));

            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({ "keypairs": {} }));
        });

        let endpoint = Keypairs::builder().build().unwrap();
        let _: serde_json::Value = endpoint.query(&client).unwrap();
        mock.assert();
    }

    #[test]
    fn endpoint_headers() {
        let client = MockServerClient::new();
        let mock = client.server.mock(|when, then| {
            when.method(httpmock::Method::GET)
                .path(format!("/os-keypairs",))
                .header("foo", "bar")
                .header("not_foo", "not_bar");
            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({ "keypairs": {} }));
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
}
