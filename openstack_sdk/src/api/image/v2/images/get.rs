//! List Images
use derive_builder::Builder;
use http::{HeaderMap, HeaderName, HeaderValue};
use std::collections::BTreeSet;

use crate::api::common::CommaSeparatedList;
use crate::api::rest_endpoint_prelude::*;

use crate::api::Pageable;

/// Query for images.get operation.
#[derive(Debug, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct Images<'a> {
    /// limit filter parameter
    #[builder(default, setter(into))]
    limit: Option<Cow<'a, str>>,

    /// marker filter parameter
    #[builder(default, setter(into))]
    marker: Option<Cow<'a, str>>,

    /// id filter parameter
    #[builder(default, setter(into))]
    id: Option<Cow<'a, str>>,

    /// name filter parameter
    #[builder(default, setter(into))]
    name: Option<Cow<'a, str>>,

    /// visibility filter parameter
    #[builder(default, setter(into))]
    visibility: Option<Cow<'a, str>>,

    /// member_status filter parameter
    #[builder(default, setter(into))]
    member_status: Option<Cow<'a, str>>,

    /// owner filter parameter
    #[builder(default, setter(into))]
    owner: Option<Cow<'a, str>>,

    /// status filter parameter
    #[builder(default, setter(into))]
    status: Option<Cow<'a, str>>,

    /// size_min filter parameter
    #[builder(default, setter(into))]
    size_min: Option<Cow<'a, str>>,

    /// size_max filter parameter
    #[builder(default, setter(into))]
    size_max: Option<Cow<'a, str>>,

    /// protected filter parameter
    #[builder(default, setter(into))]
    protected: Option<Cow<'a, str>>,

    /// is_hidden filter parameter
    #[builder(default)]
    os_hidden: Option<bool>,

    /// sort_key filter parameter
    #[builder(default, setter(into))]
    sort_key: Option<Cow<'a, str>>,

    /// sort_dir filter parameter
    #[builder(default, setter(into))]
    sort_dir: Option<Cow<'a, str>>,

    /// sort filter parameter
    #[builder(default, setter(into))]
    sort: Option<Cow<'a, str>>,

    /// tag filter parameter
    #[builder(default, private, setter(name = "_tag"))]
    tag: BTreeSet<Cow<'a, str>>,

    /// created_at filter parameter
    #[builder(default, setter(into))]
    created_at: Option<Cow<'a, str>>,

    /// updated_at filter parameter
    #[builder(default, setter(into))]
    updated_at: Option<Cow<'a, str>>,

    #[builder(setter(name = "_headers"), default, private)]
    _headers: Option<HeaderMap>,
}

impl<'a> Images<'a> {
    /// Create a builder for the endpoint.
    pub fn builder() -> ImagesBuilder<'a> {
        ImagesBuilder::default()
    }
}

impl<'a> ImagesBuilder<'a> {
    /// tag filter parameter
    pub fn tag<I, T>(&mut self, iter: I) -> &mut Self
    where
        I: Iterator<Item = T>,
        T: Into<Cow<'a, str>>,
    {
        self.tag
            .get_or_insert_with(BTreeSet::new)
            .extend(iter.map(Into::into));
        self
    }

    /// Add a single header to the Images.
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

impl<'a> RestEndpoint for Images<'a> {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        "images".to_string().into()
    }

    fn parameters(&self) -> QueryParams {
        let mut params = QueryParams::default();
        params.push_opt("limit", self.limit.as_ref());
        params.push_opt("marker", self.marker.as_ref());
        params.push_opt("id", self.id.as_ref());
        params.push_opt("name", self.name.as_ref());
        params.push_opt("visibility", self.visibility.as_ref());
        params.push_opt("member_status", self.member_status.as_ref());
        params.push_opt("owner", self.owner.as_ref());
        params.push_opt("status", self.status.as_ref());
        params.push_opt("size_min", self.size_min.as_ref());
        params.push_opt("size_max", self.size_max.as_ref());
        params.push_opt("protected", self.protected.as_ref());
        params.push_opt("os_hidden", self.os_hidden);
        params.push_opt("sort_key", self.sort_key.as_ref());
        params.push_opt("sort_dir", self.sort_dir.as_ref());
        params.push_opt("sort", self.sort.as_ref());
        params.extend(self.tag.iter().map(|value| ("tag", value)));
        params.push_opt("created_at", self.created_at.as_ref());
        params.push_opt("updated_at", self.updated_at.as_ref());

        params
    }

    fn service_type(&self) -> ServiceType {
        ServiceType::Image
    }

    fn response_key(&self) -> Option<Cow<'static, str>> {
        Some("images".into())
    }

    /// Returns headers to be set into the request
    fn request_headers(&self) -> Option<&HeaderMap> {
        self._headers.as_ref()
    }
}
impl<'a> Pageable for Images<'a> {}

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
            Images::builder().build().unwrap().service_type(),
            ServiceType::Image
        );
    }

    #[test]
    fn test_response_key() {
        assert_eq!(
            Images::builder().build().unwrap().response_key().unwrap(),
            "images"
        );
    }

    #[test]
    fn endpoint() {
        let client = MockServerClient::new();
        let mock = client.server.mock(|when, then| {
            when.method(httpmock::Method::GET).path(format!("/images",));

            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({ "images": {} }));
        });

        let endpoint = Images::builder().build().unwrap();
        let _: serde_json::Value = endpoint.query(&client).unwrap();
        mock.assert();
    }

    #[test]
    fn endpoint_headers() {
        let client = MockServerClient::new();
        let mock = client.server.mock(|when, then| {
            when.method(httpmock::Method::GET)
                .path(format!("/images",))
                .header("foo", "bar")
                .header("not_foo", "not_bar");
            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({ "images": {} }));
        });

        let endpoint = Images::builder()
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
