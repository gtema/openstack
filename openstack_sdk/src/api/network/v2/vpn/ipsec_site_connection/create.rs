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

//! Creates a site-to-site IPsec connection for a service.
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

#[derive(Debug, Deserialize, Clone, Serialize)]
pub enum Initiator {
    #[serde(rename = "bi-directional")]
    BiDirectional,
    #[serde(rename = "response-only")]
    ResponseOnly,
}

/// An `ipsec_site_connection` object.
///
#[derive(Builder, Debug, Deserialize, Clone, Serialize)]
#[builder(setter(strip_option))]
pub struct IpsecSiteConnection<'a> {
    /// The administrative state of the resource, which is up (`true`) or down
    /// (`false`).
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub(crate) admin_state_up: Option<bool>,

    /// A human-readable description for the resource. Default is an empty
    /// string.
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) description: Option<Cow<'a, str>>,

    /// A dictionary with dead peer detection (DPD) protocol controls.
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) dpd: Option<Cow<'a, str>>,

    /// The ID of the IKE policy.
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) ikepolicy_id: Option<Cow<'a, str>>,

    /// Indicates whether this VPN can only respond to connections or both
    /// respond to and initiate connections. A valid value is `response- only`
    /// or `bi-directional`. Default is `bi-directional`.
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub(crate) initiator: Option<Initiator>,

    /// The ID of the IPsec policy.
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) ipsecpolicy_id: Option<Cow<'a, str>>,

    /// The ID for the endpoint group that contains private subnets for the
    /// local side of the connection. Yo must specify this parameter with the
    /// `peer_ep_group_id` parameter unless in backward- compatible mode where
    /// `peer_cidrs` is provided with a `subnet_id` for the VPN service.
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) local_ep_group_id: Option<Option<Cow<'a, str>>>,

    /// An ID to be used instead of the external IP address for a virtual
    /// router used in traffic between instances on different networks in
    /// east-west traffic. Most often, local ID would be domain name, email
    /// address, etc. If this is not configured then the external IP address
    /// will be used as the ID.
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) local_id: Option<Cow<'a, str>>,

    /// The maximum transmission unit (MTU) value to address fragmentation.
    /// Minimum value is 68 for IPv4, and 1280 for IPv6.
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub(crate) mtu: Option<i32>,

    /// Human-readable name of the resource. Default is an empty string.
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) name: Option<Cow<'a, str>>,

    /// The peer gateway public IPv4 or IPv6 address or FQDN.
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) peer_address: Option<Cow<'a, str>>,

    /// (Deprecated) Unique list of valid peer private CIDRs in the form \<
    /// net_address > / < prefix > .
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) peer_cidrs: Option<Vec<Cow<'a, str>>>,

    /// The ID for the endpoint group that contains private CIDRs in the form
    /// \< net_address > / < prefix > for the peer side of the connection. You
    /// must specify this parameter with the `local_ep_group_id` parameter
    /// unless in backward-compatible mode where `peer_cidrs` is provided with
    /// a `subnet_id` for the VPN service.
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) peer_ep_group_id: Option<Option<Cow<'a, str>>>,

    /// The peer router identity for authentication. A valid value is an IPv4
    /// address, IPv6 address, e-mail address, key ID, or FQDN. Typically, this
    /// value matches the `peer_address` value.
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) peer_id: Option<Cow<'a, str>>,

    /// The pre-shared key. A valid value is any string.
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) psk: Option<Cow<'a, str>>,

    /// The ID of the project.
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) tenant_id: Option<Cow<'a, str>>,

    /// The ID of the VPN service.
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) vpnservice_id: Option<Cow<'a, str>>,
}

#[derive(Builder, Debug, Clone)]
#[builder(setter(strip_option))]
pub struct Request<'a> {
    /// An `ipsec_site_connection` object.
    ///
    #[builder(setter(into))]
    pub(crate) ipsec_site_connection: IpsecSiteConnection<'a>,

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
    /// Add a single header to the Ipsec_Site_Connection.
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
        "vpn/ipsec-site-connections".to_string().into()
    }

    fn parameters(&self) -> QueryParams {
        QueryParams::default()
    }

    fn body(&self) -> Result<Option<(&'static str, Vec<u8>)>, BodyError> {
        let mut params = JsonBodyParams::default();

        params.push(
            "ipsec_site_connection",
            serde_json::to_value(&self.ipsec_site_connection)?,
        );

        params.into_body()
    }

    fn service_type(&self) -> ServiceType {
        ServiceType::Network
    }

    fn response_key(&self) -> Option<Cow<'static, str>> {
        Some("ipsec_site_connection".into())
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
                .ipsec_site_connection(IpsecSiteConnectionBuilder::default().build().unwrap())
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
                .ipsec_site_connection(IpsecSiteConnectionBuilder::default().build().unwrap())
                .build()
                .unwrap()
                .response_key()
                .unwrap(),
            "ipsec_site_connection"
        );
    }

    #[cfg(feature = "sync")]
    #[test]
    fn endpoint() {
        let client = MockServerClient::new();
        let mock = client.server.mock(|when, then| {
            when.method(httpmock::Method::POST)
                .path("/vpn/ipsec-site-connections".to_string());

            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({ "ipsec_site_connection": {} }));
        });

        let endpoint = Request::builder()
            .ipsec_site_connection(IpsecSiteConnectionBuilder::default().build().unwrap())
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
                .path("/vpn/ipsec-site-connections".to_string())
                .header("foo", "bar")
                .header("not_foo", "not_bar");
            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({ "ipsec_site_connection": {} }));
        });

        let endpoint = Request::builder()
            .ipsec_site_connection(IpsecSiteConnectionBuilder::default().build().unwrap())
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
