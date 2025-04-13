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
//! Response type for the get lbaas/loadbalancers/{loadbalancer_id} operation

use serde::{Deserialize, Serialize};
use structable::{StructTable, StructTableOptions};

/// Loadbalancer response representation
#[derive(Clone, Deserialize, Serialize, StructTable)]
pub struct LoadbalancerResponse {
    /// A list of JSON objects defining “additional VIPs”. The format for these
    /// is `{"subnet_id": <subnet_id>, "ip_address": <ip_address>}`, where the
    /// `subnet_id` field is mandatory and the `ip_address` field is optional.
    /// Additional VIP subnets must all belong to the same network as the
    /// primary VIP.
    ///
    /// **New in version 2.26**
    ///
    #[structable(optional, serialize)]
    pub additional_vips: Option<Vec<AdditionalVips>>,

    /// The administrative state of the resource, which is up (`true`) or down
    /// (`false`).
    ///
    #[structable(optional)]
    pub admin_state_up: Option<bool>,

    /// An availability zone name.
    ///
    #[structable(optional)]
    pub availability_zone: Option<String>,

    /// The UTC date and timestamp when the resource was created.
    ///
    #[structable(optional)]
    pub created_at: Option<String>,

    /// A human-readable description for the resource.
    ///
    #[structable(optional)]
    pub description: Option<String>,

    /// The ID of the flavor.
    ///
    #[structable(optional)]
    pub flavor_id: Option<String>,

    /// The ID of the load balancer.
    ///
    #[structable(optional)]
    pub id: Option<String>,

    /// The associated listener IDs, if any.
    ///
    #[structable(optional, serialize)]
    pub listeners: Option<Vec<Listeners>>,

    /// Human-readable name of the resource.
    ///
    #[structable(optional)]
    pub name: Option<String>,

    /// The operating status of the resource. See
    /// [Operating Status Codes](#op-status).
    ///
    #[structable(optional)]
    pub operating_status: Option<String>,

    /// The associated pool IDs, if any.
    ///
    #[structable(optional, serialize)]
    pub pools: Option<Vec<Pools>>,

    /// The ID of the project owning this resource.
    ///
    #[structable(optional)]
    pub project_id: Option<String>,

    /// Provider name for the load balancer.
    ///
    #[structable(optional)]
    pub provider: Option<String>,

    /// The provisioning status of the resource. See
    /// [Provisioning Status Codes](#prov-status).
    ///
    #[structable(optional)]
    pub provisioning_status: Option<String>,

    /// A list of simple strings assigned to the resource.
    ///
    /// **New in version 2.5**
    ///
    #[structable(optional, serialize)]
    pub tags: Option<Vec<String>>,

    #[structable(optional)]
    pub tenant_id: Option<String>,

    /// The UTC date and timestamp when the resource was last updated.
    ///
    #[structable(optional)]
    pub updated_at: Option<String>,

    /// The IP address of the Virtual IP (VIP).
    ///
    #[structable(optional)]
    pub vip_address: Option<String>,

    /// The ID of the network for the Virtual IP (VIP).
    ///
    #[structable(optional)]
    pub vip_network_id: Option<String>,

    /// The ID of the Virtual IP (VIP) port.
    ///
    #[structable(optional)]
    pub vip_port_id: Option<String>,

    /// The ID of the QoS Policy which will apply to the Virtual IP (VIP).
    ///
    #[structable(optional)]
    pub vip_qos_policy_id: Option<String>,

    /// The list of Security Group IDs of the Virtual IP (VIP) port of the Load
    /// Balancer.
    ///
    /// **New in version 2.29**
    ///
    #[structable(optional, serialize)]
    pub vip_sg_ids: Option<Vec<String>>,

    /// The ID of the subnet for the Virtual IP (VIP).
    ///
    #[structable(optional)]
    pub vip_subnet_id: Option<String>,

    /// The VIP vNIC type used for the load balancer. One of `normal` or
    /// `direct`.
    ///
    /// **New in version 2.28**
    ///
    #[structable(optional)]
    pub vip_vnic_type: Option<String>,
}

/// Type for additional vips
///
/// `AdditionalVips` type
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AdditionalVips {
    pub ip_address: Option<String>,
    pub subnet_id: String,
}

/// Base type for complex types
///
/// `Listeners` type
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Listeners {
    pub id: String,
}

/// Base type for complex types
///
/// `Pools` type
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Pools {
    pub id: String,
}
