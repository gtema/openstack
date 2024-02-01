//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//
// SPDX-License-Identifier: Apache-2.0

//! Update a user.
//!
//! PATCH /v3/users/{user_id}
use derive_builder::Builder;
use http::{HeaderMap, HeaderName, HeaderValue};

use crate::api::rest_endpoint_prelude::*;
use serde::Serialize;

use serde::Deserialize;
use serde_json::Value;
use std::borrow::Cow;
use std::collections::BTreeMap;

use json_patch::Patch;

#[derive(Builder, Debug, Deserialize, Clone, Serialize)]
#[builder(setter(strip_option))]
pub struct Protocols<'a> {
    #[serde()]
    #[builder(setter(into))]
    protocol_id: Cow<'a, str>,

    #[serde()]
    #[builder(setter(into))]
    unique_id: Cow<'a, str>,
}

#[derive(Builder, Debug, Deserialize, Clone, Serialize)]
#[builder(setter(strip_option))]
pub struct Federated<'a> {
    #[serde()]
    #[builder(setter(into))]
    idp_id: Cow<'a, str>,

    #[serde()]
    #[builder(setter(into))]
    protocols: Vec<Protocols<'a>>,
}

#[derive(Builder, Debug, Deserialize, Clone, Serialize)]
#[builder(setter(strip_option))]
pub struct Options<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    ignore_change_password_upon_first_use: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    ignore_password_expiry: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    ignore_lockout_failure_attempts: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    lock_password: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    ignore_user_inactivity: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    multi_factor_auth_rules: Option<Vec<Cow<'a, str>>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    multi_factor_auth_enabled: Option<bool>,
}

#[derive(Builder, Debug, Clone)]
#[builder(setter(strip_option))]
pub struct Request<'a> {
    #[builder(default, setter(into))]
    default_project_id: Option<Option<Cow<'a, str>>>,

    #[builder(default, setter(into))]
    description: Option<Option<Cow<'a, str>>>,

    #[builder(default, setter(into))]
    domain_id: Option<Cow<'a, str>>,

    #[builder(default)]
    enabled: Option<bool>,

    #[builder(default, setter(into))]
    federated: Option<Vec<Federated<'a>>>,

    #[builder(default, setter(into))]
    name: Option<Cow<'a, str>>,

    #[builder(default, setter(into))]
    password: Option<Option<Cow<'a, str>>>,

    #[builder(default, setter(into))]
    options: Option<Options<'a>>,

    /// user_id parameter for /v3/users/{user_id}/access_rules/{access_rule_id}
    /// API
    #[builder(default, setter(into))]
    id: Cow<'a, str>,

    /// Patch data
    #[builder()]
    patch: Patch,

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
    /// Add a single header to the User.
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
        http::Method::PATCH
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("v3/users/{id}", id = self.id.as_ref(),).into()
    }

    fn parameters(&self) -> QueryParams {
        let mut params = QueryParams::default();

        params
    }

    fn body(&self) -> Result<Option<(&'static str, Vec<u8>)>, BodyError> {
        let mut params = JsonBodyParams::default();

        if let Some(val) = &self.default_project_id {
            params.push("default_project_id", serde_json::to_value(val)?);
        }
        if let Some(val) = &self.description {
            params.push("description", serde_json::to_value(val)?);
        }
        if let Some(val) = &self.domain_id {
            params.push("domain_id", serde_json::to_value(val)?);
        }
        if let Some(val) = &self.enabled {
            params.push("enabled", serde_json::to_value(val)?);
        }
        if let Some(val) = &self.federated {
            params.push("federated", serde_json::to_value(val)?);
        }
        if let Some(val) = &self.name {
            params.push("name", serde_json::to_value(val)?);
        }
        if let Some(val) = &self.password {
            params.push("password", serde_json::to_value(val)?);
        }
        if let Some(val) = &self.options {
            params.push("options", serde_json::to_value(val)?);
        }
        for (key, val) in &self._properties {
            params.push(key.clone(), serde_json::Value::from(val.clone()));
        }

        params.into_body()
    }

    fn service_type(&self) -> ServiceType {
        ServiceType::identity
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
    use json_patch::Patch;
    use serde::Deserialize;
    use serde::Serialize;
    use serde_json::from_value;
    use serde_json::json;

    #[test]
    fn test_service_type() {
        assert_eq!(
            Request::builder()
                .patch(from_value::<Patch>(json!([])).unwrap())
                .build()
                .unwrap()
                .service_type(),
            ServiceType::identity
        );
    }

    #[test]
    fn test_response_key() {
        assert!(Request::builder()
            .patch(from_value::<Patch>(json!([])).unwrap())
            .build()
            .unwrap()
            .response_key()
            .is_none())
    }

    #[test]
    fn endpoint() {
        let client = MockServerClient::new();
        let mock = client.server.mock(|when, then| {
            when.method(httpmock::Method::PATCH)
                .path(format!("/v3/users/{id}", id = "id",));

            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({ "dummy": {} }));
        });

        let endpoint = Request::builder()
            .id("id")
            .patch(from_value::<Patch>(json!([])).unwrap())
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
                .path(format!("/v3/users/{id}", id = "id",))
                .header("foo", "bar")
                .header("not_foo", "not_bar");
            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({ "dummy": {} }));
        });

        let endpoint = Request::builder()
            .id("id")
            .patch(from_value::<Patch>(json!([])).unwrap())
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
