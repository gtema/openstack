//! Create Port
use derive_builder::Builder;
use http::{HeaderMap, HeaderName, HeaderValue};

use crate::api::common::CommaSeparatedList;
use crate::api::rest_endpoint_prelude::*;

use serde_json::Value;
use std::collections::BTreeSet;

/// Query for port.post operation.
#[derive(Debug, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct Port<'a> {
    /// The administrative state of the port, which is up ``True`` or down
    /// ``False``.
    #[builder(default)]
    is_admin_state_up: Option<bool>,

    /// Allowed address pairs list. Dictionary key ``ip_address`` is required
    /// and key ``mac_address`` is optional.
    #[builder(default, private, setter(name = "_allowed_address_pairs"))]
    allowed_address_pairs: Option<BTreeSet<Cow<'a, str>>>,

    /// The ID of the host where the port is allocated. In some cases,
    /// different implementations can run on different hosts.
    #[builder(default, setter(into))]
    binding_host_id: Option<Cow<'a, str>>,

    /// A dictionary the enables the application running on the specified host
    /// to pass and receive vif port-specific information to the plug-in.
    #[builder(default)]
    binding_profile: Option<Value>,

    /// Read-only. A dictionary that enables the application to pass
    /// information about functions that the Networking API provides. To enable
    /// or disable port filtering features such as security group and anti-
    /// MAC/IP spoofing, specify ``port_filter: True`` or ``port_filter:
    /// False``.
    #[builder(default)]
    binding_vif_details: Option<Value>,

    /// Read-only. The vif type for the specified port.
    #[builder(default, setter(into))]
    binding_vif_type: Option<Cow<'a, str>>,

    /// The vnic type that is bound to the neutron port.  In POST and PUT
    /// operations, specify a value of ``normal`` (virtual nic), ``direct``
    /// (pci passthrough), or ``macvtap`` (virtual interface with a tap-like
    /// software interface). These values support SR-IOV PCI passthrough
    /// networking. The ML2 plug-in supports the vnic_type.  In GET operations,
    /// the binding:vnic_type extended attribute is visible to only port owners
    /// and administrative users.
    #[builder(default, setter(into))]
    binding_vnic_type: Option<Cow<'a, str>>,

    /// Underlying data plane status of this port.
    #[builder(default, setter(into))]
    data_plane_status: Option<Cow<'a, str>>,

    /// The port description.
    #[builder(default, setter(into))]
    description: Option<Cow<'a, str>>,

    /// Device ID of this port.
    #[builder(default, setter(into))]
    device_id: Option<Cow<'a, str>>,

    /// Device owner of this port (e.g. ``network:dhcp``).
    #[builder(default, setter(into))]
    device_owner: Option<Cow<'a, str>>,

    /// None
    #[builder(default, setter(into))]
    device_profile: Option<Cow<'a, str>>,

    /// DNS assignment for the port.
    #[builder(default, setter(into))]
    dns_assignment: Option<Cow<'a, str>>,

    /// DNS domain assigned to the port.
    #[builder(default, setter(into))]
    dns_domain: Option<Cow<'a, str>>,

    /// DNS name for the port.
    #[builder(default, setter(into))]
    dns_name: Option<Cow<'a, str>>,

    /// Extra DHCP options.
    #[builder(default, private, setter(name = "_extra_dhcp_opts"))]
    extra_dhcp_opts: Option<BTreeSet<Cow<'a, str>>>,

    /// IP addresses for the port. Includes the IP address and subnet ID.
    #[builder(default, setter(name = "_fixed_ips"), private)]
    fixed_ips: Option<Vec<Value>>,

    /// None
    #[builder(default, setter(into))]
    ip_allocation: Option<Cow<'a, str>>,

    /// The MAC address of an allowed address pair.
    #[builder(default, setter(into))]
    mac_address: Option<Cow<'a, str>>,

    /// The port name.
    #[builder(default, setter(into))]
    name: Option<Cow<'a, str>>,

    /// The ID of the attached network.
    #[builder(default, setter(into))]
    network_id: Option<Cow<'a, str>>,

    /// The NUMA affinity policy defined for this port.
    #[builder(default, setter(into))]
    numa_affinity_policy: Option<Cow<'a, str>>,

    /// The port security status, which is enabled ``True`` or disabled
    /// ``False``.
    #[builder(default)]
    is_port_security_enabled: Option<bool>,

    /// The ID of the project who owns the network. Only administrative users
    /// can specify a project ID other than their own.
    #[builder(default, setter(into))]
    project_id: Option<Cow<'a, str>>,

    /// Whether to propagate uplink status of the port.
    #[builder(default)]
    propagate_uplink_status: Option<bool>,

    /// None
    #[builder(default, setter(into))]
    qos_network_policy_id: Option<Cow<'a, str>>,

    /// The ID of the QoS policy attached to the port.
    #[builder(default, setter(into))]
    qos_policy_id: Option<Cow<'a, str>>,

    /// None
    #[builder(default)]
    resource_request: Option<Value>,

    /// None
    #[builder(default)]
    revision_number: Option<u32>,

    /// The IDs of any attached security groups.
    #[builder(default, private, setter(name = "_security_group_ids"))]
    security_group_ids: Option<BTreeSet<Cow<'a, str>>>,

    /// Port Tags.
    #[builder(default, private, setter(name = "_tags"))]
    tags: Option<BTreeSet<Cow<'a, str>>>,

    /// Tenant_id (deprecated attribute).
    #[builder(default, setter(into))]
    tenant_id: Option<Cow<'a, str>>,

    #[builder(setter(name = "_headers"), default, private)]
    _headers: Option<HeaderMap>,
}

impl<'a> Port<'a> {
    /// Create a builder for the endpoint.
    pub fn builder() -> PortBuilder<'a> {
        PortBuilder::default()
    }
}

impl<'a> PortBuilder<'a> {
    /// Allowed address pairs list. Dictionary key ``ip_address`` is required
    /// and key ``mac_address`` is optional.
    pub fn allowed_address_pairs<I, T>(&mut self, iter: I) -> &mut Self
    where
        I: Iterator<Item = T>,
        T: Into<Cow<'a, str>>,
    {
        self.allowed_address_pairs
            .get_or_insert(None)
            .get_or_insert_with(BTreeSet::new)
            .extend(iter.map(Into::into));
        self
    }

    /// Extra DHCP options.
    pub fn extra_dhcp_opts<I, T>(&mut self, iter: I) -> &mut Self
    where
        I: Iterator<Item = T>,
        T: Into<Cow<'a, str>>,
    {
        self.extra_dhcp_opts
            .get_or_insert(None)
            .get_or_insert_with(BTreeSet::new)
            .extend(iter.map(Into::into));
        self
    }

    /// IP addresses for the port. Includes the IP address and subnet ID.
    pub fn fixed_ips<I, T>(&mut self, iter: I) -> &mut Self
    where
        I: Iterator<Item = T>,
        T: Into<Value>,
    {
        self.fixed_ips
            .get_or_insert(None)
            .get_or_insert_with(Vec::new)
            .extend(iter.map(Into::into));
        self
    }

    /// The IDs of any attached security groups.
    pub fn security_group_ids<I, T>(&mut self, iter: I) -> &mut Self
    where
        I: Iterator<Item = T>,
        T: Into<Cow<'a, str>>,
    {
        self.security_group_ids
            .get_or_insert(None)
            .get_or_insert_with(BTreeSet::new)
            .extend(iter.map(Into::into));
        self
    }

    /// Port Tags.
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

    /// Add a single header to the Port.
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

impl<'a> RestEndpoint for Port<'a> {
    fn method(&self) -> Method {
        Method::POST
    }

    fn endpoint(&self) -> Cow<'static, str> {
        "ports".to_string().into()
    }

    fn parameters(&self) -> QueryParams {
        QueryParams::default()
    }

    fn body(&self) -> Result<Option<(&'static str, Vec<u8>)>, BodyError> {
        let mut params = JsonBodyParams::default();

        params.push_opt("admin_state_up", self.is_admin_state_up);
        params.push_opt("allowed_address_pairs", self.allowed_address_pairs.as_ref());
        params.push_opt("binding:host_id", self.binding_host_id.as_ref());
        params.push_opt("binding:profile", self.binding_profile.as_ref());
        params.push_opt("binding:vif_details", self.binding_vif_details.as_ref());
        params.push_opt("binding:vif_type", self.binding_vif_type.as_ref());
        params.push_opt("binding:vnic_type", self.binding_vnic_type.as_ref());
        params.push_opt("data_plane_status", self.data_plane_status.as_ref());
        params.push_opt("description", self.description.as_ref());
        params.push_opt("device_id", self.device_id.as_ref());
        params.push_opt("device_owner", self.device_owner.as_ref());
        params.push_opt("device_profile", self.device_profile.as_ref());
        params.push_opt("dns_assignment", self.dns_assignment.as_ref());
        params.push_opt("dns_domain", self.dns_domain.as_ref());
        params.push_opt("dns_name", self.dns_name.as_ref());
        params.push_opt("extra_dhcp_opts", self.extra_dhcp_opts.as_ref());
        params.push_opt("fixed_ips", self.fixed_ips.as_ref());
        params.push_opt("ip_allocation", self.ip_allocation.as_ref());
        params.push_opt("mac_address", self.mac_address.as_ref());
        params.push_opt("name", self.name.as_ref());
        params.push_opt("network_id", self.network_id.as_ref());
        params.push_opt("numa_affinity_policy", self.numa_affinity_policy.as_ref());
        params.push_opt("port_security_enabled", self.is_port_security_enabled);
        params.push_opt("project_id", self.project_id.as_ref());
        params.push_opt("propagate_uplink_status", self.propagate_uplink_status);
        params.push_opt("qos_network_policy_id", self.qos_network_policy_id.as_ref());
        params.push_opt("qos_policy_id", self.qos_policy_id.as_ref());
        params.push_opt("resource_request", self.resource_request.as_ref());
        params.push_opt("revision_number", self.revision_number);
        params.push_opt("security_groups", self.security_group_ids.as_ref());
        params.push_opt("tags", self.tags.as_ref());
        params.push_opt("tenant_id", self.tenant_id.as_ref());
        params.into_body_with_root_key("port")
    }

    fn service_type(&self) -> ServiceType {
        ServiceType::Network
    }

    fn response_key(&self) -> Option<Cow<'static, str>> {
        Some("port".into())
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
            Port::builder().build().unwrap().service_type(),
            ServiceType::Network
        );
    }

    #[test]
    fn test_response_key() {
        assert_eq!(
            Port::builder().build().unwrap().response_key().unwrap(),
            "port"
        );
    }

    #[test]
    fn endpoint() {
        let client = MockServerClient::new();
        let mock = client.server.mock(|when, then| {
            when.method(httpmock::Method::POST).path(format!("/ports",));

            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({ "port": {} }));
        });

        let endpoint = Port::builder().build().unwrap();
        let _: serde_json::Value = endpoint.query(&client).unwrap();
        mock.assert();
    }

    #[test]
    fn endpoint_headers() {
        let client = MockServerClient::new();
        let mock = client.server.mock(|when, then| {
            when.method(httpmock::Method::POST)
                .path(format!("/ports",))
                .header("foo", "bar")
                .header("not_foo", "not_bar");
            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({ "port": {} }));
        });

        let endpoint = Port::builder()
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
        let endpoint = Port::builder()
            .allowed_address_pairs(["allowed_address_pairs"].iter().cloned())
            .binding_host_id("binding:host_id")
            .binding_vif_type("binding:vif_type")
            .binding_vnic_type("binding:vnic_type")
            .data_plane_status("data_plane_status")
            .description("description")
            .device_id("device_id")
            .device_owner("device_owner")
            .device_profile("device_profile")
            .dns_assignment("dns_assignment")
            .dns_domain("dns_domain")
            .dns_name("dns_name")
            .extra_dhcp_opts(["extra_dhcp_opts"].iter().cloned())
            .fixed_ips(["fixed_ips"].iter().cloned())
            .ip_allocation("ip_allocation")
            .mac_address("mac_address")
            .name("name")
            .network_id("network_id")
            .numa_affinity_policy("numa_affinity_policy")
            .project_id("project_id")
            .qos_network_policy_id("qos_network_policy_id")
            .qos_policy_id("qos_policy_id")
            .security_group_ids(["security_groups"].iter().cloned())
            .tags(["tags"].iter().cloned())
            .tenant_id("tenant_id")
            .build()
            .unwrap();

        let (mime, body) = endpoint.body().unwrap().unwrap();
        assert_eq!(
            std::str::from_utf8(&body).unwrap(),
            json!({
              "port": {
                 "allowed_address_pairs": ["allowed_address_pairs"],
                 "binding:host_id": "binding:host_id",
                 "binding:vif_type": "binding:vif_type",
                 "binding:vnic_type": "binding:vnic_type",
                 "data_plane_status": "data_plane_status",
                 "description": "description",
                 "device_id": "device_id",
                 "device_owner": "device_owner",
                 "device_profile": "device_profile",
                 "dns_assignment": "dns_assignment",
                 "dns_domain": "dns_domain",
                 "dns_name": "dns_name",
                 "extra_dhcp_opts": ["extra_dhcp_opts"],
                 "fixed_ips": ["fixed_ips"],
                 "ip_allocation": "ip_allocation",
                 "mac_address": "mac_address",
                 "name": "name",
                 "network_id": "network_id",
                 "numa_affinity_policy": "numa_affinity_policy",
                 "project_id": "project_id",
                 "qos_network_policy_id": "qos_network_policy_id",
                 "qos_policy_id": "qos_policy_id",
                 "security_groups": ["security_groups"],
                 "tags": ["tags"],
                 "tenant_id": "tenant_id",
             }
            })
            .to_string()
        );
    }
}
