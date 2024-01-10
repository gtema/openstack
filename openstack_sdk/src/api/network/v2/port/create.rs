//! Creates multiple ports in a single request. Specify a list of ports in the
//! request body.
//!
//! Guarantees the atomic completion of the bulk operation.
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
use serde_json::Value;
use std::borrow::Cow;
use std::collections::BTreeMap;

#[derive(Builder, Debug, Deserialize, Clone, Serialize)]
#[builder(setter(strip_option))]
pub struct FixedIps<'a> {
    /// IP Address
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    ip_address: Option<Cow<'a, str>>,

    /// The subnet ID from which the IP address is assigned
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    subnet_id: Option<Cow<'a, str>>,
}

#[derive(Builder, Debug, Deserialize, Clone, Serialize)]
#[builder(setter(strip_option))]
pub struct AllowedAddressPairs<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    ip_address: Option<Cow<'a, str>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    max_address: Option<Cow<'a, str>>,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
#[serde(untagged)]
pub enum NumaAffinityPolicy {
    #[serde(alias = "preferred")]
    Preferred,
    #[serde(alias = "required")]
    Required,
    #[serde(alias = "legacy")]
    Legacy,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
#[serde(untagged)]
pub enum BindingVnicType {
    #[serde(alias = "vdpa")]
    Vdpa,
    #[serde(alias = "normal")]
    Normal,
    #[serde(alias = "baremetal")]
    Baremetal,
    #[serde(alias = "accelerator-direct-physical")]
    AcceleratorDirectPhysical,
    #[serde(alias = "direct-physical")]
    DirectPhysical,
    #[serde(alias = "direct")]
    Direct,
    #[serde(alias = "virtio-forwarder")]
    VirtioForwarder,
    #[serde(alias = "accelerator-direct")]
    AcceleratorDirect,
    #[serde(alias = "remote-managed")]
    RemoteManaged,
    #[serde(alias = "smart-nic")]
    SmartNic,
    #[serde(alias = "macvtap")]
    Macvtap,
}

/// A `port` object.
#[derive(Builder, Debug, Deserialize, Clone, Serialize)]
#[builder(setter(strip_option))]
pub struct Port<'a> {
    /// Human-readable name of the resource.
    /// Default is an empty string.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    name: Option<Cow<'a, str>>,

    /// The ID of the attached network.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    network_id: Option<Cow<'a, str>>,

    /// The administrative state of the resource, which is
    /// up (`true`) or down (`false`).
    /// Default is `true`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    admin_state_up: Option<bool>,

    /// The MAC address of the port.
    /// If unspecified, a MAC address is automatically generated.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    mac_address: Option<Cow<'a, str>>,

    /// The IP addresses for the port.
    /// If you would like to assign multiple IP addresses for the port,
    /// specify multiple entries in this field.
    /// Each entry consists of IP address (`ip\_address`) and the subnet ID
    /// from which the IP address is assigned (`subnet\_id`).
    ///
    ///
    /// * If you specify both a subnet ID and an IP address, OpenStack
    /// Networking
    /// tries to allocate the IP address on that subnet to the port.
    /// * If you specify only a subnet ID, OpenStack Networking allocates
    /// an available IP from that subnet to the port.
    /// * If you specify only an IP address, OpenStack Networking
    /// tries to allocate the IP address if the address is a valid IP
    /// for any of the subnets on the specified network.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    fixed_ips: Option<Vec<FixedIps<'a>>>,

    /// The ID of the device that uses this port.
    /// For example, a server instance or a logical router.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    device_id: Option<Cow<'a, str>>,

    /// The entity type that uses this port.
    /// For example, `compute:nova` (server instance), `network:dhcp`
    /// (DHCP agent) or `network:router\_interface` (router interface).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    device_owner: Option<Cow<'a, str>>,

    /// The ID of the project that owns the resource.
    /// Only administrative and users with advsvc role can specify
    /// a project ID other than their own.
    /// You cannot change this value through authorization policies.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    tenant_id: Option<Cow<'a, str>>,

    /// A set of zero or more allowed address pair objects each where address
    /// pair
    /// object contains an `ip\_address` and `mac\_address`. While the
    /// `ip\_address` is required, the `mac\_address` will be taken from the
    /// port if not specified. The value of `ip\_address` can be an IP Address
    /// or a CIDR (if supported by the underlying extension plugin).
    /// A server connected to the port can send a packet with source address
    /// which
    /// matches one of the specified allowed address pairs.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    allowed_address_pairs: Option<Vec<AllowedAddressPairs<'a>>>,

    /// A set of zero or more extra DHCP option pairs. An
    /// option pair consists of an option value and name.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    extra_dhcp_opts: Option<Vec<BTreeMap<Cow<'a, str>, Value>>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    device_profile: Option<Option<Cow<'a, str>>>,

    /// Admin-only. A dict, at the top level keyed by mechanism driver
    /// aliases (as defined in setup.cfg). To following values can be used to
    /// control Open vSwitch’s Userspace Tx packet steering feature:
    ///
    ///
    /// * `{"openvswitch": {"other\_config": {"tx-steering": "hash"}}}`
    /// * `{"openvswitch": {"other\_config": {"tx-steering": "thread"}}}`
    ///
    ///
    /// If omitted the default is defined by Open vSwitch.
    /// The field cannot be longer than 4095 characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, private, setter(name = "_hints"))]
    hints: Option<Option<BTreeMap<Cow<'a, str>, Value>>>,

    /// The port NUMA affinity policy requested during the virtual machine
    /// scheduling. Values: `None`, `required`, `preferred` or `legacy`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    numa_affinity_policy: Option<NumaAffinityPolicy>,

    /// The type of vNIC which this port should be attached to. This is used to
    /// determine which mechanism driver(s) to be used to bind the port.
    /// The valid values are `normal`, `macvtap`, `direct`, `baremetal`,
    /// `direct-physical`, `virtio-forwarder`, `smart-nic`
    /// and `remote-managed`.
    /// What type of vNIC is actually available depends on deployments.
    /// The default is `normal`.
    #[serde(rename = "binding:vnic_type", skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    binding_vnic_type: Option<BindingVnicType>,

    /// The ID of the host where the port resides.
    /// The default is an empty string.
    #[serde(rename = "binding:host_id", skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    binding_host_id: Option<Cow<'a, str>>,

    /// A dictionary that enables the application running on the specific host
    /// to
    /// pass and receive vif port information specific to the networking back-
    /// end.
    /// This field is only meant for machine-machine communication for compute
    /// services like Nova, Ironic or Zun to pass information to a Neutron
    /// back-end. It should not be used by multiple services concurrently or by
    /// cloud end users. The existing counterexamples
    /// (`capabilities: [switchdev]` for Open vSwitch hardware offload and
    /// `trusted=true` for Trusted Virtual Functions) are due to be cleaned up.
    /// The networking API does not define a specific format of this field.
    /// The default is an empty dictionary.
    /// If you update it with null then it is treated like {} in the response.
    /// Since the port-mac-address-override extension the
    /// `device\_mac\_address`
    /// field of the binding:profile can be used to provide the MAC address of
    /// the
    /// physical device a direct-physical port is being bound to. If provided,
    /// then
    /// the `mac\_address` field of the port resource will be updated to the
    /// MAC
    /// from the active binding.
    #[serde(rename = "binding:profile", skip_serializing_if = "Option::is_none")]
    #[builder(default, private, setter(name = "_binding_profile"))]
    binding_profile: Option<Option<BTreeMap<Cow<'a, str>, Value>>>,

    /// The port security status. A valid value is
    /// enabled (`true`) or disabled (`false`).
    /// If port security is enabled for the port,
    /// security group rules and anti-spoofing rules are applied to
    /// the traffic on the port. If disabled, no such rules are applied.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    port_security_enabled: Option<bool>,

    /// QoS policy associated with the port.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    qos_policy_id: Option<Option<Cow<'a, str>>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    tags: Option<Vec<Cow<'a, str>>>,

    /// The uplink status propagation of the port. Valid values are
    /// enabled (`true`) and disabled (`false`).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    propagate_uplink_status: Option<bool>,

    /// A valid DNS name.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    dns_name: Option<Cow<'a, str>>,

    /// A valid DNS domain.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    dns_domain: Option<Cow<'a, str>>,

    /// A human-readable description for the resource.
    /// Default is an empty string.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    description: Option<Cow<'a, str>>,

    /// The IDs of security groups applied to the port.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    security_groups: Option<Vec<Cow<'a, str>>>,
}

impl<'a> PortBuilder<'a> {
    /// Admin-only. A dict, at the top level keyed by mechanism driver
    /// aliases (as defined in setup.cfg). To following values can be used to
    /// control Open vSwitch’s Userspace Tx packet steering feature:
    ///
    ///
    /// * `{"openvswitch": {"other\_config": {"tx-steering": "hash"}}}`
    /// * `{"openvswitch": {"other\_config": {"tx-steering": "thread"}}}`
    ///
    ///
    /// If omitted the default is defined by Open vSwitch.
    /// The field cannot be longer than 4095 characters.
    pub fn hints<I, K, V>(&mut self, iter: I) -> &mut Self
    where
        I: Iterator<Item = (K, V)>,
        K: Into<Cow<'a, str>>,
        V: Into<Value>,
    {
        self.hints
            .get_or_insert(None)
            .get_or_insert(None)
            .get_or_insert_with(BTreeMap::new)
            .extend(iter.map(|(k, v)| (k.into(), v.into())));
        self
    }

    /// A dictionary that enables the application running on the specific host
    /// to
    /// pass and receive vif port information specific to the networking back-
    /// end.
    /// This field is only meant for machine-machine communication for compute
    /// services like Nova, Ironic or Zun to pass information to a Neutron
    /// back-end. It should not be used by multiple services concurrently or by
    /// cloud end users. The existing counterexamples
    /// (`capabilities: [switchdev]` for Open vSwitch hardware offload and
    /// `trusted=true` for Trusted Virtual Functions) are due to be cleaned up.
    /// The networking API does not define a specific format of this field.
    /// The default is an empty dictionary.
    /// If you update it with null then it is treated like {} in the response.
    /// Since the port-mac-address-override extension the
    /// `device\_mac\_address`
    /// field of the binding:profile can be used to provide the MAC address of
    /// the
    /// physical device a direct-physical port is being bound to. If provided,
    /// then
    /// the `mac\_address` field of the port resource will be updated to the
    /// MAC
    /// from the active binding.
    pub fn binding_profile<I, K, V>(&mut self, iter: I) -> &mut Self
    where
        I: Iterator<Item = (K, V)>,
        K: Into<Cow<'a, str>>,
        V: Into<Value>,
    {
        self.binding_profile
            .get_or_insert(None)
            .get_or_insert(None)
            .get_or_insert_with(BTreeMap::new)
            .extend(iter.map(|(k, v)| (k.into(), v.into())));
        self
    }
}
#[derive(Builder, Debug, Clone)]
#[builder(setter(strip_option))]
pub struct Request<'a> {
    /// A `port` object.
    #[builder(setter(into))]
    port: Port<'a>,

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

impl<'a> RestEndpoint for Request<'a> {
    fn method(&self) -> Method {
        Method::POST
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("v2.0/ports",).into()
    }

    fn parameters(&self) -> QueryParams {
        QueryParams::default()
    }

    fn body(&self) -> Result<Option<(&'static str, Vec<u8>)>, BodyError> {
        let mut params = JsonBodyParams::default();

        params.push("port", serde_json::to_value(&self.port)?);

        params.into_body()
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
    use serde::Serialize;
    use serde_json::json;

    #[test]
    fn test_service_type() {
        assert_eq!(
            Request::builder()
                .port(PortBuilder::default().build().unwrap())
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
                .port(PortBuilder::default().build().unwrap())
                .build()
                .unwrap()
                .response_key()
                .unwrap(),
            "port"
        );
    }

    #[test]
    fn endpoint() {
        let client = MockServerClient::new();
        let mock = client.server.mock(|when, then| {
            when.method(httpmock::Method::POST)
                .path(format!("/v2.0/ports",));

            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({ "port": {} }));
        });

        let endpoint = Request::builder()
            .port(PortBuilder::default().build().unwrap())
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
                .path(format!("/v2.0/ports",))
                .header("foo", "bar")
                .header("not_foo", "not_bar");
            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({ "port": {} }));
        });

        let endpoint = Request::builder()
            .port(PortBuilder::default().build().unwrap())
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
