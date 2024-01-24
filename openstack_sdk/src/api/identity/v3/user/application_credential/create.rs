//! Creates an application credential for a user on the project to which the
//! current token is scoped.
//!
//! Relationship: `https://docs.openstack.org/api/openstack-
//! identity/3/rel/application\_credentials`
//!
use derive_builder::Builder;
use http::{HeaderMap, HeaderName, HeaderValue};

use crate::api::rest_endpoint_prelude::*;
use serde::Serialize;

use serde::Deserialize;
use serde_json::Value;
use std::borrow::Cow;
use std::collections::BTreeMap;

#[derive(Builder, Debug, Deserialize, Clone, Serialize)]
#[builder(setter(strip_option))]
pub struct Roles<'a> {
    /// The ID of the domain.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) id: Option<Cow<'a, str>>,

    /// The role name.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) name: Option<Cow<'a, str>>,
}

#[derive(Builder, Debug, Deserialize, Clone, Serialize)]
#[builder(setter(strip_option))]
pub struct AccessRules<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) path: Option<Cow<'a, str>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) method: Option<Cow<'a, str>>,

    /// The ID of the domain.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) service: Option<Cow<'a, str>>,

    /// The ID of the domain.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) id: Option<Cow<'a, str>>,
}

#[derive(Builder, Debug, Clone)]
#[builder(setter(strip_option))]
pub struct Request<'a> {
    /// The role name.
    #[builder(setter(into))]
    pub(crate) name: Cow<'a, str>,

    /// A description of the application credential’s purpose.
    #[builder(default, setter(into))]
    pub(crate) description: Option<Option<Cow<'a, str>>>,

    /// The secret that the application credential will be created with. If not
    /// provided, one will be generated.
    #[builder(default, setter(into))]
    pub(crate) secret: Option<Option<Cow<'a, str>>>,

    /// An optional expiry time for the application credential. If unset, the
    /// application credential does not expire.
    #[builder(default, setter(into))]
    pub(crate) expires_at: Option<Option<Cow<'a, str>>>,

    /// An optional list of role objects, identified by ID or name. The list
    /// may only contain roles that the user has assigned on the project.
    /// If not provided, the roles assigned to the application credential will
    /// be the same as the roles in the current token.
    #[builder(default, setter(into))]
    pub(crate) roles: Option<Vec<Roles<'a>>>,

    /// If the user is enabled, this value is `true`.
    /// If the user is disabled, this value is `false`.
    #[builder(default)]
    pub(crate) unrestricted: Option<bool>,

    /// A list of `access\_rules` objects
    #[builder(default, setter(into))]
    pub(crate) access_rules: Option<Vec<AccessRules<'a>>>,

    /// user_id parameter for /v3/users/{user_id}/access_rules/{access_rule_id}
    /// API
    #[builder(default, setter(into))]
    user_id: Cow<'a, str>,

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
    /// Add a single header to the Application_Credential.
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
        http::Method::POST
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!(
            "v3/users/{user_id}/application_credentials",
            user_id = self.user_id.as_ref(),
        )
        .into()
    }

    fn parameters(&self) -> QueryParams {
        QueryParams::default()
    }

    fn body(&self) -> Result<Option<(&'static str, Vec<u8>)>, BodyError> {
        let mut params = JsonBodyParams::default();

        params.push("name", serde_json::to_value(&self.name)?);
        if let Some(val) = &self.description {
            params.push("description", serde_json::to_value(val)?);
        }
        if let Some(val) = &self.secret {
            params.push("secret", serde_json::to_value(val)?);
        }
        if let Some(val) = &self.expires_at {
            params.push("expires_at", serde_json::to_value(val)?);
        }
        if let Some(val) = &self.roles {
            params.push("roles", serde_json::to_value(val)?);
        }
        if let Some(val) = &self.unrestricted {
            params.push("unrestricted", serde_json::to_value(val)?);
        }
        if let Some(val) = &self.access_rules {
            params.push("access_rules", serde_json::to_value(val)?);
        }
        for (key, val) in &self._properties {
            params.push(key.clone(), val.clone());
        }

        params.into_body()
    }

    fn service_type(&self) -> ServiceType {
        ServiceType::Identity
    }

    fn response_key(&self) -> Option<Cow<'static, str>> {
        Some("application_credentials".into())
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
                .name("foo")
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
                .name("foo")
                .build()
                .unwrap()
                .response_key()
                .unwrap(),
            "application_credentials"
        );
    }

    #[test]
    fn endpoint() {
        let client = MockServerClient::new();
        let mock = client.server.mock(|when, then| {
            when.method(httpmock::Method::POST).path(format!(
                "/v3/users/{user_id}/application_credentials",
                user_id = "user_id",
            ));

            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({ "application_credentials": {} }));
        });

        let endpoint = Request::builder()
            .user_id("user_id")
            .name("foo")
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
                    "/v3/users/{user_id}/application_credentials",
                    user_id = "user_id",
                ))
                .header("foo", "bar")
                .header("not_foo", "not_bar");
            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({ "application_credentials": {} }));
        });

        let endpoint = Request::builder()
            .user_id("user_id")
            .name("foo")
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
