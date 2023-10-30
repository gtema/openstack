//! Create Network
use derive_builder::Builder;
use http::{HeaderMap, HeaderName, HeaderValue};

use crate::api::common::CommaSeparatedList;
use crate::api::rest_endpoint_prelude::*;

use serde_json::Value;
use std::collections::BTreeSet;

/// Query for network.post operation.
#[derive(Debug, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct Network<'a> {
    /// The administrative state of the network, which is up ``True`` or down
    /// ``False``.
    #[builder(default)]
    is_admin_state_up: Option<bool>,

    /// Availability zone hints to use when scheduling the network.
    #[builder(default, private, setter(name = "_availability_zone_hints"))]
    availability_zone_hints: Option<BTreeSet<Cow<'a, str>>>,

    /// The network description.
    #[builder(default, setter(into))]
    description: Option<Cow<'a, str>>,

    /// The DNS domain associated.
    #[builder(default, setter(into))]
    dns_domain: Option<Cow<'a, str>>,

    /// Whether or not this is the default external network.
    #[builder(default)]
    is_default: Option<bool>,

    /// Read-only. The maximum transmission unit (MTU) of the network resource.
    #[builder(default)]
    mtu: Option<u32>,

    /// The network name.
    #[builder(default, setter(into))]
    name: Option<Cow<'a, str>>,

    /// The port security status, which is enabled ``True`` or disabled
    /// ``False``.  Available for multiple provider extensions.
    #[builder(default)]
    is_port_security_enabled: Option<bool>,

    /// The ID of the project this network is associated with.
    #[builder(default, setter(into))]
    project_id: Option<Cow<'a, str>>,

    /// The type of physical network that maps to this network resource. For
    /// example, ``flat``, ``vlan``, ``vxlan``, or ``gre``. Available for
    /// multiple provider extensions.
    #[builder(default, setter(into))]
    provider_network_type: Option<Cow<'a, str>>,

    /// The physical network where this network object is implemented.
    /// Available for multiple provider extensions.
    #[builder(default, setter(into))]
    provider_physical_network: Option<Cow<'a, str>>,

    /// An isolated segment ID on the physical network. The provider network
    /// type defines the segmentation model. Available for multiple provider
    /// extensions.
    #[builder(default, setter(into))]
    provider_segmentation_id: Option<Cow<'a, str>>,

    /// The ID of the QoS policy attached to the port.
    #[builder(default, setter(into))]
    qos_policy_id: Option<Cow<'a, str>>,

    /// Whether or not the router is external.
    #[builder(default)]
    is_router_external: Option<bool>,

    /// A list of provider segment objects. Available for multiple provider
    /// extensions.
    #[builder(default, setter(name = "_segments"), private)]
    segments: Option<Vec<Value>>,

    /// Indicates whether this network is shared across all tenants. By
    /// default, only administrative users can change this value.
    #[builder(default)]
    is_shared: Option<bool>,

    /// Indicates the VLAN transparency mode of the network
    #[builder(default)]
    is_vlan_transparent: Option<bool>,

    #[builder(setter(name = "_headers"), default, private)]
    _headers: Option<HeaderMap>,
}

impl<'a> Network<'a> {
    /// Create a builder for the endpoint.
    pub fn builder() -> NetworkBuilder<'a> {
        NetworkBuilder::default()
    }
}

impl<'a> NetworkBuilder<'a> {
    /// Availability zone hints to use when scheduling the network.
    pub fn availability_zone_hints<I, T>(&mut self, iter: I) -> &mut Self
    where
        I: Iterator<Item = T>,
        T: Into<Cow<'a, str>>,
    {
        self.availability_zone_hints
            .get_or_insert(None)
            .get_or_insert_with(BTreeSet::new)
            .extend(iter.map(Into::into));
        self
    }

    /// A list of provider segment objects. Available for multiple provider
    /// extensions.
    pub fn segments<I, T>(&mut self, iter: I) -> &mut Self
    where
        I: Iterator<Item = T>,
        T: Into<Value>,
    {
        self.segments
            .get_or_insert(None)
            .get_or_insert_with(Vec::new)
            .extend(iter.map(Into::into));
        self
    }

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

impl<'a> RestEndpoint for Network<'a> {
    fn method(&self) -> Method {
        Method::POST
    }

    fn endpoint(&self) -> Cow<'static, str> {
        "networks".to_string().into()
    }

    fn parameters(&self) -> QueryParams {
        QueryParams::default()
    }

    fn body(&self) -> Result<Option<(&'static str, Vec<u8>)>, BodyError> {
        let mut params = JsonBodyParams::default();

        params.push_opt("admin_state_up", self.is_admin_state_up);
        params.push_opt(
            "availability_zone_hints",
            self.availability_zone_hints.as_ref(),
        );
        params.push_opt("description", self.description.as_ref());
        params.push_opt("dns_domain", self.dns_domain.as_ref());
        params.push_opt("is_default", self.is_default);
        params.push_opt("mtu", self.mtu);
        params.push_opt("name", self.name.as_ref());
        params.push_opt("port_security_enabled", self.is_port_security_enabled);
        params.push_opt("project_id", self.project_id.as_ref());
        params.push_opt("provider:network_type", self.provider_network_type.as_ref());
        params.push_opt(
            "provider:physical_network",
            self.provider_physical_network.as_ref(),
        );
        params.push_opt(
            "provider:segmentation_id",
            self.provider_segmentation_id.as_ref(),
        );
        params.push_opt("qos_policy_id", self.qos_policy_id.as_ref());
        params.push_opt("router:external", self.is_router_external);
        params.push_opt("segments", self.segments.as_ref());
        params.push_opt("shared", self.is_shared);
        params.push_opt("vlan_transparent", self.is_vlan_transparent);
        params.into_body_with_root_key("network")
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
    use serde_json::json;

    #[test]
    fn test_service_type() {
        assert_eq!(
            Network::builder().build().unwrap().service_type(),
            ServiceType::Network
        );
    }

    #[test]
    fn test_response_key() {
        assert_eq!(
            Network::builder().build().unwrap().response_key().unwrap(),
            "network"
        );
    }

    #[test]
    fn endpoint() {
        let client = MockServerClient::new();
        let mock = client.server.mock(|when, then| {
            when.method(httpmock::Method::POST)
                .path(format!("/networks",));

            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({ "network": {} }));
        });

        let endpoint = Network::builder().build().unwrap();
        let _: serde_json::Value = endpoint.query(&client).unwrap();
        mock.assert();
    }

    #[test]
    fn endpoint_headers() {
        let client = MockServerClient::new();
        let mock = client.server.mock(|when, then| {
            when.method(httpmock::Method::POST)
                .path(format!("/networks",))
                .header("foo", "bar")
                .header("not_foo", "not_bar");
            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({ "network": {} }));
        });

        let endpoint = Network::builder()
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
        let endpoint = Network::builder()
            .availability_zone_hints(["availability_zone_hints"].iter().cloned())
            .description("description")
            .dns_domain("dns_domain")
            .name("name")
            .project_id("project_id")
            .provider_network_type("provider:network_type")
            .provider_physical_network("provider:physical_network")
            .provider_segmentation_id("provider:segmentation_id")
            .qos_policy_id("qos_policy_id")
            .segments(["segments"].iter().cloned())
            .build()
            .unwrap();

        let (mime, body) = endpoint.body().unwrap().unwrap();
        assert_eq!(
            std::str::from_utf8(&body).unwrap(),
            json!({
              "network": {
                 "availability_zone_hints": ["availability_zone_hints"],
                 "description": "description",
                 "dns_domain": "dns_domain",
                 "name": "name",
                 "project_id": "project_id",
                 "provider:network_type": "provider:network_type",
                 "provider:physical_network": "provider:physical_network",
                 "provider:segmentation_id": "provider:segmentation_id",
                 "qos_policy_id": "qos_policy_id",
                 "segments": ["segments"],
             }
            })
            .to_string()
        );
    }
}
