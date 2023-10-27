//! Create Network
use derive_builder::Builder;
use http::{HeaderMap, HeaderName, HeaderValue};
use std::collections::BTreeSet;

use crate::api::common::CommaSeparatedList;
use crate::api::rest_endpoint_prelude::*;

/// Query for network.post operation.
#[derive(Debug, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct Network<'a> {
    /// limit filter parameter
    #[builder(default, setter(into))]
    limit: Option<Cow<'a, str>>,

    /// marker filter parameter
    #[builder(default, setter(into))]
    marker: Option<Cow<'a, str>>,

    /// The network description.
    #[builder(default, setter(into))]
    description: Option<Cow<'a, str>>,

    /// The network name.
    #[builder(default, setter(into))]
    name: Option<Cow<'a, str>>,

    /// The network status.
    #[builder(default, setter(into))]
    status: Option<Cow<'a, str>>,

    /// The ID of the project this network is associated with.
    #[builder(default, setter(into))]
    project_id: Option<Cow<'a, str>>,

    /// The ID of the IPv4 address scope for the network.
    #[builder(default, setter(into))]
    ipv4_address_scope_id: Option<Cow<'a, str>>,

    /// The ID of the IPv6 address scope for the network.
    #[builder(default, setter(into))]
    ipv6_address_scope_id: Option<Cow<'a, str>>,

    /// The administrative state of the network, which is up ``True`` or down
    /// ``False``.
    #[builder(default)]
    is_admin_state_up: Option<bool>,

    /// The port security status, which is enabled ``True`` or disabled
    /// ``False``.  Available for multiple provider extensions.
    #[builder(default)]
    is_port_security_enabled: Option<bool>,

    /// Whether or not the router is external.
    #[builder(default)]
    is_router_external: Option<bool>,

    /// Indicates whether this network is shared across all tenants. By
    /// default, only administrative users can change this value.
    #[builder(default)]
    is_shared: Option<bool>,

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

    /// Network Tags.
    #[builder(default, private, setter(name = "_tags"))]
    tags: BTreeSet<Cow<'a, str>>,

    /// any_tags filter parameter
    #[builder(default, private, setter(name = "_any_tags"))]
    any_tags: Option<CommaSeparatedList<Cow<'a, str>>>,

    /// not_tags filter parameter
    #[builder(default, private, setter(name = "_not_tags"))]
    not_tags: Option<CommaSeparatedList<Cow<'a, str>>>,

    /// not_any_tags filter parameter
    #[builder(default, private, setter(name = "_not_any_tags"))]
    not_any_tags: Option<CommaSeparatedList<Cow<'a, str>>>,

    /// Availability zone hints to use when scheduling the network.
    #[builder(default, setter(into))]
    availability_zone_hints: Option<Cow<'a, str>>,

    /// Availability zones for the network.
    #[builder(default, setter(into))]
    availability_zones: Option<Cow<'a, str>>,

    /// Timestamp when the network was created.
    #[builder(default, setter(into))]
    created_at: Option<Cow<'a, str>>,

    /// The DNS domain associated.
    #[builder(default, setter(into))]
    dns_domain: Option<Cow<'a, str>>,

    /// Id of the resource
    #[builder(default, setter(into))]
    id: Option<Cow<'a, str>>,

    /// Whether or not this is the default external network.
    #[builder(default)]
    is_default: Option<bool>,

    /// Read-only. The maximum transmission unit (MTU) of the network resource.
    #[builder(default)]
    mtu: Option<u32>,

    /// The ID of the QoS policy attached to the port.
    #[builder(default, setter(into))]
    qos_policy_id: Option<Cow<'a, str>>,

    /// None
    #[builder(default)]
    revision_number: Option<u32>,

    /// A list of provider segment objects. Available for multiple provider
    /// extensions.
    #[builder(default, setter(into))]
    segments: Option<Cow<'a, str>>,

    /// The associated subnet IDs.
    #[builder(default, private, setter(name = "_subnet_ids"))]
    subnet_ids: BTreeSet<Cow<'a, str>>,

    /// Timestamp when the network was last updated.
    #[builder(default, setter(into))]
    updated_at: Option<Cow<'a, str>>,

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
    /// tags filter parameter
    pub fn tags<I, T>(&mut self, iter: I) -> &mut Self
    where
        I: Iterator<Item = T>,
        T: Into<Cow<'a, str>>,
    {
        self.tags
            .get_or_insert(None)
            .get_or_insert_with(CommaSeparatedList::new)
            .extend(iter.map(Into::into));
        self
    }

    /// any_tags filter parameter
    pub fn any_tags<I, T>(&mut self, iter: I) -> &mut Self
    where
        I: Iterator<Item = T>,
        T: Into<Cow<'a, str>>,
    {
        self.any_tags
            .get_or_insert(None)
            .get_or_insert_with(CommaSeparatedList::new)
            .extend(iter.map(Into::into));
        self
    }

    /// not_tags filter parameter
    pub fn not_tags<I, T>(&mut self, iter: I) -> &mut Self
    where
        I: Iterator<Item = T>,
        T: Into<Cow<'a, str>>,
    {
        self.not_tags
            .get_or_insert(None)
            .get_or_insert_with(CommaSeparatedList::new)
            .extend(iter.map(Into::into));
        self
    }

    /// not_any_tags filter parameter
    pub fn not_any_tags<I, T>(&mut self, iter: I) -> &mut Self
    where
        I: Iterator<Item = T>,
        T: Into<Cow<'a, str>>,
    {
        self.not_any_tags
            .get_or_insert(None)
            .get_or_insert_with(CommaSeparatedList::new)
            .extend(iter.map(Into::into));
        self
    }

    /// The associated subnet IDs.
    pub fn subnet_ids<I, T>(&mut self, iter: I) -> &mut Self
    where
        I: Iterator<Item = T>,
        T: Into<Cow<'a, str>>,
    {
        self.subnet_ids
            .get_or_insert_with(BTreeSet::new)
            .extend(iter.map(Into::into));
        self
    }

    /// Network Tags.
    pub fn tags<I, T>(&mut self, iter: I) -> &mut Self
    where
        I: Iterator<Item = T>,
        T: Into<Cow<'a, str>>,
    {
        self.tags
            .get_or_insert_with(BTreeSet::new)
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
        format!("networks",).into()
    }

    fn parameters(&self) -> QueryParams {
        let mut params = QueryParams::default();
        params.push_opt("limit", self.limit.as_ref());
        params.push_opt("marker", self.marker.as_ref());
        params.push_opt("tags-any", self.any_tags.as_ref());
        params.push_opt("not-tags", self.not_tags.as_ref());
        params.push_opt("not-tags-any", self.not_any_tags.as_ref());

        params
    }

    fn body(&self) -> Result<Option<(&'static str, Vec<u8>)>, BodyError> {
        let mut params = JsonBodyParams::default();

        params.push_opt("description", self.description.as_ref());
        params.push_opt("name", self.name.as_ref());
        params.push_opt("status", self.status.as_ref());
        params.push_opt("project_id", self.project_id.as_ref());
        params.push_opt("ipv4_address_scope", self.ipv4_address_scope_id.as_ref());
        params.push_opt("ipv6_address_scope", self.ipv6_address_scope_id.as_ref());
        params.push_opt("admin_state_up", self.is_admin_state_up);
        params.push_opt("port_security_enabled", self.is_port_security_enabled);
        params.push_opt("router:external", self.is_router_external);
        params.push_opt("shared", self.is_shared);
        params.push_opt("provider:network_type", self.provider_network_type.as_ref());
        params.push_opt(
            "provider:physical_network",
            self.provider_physical_network.as_ref(),
        );
        params.push_opt(
            "provider:segmentation_id",
            self.provider_segmentation_id.as_ref(),
        );
        params.push("tags", &self.tags);
        params.push_opt(
            "availability_zone_hints",
            self.availability_zone_hints.as_ref(),
        );
        params.push_opt("availability_zones", self.availability_zones.as_ref());
        params.push_opt("created_at", self.created_at.as_ref());
        params.push_opt("dns_domain", self.dns_domain.as_ref());
        params.push_opt("id", self.id.as_ref());
        params.push_opt("is_default", self.is_default);
        params.push_opt("mtu", self.mtu);
        params.push_opt("qos_policy_id", self.qos_policy_id.as_ref());
        params.push_opt("revision_number", self.revision_number);
        params.push_opt("segments", self.segments.as_ref());
        params.push("subnets", &self.subnet_ids);
        params.push_opt("updated_at", self.updated_at.as_ref());
        params.push_opt("vlan_transparent", self.is_vlan_transparent);
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
            .description("description")
            .name("name")
            .status("status")
            .project_id("project_id")
            .ipv4_address_scope_id("ipv4_address_scope")
            .ipv6_address_scope_id("ipv6_address_scope")
            .provider_network_type("provider:network_type")
            .provider_physical_network("provider:physical_network")
            .provider_segmentation_id("provider:segmentation_id")
            .tags(["tags"].iter().cloned())
            .availability_zone_hints("availability_zone_hints")
            .availability_zones("availability_zones")
            .created_at("created_at")
            .dns_domain("dns_domain")
            .id("id")
            .qos_policy_id("qos_policy_id")
            .segments("segments")
            .subnet_ids(["subnets"].iter().cloned())
            .updated_at("updated_at")
            .build()
            .unwrap();

        let (mime, body) = endpoint.body().unwrap().unwrap();
        assert_eq!(
            std::str::from_utf8(&body).unwrap(),
            json!({
                 "description": "description",
                 "name": "name",
                 "status": "status",
                 "project_id": "project_id",
                 "ipv4_address_scope": "ipv4_address_scope",
                 "ipv6_address_scope": "ipv6_address_scope",
                 "provider:network_type": "provider:network_type",
                 "provider:physical_network": "provider:physical_network",
                 "provider:segmentation_id": "provider:segmentation_id",
                 "tags": ["tags"],
                 "availability_zone_hints": "availability_zone_hints",
                 "availability_zones": "availability_zones",
                 "created_at": "created_at",
                 "dns_domain": "dns_domain",
                 "id": "id",
                 "qos_policy_id": "qos_policy_id",
                 "segments": "segments",
                 "subnets": ["subnets"],
                 "updated_at": "updated_at",
            })
            .to_string()
        );
    }
}
