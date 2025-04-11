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
//! Response type for the put vpn/vpnservices/{id} operation

use crate::common::deser_bool_str_opt;
use serde::{Deserialize, Serialize};
use structable_derive::StructTable;

use crate::common::{OutputConfig, StructTable};

/// Vpnservice response representation
#[derive(Clone, Deserialize, Serialize, StructTable)]
pub struct VpnserviceResponse {
    /// The administrative state of the resource, which is up (`true`) or down
    /// (`false`).
    ///
    #[serde(deserialize_with = "deser_bool_str_opt")]
    #[structable(optional)]
    pub admin_state_up: Option<bool>,

    /// A human-readable description for the resource. Default is an empty
    /// string.
    ///
    #[structable(optional)]
    pub description: Option<String>,

    /// Read-only external (public) IPv4 address that is used for the VPN
    /// service. The VPN plugin sets this address if an IPv4 interface is
    /// available.
    ///
    #[structable(optional)]
    pub external_v4_ip: Option<String>,

    /// Read-only external (public) IPv6 address that is used for the VPN
    /// service. The VPN plugin sets this address if an IPv6 interface is
    /// available.
    ///
    #[structable(optional)]
    pub external_v6_ip: Option<String>,

    /// The ID of the flavor.
    ///
    #[structable(optional, serialize)]
    pub flavor_id: Option<String>,

    /// The ID of the VPN service.
    ///
    #[structable(optional)]
    pub id: Option<String>,

    /// Human-readable name of the resource. Default is an empty string.
    ///
    #[structable(optional)]
    pub name: Option<String>,

    #[structable(optional)]
    pub router_id: Option<String>,

    /// Indicates whether IPsec VPN service is currently operational. Values
    /// are `ACTIVE`, `DOWN`, `BUILD`, `ERROR`, `PENDING_CREATE`,
    /// `PENDING_UPDATE`, or `PENDING_DELETE`.
    ///
    #[structable(optional)]
    pub status: Option<String>,

    /// If you specify only a subnet UUID, OpenStack Networking allocates an
    /// available IP from that subnet to the port. If you specify both a subnet
    /// UUID and an IP address, OpenStack Networking tries to allocate the
    /// address to the port.
    ///
    #[structable(optional, serialize)]
    pub subnet_id: Option<String>,

    /// The ID of the project.
    ///
    #[structable(optional)]
    pub tenant_id: Option<String>,
}
