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

//! Updates a namespace.
//!
//! Normal response codes: 200
//!
//! Error response codes: 400, 401, 403, 404, 409
//!
use derive_builder::Builder;
use http::{HeaderMap, HeaderName, HeaderValue};

use crate::api::rest_endpoint_prelude::*;

use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;
use std::borrow::Cow;
use std::collections::BTreeMap;

#[derive(Debug, Deserialize, Clone, Serialize)]
pub enum Visibility {
    #[serde(rename = "private")]
    Private,
    #[serde(rename = "public")]
    Public,
}

#[derive(Builder, Debug, Deserialize, Clone, Serialize)]
#[builder(setter(strip_option))]
pub struct ResourceTypeAssociations<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) name: Option<Cow<'a, str>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) prefix: Option<Cow<'a, str>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) properties_target: Option<Cow<'a, str>>,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub enum Type {
    #[serde(rename = "array")]
    Array,
    #[serde(rename = "boolean")]
    Boolean,
    #[serde(rename = "integer")]
    Integer,
    #[serde(rename = "number")]
    Number,
    #[serde(rename = "object")]
    Object,
    #[serde(rename = "string")]
    String,
}

#[derive(Builder, Debug, Deserialize, Clone, Serialize)]
#[builder(setter(strip_option))]
pub struct Items<'a> {
    #[serde(rename = "enum", skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) _enum: Option<Vec<Cow<'a, str>>>,

    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub(crate) _type: Option<Type>,
}

#[derive(Builder, Debug, Deserialize, Clone, Serialize)]
#[builder(setter(strip_option))]
pub struct Properties<'a> {
    #[serde(rename = "additionalItems", skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub(crate) additional_items: Option<bool>,

    #[serde(rename = "default", skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) _default: Option<Value>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) description: Option<Cow<'a, str>>,

    #[serde(rename = "enum", skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) _enum: Option<Vec<Cow<'a, str>>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) items: Option<Items<'a>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub(crate) maximum: Option<f32>,

    #[serde(rename = "maxItems", skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub(crate) max_items: Option<i32>,

    #[serde(rename = "maxLength", skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub(crate) max_length: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub(crate) minimum: Option<f32>,

    #[serde(rename = "minItems", skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub(crate) min_items: Option<i32>,

    #[serde(rename = "minLength", skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub(crate) min_length: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) name: Option<Cow<'a, str>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) operators: Option<Vec<Cow<'a, str>>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) pattern: Option<Cow<'a, str>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub(crate) readonly: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) required: Option<Vec<Cow<'a, str>>>,

    #[serde()]
    #[builder(setter(into))]
    pub(crate) title: Cow<'a, str>,

    #[serde(rename = "type")]
    #[builder()]
    pub(crate) _type: Type,

    #[serde(rename = "uniqueItems", skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub(crate) unique_items: Option<bool>,
}

#[derive(Builder, Debug, Deserialize, Clone, Serialize)]
#[builder(setter(strip_option))]
pub struct Objects<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) description: Option<Cow<'a, str>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) name: Option<Cow<'a, str>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, private, setter(name = "_properties"))]
    pub(crate) properties: Option<BTreeMap<Cow<'a, str>, Properties<'a>>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) required: Option<Vec<Cow<'a, str>>>,
}

impl<'a> ObjectsBuilder<'a> {
    pub fn properties<I, K, V>(&mut self, iter: I) -> &mut Self
    where
        I: Iterator<Item = (K, V)>,
        K: Into<Cow<'a, str>>,
        V: Into<Properties<'a>>,
    {
        self.properties
            .get_or_insert(None)
            .get_or_insert_with(BTreeMap::new)
            .extend(iter.map(|(k, v)| (k.into(), v.into())));
        self
    }
}

#[derive(Builder, Debug, Deserialize, Clone, Serialize)]
#[builder(setter(strip_option))]
pub struct Tags<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) name: Option<Cow<'a, str>>,
}

#[derive(Builder, Debug, Clone)]
#[builder(setter(strip_option))]
pub struct Request<'a> {
    /// The description of the namespace.
    ///
    #[builder(default, setter(into))]
    pub(crate) description: Option<Cow<'a, str>>,

    /// User-friendly name to use in a UI to display the namespace name.
    ///
    #[builder(default, setter(into))]
    pub(crate) display_name: Option<Cow<'a, str>>,

    /// An identifier (a name) for the namespace. The value must be unique
    /// across all users.
    ///
    #[builder(setter(into))]
    pub(crate) namespace: Cow<'a, str>,

    #[builder(default, setter(into))]
    pub(crate) objects: Option<Vec<Objects<'a>>>,

    /// Owner of the namespace.
    ///
    #[builder(default, setter(into))]
    pub(crate) owner: Option<Cow<'a, str>>,

    #[builder(default, private, setter(name = "_properties"))]
    pub(crate) properties: Option<BTreeMap<Cow<'a, str>, Properties<'a>>>,

    /// Namespace protection for deletion. A valid value is `true` or `false`.
    /// Default is `false`.
    ///
    #[builder(default)]
    pub(crate) protected: Option<bool>,

    #[builder(default, setter(into))]
    pub(crate) resource_type_associations: Option<Vec<ResourceTypeAssociations<'a>>>,

    #[builder(default, setter(into))]
    pub(crate) tags: Option<Vec<Tags<'a>>>,

    /// The namespace visibility. A valid value is `public` or `private`.
    /// Default is `private`.
    ///
    #[builder(default)]
    pub(crate) visibility: Option<Visibility>,

    /// namespace_name parameter for /v2/metadefs/namespaces/{namespace_name}
    /// API
    ///
    #[builder(default, setter(into))]
    namespace_name: Cow<'a, str>,

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
        V: Into<Properties<'a>>,
    {
        self.properties
            .get_or_insert(None)
            .get_or_insert_with(BTreeMap::new)
            .extend(iter.map(|(k, v)| (k.into(), v.into())));
        self
    }

    /// Add a single header to the Namespace.
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
        http::Method::PUT
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!(
            "v2/metadefs/namespaces/{namespace_name}",
            namespace_name = self.namespace_name.as_ref(),
        )
        .into()
    }

    fn parameters(&self) -> QueryParams {
        QueryParams::default()
    }

    fn body(&self) -> Result<Option<(&'static str, Vec<u8>)>, BodyError> {
        let mut params = JsonBodyParams::default();

        params.push("namespace", serde_json::to_value(&self.namespace)?);
        if let Some(val) = &self.display_name {
            params.push("display_name", serde_json::to_value(val)?);
        }
        if let Some(val) = &self.description {
            params.push("description", serde_json::to_value(val)?);
        }
        if let Some(val) = &self.visibility {
            params.push("visibility", serde_json::to_value(val)?);
        }
        if let Some(val) = &self.protected {
            params.push("protected", serde_json::to_value(val)?);
        }
        if let Some(val) = &self.owner {
            params.push("owner", serde_json::to_value(val)?);
        }
        if let Some(val) = &self.resource_type_associations {
            params.push("resource_type_associations", serde_json::to_value(val)?);
        }
        if let Some(val) = &self.properties {
            params.push("properties", serde_json::to_value(val)?);
        }
        if let Some(val) = &self.objects {
            params.push("objects", serde_json::to_value(val)?);
        }
        if let Some(val) = &self.tags {
            params.push("tags", serde_json::to_value(val)?);
        }

        params.into_body()
    }

    fn service_type(&self) -> ServiceType {
        ServiceType::Image
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
    #![allow(unused_imports)]
    use super::*;
    use crate::api::Query;
    use crate::test::client::MockServerClient;
    use crate::types::ServiceType;
    use http::{HeaderName, HeaderValue};
    use serde_json::json;

    #[test]
    fn test_service_type() {
        assert_eq!(
            Request::builder()
                .namespace("foo")
                .build()
                .unwrap()
                .service_type(),
            ServiceType::Image
        );
    }

    #[test]
    fn test_response_key() {
        assert!(Request::builder()
            .namespace("foo")
            .build()
            .unwrap()
            .response_key()
            .is_none())
    }

    #[test]
    fn endpoint() {
        let client = MockServerClient::new();
        let mock = client.server.mock(|when, then| {
            when.method(httpmock::Method::PUT).path(format!(
                "/v2/metadefs/namespaces/{namespace_name}",
                namespace_name = "namespace_name",
            ));

            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({ "dummy": {} }));
        });

        let endpoint = Request::builder()
            .namespace_name("namespace_name")
            .namespace("foo")
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
                    "/v2/metadefs/namespaces/{namespace_name}",
                    namespace_name = "namespace_name",
                ))
                .header("foo", "bar")
                .header("not_foo", "not_bar");
            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({ "dummy": {} }));
        });

        let endpoint = Request::builder()
            .namespace_name("namespace_name")
            .namespace("foo")
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