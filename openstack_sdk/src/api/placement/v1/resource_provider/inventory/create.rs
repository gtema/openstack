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

//! POST to create one inventory.
//!
//! On success return a 201 response, a location header pointing to the newly
//! created inventory and an application/json representation of the inventory.
//!
use derive_builder::Builder;
use http::{HeaderMap, HeaderName, HeaderValue};

use crate::api::rest_endpoint_prelude::*;

use serde::Deserialize;
use serde::Serialize;
use std::borrow::Cow;
use std::collections::BTreeMap;

#[derive(Builder, Debug, Deserialize, Clone, Serialize)]
#[builder(setter(strip_option))]
pub struct InventoriesItem {
    /// It is used in determining whether consumption of the resource of the
    /// provider can exceed physical constraints.
    ///
    /// For example, for a vCPU resource with:
    ///
    /// ```text
    /// allocation_ratio = 16.0
    /// total = 8
    ///
    /// ```
    ///
    /// Overall capacity is equal to 128 vCPUs.
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub(crate) allocation_ratio: Option<f32>,

    /// A maximum amount any single allocation against an inventory can have.
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub(crate) max_unit: Option<i32>,

    /// A minimum amount any single allocation against an inventory can have.
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub(crate) min_unit: Option<i32>,

    /// The amount of the resource a provider has reserved for its own use.
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub(crate) reserved: Option<i32>,

    /// A representation of the divisible amount of the resource that may be
    /// requested. For example, step_size = 5 means that only values divisible
    /// by 5 (5, 10, 15, etc.) can be requested.
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub(crate) step_size: Option<i32>,

    /// The actual amount of the resource that the provider can accommodate.
    ///
    #[serde()]
    #[builder()]
    pub(crate) total: i32,
}

#[derive(Builder, Debug, Clone)]
#[builder(setter(strip_option))]
pub struct Request<'a> {
    /// A dictionary of inventories keyed by resource classes.
    ///
    #[builder(private, setter(name = "_inventories"))]
    pub(crate) inventories: BTreeMap<Cow<'a, str>, InventoriesItem>,

    /// A consistent view marker that assists with the management of concurrent
    /// resource provider updates.
    ///
    #[builder()]
    pub(crate) resource_provider_generation: i32,

    /// uuid parameter for
    /// /resource_providers/{uuid}/inventories/{resource_class} API
    ///
    #[builder(default, setter(into))]
    uuid: Cow<'a, str>,

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
    /// A dictionary of inventories keyed by resource classes.
    ///
    pub fn inventories<I, K, V>(&mut self, iter: I) -> &mut Self
    where
        I: Iterator<Item = (K, V)>,
        K: Into<Cow<'a, str>>,
        V: Into<InventoriesItem>,
    {
        self.inventories
            .get_or_insert_with(BTreeMap::new)
            .extend(iter.map(|(k, v)| (k.into(), v.into())));
        self
    }

    /// Add a single header to the Inventory.
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
        format!(
            "resource_providers/{uuid}/inventories",
            uuid = self.uuid.as_ref(),
        )
        .into()
    }

    fn parameters(&self) -> QueryParams {
        QueryParams::default()
    }

    fn body(&self) -> Result<Option<(&'static str, Vec<u8>)>, BodyError> {
        let mut params = JsonBodyParams::default();

        params.push("inventories", serde_json::to_value(&self.inventories)?);
        params.push(
            "resource_provider_generation",
            serde_json::to_value(self.resource_provider_generation)?,
        );

        params.into_body()
    }

    fn service_type(&self) -> ServiceType {
        ServiceType::Placement
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
            Request::builder()
                .inventories(BTreeMap::<String, InventoriesItem>::new().into_iter())
                .resource_provider_generation(123)
                .build()
                .unwrap()
                .service_type(),
            ServiceType::Placement
        );
    }

    #[test]
    fn test_response_key() {
        assert!(Request::builder()
            .inventories(BTreeMap::<String, InventoriesItem>::new().into_iter())
            .resource_provider_generation(123)
            .build()
            .unwrap()
            .response_key()
            .is_none())
    }

    #[cfg(feature = "sync")]
    #[test]
    fn endpoint() {
        let client = MockServerClient::new();
        let mock = client.server.mock(|when, then| {
            when.method(httpmock::Method::POST).path(format!(
                "/resource_providers/{uuid}/inventories",
                uuid = "uuid",
            ));

            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({ "dummy": {} }));
        });

        let endpoint = Request::builder()
            .uuid("uuid")
            .inventories(BTreeMap::<String, InventoriesItem>::new().into_iter())
            .resource_provider_generation(123)
            .build()
            .unwrap();
        let _: serde_json::Value = endpoint.query(&client).unwrap();
        mock.assert();
    }

    #[cfg(feature = "sync")]
    #[test]
    fn endpoint_headers() {
        let client = MockServerClient::new();
        let mock = client.server.mock(|when, then| {
            when.method(httpmock::Method::POST)
                .path(format!(
                    "/resource_providers/{uuid}/inventories",
                    uuid = "uuid",
                ))
                .header("foo", "bar")
                .header("not_foo", "not_bar");
            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({ "dummy": {} }));
        });

        let endpoint = Request::builder()
            .uuid("uuid")
            .inventories(BTreeMap::<String, InventoriesItem>::new().into_iter())
            .resource_provider_generation(123)
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
