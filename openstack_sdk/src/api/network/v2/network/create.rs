//! Creates multiple networks in a single request.
//!
//! In the request body, specify a list of networks.
//!
//! The bulk create operation is always atomic. Either all or no
//! networks in the request body are created.
//!
//! Normal response codes: 201
//!
//! Error response codes: 400, 401
//!
use derive_builder::Builder;
use http::{HeaderMap, HeaderName, HeaderValue};

use crate::api::rest_endpoint_prelude::*;
use serde::Serialize;

use serde::Deserialize;
use std::borrow::Cow;

#[derive(Builder, Debug, Deserialize, Clone, Serialize)]
#[builder(setter(strip_option))]
pub struct Segments<'a> {
    /// The ID of the isolated segment on the physical network.
    /// The `network\_type` attribute defines the segmentation model.
    /// For example, if the `network\_type` value is vlan, this ID is a vlan
    /// identifier. If the `network\_type` value is gre, this ID is a gre key.
    #[serde(
        rename = "provider:segmentation_id",
        skip_serializing_if = "Option::is_none"
    )]
    #[builder(default)]
    provider_segmentation_id: Option<i32>,

    /// The physical network where this network should be implemented.
    /// The Networking API v2.0 does not provide a way to list available
    /// physical networks. For example, the Open vSwitch plug-in configuration
    /// file defines a symbolic name that maps to specific bridges on each
    /// compute host.
    #[serde(
        rename = "provider:physical_network",
        skip_serializing_if = "Option::is_none"
    )]
    #[builder(default, setter(into))]
    provider_physical_network: Option<Cow<'a, str>>,

    /// The type of physical network that this network should be mapped to.
    /// For example, `flat`, `vlan`, `vxlan`, or `gre`.
    /// Valid values depend on a networking back-end.
    #[serde(
        rename = "provider:network_type",
        skip_serializing_if = "Option::is_none"
    )]
    #[builder(default, setter(into))]
    provider_network_type: Option<Cow<'a, str>>,
}

/// A `network` object.
#[derive(Builder, Debug, Deserialize, Clone, Serialize)]
#[builder(setter(strip_option))]
pub struct Network<'a> {
    /// Human-readable name of the network.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    name: Option<Cow<'a, str>>,

    /// The administrative state of the network, which is
    /// up (`true`) or down (`false`).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    admin_state_up: Option<bool>,

    /// The ID of the project that owns the resource.
    /// Only administrative and users with advsvc role can specify
    /// a project ID other than their own.
    /// You cannot change this value through authorization policies.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    tenant_id: Option<Cow<'a, str>>,

    /// Indicates whether this resource is shared across all projects.
    /// By default, only administrative users can change this value.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    shared: Option<bool>,

    /// Indicates whether the network has an external routing facility that’s
    /// not
    /// managed by the networking service.
    #[serde(rename = "router:external", skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    router_external: Option<bool>,

    /// A list of provider `segment` objects.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    segments: Option<Vec<Segments<'a>>>,

    /// The maximum transmission unit (MTU) value to
    /// address fragmentation. Minimum value is 68 for IPv4, and 1280 for
    /// IPv6.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    mtu: Option<i32>,

    /// The availability zone candidate for the network.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    availability_zone_hints: Option<Vec<Cow<'a, str>>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    ha: Option<bool>,

    /// The port security status of the network. Valid values are
    /// enabled (`true`) and disabled (`false`).
    /// This value is used as the default value of `port\_security\_enabled`
    /// field of a newly created port.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    port_security_enabled: Option<bool>,

    #[serde(
        rename = "provider:network_type",
        skip_serializing_if = "Option::is_none"
    )]
    #[builder(default, setter(into))]
    provider_network_type: Option<Cow<'a, str>>,

    #[serde(
        rename = "provider:physical_network",
        skip_serializing_if = "Option::is_none"
    )]
    #[builder(default, setter(into))]
    provider_physical_network: Option<Cow<'a, str>>,

    #[serde(
        rename = "provider:segmentation_id",
        skip_serializing_if = "Option::is_none"
    )]
    #[builder(default, setter(into))]
    provider_segmentation_id: Option<Cow<'a, str>>,

    /// The ID of the QoS policy associated with the network.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    qos_policy_id: Option<Option<Cow<'a, str>>>,

    /// The network is default or not.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    is_default: Option<bool>,

    /// A valid DNS domain.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    dns_domain: Option<Cow<'a, str>>,

    /// A human-readable description for the resource.
    /// Default is an empty string.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    description: Option<Cow<'a, str>>,
}

#[derive(Builder, Debug, Clone)]
#[builder(setter(strip_option))]
pub struct Request<'a> {
    /// A `network` object.
    #[builder(setter(into))]
    network: Network<'a>,

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
    /// Add a single header to the Network.
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
        "v2.0/networks".to_string().into()
    }

    fn parameters(&self) -> QueryParams {
        QueryParams::default()
    }

    fn body(&self) -> Result<Option<(&'static str, Vec<u8>)>, BodyError> {
        let mut params = JsonBodyParams::default();

        params.push("network", serde_json::to_value(&self.network)?);

        params.into_body()
    }

    fn service_type(&self) -> ServiceType {
        ServiceType::Network
    }

    fn response_key(&self) -> Option<Cow<'static, str>> {
        Some("network".into())
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
                .network(NetworkBuilder::default().build().unwrap())
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
                .network(NetworkBuilder::default().build().unwrap())
                .build()
                .unwrap()
                .response_key()
                .unwrap(),
            "network"
        );
    }

    #[test]
    fn endpoint() {
        let client = MockServerClient::new();
        let mock = client.server.mock(|when, then| {
            when.method(httpmock::Method::POST)
                .path("/v2.0/networks".to_string());

            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({ "network": {} }));
        });

        let endpoint = Request::builder()
            .network(NetworkBuilder::default().build().unwrap())
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
                .path("/v2.0/networks".to_string())
                .header("foo", "bar")
                .header("not_foo", "not_bar");
            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({ "network": {} }));
        });

        let endpoint = Request::builder()
            .network(NetworkBuilder::default().build().unwrap())
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
