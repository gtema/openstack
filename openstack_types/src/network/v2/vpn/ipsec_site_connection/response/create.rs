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
//! Response type for the POST `vpn/ipsec-site-connections` operation

use serde::{Deserialize, Serialize};
use structable::{StructTable, StructTableOptions};

/// IpsecSiteConnection response representation
#[derive(Clone, Deserialize, Serialize, StructTable)]
pub struct IpsecSiteConnectionResponse {
    /// The administrative state of the resource, which is up (`true`) or down
    /// (`false`).
    #[serde(default, deserialize_with = "crate::common::deser_bool_str_opt")]
    #[structable(optional)]
    pub admin_state_up: Option<bool>,

    /// The authentication mode. A valid value is `psk`, which is the default.
    #[serde(default)]
    #[structable(optional, serialize)]
    pub auth_mode: Option<AuthMode>,

    /// A human-readable description for the resource. Default is an empty
    /// string.
    #[serde(default)]
    #[structable(optional)]
    pub description: Option<String>,

    /// A dictionary with dead peer detection (DPD) protocol controls.
    #[serde(default)]
    #[structable(optional)]
    pub dpd: Option<String>,

    /// The ID of the IPsec site-to-site connection.
    #[serde(default)]
    #[structable(optional)]
    pub id: Option<String>,

    /// The ID of the IKE policy.
    #[serde(default)]
    #[structable(optional)]
    pub ikepolicy_id: Option<String>,

    /// Indicates whether this VPN can only respond to connections or both
    /// respond to and initiate connections. A valid value is `response- only`
    /// or `bi-directional`. Default is `bi-directional`.
    #[serde(default)]
    #[structable(optional, serialize)]
    pub initiator: Option<Initiator>,

    /// The ID of the IPsec policy.
    #[serde(default)]
    #[structable(optional)]
    pub ipsecpolicy_id: Option<String>,

    /// The ID for the endpoint group that contains private subnets for the
    /// local side of the connection. Yo must specify this parameter with the
    /// `peer_ep_group_id` parameter unless in backward- compatible mode where
    /// `peer_cidrs` is provided with a `subnet_id` for the VPN service.
    #[serde(default)]
    #[structable(optional)]
    pub local_ep_group_id: Option<String>,

    /// An ID to be used instead of the external IP address for a virtual
    /// router used in traffic between instances on different networks in
    /// east-west traffic. Most often, local ID would be domain name, email
    /// address, etc. If this is not configured then the external IP address
    /// will be used as the ID.
    #[serde(default)]
    #[structable(optional)]
    pub local_id: Option<String>,

    /// The maximum transmission unit (MTU) value to address fragmentation.
    /// Minimum value is 68 for IPv4, and 1280 for IPv6.
    #[serde(default, deserialize_with = "crate::common::deser_num_str_opt")]
    #[structable(optional)]
    pub mtu: Option<i64>,

    /// Human-readable name of the resource. Default is an empty string.
    #[serde(default)]
    #[structable(optional)]
    pub name: Option<String>,

    /// The peer gateway public IPv4 or IPv6 address or FQDN.
    #[serde(default)]
    #[structable(optional)]
    pub peer_address: Option<String>,

    /// (Deprecated) Unique list of valid peer private CIDRs in the form \<
    /// net_address > / < prefix > .
    #[serde(default)]
    #[structable(optional, serialize)]
    pub peer_cidrs: Option<Vec<String>>,

    /// The ID for the endpoint group that contains private CIDRs in the form
    /// \< net_address > / < prefix > for the peer side of the connection. You
    /// must specify this parameter with the `local_ep_group_id` parameter
    /// unless in backward-compatible mode where `peer_cidrs` is provided with
    /// a `subnet_id` for the VPN service.
    #[serde(default)]
    #[structable(optional)]
    pub peer_ep_group_id: Option<String>,

    /// The peer router identity for authentication. A valid value is an IPv4
    /// address, IPv6 address, e-mail address, key ID, or FQDN. Typically, this
    /// value matches the `peer_address` value.
    #[serde(default)]
    #[structable(optional)]
    pub peer_id: Option<String>,

    /// The pre-shared key. A valid value is any string.
    #[serde(default)]
    #[structable(optional)]
    pub psk: Option<String>,

    /// The route mode. A valid value is `static`, which is the default.
    #[serde(default)]
    #[structable(optional)]
    pub route_mode: Option<String>,

    /// Indicates whether the IPsec connection is currently operational. Values
    /// are `ACTIVE`, `DOWN`, `BUILD`, `ERROR`, `PENDING_CREATE`,
    /// `PENDING_UPDATE`, or `PENDING_DELETE`.
    #[serde(default)]
    #[structable(optional)]
    pub status: Option<String>,

    /// The ID of the project.
    #[serde(default)]
    #[structable(optional)]
    pub tenant_id: Option<String>,

    /// The ID of the VPN service.
    #[serde(default)]
    #[structable(optional)]
    pub vpnservice_id: Option<String>,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub enum AuthMode {
    // Psk
    #[serde(rename = "psk")]
    Psk,
}

impl std::str::FromStr for AuthMode {
    type Err = ();
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "psk" => Ok(Self::Psk),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub enum Initiator {
    // BiDirectional
    #[serde(rename = "bi-directional")]
    BiDirectional,

    // ResponseOnly
    #[serde(rename = "response-only")]
    ResponseOnly,
}

impl std::str::FromStr for Initiator {
    type Err = ();
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "bi-directional" => Ok(Self::BiDirectional),
            "response-only" => Ok(Self::ResponseOnly),
            _ => Err(()),
        }
    }
}
