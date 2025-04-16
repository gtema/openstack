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
//! Response type for the GET `octavia/amphorae/{amphora_id}` operation

use serde::{Deserialize, Serialize};
use structable::{StructTable, StructTableOptions};

/// Amphorae response representation
#[derive(Clone, Deserialize, Serialize, StructTable)]
pub struct AmphoraeResponse {
    /// The availability zone of a compute instance, cached at create time.
    /// This is not guaranteed to be current. May be an empty-string if the
    /// compute service does not use zones.
    #[serde(default)]
    #[structable(optional)]
    pub cached_zone: Option<String>,

    /// Whether the certificate is in the process of being replaced.
    #[serde(default)]
    #[structable(optional)]
    pub cert_busy: Option<bool>,

    /// The date the certificate for the amphora expires.
    #[serde(default)]
    #[structable(optional)]
    pub cert_expiration: Option<String>,

    /// The ID of the compute flavor used for the amphora.
    ///
    /// **New in version 2.3**
    #[serde(default)]
    #[structable(optional)]
    pub compute_flavor: Option<String>,

    /// The ID of the amphora resource in the compute system.
    #[serde(default)]
    #[structable(optional)]
    pub compute_id: Option<String>,

    /// The UTC date and timestamp when the resource was created.
    #[serde(default)]
    #[structable(optional)]
    pub created_at: Option<String>,

    /// The IP address of the Virtual IP (VIP).
    #[serde(default)]
    #[structable(optional)]
    pub ha_ip: Option<String>,

    /// The ID of the Virtual IP (VIP) port.
    #[serde(default)]
    #[structable(optional)]
    pub ha_port_id: Option<String>,

    /// The associated amphora ID.
    #[serde(default)]
    #[structable(optional)]
    pub id: Option<String>,

    /// The ID of the glance image used for the amphora.
    ///
    /// **New in version 2.1**
    #[serde(default)]
    #[structable(optional)]
    pub image_id: Option<String>,

    /// The management IP of the amphora.
    #[serde(default)]
    #[structable(optional)]
    pub lb_network_ip: Option<String>,

    /// The ID of the load balancer.
    #[serde(default)]
    #[structable(optional)]
    pub loadbalancer_id: Option<String>,

    /// The role of the amphora. One of `STANDALONE`, `MASTER`, `BACKUP`.
    #[serde(default)]
    #[structable(optional)]
    pub role: Option<String>,

    /// The status of the amphora. One of: `BOOTING`, `ALLOCATED`, `READY`,
    /// `PENDING_CREATE`, `PENDING_DELETE`, `DELETED`, `ERROR`.
    #[serde(default)]
    #[structable(optional)]
    pub status: Option<String>,

    /// The UTC date and timestamp when the resource was last updated.
    #[serde(default)]
    #[structable(optional)]
    pub updated_at: Option<String>,

    /// The vrrp group’s ID for the amphora.
    #[serde(default)]
    #[structable(optional)]
    pub vrrp_id: Option<i32>,

    /// The bound interface name of the vrrp port on the amphora.
    #[serde(default)]
    #[structable(optional)]
    pub vrrp_interface: Option<String>,

    /// The address of the vrrp port on the amphora.
    #[serde(default)]
    #[structable(optional)]
    pub vrrp_ip: Option<String>,

    /// The vrrp port’s ID in the networking system.
    #[serde(default)]
    #[structable(optional)]
    pub vrrp_port_id: Option<String>,

    /// The priority of the amphora in the vrrp group.
    #[serde(default)]
    #[structable(optional)]
    pub vrrp_priority: Option<i32>,
}
