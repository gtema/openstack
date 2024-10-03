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

//! Create a zone
//!
use derive_builder::Builder;
use http::{HeaderMap, HeaderName, HeaderValue};

use crate::api::rest_endpoint_prelude::*;

use serde::Deserialize;
use serde::Serialize;
use std::borrow::Cow;
use std::collections::BTreeMap;

#[derive(Debug, Deserialize, Clone, Serialize)]
pub enum Type {
    #[serde(rename = "CATALOG")]
    Catalog,
    #[serde(rename = "PRIMARY")]
    Primary,
    #[serde(rename = "SECONDARY")]
    Secondary,
}

#[derive(Builder, Debug, Clone)]
#[builder(setter(strip_option))]
pub struct Request<'a> {
    /// Key:Value pairs of information about this zone, and the pool the user
    /// would like to place the zone in. This information can be used by the
    /// scheduler to place zones on the correct pool.
    ///
    #[builder(default, private, setter(name = "_attributes"))]
    pub(crate) attributes: Option<BTreeMap<Cow<'a, str>, Cow<'a, str>>>,

    /// Description for this zone
    ///
    #[builder(default, setter(into))]
    pub(crate) description: Option<Cow<'a, str>>,

    /// e-mail for the zone. Used in SOA records for the zone
    ///
    #[builder(default, setter(into))]
    pub(crate) email: Option<Cow<'a, str>>,

    /// Mandatory for secondary zones. The servers to slave from to get DNS
    /// information
    ///
    #[builder(default, setter(into))]
    pub(crate) masters: Option<Vec<Cow<'a, str>>>,

    /// DNS Name for the zone
    ///
    #[builder(default, setter(into))]
    pub(crate) name: Option<Cow<'a, str>>,

    /// TTL (Time to Live) for the zone.
    ///
    #[builder(default)]
    pub(crate) ttl: Option<i32>,

    /// Type of zone. PRIMARY is controlled by Designate, SECONDARY zones are
    /// slaved from another DNS Server. Defaults to PRIMARY
    ///
    #[builder(default)]
    pub(crate) _type: Option<Type>,

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
    /// Key:Value pairs of information about this zone, and the pool the user
    /// would like to place the zone in. This information can be used by the
    /// scheduler to place zones on the correct pool.
    ///
    pub fn attributes<I, K, V>(&mut self, iter: I) -> &mut Self
    where
        I: Iterator<Item = (K, V)>,
        K: Into<Cow<'a, str>>,
        V: Into<Cow<'a, str>>,
    {
        self.attributes
            .get_or_insert(None)
            .get_or_insert_with(BTreeMap::new)
            .extend(iter.map(|(k, v)| (k.into(), v.into())));
        self
    }

    /// Add a single header to the Zone.
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
        http::Method::POST
    }

    fn endpoint(&self) -> Cow<'static, str> {
        "zones".to_string().into()
    }

    fn parameters(&self) -> QueryParams {
        QueryParams::default()
    }

    fn body(&self) -> Result<Option<(&'static str, Vec<u8>)>, BodyError> {
        let mut params = JsonBodyParams::default();

        if let Some(val) = &self.name {
            params.push("name", serde_json::to_value(val)?);
        }
        if let Some(val) = &self.email {
            params.push("email", serde_json::to_value(val)?);
        }
        if let Some(val) = &self.ttl {
            params.push("ttl", serde_json::to_value(val)?);
        }
        if let Some(val) = &self.description {
            params.push("description", serde_json::to_value(val)?);
        }
        if let Some(val) = &self.masters {
            params.push("masters", serde_json::to_value(val)?);
        }
        if let Some(val) = &self._type {
            params.push("type", serde_json::to_value(val)?);
        }
        if let Some(val) = &self.attributes {
            params.push("attributes", serde_json::to_value(val)?);
        }

        params.into_body()
    }

    fn service_type(&self) -> ServiceType {
        ServiceType::Dns
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
        Some(ApiVersion::new(2, 0))
    }
}

#[cfg(test)]
mod tests {
    #![allow(unused_imports)]
    use super::*;
    #[cfg(feature = "sync")]
    use crate::api::Query;
    #[cfg(feature = "sync")]
    use crate::test::client::MockServerClient;
    use crate::types::ServiceType;
    use http::{HeaderName, HeaderValue};
    use serde_json::json;

    #[test]
    fn test_service_type() {
        assert_eq!(
            Request::builder().build().unwrap().service_type(),
            ServiceType::Dns
        );
    }

    #[test]
    fn test_response_key() {
        assert!(Request::builder().build().unwrap().response_key().is_none())
    }

    #[cfg(feature = "sync")]
    #[test]
    fn endpoint() {
        let client = MockServerClient::new();
        let mock = client.server.mock(|when, then| {
            when.method(httpmock::Method::POST)
                .path("/zones".to_string());

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
        let client = MockServerClient::new();
        let mock = client.server.mock(|when, then| {
            when.method(httpmock::Method::POST)
                .path("/zones".to_string())
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
