//! Lists networks to which the project has access.
//!
//! Default policy settings return only networks that the project who submits
//! the
//! request owns, unless an administrative user submits the request. In
//! addition,
//! networks shared with the project who submits the request are also returned.
//!
//! Use the `fields` query parameter to control which fields are
//! returned in the response body. Additionally, you can filter results
//! by using query string parameters. For information, see [Filtering
//! and Column Selection](https://wiki.openstack.org/wiki/Neutron/APIv2-
//! specification#Filtering_and_Column_Selection).
//!
//! You can also use the `tags`, `tags-any`, `not-tags`, `not-tags-any`
//! query parameter to filter the response with tags. For information,
//! see [REST API Impact](http://specs.openstack.org/openstack/neutron-
//! specs/specs/mitaka/add-tags-to-core-resources.html#rest-api-impact).
//!
//! Normal response codes: 200
//!
//! Error response codes: 401
//!
use derive_builder::Builder;
use http::{HeaderMap, HeaderName, HeaderValue};

use crate::api::rest_endpoint_prelude::*;
use serde::Serialize;

use crate::api::common::CommaSeparatedList;
use std::borrow::Cow;

use crate::api::Pageable;
#[derive(Builder, Debug, Clone)]
#[builder(setter(strip_option))]
pub struct Request<'a> {
    /// id query parameter for /v2.0/networks API
    #[builder(default, setter(into))]
    id: Option<Cow<'a, str>>,

    /// name query parameter for /v2.0/networks API
    #[builder(default, setter(into))]
    name: Option<Cow<'a, str>>,

    /// admin_state_up query parameter for /v2.0/networks API
    #[builder(default)]
    admin_state_up: Option<bool>,

    /// status query parameter for /v2.0/networks API
    #[builder(default, setter(into))]
    status: Option<Cow<'a, str>>,

    /// tenant_id query parameter for /v2.0/networks API
    #[builder(default, setter(into))]
    tenant_id: Option<Cow<'a, str>>,

    /// shared query parameter for /v2.0/networks API
    #[builder(default)]
    shared: Option<bool>,

    /// router:external query parameter for /v2.0/networks API
    #[builder(default)]
    router_external: Option<bool>,

    /// mtu query parameter for /v2.0/networks API
    #[builder(default)]
    mtu: Option<i32>,

    /// provider:network_type query parameter for /v2.0/networks API
    #[builder(default, setter(into))]
    provider_network_type: Option<Cow<'a, str>>,

    /// provider:physical_network query parameter for /v2.0/networks API
    #[builder(default, setter(into))]
    provider_physical_network: Option<Cow<'a, str>>,

    /// provider:segmentation_id query parameter for /v2.0/networks API
    #[builder(default)]
    provider_segmentation_id: Option<i32>,

    /// revision_number query parameter for /v2.0/networks API
    #[builder(default, setter(into))]
    revision_number: Option<Cow<'a, str>>,

    /// tags query parameter for /v2.0/networks API
    #[builder(default, private, setter(name = "_tags"))]
    tags: Option<CommaSeparatedList<Cow<'a, str>>>,

    /// tags-any query parameter for /v2.0/networks API
    #[builder(default, private, setter(name = "_tags_any"))]
    tags_any: Option<CommaSeparatedList<Cow<'a, str>>>,

    /// not-tags query parameter for /v2.0/networks API
    #[builder(default, private, setter(name = "_not_tags"))]
    not_tags: Option<CommaSeparatedList<Cow<'a, str>>>,

    /// not-tags-any query parameter for /v2.0/networks API
    #[builder(default, private, setter(name = "_not_tags_any"))]
    not_tags_any: Option<CommaSeparatedList<Cow<'a, str>>>,

    /// is_default query parameter for /v2.0/networks API
    #[builder(default)]
    is_default: Option<bool>,

    /// description query parameter for /v2.0/networks API
    #[builder(default, setter(into))]
    description: Option<Cow<'a, str>>,

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
    /// tags query parameter for /v2.0/networks API
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

    /// tags-any query parameter for /v2.0/networks API
    pub fn tags_any<I, T>(&mut self, iter: I) -> &mut Self
    where
        I: Iterator<Item = T>,
        T: Into<Cow<'a, str>>,
    {
        self.tags_any
            .get_or_insert(None)
            .get_or_insert_with(CommaSeparatedList::new)
            .extend(iter.map(Into::into));
        self
    }

    /// not-tags query parameter for /v2.0/networks API
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

    /// not-tags-any query parameter for /v2.0/networks API
    pub fn not_tags_any<I, T>(&mut self, iter: I) -> &mut Self
    where
        I: Iterator<Item = T>,
        T: Into<Cow<'a, str>>,
    {
        self.not_tags_any
            .get_or_insert(None)
            .get_or_insert_with(CommaSeparatedList::new)
            .extend(iter.map(Into::into));
        self
    }

    /// Add a single header to the Network.
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
        http::Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("v2.0/networks",).into()
    }

    fn parameters(&self) -> QueryParams {
        let mut params = QueryParams::default();
        params.push_opt("id", self.id.as_ref());
        params.push_opt("name", self.name.as_ref());
        params.push_opt("admin_state_up", self.admin_state_up);
        params.push_opt("status", self.status.as_ref());
        params.push_opt("tenant_id", self.tenant_id.as_ref());
        params.push_opt("shared", self.shared);
        params.push_opt("router:external", self.router_external);
        params.push_opt("mtu", self.mtu);
        params.push_opt("provider:network_type", self.provider_network_type.as_ref());
        params.push_opt(
            "provider:physical_network",
            self.provider_physical_network.as_ref(),
        );
        params.push_opt("provider:segmentation_id", self.provider_segmentation_id);
        params.push_opt("revision_number", self.revision_number.as_ref());
        params.push_opt("tags", self.tags.as_ref());
        params.push_opt("tags-any", self.tags_any.as_ref());
        params.push_opt("not-tags", self.not_tags.as_ref());
        params.push_opt("not-tags-any", self.not_tags_any.as_ref());
        params.push_opt("is_default", self.is_default);
        params.push_opt("description", self.description.as_ref());

        params
    }

    fn service_type(&self) -> ServiceType {
        ServiceType::Network
    }

    fn response_key(&self) -> Option<Cow<'static, str>> {
        Some("networks".into())
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
            "networks"
        );
    }

    #[test]
    fn endpoint() {
        let client = MockServerClient::new();
        let mock = client.server.mock(|when, then| {
            when.method(httpmock::Method::GET)
                .path(format!("/v2.0/networks",));

            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({ "networks": {} }));
        });

        let endpoint = Request::builder().build().unwrap();
        let _: serde_json::Value = endpoint.query(&client).unwrap();
        mock.assert();
    }

    #[test]
    fn endpoint_headers() {
        let client = MockServerClient::new();
        let mock = client.server.mock(|when, then| {
            when.method(httpmock::Method::GET)
                .path(format!("/v2.0/networks",))
                .header("foo", "bar")
                .header("not_foo", "not_bar");
            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({ "networks": {} }));
        });

        let endpoint = Request::builder()
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
