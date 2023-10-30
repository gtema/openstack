//! Create Subnet
use derive_builder::Builder;
use http::{HeaderMap, HeaderName, HeaderValue};

use crate::api::common::CommaSeparatedList;
use crate::api::rest_endpoint_prelude::*;

use serde_json::Value;
use std::collections::BTreeSet;

/// Query for subnet.post operation.
#[derive(Debug, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct Subnet<'a> {
    /// List of allocation pools each of which has a start and an end address
    /// for this subnet
    #[builder(default, setter(name = "_allocation_pools"), private)]
    allocation_pools: Option<Vec<Value>>,

    /// The CIDR.
    #[builder(default, setter(into))]
    cidr: Option<Cow<'a, str>>,

    /// The subnet description.
    #[builder(default, setter(into))]
    description: Option<Cow<'a, str>>,

    /// A list of DNS nameservers.
    #[builder(default, private, setter(name = "_dns_nameservers"))]
    dns_nameservers: Option<BTreeSet<Cow<'a, str>>>,

    /// Whether to publish DNS records for fixed IPs
    #[builder(default)]
    dns_publish_fixed_ip: Option<bool>,

    /// Set to ``True`` if DHCP is enabled and ``False`` if DHCP is disabled.
    #[builder(default)]
    is_dhcp_enabled: Option<bool>,

    /// The gateway IP address.
    #[builder(default, setter(into))]
    gateway_ip: Option<Cow<'a, str>>,

    /// A list of host routes.
    #[builder(default, private, setter(name = "_host_routes"))]
    host_routes: Option<BTreeSet<Cow<'a, str>>>,

    /// The IP version, which is 4 or 6.
    #[builder(default)]
    ip_version: Option<u32>,

    /// The IPv6 address modes which are 'dhcpv6-stateful', 'dhcpv6-stateless'
    /// or 'slaac'.
    #[builder(default, setter(into))]
    ipv6_address_mode: Option<Cow<'a, str>>,

    /// The IPv6 router advertisements modes which can be 'slaac',
    /// 'dhcpv6-stateful', 'dhcpv6-stateless'.
    #[builder(default, setter(into))]
    ipv6_ra_mode: Option<Cow<'a, str>>,

    /// The subnet name.
    #[builder(default, setter(into))]
    name: Option<Cow<'a, str>>,

    /// The ID of the attached network.
    #[builder(default, setter(into))]
    network_id: Option<Cow<'a, str>>,

    /// The prefix length to use for subnet allocation from a subnet pool
    #[builder(default, setter(into))]
    prefix_length: Option<Cow<'a, str>>,

    /// The ID of the project this subnet is associated with.
    #[builder(default, setter(into))]
    project_id: Option<Cow<'a, str>>,

    /// None
    #[builder(default)]
    revision_number: Option<u32>,

    /// The ID of the segment this subnet is associated with.
    #[builder(default, setter(into))]
    segment_id: Option<Cow<'a, str>>,

    /// Service types for this subnet
    #[builder(default, private, setter(name = "_service_types"))]
    service_types: Option<BTreeSet<Cow<'a, str>>>,

    /// The subnet pool ID from which to obtain a CIDR.
    #[builder(default, setter(into))]
    subnet_pool_id: Option<Cow<'a, str>>,

    /// Subnet Tags.
    #[builder(default, private, setter(name = "_tags"))]
    tags: Option<BTreeSet<Cow<'a, str>>>,

    /// Tenant_id (deprecated attribute).
    #[builder(default, setter(into))]
    tenant_id: Option<Cow<'a, str>>,

    /// Whether to use the default subnet pool to obtain a CIDR.
    #[builder(default)]
    use_default_subnet_pool: Option<bool>,

    #[builder(setter(name = "_headers"), default, private)]
    _headers: Option<HeaderMap>,
}

impl<'a> Subnet<'a> {
    /// Create a builder for the endpoint.
    pub fn builder() -> SubnetBuilder<'a> {
        SubnetBuilder::default()
    }
}

impl<'a> SubnetBuilder<'a> {
    /// List of allocation pools each of which has a start and an end address
    /// for this subnet
    pub fn allocation_pools<I, T>(&mut self, iter: I) -> &mut Self
    where
        I: Iterator<Item = T>,
        T: Into<Value>,
    {
        self.allocation_pools
            .get_or_insert(None)
            .get_or_insert_with(Vec::new)
            .extend(iter.map(Into::into));
        self
    }

    /// A list of DNS nameservers.
    pub fn dns_nameservers<I, T>(&mut self, iter: I) -> &mut Self
    where
        I: Iterator<Item = T>,
        T: Into<Cow<'a, str>>,
    {
        self.dns_nameservers
            .get_or_insert(None)
            .get_or_insert_with(BTreeSet::new)
            .extend(iter.map(Into::into));
        self
    }

    /// A list of host routes.
    pub fn host_routes<I, T>(&mut self, iter: I) -> &mut Self
    where
        I: Iterator<Item = T>,
        T: Into<Cow<'a, str>>,
    {
        self.host_routes
            .get_or_insert(None)
            .get_or_insert_with(BTreeSet::new)
            .extend(iter.map(Into::into));
        self
    }

    /// Service types for this subnet
    pub fn service_types<I, T>(&mut self, iter: I) -> &mut Self
    where
        I: Iterator<Item = T>,
        T: Into<Cow<'a, str>>,
    {
        self.service_types
            .get_or_insert(None)
            .get_or_insert_with(BTreeSet::new)
            .extend(iter.map(Into::into));
        self
    }

    /// Subnet Tags.
    pub fn tags<I, T>(&mut self, iter: I) -> &mut Self
    where
        I: Iterator<Item = T>,
        T: Into<Cow<'a, str>>,
    {
        self.tags
            .get_or_insert(None)
            .get_or_insert_with(BTreeSet::new)
            .extend(iter.map(Into::into));
        self
    }

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

impl<'a> RestEndpoint for Subnet<'a> {
    fn method(&self) -> Method {
        Method::POST
    }

    fn endpoint(&self) -> Cow<'static, str> {
        "subnets".to_string().into()
    }

    fn parameters(&self) -> QueryParams {
        QueryParams::default()
    }

    fn body(&self) -> Result<Option<(&'static str, Vec<u8>)>, BodyError> {
        let mut params = JsonBodyParams::default();

        params.push_opt("allocation_pools", self.allocation_pools.as_ref());
        params.push_opt("cidr", self.cidr.as_ref());
        params.push_opt("description", self.description.as_ref());
        params.push_opt("dns_nameservers", self.dns_nameservers.as_ref());
        params.push_opt("dns_publish_fixed_ip", self.dns_publish_fixed_ip);
        params.push_opt("enable_dhcp", self.is_dhcp_enabled);
        params.push_opt("gateway_ip", self.gateway_ip.as_ref());
        params.push_opt("host_routes", self.host_routes.as_ref());
        params.push_opt("ip_version", self.ip_version);
        params.push_opt("ipv6_address_mode", self.ipv6_address_mode.as_ref());
        params.push_opt("ipv6_ra_mode", self.ipv6_ra_mode.as_ref());
        params.push_opt("name", self.name.as_ref());
        params.push_opt("network_id", self.network_id.as_ref());
        params.push_opt("prefixlen", self.prefix_length.as_ref());
        params.push_opt("project_id", self.project_id.as_ref());
        params.push_opt("revision_number", self.revision_number);
        params.push_opt("segment_id", self.segment_id.as_ref());
        params.push_opt("service_types", self.service_types.as_ref());
        params.push_opt("subnetpool_id", self.subnet_pool_id.as_ref());
        params.push_opt("tags", self.tags.as_ref());
        params.push_opt("tenant_id", self.tenant_id.as_ref());
        params.push_opt("use_default_subnetpool", self.use_default_subnet_pool);
        params.into_body_with_root_key("subnet")
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
    use serde_json::json;

    #[test]
    fn test_service_type() {
        assert_eq!(
            Subnet::builder().build().unwrap().service_type(),
            ServiceType::Network
        );
    }

    #[test]
    fn test_response_key() {
        assert_eq!(
            Subnet::builder().build().unwrap().response_key().unwrap(),
            "subnet"
        );
    }

    #[test]
    fn endpoint() {
        let client = MockServerClient::new();
        let mock = client.server.mock(|when, then| {
            when.method(httpmock::Method::POST)
                .path(format!("/subnets",));

            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({ "subnet": {} }));
        });

        let endpoint = Subnet::builder().build().unwrap();
        let _: serde_json::Value = endpoint.query(&client).unwrap();
        mock.assert();
    }

    #[test]
    fn endpoint_headers() {
        let client = MockServerClient::new();
        let mock = client.server.mock(|when, then| {
            when.method(httpmock::Method::POST)
                .path(format!("/subnets",))
                .header("foo", "bar")
                .header("not_foo", "not_bar");
            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({ "subnet": {} }));
        });

        let endpoint = Subnet::builder()
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

    #[test]
    fn endpoint_body() {
        let endpoint = Subnet::builder()
            .allocation_pools(["allocation_pools"].iter().cloned())
            .cidr("cidr")
            .description("description")
            .dns_nameservers(["dns_nameservers"].iter().cloned())
            .gateway_ip("gateway_ip")
            .host_routes(["host_routes"].iter().cloned())
            .ipv6_address_mode("ipv6_address_mode")
            .ipv6_ra_mode("ipv6_ra_mode")
            .name("name")
            .network_id("network_id")
            .prefix_length("prefixlen")
            .project_id("project_id")
            .segment_id("segment_id")
            .service_types(["service_types"].iter().cloned())
            .subnet_pool_id("subnetpool_id")
            .tags(["tags"].iter().cloned())
            .tenant_id("tenant_id")
            .build()
            .unwrap();

        let (mime, body) = endpoint.body().unwrap().unwrap();
        assert_eq!(
            std::str::from_utf8(&body).unwrap(),
            json!({
              "subnet": {
                 "allocation_pools": ["allocation_pools"],
                 "cidr": "cidr",
                 "description": "description",
                 "dns_nameservers": ["dns_nameservers"],
                 "gateway_ip": "gateway_ip",
                 "host_routes": ["host_routes"],
                 "ipv6_address_mode": "ipv6_address_mode",
                 "ipv6_ra_mode": "ipv6_ra_mode",
                 "name": "name",
                 "network_id": "network_id",
                 "prefixlen": "prefixlen",
                 "project_id": "project_id",
                 "segment_id": "segment_id",
                 "service_types": ["service_types"],
                 "subnetpool_id": "subnetpool_id",
                 "tags": ["tags"],
                 "tenant_id": "tenant_id",
             }
            })
            .to_string()
        );
    }
}
