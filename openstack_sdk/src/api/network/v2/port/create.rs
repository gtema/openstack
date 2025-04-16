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

//! Creates a port on a network.
//!
//! To define the network in which to create the port, specify the `network_id`
//! attribute in the request body.
//!
//! Normal response codes: 201
//!
//! Error response codes: 400, 401, 403, 404
//!
use derive_builder::Builder;
use http::{HeaderMap, HeaderName, HeaderValue};

use crate::api::rest_endpoint_prelude::*;

use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;
use std::borrow::Cow;
use std::collections::BTreeMap;

#[derive(Builder, Debug, Deserialize, Clone, Serialize)]
#[builder(setter(strip_option))]
pub struct FixedIps<'a> {
    /// IP Address
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) ip_address: Option<Cow<'a, str>>,

    /// The subnet ID from which the IP address is assigned
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) subnet_id: Option<Cow<'a, str>>,
}

#[derive(Builder, Debug, Deserialize, Clone, Serialize)]
#[builder(setter(strip_option))]
pub struct AllowedAddressPairs<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) ip_address: Option<Cow<'a, str>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) max_address: Option<Cow<'a, str>>,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub enum NumaAffinityPolicy {
    #[serde(rename = "legacy")]
    Legacy,
    #[serde(rename = "preferred")]
    Preferred,
    #[serde(rename = "required")]
    Required,
    #[serde(rename = "socket")]
    Socket,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub enum BindingVnicType {
    #[serde(rename = "accelerator-direct")]
    AcceleratorDirect,
    #[serde(rename = "accelerator-direct-physical")]
    AcceleratorDirectPhysical,
    #[serde(rename = "baremetal")]
    Baremetal,
    #[serde(rename = "direct")]
    Direct,
    #[serde(rename = "direct-physical")]
    DirectPhysical,
    #[serde(rename = "macvtap")]
    Macvtap,
    #[serde(rename = "normal")]
    Normal,
    #[serde(rename = "remote-managed")]
    RemoteManaged,
    #[serde(rename = "smart-nic")]
    SmartNic,
    #[serde(rename = "vdpa")]
    Vdpa,
    #[serde(rename = "virtio-forwarder")]
    VirtioForwarder,
}

/// A `port` object.
#[derive(Builder, Debug, Deserialize, Clone, Serialize)]
#[builder(setter(strip_option))]
pub struct Port<'a> {
    /// The administrative state of the resource, which is up (`true`) or down
    /// (`false`). Default is `true`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) admin_state_up: Option<bool>,

    /// A set of zero or more allowed address pair objects each where address
    /// pair object contains an `ip_address` and `mac_address`. While the
    /// `ip_address` is required, the `mac_address` will be taken from the port
    /// if not specified. The value of `ip_address` can be an IP Address or a
    /// CIDR (if supported by the underlying extension plugin). A server
    /// connected to the port can send a packet with source address which
    /// matches one of the specified allowed address pairs.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) allowed_address_pairs: Option<Vec<AllowedAddressPairs<'a>>>,

    /// The ID of the host where the port resides. The default is an empty
    /// string.
    #[serde(rename = "binding:host_id", skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) binding_host_id: Option<Cow<'a, str>>,

    /// A dictionary that enables the application running on the specific host
    /// to pass and receive vif port information specific to the networking
    /// back-end. This field is only meant for machine-machine communication
    /// for compute services like Nova, Ironic or Zun to pass information to a
    /// Neutron back-end. It should not be used by multiple services
    /// concurrently or by cloud end users. The existing counterexamples
    /// (`capabilities: [switchdev]` for Open vSwitch hardware offload and
    /// `trusted=true` for Trusted Virtual Functions) are due to be cleaned up.
    /// The networking API does not define a specific format of this field. The
    /// default is an empty dictionary. If you update it with null then it is
    /// treated like {} in the response. Since the port-mac-address-override
    /// extension the `device_mac_address` field of the binding:profile can be
    /// used to provide the MAC address of the physical device a
    /// direct-physical port is being bound to. If provided, then the
    /// `mac_address` field of the port resource will be updated to the MAC
    /// from the active binding.
    #[serde(rename = "binding:profile", skip_serializing_if = "Option::is_none")]
    #[builder(default, private, setter(into, name = "_binding_profile"))]
    pub(crate) binding_profile: Option<Option<BTreeMap<Cow<'a, str>, Value>>>,

    /// The type of vNIC which this port should be attached to. This is used to
    /// determine which mechanism driver(s) to be used to bind the port. The
    /// valid values are `normal`, `macvtap`, `direct`, `baremetal`,
    /// `direct-physical`, `virtio-forwarder`, `smart-nic` and
    /// `remote-managed`. What type of vNIC is actually available depends on
    /// deployments. The default is `normal`.
    #[serde(rename = "binding:vnic_type", skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub(crate) binding_vnic_type: Option<BindingVnicType>,

    /// A human-readable description for the resource. Default is an empty
    /// string.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) description: Option<Cow<'a, str>>,

    /// The ID of the device that uses this port. For example, a server
    /// instance or a logical router.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) device_id: Option<Cow<'a, str>>,

    /// The entity type that uses this port. For example, `compute:nova`
    /// (server instance), `network:dhcp` (DHCP agent) or
    /// `network:router_interface` (router interface).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) device_owner: Option<Cow<'a, str>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) device_profile: Option<Option<Cow<'a, str>>>,

    /// A valid DNS domain.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) dns_domain: Option<Cow<'a, str>>,

    /// A valid DNS name.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) dns_name: Option<Cow<'a, str>>,

    /// A set of zero or more extra DHCP option pairs. An option pair consists
    /// of an option value and name.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) extra_dhcp_opts: Option<Vec<BTreeMap<Cow<'a, str>, Value>>>,

    /// The IP addresses for the port. If you would like to assign multiple IP
    /// addresses for the port, specify multiple entries in this field. Each
    /// entry consists of IP address (`ip_address`) and the subnet ID from
    /// which the IP address is assigned (`subnet_id`).
    ///
    /// - If you specify both a subnet ID and an IP address, OpenStack
    ///   Networking tries to allocate the IP address on that subnet to the
    ///   port.
    /// - If you specify only a subnet ID, OpenStack Networking allocates an
    ///   available IP from that subnet to the port.
    /// - If you specify only an IP address, OpenStack Networking tries to
    ///   allocate the IP address if the address is a valid IP for any of the
    ///   subnets on the specified network.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) fixed_ips: Option<Vec<FixedIps<'a>>>,

    /// Admin-only. A dict, at the top level keyed by mechanism driver aliases
    /// (as defined in setup.cfg). To following values can be used to control
    /// Open vSwitch’s Userspace Tx packet steering feature:
    ///
    /// - `{"openvswitch": {"other_config": {"tx-steering": "hash"}}}`
    /// - `{"openvswitch": {"other_config": {"tx-steering": "thread"}}}`
    ///
    /// If omitted the default is defined by Open vSwitch. The field cannot be
    /// longer than 4095 characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, private, setter(into, name = "_hints"))]
    pub(crate) hints: Option<Option<BTreeMap<Cow<'a, str>, Value>>>,

    /// The MAC address of the port. If unspecified, a MAC address is
    /// automatically generated.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) mac_address: Option<Cow<'a, str>>,

    /// Human-readable name of the resource. Default is an empty string.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) name: Option<Cow<'a, str>>,

    /// The ID of the attached network.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) network_id: Option<Cow<'a, str>>,

    /// The port NUMA affinity policy requested during the virtual machine
    /// scheduling. Values: `None`, `required`, `preferred` or `legacy`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub(crate) numa_affinity_policy: Option<NumaAffinityPolicy>,

    /// The port security status. A valid value is enabled (`true`) or disabled
    /// (`false`). If port security is enabled for the port, security group
    /// rules and anti-spoofing rules are applied to the traffic on the port.
    /// If disabled, no such rules are applied.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) port_security_enabled: Option<bool>,

    /// The uplink status propagation of the port. Valid values are enabled
    /// (`true`) and disabled (`false`).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) propagate_uplink_status: Option<bool>,

    /// QoS policy associated with the port.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) qos_policy_id: Option<Option<Cow<'a, str>>>,

    /// The IDs of security groups applied to the port.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) security_groups: Option<Vec<Cow<'a, str>>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) tags: Option<Vec<Cow<'a, str>>>,

    /// The ID of the project that owns the resource. Only administrative and
    /// users with advsvc role can specify a project ID other than their own.
    /// You cannot change this value through authorization policies.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) tenant_id: Option<Cow<'a, str>>,
}

impl<'a> PortBuilder<'a> {
    /// A dictionary that enables the application running on the specific host
    /// to pass and receive vif port information specific to the networking
    /// back-end. This field is only meant for machine-machine communication
    /// for compute services like Nova, Ironic or Zun to pass information to a
    /// Neutron back-end. It should not be used by multiple services
    /// concurrently or by cloud end users. The existing counterexamples
    /// (`capabilities: [switchdev]` for Open vSwitch hardware offload and
    /// `trusted=true` for Trusted Virtual Functions) are due to be cleaned up.
    /// The networking API does not define a specific format of this field. The
    /// default is an empty dictionary. If you update it with null then it is
    /// treated like {} in the response. Since the port-mac-address-override
    /// extension the `device_mac_address` field of the binding:profile can be
    /// used to provide the MAC address of the physical device a
    /// direct-physical port is being bound to. If provided, then the
    /// `mac_address` field of the port resource will be updated to the MAC
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

    /// Admin-only. A dict, at the top level keyed by mechanism driver aliases
    /// (as defined in setup.cfg). To following values can be used to control
    /// Open vSwitch’s Userspace Tx packet steering feature:
    ///
    /// - `{"openvswitch": {"other_config": {"tx-steering": "hash"}}}`
    /// - `{"openvswitch": {"other_config": {"tx-steering": "thread"}}}`
    ///
    /// If omitted the default is defined by Open vSwitch. The field cannot be
    /// longer than 4095 characters.
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
}

#[derive(Builder, Debug, Clone)]
#[builder(setter(strip_option))]
pub struct Request<'a> {
    /// A `port` object.
    #[builder(setter(into))]
    pub(crate) port: Port<'a>,

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

impl RestEndpoint for Request<'_> {
    fn method(&self) -> http::Method {
        http::Method::POST
    }

    fn endpoint(&self) -> Cow<'static, str> {
        "ports".to_string().into()
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

    /// Returns required API version
    fn api_version(&self) -> Option<ApiVersion> {
        Some(ApiVersion::new(2, 0))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[cfg(feature = "sync")]
    use crate::api::Query;
    use crate::test::client::FakeOpenStackClient;
    use crate::types::ServiceType;
    use http::{HeaderName, HeaderValue};
    use httpmock::MockServer;
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

    #[cfg(feature = "sync")]
    #[test]
    fn endpoint() {
        let server = MockServer::start();
        let client = FakeOpenStackClient::new(server.base_url());
        let mock = server.mock(|when, then| {
            when.method(httpmock::Method::POST)
                .path("/ports".to_string());

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

    #[cfg(feature = "sync")]
    #[test]
    fn endpoint_headers() {
        let server = MockServer::start();
        let client = FakeOpenStackClient::new(server.base_url());
        let mock = server.mock(|when, then| {
            when.method(httpmock::Method::POST)
                .path("/ports".to_string())
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
                .into_iter(),
            )
            .header("not_foo", "not_bar")
            .build()
            .unwrap();
        let _: serde_json::Value = endpoint.query(&client).unwrap();
        mock.assert();
    }
}
