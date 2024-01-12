//! Update Volume Details
use derive_builder::Builder;
use http::{HeaderMap, HeaderName, HeaderValue};

use crate::api::common::CommaSeparatedList;
use crate::api::rest_endpoint_prelude::*;

use std::collections::BTreeMap;

/// Query for volume.put operation.
#[derive(Debug, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct Volume<'a> {
    /// The UUID of the project in a multi-tenancy cloud.
    #[builder(default, setter(into))]
    project_id: Cow<'a, str>,

    /// Volume ID
    #[builder(default, setter(into))]
    id: Cow<'a, str>,

    /// The volume description.
    #[builder(default, setter(into))]
    description: Option<Cow<'a, str>>,

    /// The volume name.
    #[builder(default, setter(into))]
    display_name: Option<Cow<'a, str>>,

    /// A metadata object. Contains one or more metadata key and value pairs
    /// that are associated with the volume.
    #[builder(default, private, setter(name = "_metadata"))]
    metadata: Option<BTreeMap<Cow<'a, str>, Cow<'a, str>>>,

    /// The volume name.
    #[builder(default, setter(into))]
    name: Option<Cow<'a, str>>,

    #[builder(setter(name = "_headers"), default, private)]
    _headers: Option<HeaderMap>,
}

impl<'a> Volume<'a> {
    /// Create a builder for the endpoint.
    pub fn builder() -> VolumeBuilder<'a> {
        VolumeBuilder::default()
    }
}

impl<'a> VolumeBuilder<'a> {
    /// A metadata object. Contains one or more metadata key and value pairs
    /// that are associated with the volume.
    pub fn metadata<I, K, V>(&mut self, iter: I) -> &mut Self
    where
        I: Iterator<Item = (K, V)>,
        K: Into<Cow<'a, str>>,
        V: Into<Cow<'a, str>>,
    {
        self.metadata
            .get_or_insert(None)
            .get_or_insert_with(BTreeMap::new)
            .extend(iter.map(|(k, v)| (k.into(), v.into())));
        self
    }

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

impl<'a> RestEndpoint for Volume<'a> {
    fn method(&self) -> http::Method {
        http::Method::PUT
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!(
            "{project_id}/volumes/{id}",
            project_id = self.project_id.as_ref(),
            id = self.id.as_ref(),
        )
        .into()
    }

    fn parameters(&self) -> QueryParams {
        QueryParams::default()
    }

    fn body(&self) -> Result<Option<(&'static str, Vec<u8>)>, BodyError> {
        let mut params = JsonBodyParams::default();

        params.push_opt("description", self.description.as_ref());
        params.push_opt("display_name", self.display_name.as_ref());
        params.push_opt("metadata", self.metadata.as_ref());
        params.push_opt("name", self.name.as_ref());
        params.into_body_with_root_key("volume")
    }

    fn service_type(&self) -> ServiceType {
        ServiceType::BlockStorage
    }

    fn response_key(&self) -> Option<Cow<'static, str>> {
        Some("volume".into())
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
    use serde_json::json;

    #[test]
    fn test_service_type() {
        assert_eq!(
            Volume::builder().build().unwrap().service_type(),
            ServiceType::BlockStorage
        );
    }

    #[test]
    fn test_response_key() {
        assert_eq!(
            Volume::builder().build().unwrap().response_key().unwrap(),
            "volume"
        );
    }

    #[test]
    fn endpoint() {
        let client = MockServerClient::new();
        let mock = client.server.mock(|when, then| {
            when.method(httpmock::Method::PUT).path(format!(
                "/{project_id}/volumes/{id}",
                project_id = "project_id",
                id = "id",
            ));

            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({ "volume": {} }));
        });

        let endpoint = Volume::builder()
            .project_id("project_id")
            .id("id")
            .build()
            .unwrap();
        let _: serde_json::Value = endpoint.query(&client).unwrap();
        mock.assert();
    }

    #[test]
    fn endpoint_headers() {
        let client = MockServerClient::new();
        let mock = client.server.mock(|when, then| {
            when.method(httpmock::Method::PUT)
                .path(format!(
                    "/{project_id}/volumes/{id}",
                    project_id = "project_id",
                    id = "id",
                ))
                .header("foo", "bar")
                .header("not_foo", "not_bar");
            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({ "volume": {} }));
        });

        let endpoint = Volume::builder()
            .project_id("project_id")
            .id("id")
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

    #[test]
    fn endpoint_body() {
        let endpoint = Volume::builder()
            .description("description")
            .display_name("display_name")
            .name("name")
            .build()
            .unwrap();

        let (mime, body) = endpoint.body().unwrap().unwrap();
        assert_eq!(
            std::str::from_utf8(&body).unwrap(),
            json!({
              "volume": {
                 "description": "description",
                 "display_name": "display_name",
                 "name": "name",
             }
            })
            .to_string()
        );
    }
}
