//! Assigns a role to a user on a project.
//!
//! Relationship: `https://docs.openstack.org/api/openstack-
//! identity/3/rel/project\_user\_role`
//!
use derive_builder::Builder;
use http::{HeaderMap, HeaderName, HeaderValue};

use crate::api::rest_endpoint_prelude::*;
use serde::Serialize;

use serde_json::Value;
use std::borrow::Cow;
use std::collections::BTreeMap;

#[derive(Builder, Debug, Clone)]
#[builder(setter(strip_option))]
pub struct Request<'a> {
    /// project_id parameter for
    /// /v3/projects/{project_id}/groups/{group_id}/roles API
    #[builder(default, setter(into))]
    project_id: Cow<'a, str>,

    /// user_id parameter for /v3/projects/{project_id}/users/{user_id}/roles
    /// API
    #[builder(default, setter(into))]
    user_id: Cow<'a, str>,

    /// role_id parameter for
    /// /v3/projects/{project_id}/users/{user_id}/roles/{role_id} API
    #[builder(default, setter(into))]
    id: Cow<'a, str>,

    #[builder(setter(name = "_headers"), default, private)]
    _headers: Option<HeaderMap>,

    #[builder(setter(name = "_properties"), default, private)]
    _properties: BTreeMap<Cow<'a, str>, Value>,
}
impl<'a> Request<'a> {
    /// Create a builder for the endpoint.
    pub fn builder() -> RequestBuilder<'a> {
        RequestBuilder::default()
    }
}

impl<'a> RequestBuilder<'a> {
    /// Add a single header to the Role.
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

    pub fn properties<I, K, V>(&mut self, iter: I) -> &mut Self
    where
        I: Iterator<Item = (K, V)>,
        K: Into<Cow<'a, str>>,
        V: Into<Value>,
    {
        self._properties
            .get_or_insert_with(BTreeMap::new)
            .extend(iter.map(|(k, v)| (k.into(), v.into())));
        self
    }
}

impl<'a> RestEndpoint for Request<'a> {
    fn method(&self) -> http::Method {
        http::Method::PUT
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!(
            "v3/projects/{project_id}/users/{user_id}/roles/{id}",
            project_id = self.project_id.as_ref(),
            user_id = self.user_id.as_ref(),
            id = self.id.as_ref(),
        )
        .into()
    }

    fn parameters(&self) -> QueryParams {
        QueryParams::default()
    }

    fn body(&self) -> Result<Option<(&'static str, Vec<u8>)>, BodyError> {
        let mut params = JsonBodyParams::default();

        for (key, val) in &self._properties {
            params.push(key.clone(), serde_json::Value::from(val.clone()));
        }

        params.into_body()
    }

    fn service_type(&self) -> ServiceType {
        ServiceType::Identity
    }

    fn response_key(&self) -> Option<Cow<'static, str>> {
        None
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
            ServiceType::Identity
        );
    }

    #[test]
    fn test_response_key() {
        assert!(Request::builder().build().unwrap().response_key().is_none())
    }

    #[test]
    fn endpoint() {
        let client = MockServerClient::new();
        let mock = client.server.mock(|when, then| {
            when.method(httpmock::Method::PUT).path(format!(
                "/v3/projects/{project_id}/users/{user_id}/roles/{id}",
                project_id = "project_id",
                user_id = "user_id",
                id = "id",
            ));

            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({ "dummy": {} }));
        });

        let endpoint = Request::builder()
            .project_id("project_id")
            .user_id("user_id")
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
                    "/v3/projects/{project_id}/users/{user_id}/roles/{id}",
                    project_id = "project_id",
                    user_id = "user_id",
                    id = "id",
                ))
                .header("foo", "bar")
                .header("not_foo", "not_bar");
            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({ "dummy": {} }));
        });

        let endpoint = Request::builder()
            .project_id("project_id")
            .user_id("user_id")
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
}
