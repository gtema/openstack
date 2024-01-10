//! Creates multiple subnets in a single request. Specify a list of subnets in
//! the request body.
//!
//! The bulk create operation is always atomic. Either all or no
//! subnets in the request body are created.
//!
//! Normal response codes: 201
//!
//! Error response codes: 400, 401, 403, 404, 409
//!
use derive_builder::Builder;
use http::{HeaderMap, HeaderName, HeaderValue};

use crate::api::rest_endpoint_prelude::*;
use serde::Serialize;

use serde::Deserialize;
use std::borrow::Cow;

#[derive(Builder, Debug, Deserialize, Clone, Serialize)]
#[builder(setter(strip_option))]
pub struct AllocationPools<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    start: Option<Cow<'a, str>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    end: Option<Cow<'a, str>>,
}

#[derive(Builder, Debug, Deserialize, Clone, Serialize)]
#[builder(setter(strip_option))]
pub struct HostRoutes<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    destination: Option<Cow<'a, str>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    nexthop: Option<Cow<'a, str>>,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
#[serde(untagged)]
pub enum Ipv6RaMode {
    #[serde(alias = "slaac")]
    Slaac,
    #[serde(alias = "dhcpv6-stateless")]
    Dhcpv6Stateless,
    #[serde(alias = "dhcpv6-stateful")]
    Dhcpv6Stateful,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
#[serde(untagged)]
pub enum Ipv6AddressMode {
    #[serde(alias = "slaac")]
    Slaac,
    #[serde(alias = "dhcpv6-stateless")]
    Dhcpv6Stateless,
    #[serde(alias = "dhcpv6-stateful")]
    Dhcpv6Stateful,
}

/// A `subnet` object.
#[derive(Builder, Debug, Deserialize, Clone, Serialize)]
#[builder(setter(strip_option))]
pub struct Subnet<'a> {
    /// Human-readable name of the resource. Default is an empty string.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    name: Option<Cow<'a, str>>,

    /// The IP protocol version. Value is `4` or `6`.
    #[serde()]
    #[builder()]
    ip_version: i32,

    /// The ID of the network to which the subnet belongs.
    #[serde()]
    #[builder(setter(into))]
    network_id: Cow<'a, str>,

    /// The ID of the subnet pool associated with the subnet.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    subnetpool_id: Option<Option<Cow<'a, str>>>,

    /// The prefix length to use for subnet allocation from a subnet pool.
    /// If not specified, the `default\_prefixlen` value of the subnet pool
    /// will be used.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    prefixlen: Option<i32>,

    /// The CIDR of the subnet.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    cidr: Option<Option<Cow<'a, str>>>,

    /// Gateway IP of this subnet. If the value is `null` that implies no
    /// gateway is associated with the subnet. If the gateway\_ip is not
    /// specified, OpenStack Networking allocates an address from the CIDR
    /// for the gateway for the subnet by default.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    gateway_ip: Option<Cow<'a, str>>,

    /// Allocation pools with `start` and `end` IP addresses
    /// for this subnet. If allocation\_pools are not specified, OpenStack
    /// Networking automatically allocates pools for covering all IP addresses
    /// in the CIDR, excluding the address reserved for the subnet gateway by
    /// default.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    allocation_pools: Option<Vec<AllocationPools<'a>>>,

    /// List of dns name servers associated with the subnet. Default is an
    /// empty list.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    dns_nameservers: Option<Vec<Cow<'a, str>>>,

    /// Additional routes for the subnet. A list of dictionaries with
    /// `destination` and `nexthop` parameters. Default value is
    /// an empty list.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    host_routes: Option<Vec<HostRoutes<'a>>>,

    /// The ID of the project that owns the resource.
    /// Only administrative and users with advsvc role can specify
    /// a project ID other than their own.
    /// You cannot change this value through authorization policies.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    tenant_id: Option<Cow<'a, str>>,

    /// Indicates whether dhcp is enabled or disabled
    /// for the subnet. Default is `true`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    enable_dhcp: Option<bool>,

    /// The IPv6 router advertisement specifies whether the networking service
    /// should transmit ICMPv6 packets, for a subnet. Value is `slaac`,
    /// `dhcpv6-stateful`, `dhcpv6-stateless`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    ipv6_ra_mode: Option<Ipv6RaMode>,

    /// The IPv6 address modes specifies mechanisms for assigning IP addresses.
    /// Value is `slaac`, `dhcpv6-stateful`, `dhcpv6-stateless`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    ipv6_address_mode: Option<Ipv6AddressMode>,

    /// The service types associated with the subnet.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    service_types: Option<Vec<Cow<'a, str>>>,

    /// Whether to allocate this subnet from the default subnet pool.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    use_default_subnetpool: Option<bool>,

    /// Whether to publish DNS records for IPs from this subnet. Default
    /// is `false`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    dns_publish_fixed_ip: Option<bool>,

    /// A human-readable description for the resource.
    /// Default is an empty string.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    description: Option<Cow<'a, str>>,

    /// The ID of a network segment the subnet is associated with.
    /// It is available when `segment` extension is enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    segment_id: Option<Option<Cow<'a, str>>>,
}

#[derive(Builder, Debug, Clone)]
#[builder(setter(strip_option))]
pub struct Request<'a> {
    /// A `subnet` object.
    #[builder(setter(into))]
    subnet: Subnet<'a>,

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
    /// Add a single header to the Subnet.
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
    fn method(&self) -> Method {
        Method::POST
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("v2.0/subnets",).into()
    }

    fn parameters(&self) -> QueryParams {
        QueryParams::default()
    }

    fn body(&self) -> Result<Option<(&'static str, Vec<u8>)>, BodyError> {
        let mut params = JsonBodyParams::default();

        params.push("subnet", serde_json::to_value(&self.subnet)?);

        params.into_body()
    }

    fn service_type(&self) -> ServiceType {
        ServiceType::Network
    }

    fn response_key(&self) -> Option<Cow<'static, str>> {
        Some("subnet".into())
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
                .subnet(
                    SubnetBuilder::default()
                        .ip_version(123)
                        .network_id("foo")
                        .build()
                        .unwrap()
                )
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
                .subnet(
                    SubnetBuilder::default()
                        .ip_version(123)
                        .network_id("foo")
                        .build()
                        .unwrap()
                )
                .build()
                .unwrap()
                .response_key()
                .unwrap(),
            "subnet"
        );
    }

    #[test]
    fn endpoint() {
        let client = MockServerClient::new();
        let mock = client.server.mock(|when, then| {
            when.method(httpmock::Method::POST)
                .path(format!("/v2.0/subnets",));

            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({ "subnet": {} }));
        });

        let endpoint = Request::builder()
            .subnet(
                SubnetBuilder::default()
                    .ip_version(123)
                    .network_id("foo")
                    .build()
                    .unwrap(),
            )
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
                .path(format!("/v2.0/subnets",))
                .header("foo", "bar")
                .header("not_foo", "not_bar");
            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({ "subnet": {} }));
        });

        let endpoint = Request::builder()
            .subnet(
                SubnetBuilder::default()
                    .ip_version(123)
                    .network_id("foo")
                    .build()
                    .unwrap(),
            )
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
