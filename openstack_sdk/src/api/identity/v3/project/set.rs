//! Updates a project.
//!
//! Relationship: `https://docs.openstack.org/api/openstack-
//! identity/3/rel/project`
//!
use derive_builder::Builder;
use http::{HeaderMap, HeaderName, HeaderValue};

use crate::api::rest_endpoint_prelude::*;
use serde::Serialize;

use serde::Deserialize;
use serde_json::Value;
use std::borrow::Cow;
use std::collections::BTreeMap;

/// The resource options for the project. Available resource options are
/// `immutable`.
#[derive(Builder, Debug, Deserialize, Clone, Serialize)]
#[builder(setter(strip_option))]
pub struct Options {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    immutable: Option<bool>,
}

/// A `project` object
#[derive(Builder, Debug, Deserialize, Clone, Serialize)]
#[builder(setter(strip_option))]
pub struct Project<'a> {
    /// The description of the project.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    description: Option<Option<Cow<'a, str>>>,

    /// The ID of the new domain for the project. The ability to change the
    /// domain
    /// of a project is now deprecated, and will be removed in subequent
    /// release.
    /// It is already disabled by default in most Identity service
    /// implementations.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    domain_id: Option<Option<Cow<'a, str>>>,

    /// If set to `true`, project is enabled. If set to
    /// `false`, project is disabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    enabled: Option<bool>,

    /// If set to `true`, project is enabled. If set to
    /// `false`, project is disabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    is_domain: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    parent_id: Option<Option<Cow<'a, str>>>,

    /// The name of the project, which must be unique within the
    /// owning domain. A project can have the same name as its domain.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    name: Option<Cow<'a, str>>,

    /// A list of simple strings assigned to a project.
    /// Tags can be used to classify projects into groups.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    tags: Option<Vec<Cow<'a, str>>>,

    /// The resource options for the project. Available resource options are
    /// `immutable`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    options: Option<Options>,
}

#[derive(Builder, Debug, Clone)]
#[builder(setter(strip_option))]
pub struct Request<'a> {
    /// A `project` object
    #[builder(setter(into))]
    project: Project<'a>,

    /// project_id parameter for
    /// /v3/projects/{project_id}/groups/{group_id}/roles API
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
    /// Add a single header to the Project.
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
        http::Method::PATCH
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("v3/projects/{id}", id = self.id.as_ref(),).into()
    }

    fn parameters(&self) -> QueryParams {
        QueryParams::default()
    }

    fn body(&self) -> Result<Option<(&'static str, Vec<u8>)>, BodyError> {
        let mut params = JsonBodyParams::default();

        params.push("project", serde_json::to_value(&self.project)?);

        params.into_body()
    }

    fn service_type(&self) -> ServiceType {
        ServiceType::Identity
    }

    fn response_key(&self) -> Option<Cow<'static, str>> {
        Some("project".into())
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
                .project(ProjectBuilder::default().build().unwrap())
                .build()
                .unwrap()
                .service_type(),
            ServiceType::Identity
        );
    }

    #[test]
    fn test_response_key() {
        assert_eq!(
            Request::builder()
                .project(ProjectBuilder::default().build().unwrap())
                .build()
                .unwrap()
                .response_key()
                .unwrap(),
            "project"
        );
    }

    #[test]
    fn endpoint() {
        let client = MockServerClient::new();
        let mock = client.server.mock(|when, then| {
            when.method(httpmock::Method::PATCH)
                .path(format!("/v3/projects/{id}", id = "id",));

            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({ "project": {} }));
        });

        let endpoint = Request::builder()
            .id("id")
            .project(ProjectBuilder::default().build().unwrap())
            .build()
            .unwrap();
        let _: serde_json::Value = endpoint.query(&client).unwrap();
        mock.assert();
    }

    #[test]
    fn endpoint_headers() {
        let client = MockServerClient::new();
        let mock = client.server.mock(|when, then| {
            when.method(httpmock::Method::PATCH)
                .path(format!("/v3/projects/{id}", id = "id",))
                .header("foo", "bar")
                .header("not_foo", "not_bar");
            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({ "project": {} }));
        });

        let endpoint = Request::builder()
            .id("id")
            .project(ProjectBuilder::default().build().unwrap())
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