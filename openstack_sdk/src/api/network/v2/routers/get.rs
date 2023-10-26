//! List Routers
use derive_builder::Builder;
use http::{HeaderMap, HeaderName, HeaderValue};
use std::collections::BTreeSet;

use crate::api::common::CommaSeparatedList;
use crate::api::rest_endpoint_prelude::*;

use crate::api::Pageable;

/// Query for routers.get operation.
#[derive(Debug, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct Routers<'a> {
    /// limit filter parameter
    #[builder(default, setter(into))]
    limit: Option<Cow<'a, str>>,

    /// marker filter parameter
    #[builder(default, setter(into))]
    marker: Option<Cow<'a, str>>,

    /// description filter parameter
    #[builder(default, setter(into))]
    description: Option<Cow<'a, str>>,

    /// flavor_id filter parameter
    #[builder(default, setter(into))]
    flavor_id: Option<Cow<'a, str>>,

    /// name filter parameter
    #[builder(default, setter(into))]
    name: Option<Cow<'a, str>>,

    /// status filter parameter
    #[builder(default, setter(into))]
    status: Option<Cow<'a, str>>,

    /// project_id filter parameter
    #[builder(default, setter(into))]
    project_id: Option<Cow<'a, str>>,

    /// is_admin_state_up filter parameter
    #[builder(default)]
    is_admin_state_up: Option<bool>,

    /// is_distributed filter parameter
    #[builder(default)]
    is_distributed: Option<bool>,

    /// is_ha filter parameter
    #[builder(default)]
    is_ha: Option<bool>,

    /// tags filter parameter
    #[builder(default, private, setter(name = "_tags"))]
    tags: Option<CommaSeparatedList<Cow<'a, str>>>,

    /// any_tags filter parameter
    #[builder(default, private, setter(name = "_any_tags"))]
    any_tags: Option<CommaSeparatedList<Cow<'a, str>>>,

    /// not_tags filter parameter
    #[builder(default, private, setter(name = "_not_tags"))]
    not_tags: Option<CommaSeparatedList<Cow<'a, str>>>,

    /// not_any_tags filter parameter
    #[builder(default, private, setter(name = "_not_any_tags"))]
    not_any_tags: Option<CommaSeparatedList<Cow<'a, str>>>,

    #[builder(setter(name = "_headers"), default, private)]
    _headers: Option<HeaderMap>,
}

impl<'a> Routers<'a> {
    /// Create a builder for the endpoint.
    pub fn builder() -> RoutersBuilder<'a> {
        RoutersBuilder::default()
    }
}

impl<'a> RoutersBuilder<'a> {
    /// tags filter parameter
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

    /// any_tags filter parameter
    pub fn any_tags<I, T>(&mut self, iter: I) -> &mut Self
    where
        I: Iterator<Item = T>,
        T: Into<Cow<'a, str>>,
    {
        self.any_tags
            .get_or_insert(None)
            .get_or_insert_with(CommaSeparatedList::new)
            .extend(iter.map(Into::into));
        self
    }

    /// not_tags filter parameter
    pub fn not_tags<I, T>(&mut self, iter: I) -> &mut Self
    where
        I: Iterator<Item = T>,
        T: Into<Cow<'a, str>>,
    {
        self.not_tags
            .get_or_insert(None)
            .get_or_insert_with(CommaSeparatedList::new)
            .extend(iter.map(Into::into));
        self
    }

    /// not_any_tags filter parameter
    pub fn not_any_tags<I, T>(&mut self, iter: I) -> &mut Self
    where
        I: Iterator<Item = T>,
        T: Into<Cow<'a, str>>,
    {
        self.not_any_tags
            .get_or_insert(None)
            .get_or_insert_with(CommaSeparatedList::new)
            .extend(iter.map(Into::into));
        self
    }

    /// Add a single header to the Routers.
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

impl<'a> RestEndpoint for Routers<'a> {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        "routers".to_string().into()
    }

    fn parameters(&self) -> QueryParams {
        let mut params = QueryParams::default();
        params.push_opt("limit", self.limit.as_ref());
        params.push_opt("marker", self.marker.as_ref());
        params.push_opt("description", self.description.as_ref());
        params.push_opt("flavor_id", self.flavor_id.as_ref());
        params.push_opt("name", self.name.as_ref());
        params.push_opt("status", self.status.as_ref());
        params.push_opt("project_id", self.project_id.as_ref());
        params.push_opt("admin_state_up", self.is_admin_state_up);
        params.push_opt("distributed", self.is_distributed);
        params.push_opt("ha", self.is_ha);
        params.push_opt("tags", self.tags.as_ref());
        params.push_opt("tags-any", self.any_tags.as_ref());
        params.push_opt("not-tags", self.not_tags.as_ref());
        params.push_opt("not-tags-any", self.not_any_tags.as_ref());

        params
    }

    fn service_type(&self) -> ServiceType {
        ServiceType::Network
    }

    fn response_key(&self) -> Option<Cow<'static, str>> {
        Some("routers".into())
    }

    /// Returns headers to be set into the request
    fn request_headers(&self) -> Option<&HeaderMap> {
        self._headers.as_ref()
    }
}
impl<'a> Pageable for Routers<'a> {}

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
            Routers::builder().build().unwrap().service_type(),
            ServiceType::Network
        );
    }

    #[test]
    fn test_response_key() {
        assert_eq!(
            Routers::builder().build().unwrap().response_key().unwrap(),
            "routers"
        );
    }

    #[test]
    fn endpoint() {
        let client = MockServerClient::new();
        let mock = client.server.mock(|when, then| {
            when.method(httpmock::Method::GET)
                .path(format!("/routers",));

            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({ "routers": {} }));
        });

        let endpoint = Routers::builder().build().unwrap();
        let _: serde_json::Value = endpoint.query(&client).unwrap();
        mock.assert();
    }

    #[test]
    fn endpoint_headers() {
        let client = MockServerClient::new();
        let mock = client.server.mock(|when, then| {
            when.method(httpmock::Method::GET)
                .path(format!("/routers",))
                .header("foo", "bar")
                .header("not_foo", "not_bar");
            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({ "routers": {} }));
        });

        let endpoint = Routers::builder()
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
