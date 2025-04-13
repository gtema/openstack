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
//! Response type for the get floatingips/{id} operation

use serde::{Deserialize, Serialize};
use structable::{StructTable, StructTableOptions};

/// Floatingip response representation
#[derive(Clone, Deserialize, Serialize, StructTable)]
pub struct FloatingipResponse {
    /// Time at which the resource has been created (in UTC ISO8601 format).
    ///
    #[structable(optional)]
    pub created_at: Option<String>,

    #[structable(optional)]
    pub description: Option<String>,

    /// A valid DNS domain.
    ///
    #[structable(optional)]
    pub dns_domain: Option<String>,

    /// A valid DNS name.
    ///
    #[structable(optional)]
    pub dns_name: Option<String>,

    /// The fixed IP address that is associated with the floating IP address.
    ///
    #[structable(optional)]
    pub fixed_ip_address: Option<String>,

    /// The floating IP address.
    ///
    #[structable(optional)]
    pub floating_ip_address: Option<String>,

    /// The ID of the network associated with the floating IP.
    ///
    #[structable(optional)]
    pub floating_network_id: Option<String>,

    /// The ID of the floating IP address.
    ///
    #[structable(optional)]
    pub id: Option<String>,

    /// The information of the port that this floating IP associates with. In
    /// particular, if the floating IP is associated with a port, this field
    /// contains some attributes of the associated port, including `name`,
    /// `network_id`, `mac_address`, `admin_state_up`, `status`, `device_id`
    /// and `device_owner`. If the floating IP is not associated with a port,
    /// this field is `null`.
    ///
    #[structable(optional, serialize)]
    pub port_details: Option<Vec<PortDetails>>,

    /// The associated port forwarding resources for the floating IP. If the
    /// floating IP has multiple port forwarding resources, this field has
    /// multiple entries. Each entry consists of network IP protocol
    /// (`protocol`), the fixed IP address of internal neutron port
    /// (`internal_ip_address`), the TCP or UDP port or port range used by
    /// internal neutron port (`internal_port`) or (`internal_port_range`) and
    /// the TCP or UDP port or port range used by floating IP (`external_port`)
    /// or (`external_port_range`).
    ///
    #[structable(optional, serialize)]
    pub port_forwardings: Option<Vec<PortForwardings>>,

    /// The ID of a port associated with the floating IP.
    ///
    #[structable(optional, serialize)]
    pub port_id: Option<String>,

    /// The ID of the QoS policy associated with the floating IP.
    ///
    #[structable(optional, serialize)]
    pub qos_policy_id: Option<String>,

    /// The revision number of the resource.
    ///
    #[structable(optional)]
    pub revision_number: Option<i32>,

    /// The ID of the router for the floating IP.
    ///
    #[structable(optional, serialize)]
    pub router_id: Option<String>,

    /// The status of the floating IP. Values are `ACTIVE`, `DOWN` and `ERROR`.
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

/// `PortDetails` type
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PortDetails {
    pub admin_state_up: Option<bool>,
    pub device_id: Option<String>,
    pub device_owner: Option<String>,
    pub mac_address: Option<String>,
    pub name: Option<String>,
    pub network_id: Option<String>,
    pub status: Option<String>,
}

/// `PortForwardings` type
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PortForwardings {
    pub description: Option<String>,
    pub external_port: Option<f32>,
    pub external_port_range: Option<f32>,
    pub id: Option<String>,
    pub internal_ip_address: Option<String>,
    pub internal_port: Option<f32>,
    pub internal_port_id: Option<String>,
    pub internal_port_range: Option<f32>,
    pub protocol: Option<String>,
}
