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
//
// WARNING: This file is automatically generated from OpenAPI schema using
// `openstack-codegenerator`.

//! Create a new federation.
//!
//! | param federation: | | | --- | --- | | | a federation within the request
//! body. |
//!
use derive_builder::Builder;
use http::{HeaderMap, HeaderName, HeaderValue};

use crate::api::rest_endpoint_prelude::*;

use serde::Deserialize;
use serde::Serialize;
use std::borrow::Cow;
use std::collections::BTreeMap;

#[derive(Debug, Deserialize, Clone, Serialize)]
pub enum Status {
    #[serde(rename = "CREATE_COMPLETE")]
    CreateComplete,
    #[serde(rename = "CREATE_FAILED")]
    CreateFailed,
    #[serde(rename = "CREATE_IN_PROGRESS")]
    CreateInProgress,
    #[serde(rename = "DELETE_COMPLETE")]
    DeleteComplete,
    #[serde(rename = "DELETE_FAILED")]
    DeleteFailed,
    #[serde(rename = "DELETE_IN_PROGRESS")]
    DeleteInProgress,
    #[serde(rename = "UPDATE_COMPLETE")]
    UpdateComplete,
    #[serde(rename = "UPDATE_FAILED")]
    UpdateFailed,
    #[serde(rename = "UPDATE_IN_PROGRESS")]
    UpdateInProgress,
}

/// A link representation.
#[derive(Builder, Debug, Deserialize, Clone, Serialize)]
#[builder(setter(strip_option))]
pub struct Links<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) created_at: Option<Cow<'a, str>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) href: Option<Cow<'a, str>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) rel: Option<Cow<'a, str>>,

    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) _type: Option<Cow<'a, str>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) updated_at: Option<Cow<'a, str>>,
}

#[derive(Builder, Debug, Clone)]
#[builder(setter(strip_option))]
pub struct Request<'a> {
    #[builder(default, setter(into))]
    pub(crate) created_at: Option<Cow<'a, str>>,

    #[builder(default, setter(into))]
    pub(crate) hostcluster_id: Option<Cow<'a, str>>,

    #[builder(default, setter(into))]
    pub(crate) links: Option<Vec<Links<'a>>>,

    #[builder(default, setter(into))]
    pub(crate) member_ids: Option<Vec<Cow<'a, str>>>,

    #[builder(default, setter(into))]
    pub(crate) name: Option<Cow<'a, str>>,

    #[builder(default, private, setter(into, name = "_properties"))]
    pub(crate) properties: Option<BTreeMap<Cow<'a, str>, Cow<'a, str>>>,

    #[builder(default)]
    pub(crate) status: Option<Status>,

    #[builder(default, setter(into))]
    pub(crate) status_reason: Option<Cow<'a, str>>,

    #[builder(default, setter(into))]
    pub(crate) updated_at: Option<Cow<'a, str>>,

    #[builder(default, setter(into))]
    pub(crate) uuid: Option<Cow<'a, str>>,

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
    pub fn properties<I, K, V>(&mut self, iter: I) -> &mut Self
    where
        I: Iterator<Item = (K, V)>,
        K: Into<Cow<'a, str>>,
        V: Into<Cow<'a, str>>,
    {
        self.properties
            .get_or_insert(None)
            .get_or_insert_with(BTreeMap::new)
            .extend(iter.map(|(k, v)| (k.into(), v.into())));
        self
    }

    /// Add a single header to the Federation.
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

impl RestEndpoint for Request<'_> {
    fn method(&self) -> http::Method {
        http::Method::POST
    }

    fn endpoint(&self) -> Cow<'static, str> {
        "federations".to_string().into()
    }

    fn parameters(&self) -> QueryParams {
        QueryParams::default()
    }

    fn body(&self) -> Result<Option<(&'static str, Vec<u8>)>, BodyError> {
        let mut params = JsonBodyParams::default();

        if let Some(val) = &self.uuid {
            params.push("uuid", serde_json::to_value(val)?);
        }
        if let Some(val) = &self.name {
            params.push("name", serde_json::to_value(val)?);
        }
        if let Some(val) = &self.hostcluster_id {
            params.push("hostcluster_id", serde_json::to_value(val)?);
        }
        if let Some(val) = &self.member_ids {
            params.push("member_ids", serde_json::to_value(val)?);
        }
        if let Some(val) = &self.status {
            params.push("status", serde_json::to_value(val)?);
        }
        if let Some(val) = &self.status_reason {
            params.push("status_reason", serde_json::to_value(val)?);
        }
        if let Some(val) = &self.properties {
            params.push("properties", serde_json::to_value(val)?);
        }
        if let Some(val) = &self.links {
            params.push("links", serde_json::to_value(val)?);
        }
        if let Some(val) = &self.created_at {
            params.push("created_at", serde_json::to_value(val)?);
        }
        if let Some(val) = &self.updated_at {
            params.push("updated_at", serde_json::to_value(val)?);
        }

        params.into_body()
    }

    fn service_type(&self) -> ServiceType {
        ServiceType::ContainerInfrastructureManagement
    }

    fn response_key(&self) -> Option<Cow<'static, str>> {
        None
    }

    /// Returns headers to be set into the request
    fn request_headers(&self) -> Option<&HeaderMap> {
        self._headers.as_ref()
    }

    /// Returns required API version
    fn api_version(&self) -> Option<ApiVersion> {
        Some(ApiVersion::new(1, 0))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[cfg(feature = "sync")]
    use crate::api::Query;
    use crate::test::client::FakeOpenStackClient;
    use crate::types::ServiceType;
    use http::{HeaderName, HeaderValue};
    use httpmock::MockServer;
    use serde_json::json;

    #[test]
    fn test_service_type() {
        assert_eq!(
            Request::builder().build().unwrap().service_type(),
            ServiceType::ContainerInfrastructureManagement
        );
    }

    #[test]
    fn test_response_key() {
        assert!(Request::builder().build().unwrap().response_key().is_none())
    }

    #[cfg(feature = "sync")]
    #[test]
    fn endpoint() {
        let server = MockServer::start();
        let client = FakeOpenStackClient::new(server.base_url());
        let mock = server.mock(|when, then| {
            when.method(httpmock::Method::POST)
                .path("/federations".to_string());

            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({ "dummy": {} }));
        });

        let endpoint = Request::builder().build().unwrap();
        let _: serde_json::Value = endpoint.query(&client).unwrap();
        mock.assert();
    }

    #[cfg(feature = "sync")]
    #[test]
    fn endpoint_headers() {
        let server = MockServer::start();
        let client = FakeOpenStackClient::new(server.base_url());
        let mock = server.mock(|when, then| {
            when.method(httpmock::Method::POST)
                .path("/federations".to_string())
                .header("foo", "bar")
                .header("not_foo", "not_bar");
            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({ "dummy": {} }));
        });

        let endpoint = Request::builder()
            .headers(
                [(
                    Some(HeaderName::from_static("foo")),
                    HeaderValue::from_static("bar"),
                )]
                .into_iter(),
            )
            .header("not_foo", "not_bar")
            .build()
            .unwrap();
        let _: serde_json::Value = endpoint.query(&client).unwrap();
        mock.assert();
    }
}
