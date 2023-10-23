//! List Flavors
use derive_builder::Builder;
use http::{HeaderMap, HeaderName, HeaderValue};
use std::collections::BTreeSet;

use crate::api::common::CommaSeparatedList;
use crate::api::rest_endpoint_prelude::*;

use crate::api::Pageable;

/// Query for flavors.get_detailed operation.
#[derive(Debug, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct Flavors<'a> {
    /// Filters the response by a minimum disk space, in GiB. For example, 100.
    #[builder(default)]
    min_disk: Option<u32>,

    /// Filters the response by a minimum RAM, in MiB. For example, 512.
    #[builder(default)]
    min_ram: Option<u32>,

    /// This parameter is only applicable to users with the administrative
    /// role. For all other non-admin users, the parameter is ignored and only
    /// public flavors will be returned. Filters the flavor list based on
    /// whether the flavor is public or private. If the value of this parameter
    /// is not specified, it is treated as True. If the value is specified, 1,
    /// t, true, on, y and yes are treated as True. 0, f, false, off, n and no
    /// are treated as False (they are case-insensitive). If the value is None
    /// (case-insensitive) both public and private flavors will be listed in a
    /// single request.
    #[builder(default)]
    is_public: Option<bool>,

    /// Requests a page size of items. Returns a number of items up to a limit
    /// value. Use the limit parameter to make an initial limited request and
    /// use the ID of the last-seen item from the response as the marker
    /// parameter value in a subsequent limited request.
    #[builder(default)]
    limit: Option<u32>,

    /// The ID of the last-seen item. Use the limit parameter to make an
    /// initial limited request and use the ID of the last-seen item from the
    /// response as the marker parameter value in a subsequent limited request.
    #[builder(default, setter(into))]
    marker: Option<Cow<'a, str>>,

    #[builder(setter(name = "_headers"), default, private)]
    _headers: Option<HeaderMap>,
}

impl<'a> Flavors<'a> {
    /// Create a builder for the endpoint.
    pub fn builder() -> FlavorsBuilder<'a> {
        FlavorsBuilder::default()
    }
}

impl<'a> FlavorsBuilder<'a> {
    /// Add a single header to the Flavors.
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

impl<'a> RestEndpoint for Flavors<'a> {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("flavors/detail",).into()
    }

    fn parameters(&self) -> QueryParams {
        let mut params = QueryParams::default();
        params.push_opt("minDisk", self.min_disk);
        params.push_opt("minRam", self.min_ram);
        params.push_opt("is_public", self.is_public);
        params.push_opt("limit", self.limit);
        params.push_opt("marker", self.marker.as_ref());

        params
    }

    fn service_type(&self) -> ServiceType {
        ServiceType::Compute
    }

    fn response_key(&self) -> Option<Cow<'static, str>> {
        Some("flavors".into())
    }

    /// Returns headers to be set into the request
    fn request_headers(&self) -> Option<&HeaderMap> {
        self._headers.as_ref()
    }
}
impl<'a> Pageable for Flavors<'a> {}

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
            Flavors::builder().build().unwrap().service_type(),
            ServiceType::Compute
        );
    }

    #[test]
    fn test_response_key() {
        assert_eq!(
            Flavors::builder().build().unwrap().response_key().unwrap(),
            "flavors"
        );
    }

    #[test]
    fn endpoint() {
        let client = MockServerClient::new();
        let mock = client.server.mock(|when, then| {
            when.method(httpmock::Method::GET)
                .path(format!("/flavors/detail",));

            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({ "flavors": {} }));
        });

        let endpoint = Flavors::builder().build().unwrap();
        let _: serde_json::Value = endpoint.query(&client).unwrap();
        mock.assert();
    }

    #[test]
    fn endpoint_headers() {
        let client = MockServerClient::new();
        let mock = client.server.mock(|when, then| {
            when.method(httpmock::Method::GET)
                .path(format!("/flavors/detail",))
                .header("foo", "bar")
                .header("not_foo", "not_bar");
            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({ "flavors": {} }));
        });

        let endpoint = Flavors::builder()
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
