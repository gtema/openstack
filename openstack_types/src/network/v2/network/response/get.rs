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
//! Response type for the get networks/{network_id} operation

use crate::common::deser_bool_str_opt;
use crate::common::deser_num_str_opt;
use serde::{Deserialize, Serialize};
use structable_derive::StructTable;

use crate::common::{OutputConfig, StructTable};

/// Network response representation
#[derive(Clone, Deserialize, Serialize, StructTable)]
pub struct NetworkResponse {
    /// The administrative state of the network, which is up (`true`) or down
    /// (`false`).
    ///
    #[serde(deserialize_with = "deser_bool_str_opt")]
    #[structable(optional)]
    pub admin_state_up: Option<bool>,

    /// The availability zone candidate for the network.
    ///
    #[structable(optional, serialize)]
    pub availability_zone_hints: Option<Vec<String>>,

    /// The availability zone for the network.
    ///
    #[structable(optional, serialize)]
    pub availability_zones: Option<Vec<String>>,

    /// Time at which the resource has been created (in UTC ISO8601 format).
    ///
    #[structable(optional)]
    pub created_at: Option<String>,

    /// A human-readable description for the resource.
    ///
    #[structable(optional)]
    pub description: Option<String>,

    /// A valid DNS domain.
    ///
    #[structable(optional)]
    pub dns_domain: Option<String>,

    /// The ID of the network.
    ///
    #[structable(optional)]
    pub id: Option<String>,

    /// The ID of the IPv4 address scope that the network is associated with.
    ///
    #[structable(optional)]
    pub ipv4_address_scope: Option<String>,

    /// The ID of the IPv6 address scope that the network is associated with.
    ///
    #[structable(optional)]
    pub ipv6_address_scope: Option<String>,

    /// The network is default pool or not.
    ///
    #[serde(deserialize_with = "deser_bool_str_opt")]
    #[structable(optional)]
    pub is_default: Option<bool>,

    /// Indicates whether L2 connectivity is available throughout the
    /// `network`.
    ///
    #[structable(optional)]
    pub l2_adjacency: Option<String>,

    /// The maximum transmission unit (MTU) value to address fragmentation.
    /// Minimum value is 68 for IPv4, and 1280 for IPv6.
    ///
    #[serde(deserialize_with = "deser_num_str_opt")]
    #[structable(optional)]
    pub mtu: Option<i64>,

    /// Human-readable name of the network.
    ///
    #[structable(optional)]
    pub name: Option<String>,

    /// The port security status of the network. Valid values are enabled
    /// (`true`) and disabled (`false`). This value is used as the default
    /// value of `port_security_enabled` field of a newly created port.
    ///
    #[serde(deserialize_with = "deser_bool_str_opt")]
    #[structable(optional)]
    pub port_security_enabled: Option<bool>,

    #[serde(rename = "provider:network_type")]
    #[structable(optional, title = "provider:network_type")]
    pub provider_network_type: Option<String>,

    #[serde(rename = "provider:physical_network")]
    #[structable(optional, title = "provider:physical_network")]
    pub provider_physical_network: Option<String>,

    #[serde(
        deserialize_with = "deser_num_str_opt",
        rename = "provider:segmentation_id"
    )]
    #[structable(optional, title = "provider:segmentation_id")]
    pub provider_segmentation_id: Option<i64>,

    /// The ID of the QoS policy associated with the network.
    ///
    #[structable(optional, serialize)]
    pub qos_policy_id: Option<String>,

    /// The revision number of the resource.
    ///
    #[structable(optional)]
    pub revision_number: Option<i32>,

    /// Defines whether the network may be used for creation of floating IPs.
    /// Only networks with this flag may be an external gateway for routers.
    /// The network must have an external routing facility that is not managed
    /// by the networking service. If the network is updated from external to
    /// internal the unused floating IPs of this network are automatically
    /// deleted when extension `floatingip-autodelete-internal` is present.
    ///
    #[serde(deserialize_with = "deser_bool_str_opt", rename = "router:external")]
    #[structable(optional, title = "router:external")]
    pub router_external: Option<bool>,

    /// A list of provider `segment` objects.
    ///
    #[structable(optional, serialize)]
    pub segments: Option<Vec<Segments>>,

    /// Indicates whether this network is shared across all tenants. By
    /// default, only administrative users can change this value.
    ///
    #[serde(deserialize_with = "deser_bool_str_opt")]
    #[structable(optional)]
    pub shared: Option<bool>,

    /// The network status. Values are `ACTIVE`, `DOWN`, `BUILD` or `ERROR`.
    ///
    #[structable(optional)]
    pub status: Option<String>,

    /// The associated subnets.
    ///
    #[structable(optional, serialize)]
    pub subnets: Option<Vec<String>>,

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

/// `Segments` type
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Segments {
    pub provider_network_type: Option<String>,
    pub provider_physical_network: Option<String>,
    pub provider_segmentation_id: Option<i32>,
}
