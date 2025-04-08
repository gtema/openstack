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
//! Response type for the get vpn/ipsec-site-connections operation

use crate::common::BoolString;
use crate::common::IntString;
use serde::{Deserialize, Serialize};

/// IpsecSiteConnection response representation
#[derive(Clone, Deserialize, Serialize)]
struct IpsecSiteConnectionResponse {
    /// The administrative state of the resource, which is up (`true`) or down
    /// (`false`).
    ///
    admin_state_up: Option<BoolString>,

    /// The authentication mode. A valid value is `psk`, which is the default.
    ///
    auth_mode: Option<AuthMode>,

    /// A human-readable description for the resource. Default is an empty
    /// string.
    ///
    description: Option<String>,

    /// A dictionary with dead peer detection (DPD) protocol controls.
    ///
    dpd: Option<String>,

    /// The ID of the IPsec site-to-site connection.
    ///
    id: Option<String>,

    /// The ID of the IKE policy.
    ///
    ikepolicy_id: Option<String>,

    /// Indicates whether this VPN can only respond to connections or both
    /// respond to and initiate connections. A valid value is `response- only`
    /// or `bi-directional`. Default is `bi-directional`.
    ///
    initiator: Option<Initiator>,

    /// The ID of the IPsec policy.
    ///
    ipsecpolicy_id: Option<String>,

    /// The ID for the endpoint group that contains private subnets for the
    /// local side of the connection. Yo must specify this parameter with the
    /// `peer_ep_group_id` parameter unless in backward- compatible mode where
    /// `peer_cidrs` is provided with a `subnet_id` for the VPN service.
    ///
    local_ep_group_id: Option<String>,

    /// An ID to be used instead of the external IP address for a virtual
    /// router used in traffic between instances on different networks in
    /// east-west traffic. Most often, local ID would be domain name, email
    /// address, etc. If this is not configured then the external IP address
    /// will be used as the ID.
    ///
    local_id: Option<String>,

    /// The maximum transmission unit (MTU) value to address fragmentation.
    /// Minimum value is 68 for IPv4, and 1280 for IPv6.
    ///
    mtu: Option<IntString>,

    /// Human-readable name of the resource. Default is an empty string.
    ///
    name: Option<String>,

    /// The peer gateway public IPv4 or IPv6 address or FQDN.
    ///
    peer_address: Option<String>,

    /// (Deprecated) Unique list of valid peer private CIDRs in the form \<
    /// net_address > / < prefix > .
    ///
    peer_cidrs: Option<Vec<String>>,

    /// The ID for the endpoint group that contains private CIDRs in the form
    /// \< net_address > / < prefix > for the peer side of the connection. You
    /// must specify this parameter with the `local_ep_group_id` parameter
    /// unless in backward-compatible mode where `peer_cidrs` is provided with
    /// a `subnet_id` for the VPN service.
    ///
    peer_ep_group_id: Option<String>,

    /// The peer router identity for authentication. A valid value is an IPv4
    /// address, IPv6 address, e-mail address, key ID, or FQDN. Typically, this
    /// value matches the `peer_address` value.
    ///
    peer_id: Option<String>,

    /// The pre-shared key. A valid value is any string.
    ///
    psk: Option<String>,

    /// The route mode. A valid value is `static`, which is the default.
    ///
    route_mode: Option<String>,

    /// Indicates whether the IPsec connection is currently operational. Values
    /// are `ACTIVE`, `DOWN`, `BUILD`, `ERROR`, `PENDING_CREATE`,
    /// `PENDING_UPDATE`, or `PENDING_DELETE`.
    ///
    status: Option<String>,

    /// The ID of the project.
    ///
    tenant_id: Option<String>,

    /// The ID of the VPN service.
    ///
    vpnservice_id: Option<String>,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
#[serde(untagged)]
pub enum Initiator {
    // ResponseOnly
    #[serde(rename = "response-only")]
    ResponseOnly,

    // BiDirectional
    #[serde(rename = "bi-directional")]
    BiDirectional,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
#[serde(untagged)]
pub enum AuthMode {
    // Psk
    #[serde(rename = "psk")]
    Psk,
}
