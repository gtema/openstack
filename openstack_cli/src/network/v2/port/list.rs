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

//! List Ports command
//!
//! Wraps invoking of the `v2.0/ports` with `GET` method

use clap::Args;
use serde::{Deserialize, Serialize};
use tracing::info;

use anyhow::Result;

use openstack_sdk::AsyncOpenStack;

use crate::output::OutputProcessor;
use crate::Cli;
use crate::OpenStackCliError;
use crate::OutputConfig;
use crate::StructTable;

use crate::common::BoolString;
use openstack_sdk::api::network::v2::port::list;
use openstack_sdk::api::QueryAsync;
use serde_json::Value;
use std::collections::HashMap;
use std::fmt;
use structable_derive::StructTable;

/// Lists ports to which the user has access.
///
/// Default policy settings return only those ports that are owned by the
/// project of the user who submits the request, unless the request is
/// submitted by a user with administrative rights.
///
/// Use the `fields` query parameter to control which fields are returned in
/// the response body. Additionally, you can filter results by using query
/// string parameters. For information, see
/// [Filtering and Column Selection](https://wiki.openstack.org/wiki/Neutron/APIv2-specification#Filtering_and_Column_Selection).
///
/// If the `ip-substring-filtering` extension is enabled, the Neutron API
/// supports IP address substring filtering on the `fixed_ips` attribute. If
/// you specify an IP address substring (`ip_address_substr`) in an entry of
/// the `fixed_ips` attribute, the Neutron API will list all ports that have an
/// IP address matching the substring.
///
/// Normal response codes: 200
///
/// Error response codes: 401
///
#[derive(Args)]
#[command(about = "List ports")]
pub struct PortsCommand {
    /// Request Query parameters
    #[command(flatten)]
    query: QueryParameters,

    /// Path parameters
    #[command(flatten)]
    path: PathParameters,
}

/// Query parameters
#[derive(Args)]
struct QueryParameters {
    /// id query parameter for /v2.0/ports API
    ///
    #[arg(long)]
    id: Option<String>,

    /// name query parameter for /v2.0/ports API
    ///
    #[arg(long)]
    name: Option<String>,

    /// network_id query parameter for /v2.0/ports API
    ///
    #[arg(long)]
    network_id: Option<String>,

    /// admin_state_up query parameter for /v2.0/ports API
    ///
    #[arg(long)]
    admin_state_up: Option<bool>,

    /// mac_address query parameter for /v2.0/ports API
    ///
    #[arg(long)]
    mac_address: Option<String>,

    /// fixed_ips query parameter for /v2.0/ports API
    ///
    #[arg(long)]
    fixed_ips: Option<Vec<String>>,

    /// device_id query parameter for /v2.0/ports API
    ///
    #[arg(long)]
    device_id: Option<String>,

    /// device_owner query parameter for /v2.0/ports API
    ///
    #[arg(long)]
    device_owner: Option<String>,

    /// tenant_id query parameter for /v2.0/ports API
    ///
    #[arg(long)]
    tenant_id: Option<String>,

    /// status query parameter for /v2.0/ports API
    ///
    #[arg(long)]
    status: Option<String>,

    /// ip_allocation query parameter for /v2.0/ports API
    ///
    #[arg(long)]
    ip_allocation: Option<String>,

    /// binding:host_id query parameter for /v2.0/ports API
    ///
    #[arg(long)]
    binding_host_id: Option<String>,

    /// revision_number query parameter for /v2.0/ports API
    ///
    #[arg(long)]
    revision_number: Option<String>,

    /// tags query parameter for /v2.0/ports API
    ///
    #[arg(long)]
    tags: Option<Vec<String>>,

    /// tags-any query parameter for /v2.0/ports API
    ///
    #[arg(long)]
    tags_any: Option<Vec<String>>,

    /// not-tags query parameter for /v2.0/ports API
    ///
    #[arg(long)]
    not_tags: Option<Vec<String>>,

    /// not-tags-any query parameter for /v2.0/ports API
    ///
    #[arg(long)]
    not_tags_any: Option<Vec<String>>,

    /// description query parameter for /v2.0/ports API
    ///
    #[arg(long)]
    description: Option<String>,

    /// security_groups query parameter for /v2.0/ports API
    ///
    #[arg(long)]
    security_groups: Option<Vec<String>>,
}

/// Path parameters
#[derive(Args)]
struct PathParameters {}
/// Ports response representation
#[derive(Deserialize, Serialize, Clone, StructTable)]
struct ResponseData {
    /// The ID of the resource.
    ///
    #[serde()]
    #[structable(optional)]
    id: Option<String>,

    /// Human-readable name of the resource.
    ///
    #[serde()]
    #[structable(optional)]
    name: Option<String>,

    /// The ID of the attached network.
    ///
    #[serde()]
    #[structable(optional, wide)]
    network_id: Option<String>,

    /// The administrative state of the resource, which is up (`true`) or down
    /// (`false`).
    ///
    #[serde()]
    #[structable(optional, wide)]
    admin_state_up: Option<BoolString>,

    /// The MAC address of the port. If the port uses the `direct-physical`
    /// `vnic_type` then the value of this field is overwritten with the MAC
    /// address provided in the active binding:profile if any.
    ///
    #[serde()]
    #[structable(optional, wide)]
    mac_address: Option<String>,

    /// The IP addresses for the port. If the port has multiple IP addresses,
    /// this field has multiple entries. Each entry consists of IP address
    /// (`ip_address`) and the subnet ID from which the IP address is assigned
    /// (`subnet_id`).
    ///
    #[serde()]
    #[structable(optional, wide)]
    fixed_ips: Option<Value>,

    /// The ID of the device that uses this port. For example, a server
    /// instance or a logical router.
    ///
    #[serde()]
    #[structable(optional, wide)]
    device_id: Option<String>,

    /// The entity type that uses this port. For example, `compute:nova`
    /// (server instance), `network:dhcp` (DHCP agent) or
    /// `network:router_interface` (router interface).
    ///
    #[serde()]
    #[structable(optional, wide)]
    device_owner: Option<String>,

    /// The ID of the project.
    ///
    #[serde()]
    #[structable(optional, wide)]
    tenant_id: Option<String>,

    /// The port status. Values are `ACTIVE`, `DOWN`, `BUILD` and `ERROR`.
    ///
    #[serde()]
    #[structable(optional)]
    status: Option<String>,

    /// A set of zero or more allowed address pair objects each where address
    /// pair object contains an `ip_address` and `mac_address`. While the
    /// `ip_address` is required, the `mac_address` will be taken from the port
    /// if not specified. The value of `ip_address` can be an IP Address or a
    /// CIDR (if supported by the underlying extension plugin). A server
    /// connected to the port can send a packet with source address which
    /// matches one of the specified allowed address pairs.
    ///
    #[serde()]
    #[structable(optional, wide)]
    allowed_address_pairs: Option<Value>,

    /// Status of the underlying data plane of a port.
    ///
    #[serde()]
    #[structable(optional, wide)]
    data_plane_status: Option<String>,

    /// A set of zero or more extra DHCP option pairs. An option pair consists
    /// of an option value and name.
    ///
    #[serde()]
    #[structable(optional, wide)]
    extra_dhcp_opts: Option<VecHashMapStringValue>,

    /// Indicates when ports use either `deferred`, `immediate` or no IP
    /// allocation (`none`).
    ///
    #[serde()]
    #[structable(optional, wide)]
    ip_allocation: Option<String>,

    #[serde()]
    #[structable(optional, wide)]
    device_profile: Option<String>,

    /// Admin-only. The following values control Open vSwitch’s Userspace Tx
    /// packet steering feature:
    ///
    /// - `{"openvswitch": {"other_config": {"tx-steering": "hash|thread"}}}`
    ///
    #[serde()]
    #[structable(optional, wide)]
    hints: Option<HashMapStringValue>,

    /// The port NUMA affinity policy requested during the virtual machine
    /// scheduling. Values: `None`, `required`, `preferred` or `legacy`.
    ///
    #[serde()]
    #[structable(optional, wide)]
    numa_affinity_policy: Option<String>,

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
    #[serde()]
    #[structable(optional, wide)]
    resource_request: Option<String>,

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
    #[structable(optional, title = "binding:vif_type", wide)]
    binding_vif_type: Option<String>,

    /// A dictionary which contains additional information on the port.
    /// Currently the following fields are defined: `port_filter` and
    /// `ovs_hybrid_plug`. `port_filter` is a boolean indicating the networking
    /// service provides port filtering features such as security group and/or
    /// anti MAC/IP spoofing. `ovs_hybrid_plug` is a boolean used to inform an
    /// API consumer like nova that the hybrid plugging strategy for OVS should
    /// be used.
    ///
    #[serde(rename = "binding:vif_details")]
    #[structable(optional, title = "binding:vif_details", wide)]
    binding_vif_details: Option<HashMapStringValue>,

    /// The type of vNIC which this port should be attached to. This is used to
    /// determine which mechanism driver(s) to be used to bind the port. The
    /// valid values are `normal`, `macvtap`, `direct`, `baremetal`,
    /// `direct-physical`, `virtio-forwarder`, `smart-nic` and
    /// `remote-managed`. What type of vNIC is actually available depends on
    /// deployments.
    ///
    #[serde(rename = "binding:vnic_type")]
    #[structable(optional, title = "binding:vnic_type", wide)]
    binding_vnic_type: Option<String>,

    /// The ID of the host where the port resides.
    ///
    #[serde(rename = "binding:host_id")]
    #[structable(optional, title = "binding:host_id", wide)]
    binding_host_id: Option<String>,

    /// A dictionary that enables the application running on the specific host
    /// to pass and receive vif port information specific to the networking
    /// back-end. The networking API does not define a specific format of this
    /// field. If the update request is null this response field will be {}.
    ///
    #[serde(rename = "binding:profile")]
    #[structable(optional, title = "binding:profile", wide)]
    binding_profile: Option<HashMapStringValue>,

    /// The port security status. A valid value is enabled (`true`) or disabled
    /// (`false`). If port security is enabled for the port, security group
    /// rules and anti-spoofing rules are applied to the traffic on the port.
    /// If disabled, no such rules are applied.
    ///
    #[serde()]
    #[structable(optional, wide)]
    port_security_enabled: Option<BoolString>,

    /// The ID of the QoS policy associated with the port.
    ///
    #[serde()]
    #[structable(optional, wide)]
    qos_policy_id: Option<String>,

    /// The ID of the QoS policy of the network where this port is plugged.
    ///
    #[serde()]
    #[structable(optional, wide)]
    qos_network_policy_id: Option<String>,

    /// The revision number of the resource.
    ///
    #[serde()]
    #[structable(optional, wide)]
    revision_number: Option<i32>,

    /// The list of tags on the resource.
    ///
    #[serde()]
    #[structable(optional, wide)]
    tags: Option<VecString>,

    /// Time at which the resource has been created (in UTC ISO8601 format).
    ///
    #[serde()]
    #[structable(optional)]
    created_at: Option<String>,

    /// Time at which the resource has been updated (in UTC ISO8601 format).
    ///
    #[serde()]
    #[structable(optional)]
    updated_at: Option<String>,

    /// The uplink status propagation of the port. Valid values are enabled
    /// (`true`) and disabled (`false`).
    ///
    #[serde()]
    #[structable(optional, wide)]
    propagate_uplink_status: Option<BoolString>,

    /// A valid DNS name.
    ///
    #[serde()]
    #[structable(optional, wide)]
    dns_name: Option<String>,

    /// Data assigned to a port by the Networking internal DNS including the
    /// `hostname`, `ip_address` and `fqdn`.
    ///
    #[serde()]
    #[structable(optional, wide)]
    dns_assignment: Option<String>,

    /// A valid DNS domain.
    ///
    #[serde()]
    #[structable(optional, wide)]
    dns_domain: Option<String>,

    /// A human-readable description for the resource.
    ///
    #[serde()]
    #[structable(optional, wide)]
    description: Option<String>,

    /// The IDs of security groups applied to the port.
    ///
    #[serde()]
    #[structable(optional, wide)]
    security_groups: Option<VecString>,
}
/// HashMap of `Value` response type
#[derive(Default, Clone, Deserialize, Serialize)]
struct HashMapStringValue(HashMap<String, Value>);
impl fmt::Display for HashMapStringValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{{{}}}",
            self.0
                .iter()
                .map(|v| format!("{}={}", v.0, v.1))
                .collect::<Vec<String>>()
                .join("\n")
        )
    }
}
/// Vector of `HashMapStringValue` response type
#[derive(Default, Clone, Deserialize, Serialize)]
struct VecHashMapStringValue(Vec<HashMapStringValue>);
impl fmt::Display for VecHashMapStringValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[{}]",
            self.0
                .iter()
                .map(|v| v.to_string())
                .collect::<Vec<String>>()
                .join(",")
        )
    }
}
/// Vector of `String` response type
#[derive(Default, Clone, Deserialize, Serialize)]
struct VecString(Vec<String>);
impl fmt::Display for VecString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[{}]",
            self.0
                .iter()
                .map(|v| v.to_string())
                .collect::<Vec<String>>()
                .join(",")
        )
    }
}

impl PortsCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("List Ports");

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;

        let mut ep_builder = list::Request::builder();

        // Set path parameters
        // Set query parameters
        if let Some(val) = &self.query.id {
            ep_builder.id(val.clone());
        }
        if let Some(val) = &self.query.name {
            ep_builder.name(val.clone());
        }
        if let Some(val) = &self.query.network_id {
            ep_builder.network_id(val.clone());
        }
        if let Some(val) = &self.query.admin_state_up {
            ep_builder.admin_state_up(*val);
        }
        if let Some(val) = &self.query.mac_address {
            ep_builder.mac_address(val.clone());
        }
        if let Some(val) = &self.query.fixed_ips {
            ep_builder.fixed_ips(val.iter());
        }
        if let Some(val) = &self.query.device_id {
            ep_builder.device_id(val.clone());
        }
        if let Some(val) = &self.query.device_owner {
            ep_builder.device_owner(val.clone());
        }
        if let Some(val) = &self.query.tenant_id {
            ep_builder.tenant_id(val.clone());
        }
        if let Some(val) = &self.query.status {
            ep_builder.status(val.clone());
        }
        if let Some(val) = &self.query.ip_allocation {
            ep_builder.ip_allocation(val.clone());
        }
        if let Some(val) = &self.query.binding_host_id {
            ep_builder.binding_host_id(val.clone());
        }
        if let Some(val) = &self.query.revision_number {
            ep_builder.revision_number(val.clone());
        }
        if let Some(val) = &self.query.tags {
            ep_builder.tags(val.iter());
        }
        if let Some(val) = &self.query.tags_any {
            ep_builder.tags_any(val.iter());
        }
        if let Some(val) = &self.query.not_tags {
            ep_builder.not_tags(val.iter());
        }
        if let Some(val) = &self.query.not_tags_any {
            ep_builder.not_tags_any(val.iter());
        }
        if let Some(val) = &self.query.description {
            ep_builder.description(val.clone());
        }
        if let Some(val) = &self.query.security_groups {
            ep_builder.security_groups(val.iter());
        }
        // Set body parameters

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;

        let data: Vec<serde_json::Value> = ep.query_async(client).await?;

        op.output_list::<ResponseData>(data)?;
        Ok(())
    }
}
