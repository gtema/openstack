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
//! Response type for the post ports operation

use crate::common::deser_bool_str_opt;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use structable::{StructTable, StructTableOptions};

/// Port response representation
#[derive(Clone, Deserialize, Serialize, StructTable)]
pub struct PortResponse {
    /// The administrative state of the resource, which is up (`true`) or down
    /// (`false`).
    ///
    #[serde(deserialize_with = "deser_bool_str_opt")]
    #[structable(optional)]
    pub admin_state_up: Option<bool>,

    /// A set of zero or more allowed address pair objects each where address
    /// pair object contains an `ip_address` and `mac_address`. While the
    /// `ip_address` is required, the `mac_address` will be taken from the port
    /// if not specified. The value of `ip_address` can be an IP Address or a
    /// CIDR (if supported by the underlying extension plugin). A server
    /// connected to the port can send a packet with source address which
    /// matches one of the specified allowed address pairs.
    ///
    #[structable(optional, serialize)]
    pub allowed_address_pairs: Option<Vec<AllowedAddressPairs>>,

    /// The ID of the host where the port resides.
    ///
    #[serde(rename = "binding:host_id")]
    #[structable(optional, title = "binding:host_id")]
    pub binding_host_id: Option<String>,

    /// A dictionary that enables the application running on the specific host
    /// to pass and receive vif port information specific to the networking
    /// back-end. The networking API does not define a specific format of this
    /// field. If the update request is null this response field will be {}.
    ///
    #[serde(rename = "binding:profile")]
    #[structable(optional, serialize, title = "binding:profile")]
    pub binding_profile: Option<HashMap<String, Value>>,

    /// A dictionary which contains additional information on the port.
    /// Currently the following fields are defined: `port_filter` and
    /// `ovs_hybrid_plug`. `port_filter` is a boolean indicating the networking
    /// service provides port filtering features such as security group and/or
    /// anti MAC/IP spoofing. `ovs_hybrid_plug` is a boolean used to inform an
    /// API consumer like nova that the hybrid plugging strategy for OVS should
    /// be used.
    ///
    #[serde(rename = "binding:vif_details")]
    #[structable(optional, serialize, title = "binding:vif_details")]
    pub binding_vif_details: Option<HashMap<String, Value>>,

    /// The type of which mechanism is used for the port. An API consumer like
    /// nova can use this to determine an appropriate way to attach a device
    /// (for example an interface of a virtual server) to the port. Available
    /// values currently defined includes `ovs`, `bridge`, `macvtap`, `hw_veb`,
    /// `hostdev_physical`, `vhostuser`, `distributed` and `other`. There are
    /// also special values: `unbound` and `binding_failed`. `unbound` means
    /// the port is not bound to a networking back-end. `binding_failed` means
    /// an error that the port failed to be bound to a networking back-end.
    ///
    #[serde(rename = "binding:vif_type")]
    #[structable(optional, title = "binding:vif_type")]
    pub binding_vif_type: Option<String>,

    /// The type of vNIC which this port should be attached to. This is used to
    /// determine which mechanism driver(s) to be used to bind the port. The
    /// valid values are `normal`, `macvtap`, `direct`, `baremetal`,
    /// `direct-physical`, `virtio-forwarder`, `smart-nic` and
    /// `remote-managed`. What type of vNIC is actually available depends on
    /// deployments.
    ///
    #[serde(rename = "binding:vnic_type")]
    #[structable(optional, serialize, title = "binding:vnic_type")]
    pub binding_vnic_type: Option<BindingVnicType>,

    /// Time at which the resource has been created (in UTC ISO8601 format).
    ///
    #[structable(optional)]
    pub created_at: Option<String>,

    /// Status of the underlying data plane of a port.
    ///
    #[structable(optional, serialize)]
    pub data_plane_status: Option<DataPlaneStatus>,

    /// A human-readable description for the resource.
    ///
    #[structable(optional)]
    pub description: Option<String>,

    /// The ID of the device that uses this port. For example, a server
    /// instance or a logical router.
    ///
    #[structable(optional)]
    pub device_id: Option<String>,

    /// The entity type that uses this port. For example, `compute:nova`
    /// (server instance), `network:dhcp` (DHCP agent) or
    /// `network:router_interface` (router interface).
    ///
    #[structable(optional)]
    pub device_owner: Option<String>,

    #[structable(optional, serialize)]
    pub device_profile: Option<String>,

    /// Data assigned to a port by the Networking internal DNS including the
    /// `hostname`, `ip_address` and `fqdn`.
    ///
    #[structable(optional, serialize)]
    pub dns_assignment: Option<Vec<DnsAssignment>>,

    /// A valid DNS domain.
    ///
    #[structable(optional)]
    pub dns_domain: Option<String>,

    /// A valid DNS name.
    ///
    #[structable(optional)]
    pub dns_name: Option<String>,

    /// A set of zero or more extra DHCP option pairs. An option pair consists
    /// of an option value and name.
    ///
    #[structable(optional, serialize)]
    pub extra_dhcp_opts: Option<Vec<HashMap<String, Value>>>,

    /// The IP addresses for the port. If the port has multiple IP addresses,
    /// this field has multiple entries. Each entry consists of IP address
    /// (`ip_address`) and the subnet ID from which the IP address is assigned
    /// (`subnet_id`).
    ///
    #[structable(optional, serialize)]
    pub fixed_ips: Option<Vec<FixedIps>>,

    /// Admin-only. The following values control Open vSwitch’s Userspace Tx
    /// packet steering feature:
    ///
    /// - `{"openvswitch": {"other_config": {"tx-steering": "hash|thread"}}}`
    ///
    #[structable(optional, serialize)]
    pub hints: Option<HashMap<String, Value>>,

    /// The ID of the resource.
    ///
    #[structable(optional)]
    pub id: Option<String>,

    /// Indicates when ports use either `deferred`, `immediate` or no IP
    /// allocation (`none`).
    ///
    #[structable(optional)]
    pub ip_allocation: Option<String>,

    /// The MAC address of the port. If the port uses the `direct-physical`
    /// `vnic_type` then the value of this field is overwritten with the MAC
    /// address provided in the active binding:profile if any.
    ///
    #[structable(optional)]
    pub mac_address: Option<String>,

    /// Human-readable name of the resource.
    ///
    #[structable(optional)]
    pub name: Option<String>,

    /// The ID of the attached network.
    ///
    #[structable(optional)]
    pub network_id: Option<String>,

    /// The port NUMA affinity policy requested during the virtual machine
    /// scheduling. Values: `None`, `required`, `preferred` or `legacy`.
    ///
    #[structable(optional, serialize)]
    pub numa_affinity_policy: Option<NumaAffinityPolicy>,

    /// The port security status. A valid value is enabled (`true`) or disabled
    /// (`false`). If port security is enabled for the port, security group
    /// rules and anti-spoofing rules are applied to the traffic on the port.
    /// If disabled, no such rules are applied.
    ///
    #[serde(deserialize_with = "deser_bool_str_opt")]
    #[structable(optional)]
    pub port_security_enabled: Option<bool>,

    /// The uplink status propagation of the port. Valid values are enabled
    /// (`true`) and disabled (`false`).
    ///
    #[serde(deserialize_with = "deser_bool_str_opt")]
    #[structable(optional)]
    pub propagate_uplink_status: Option<bool>,

    /// The ID of the QoS policy of the network where this port is plugged.
    ///
    #[structable(optional, serialize)]
    pub qos_network_policy_id: Option<String>,

    /// The ID of the QoS policy associated with the port.
    ///
    #[structable(optional, serialize)]
    pub qos_policy_id: Option<String>,

    /// Expose Placement resources (i.e.: `minimum-bandwidth`) and traits
    /// (i.e.: `vnic-type`, `physnet`) requested by a port to Nova and
    /// Placement. A `resource_request` object contains `request_groups` and
    /// `same_subtree` keys. `request_groups` is a list of dicts, where each
    /// dict represents one group of resources and traits that needs to be
    /// fulfilled from a single resource provider. Every dict in the list must
    /// contain `id`, `required` and `resources` keys. The `id` field is a
    /// string which represents a unique UUID that is generated for each group
    /// by combining the `port_id` and UUIDs of the QoS rules contributing to
    /// the group via the UUID5 method. `required` key contains the traits
    /// (generated from the `vnic_type` and the `physnet`) required by the
    /// port, and a `resources` key contains a mapping of requested resource
    /// class name and requested amount from the QoS policy. `same_subtree` key
    /// contains a list of `id` values from every resource group.
    ///
    #[structable(optional)]
    pub resource_request: Option<String>,

    /// The revision number of the resource.
    ///
    #[structable(optional)]
    pub revision_number: Option<i32>,

    /// The IDs of security groups applied to the port.
    ///
    #[structable(optional, serialize)]
    pub security_groups: Option<Vec<String>>,

    /// The port status. Values are `ACTIVE`, `DOWN`, `BUILD` and `ERROR`.
    ///
    #[structable(optional)]
    pub status: Option<String>,

    /// The list of tags on the resource.
    ///
    #[structable(optional, serialize)]
    pub tags: Option<Vec<String>>,

    /// The ID of the project.
    ///
    #[structable(optional)]
    pub tenant_id: Option<String>,

    /// Time at which the resource has been updated (in UTC ISO8601 format).
    ///
    #[structable(optional)]
    pub updated_at: Option<String>,
}

/// `FixedIps` type
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct FixedIps {
    pub ip_address: Option<String>,
    pub subnet_id: Option<String>,
}

/// `AllowedAddressPairs` type
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AllowedAddressPairs {
    pub ip_address: Option<String>,
    pub max_address: Option<String>,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
#[serde(untagged)]
pub enum DataPlaneStatus {
    // Active
    #[serde(rename = "ACTIVE")]
    Active,

    // Down
    #[serde(rename = "DOWN")]
    Down,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
#[serde(untagged)]
pub enum NumaAffinityPolicy {
    // Legacy
    #[serde(rename = "legacy")]
    Legacy,

    // Preferred
    #[serde(rename = "preferred")]
    Preferred,

    // Required
    #[serde(rename = "required")]
    Required,

    // Socket
    #[serde(rename = "socket")]
    Socket,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
#[serde(untagged)]
pub enum BindingVnicType {
    // AcceleratorDirect
    #[serde(rename = "accelerator-direct")]
    AcceleratorDirect,

    // AcceleratorDirectPhysical
    #[serde(rename = "accelerator-direct-physical")]
    AcceleratorDirectPhysical,

    // Baremetal
    #[serde(rename = "baremetal")]
    Baremetal,

    // Direct
    #[serde(rename = "direct")]
    Direct,

    // DirectPhysical
    #[serde(rename = "direct-physical")]
    DirectPhysical,

    // Macvtap
    #[serde(rename = "macvtap")]
    Macvtap,

    // Normal
    #[serde(rename = "normal")]
    Normal,

    // RemoteManaged
    #[serde(rename = "remote-managed")]
    RemoteManaged,

    // SmartNic
    #[serde(rename = "smart-nic")]
    SmartNic,

    // Vdpa
    #[serde(rename = "vdpa")]
    Vdpa,

    // VirtioForwarder
    #[serde(rename = "virtio-forwarder")]
    VirtioForwarder,
}

/// `DnsAssignment` type
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DnsAssignment {
    pub fqdn: Option<String>,
    pub hostname: Option<String>,
    pub ip_address: Option<String>,
}
