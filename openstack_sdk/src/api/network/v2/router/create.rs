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

//! Creates a logical router.
//!
//! This operation creates a logical router. The logical router does not have
//! any internal interface and it is not associated with any subnet. You can
//! optionally specify an external gateway for a router at create time. The
//! external gateway for the router must be plugged into an external network.
//! An external network has its `router:external` extended field set to `true`.
//! To specify an external gateway, the ID of the external network must be
//! passed in the `network_id` parameter of the `external_gateway_info`
//! attribute in the request body.
//!
//! Normal response codes: 201
//!
//! Error response codes: 400, 401
//!
use derive_builder::Builder;
use http::{HeaderMap, HeaderName, HeaderValue};

use crate::api::rest_endpoint_prelude::*;

use serde::Deserialize;
use serde::Serialize;
use std::borrow::Cow;

#[derive(Builder, Debug, Deserialize, Clone, Serialize)]
#[builder(setter(strip_option))]
pub struct ExternalFixedIps<'a> {
    /// IP Address
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) ip_address: Option<Cow<'a, str>>,

    /// The subnet ID from which the IP address is assigned
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) subnet_id: Option<Cow<'a, str>>,
}

/// The external gateway information of the router. If the router has an
/// external gateway, this would be a dict with `network_id`, `enable_snat`,
/// `external_fixed_ips` and `qos_policy_id`. Otherwise, this would be `null`.
///
#[derive(Builder, Debug, Deserialize, Clone, Serialize)]
#[builder(setter(strip_option))]
pub struct ExternalGatewayInfo<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub(crate) enable_snat: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) external_fixed_ips: Option<Vec<ExternalFixedIps<'a>>>,

    #[serde()]
    #[builder(setter(into))]
    pub(crate) network_id: Cow<'a, str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) qos_policy_id: Option<Option<Cow<'a, str>>>,
}

/// A `router` object.
///
#[derive(Builder, Debug, Deserialize, Clone, Serialize)]
#[builder(setter(strip_option))]
pub struct Router<'a> {
    /// The administrative state of the resource, which is up (`true`) or down
    /// (`false`). Default is `true`.
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub(crate) admin_state_up: Option<bool>,

    /// The availability zone candidates for the router. It is available when
    /// `router_availability_zone` extension is enabled.
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) availability_zone_hints: Option<Vec<Cow<'a, str>>>,

    /// A human-readable description for the resource. Default is an empty
    /// string.
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) description: Option<Cow<'a, str>>,

    /// `true` indicates a distributed router. It is available when `dvr`
    /// extension is enabled.
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) distributed: Option<Option<bool>>,

    /// Enable NDP proxy attribute. Default is `false`, To persist this
    /// attribute value, set the `enable_ndp_proxy_by_default` option in the
    /// `neutron.conf` file. It is available when `router-extend-ndp-proxy`
    /// extension is enabled.
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) enable_ndp_proxy: Option<Option<bool>>,

    /// The external gateway information of the router. If the router has an
    /// external gateway, this would be a dict with `network_id`,
    /// `enable_snat`, `external_fixed_ips` and `qos_policy_id`. Otherwise,
    /// this would be `null`.
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) external_gateway_info: Option<Option<ExternalGatewayInfo<'a>>>,

    /// The ID of the flavor associated with the router.
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) flavor_id: Option<Cow<'a, str>>,

    /// `true` indicates a highly-available router. It is available when
    /// `l3-ha` extension is enabled.
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) ha: Option<Option<bool>>,

    /// Human-readable name of the resource. Default is an empty string.
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) name: Option<Cow<'a, str>>,

    /// The ID of the project that owns the resource. Only administrative and
    /// users with advsvc role can specify a project ID other than their own.
    /// You cannot change this value through authorization policies.
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) tenant_id: Option<Cow<'a, str>>,
}

#[derive(Builder, Debug, Clone)]
#[builder(setter(strip_option))]
pub struct Request<'a> {
    /// A `router` object.
    ///
    #[builder(setter(into))]
    pub(crate) router: Router<'a>,

    #[builder(setter(name = "_headers"), default, private)]
    _headers: Option<HeaderMap>,
}
impl<'a> Request<'a> {
    /// Create a builder for the endpoint.
    pub fn builder() -> RequestBuilder<'a> {
        RequestBuilder::default()
    }
}

impl RequestBuilder<'_> {
    /// Add a single header to the Router.
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
        "routers".to_string().into()
    }

    fn parameters(&self) -> QueryParams {
        QueryParams::default()
    }

    fn body(&self) -> Result<Option<(&'static str, Vec<u8>)>, BodyError> {
        let mut params = JsonBodyParams::default();

        params.push("router", serde_json::to_value(&self.router)?);

        params.into_body()
    }

    fn service_type(&self) -> ServiceType {
        ServiceType::Network
    }

    fn response_key(&self) -> Option<Cow<'static, str>> {
        Some("router".into())
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
            Request::builder()
                .router(RouterBuilder::default().build().unwrap())
                .build()
                .unwrap()
                .service_type(),
            ServiceType::Network
        );
    }

    #[test]
    fn test_response_key() {
        assert_eq!(
            Request::builder()
                .router(RouterBuilder::default().build().unwrap())
                .build()
                .unwrap()
                .response_key()
                .unwrap(),
            "router"
        );
    }

    #[cfg(feature = "sync")]
    #[test]
    fn endpoint() {
        let client = MockServerClient::new();
        let mock = client.server.mock(|when, then| {
            when.method(httpmock::Method::POST)
                .path("/routers".to_string());

            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({ "router": {} }));
        });

        let endpoint = Request::builder()
            .router(RouterBuilder::default().build().unwrap())
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
                .path("/routers".to_string())
                .header("foo", "bar")
                .header("not_foo", "not_bar");
            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({ "router": {} }));
        });

        let endpoint = Request::builder()
            .router(RouterBuilder::default().build().unwrap())
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
