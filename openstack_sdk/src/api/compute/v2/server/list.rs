use derive_builder::Builder;
use http::{HeaderMap, HeaderName, HeaderValue};

use crate::api::common::CommaSeparatedList;
use crate::api::rest_endpoint_prelude::*;
use crate::api::Pageable;
use crate::api::ParamValue;

/// Query for flavors.
#[derive(Debug, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct Servers<'a> {
    #[builder(setter(into), default)]
    name: Option<Cow<'a, str>>,
    // #[builder(setter(custom), default)]
    #[builder(setter(name = "_tags"), default, private)]
    tags: Option<CommaSeparatedList<Cow<'a, str>>>,

    #[builder(setter(name = "_headers"), default, private)]
    _headers: Option<HeaderMap>,
}

impl<'a> Servers<'a> {
    /// Create a builder for the endpoint.
    pub fn builder() -> ServersBuilder<'a> {
        ServersBuilder::default()
    }
}

impl<'a> ServersBuilder<'a> {
    /// Add multiple tags search parameters.
    pub fn tags<I, T>(&mut self, iter: I) -> &mut Self
    where
        I: Iterator<Item = T>,
        T: Into<Cow<'a, str>>,
    {
        self.tags
            .get_or_insert(None)
            .get_or_insert_with(CommaSeparatedList::new)
            .extend(iter.map(Into::into));
        self
    }

    /// Add a single header to the server.
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

impl<'a> RestEndpoint for Servers<'a> {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        "servers/detail".into()
    }

    fn parameters(&self) -> QueryParams {
        let mut params = QueryParams::default();

        params.push_opt("name", self.name.as_ref());
        params.push_opt("tags", self.tags.as_ref());

        params
    }

    /// OpenStack service type
    fn service_type(&self) -> Cow<'static, str> {
        "compute".into()
    }

    /// Response key name with results
    fn response_key(&self) -> Option<Cow<'static, str>> {
        Some("servers".into())
    }

    /// Returns headers to be set into the request
    fn request_headers(&self) -> Option<&HeaderMap> {
        self._headers.as_ref()
    }
}

impl<'a> Pageable for Servers<'a> {}

#[cfg(test)]
mod tests {
    use crate::api::compute::v2::server::list::Servers;
    use crate::api::{self, Query};
    use crate::test::client::{ExpectedUrl, MockServerClient};
    use http::{HeaderName, HeaderValue};
    use serde::Deserialize;
    use serde_json::json;
    use std::borrow::Cow;

    #[test]
    fn endpoint() {
        let client = MockServerClient::new();
        let mock = client.server.mock(|when, then| {
            when.method(httpmock::Method::GET).path("/servers/detail");
            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({ "servers": [] }));
        });

        let endpoint = Servers::builder().build().unwrap();
        let _: serde_json::Value = endpoint.query(&client).unwrap();
        mock.assert();
    }

    #[test]
    fn endpoint_tags() {
        let client = MockServerClient::new();
        let mock = client.server.mock(|when, then| {
            when.method(httpmock::Method::GET)
                .path("/servers/detail")
                .query_param("tags", "a,b");
            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({ "servers": [] }));
        });

        let endpoint = Servers::builder()
            .tags(["a", "b"].iter().copied())
            .build()
            .unwrap();
        let _: serde_json::Value = endpoint.query(&client).unwrap();
        mock.assert();
    }

    #[test]
    fn endpoint_headers() {
        let client = MockServerClient::new();
        let mock = client.server.mock(|when, then| {
            when.method(httpmock::Method::GET)
                .path("/servers/detail")
                .header("foo", "bar")
                .header("not_foo", "not_bar");
            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({ "resources": [] }));
        });

        let endpoint = Servers::builder()
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
