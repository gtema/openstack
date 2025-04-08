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
//! Response type for the post networks operation

use crate::common::BoolString;
use crate::common::IntString;
use serde::{Deserialize, Serialize};

/// Network response representation
#[derive(Clone, Deserialize, Serialize)]
struct NetworkResponse {
    /// The administrative state of the network, which is up (`true`) or down
    /// (`false`).
    ///
    admin_state_up: Option<BoolString>,

    /// The availability zone candidate for the network.
    ///
    availability_zone_hints: Option<Vec<String>>,

    /// The availability zone for the network.
    ///
    availability_zones: Option<Vec<String>>,

    /// Time at which the resource has been created (in UTC ISO8601 format).
    ///
    created_at: Option<String>,

    /// A human-readable description for the resource.
    ///
    description: Option<String>,

    /// A valid DNS domain.
    ///
    dns_domain: Option<String>,

    /// The ID of the network.
    ///
    id: Option<String>,

    /// The ID of the IPv4 address scope that the network is associated with.
    ///
    ipv4_address_scope: Option<String>,

    /// The ID of the IPv6 address scope that the network is associated with.
    ///
    ipv6_address_scope: Option<String>,

    /// The network is default pool or not.
    ///
    is_default: Option<BoolString>,

    /// Indicates whether L2 connectivity is available throughout the
    /// `network`.
    ///
    l2_adjacency: Option<String>,

    /// The maximum transmission unit (MTU) value to address fragmentation.
    /// Minimum value is 68 for IPv4, and 1280 for IPv6.
    ///
    mtu: Option<IntString>,

    /// Human-readable name of the network.
    ///
    name: Option<String>,

    /// The port security status of the network. Valid values are enabled
    /// (`true`) and disabled (`false`). This value is used as the default
    /// value of `port_security_enabled` field of a newly created port.
    ///
    port_security_enabled: Option<BoolString>,

    #[serde(rename = "provider:network_type")]
    provider_network_type: Option<String>,

    #[serde(rename = "provider:physical_network")]
    provider_physical_network: Option<String>,

    #[serde(rename = "provider:segmentation_id")]
    provider_segmentation_id: Option<IntString>,

    /// The ID of the QoS policy associated with the network.
    ///
    qos_policy_id: Option<String>,

    /// The revision number of the resource.
    ///
    revision_number: Option<i32>,

    /// Defines whether the network may be used for creation of floating IPs.
    /// Only networks with this flag may be an external gateway for routers.
    /// The network must have an external routing facility that is not managed
    /// by the networking service. If the network is updated from external to
    /// internal the unused floating IPs of this network are automatically
    /// deleted when extension `floatingip-autodelete-internal` is present.
    ///
    #[serde(rename = "router:external")]
    router_external: Option<BoolString>,

    /// A list of provider `segment` objects.
    ///
    segments: Option<Vec<Segments>>,

    /// Indicates whether this network is shared across all tenants. By
    /// default, only administrative users can change this value.
    ///
    shared: Option<BoolString>,

    /// The network status. Values are `ACTIVE`, `DOWN`, `BUILD` or `ERROR`.
    ///
    status: Option<String>,

    /// The associated subnets.
    ///
    subnets: Option<Vec<String>>,

    /// The list of tags on the resource.
    ///
    tags: Option<Vec<String>>,

    /// The ID of the project.
    ///
    tenant_id: Option<String>,

    /// Time at which the resource has been updated (in UTC ISO8601 format).
    ///
    updated_at: Option<String>,
}

/// `Segments` type
#[derive(Clone, Debug, Deserialize, Serialize)]
struct Segments {
    provider_network_type: Option<String>,
    provider_physical_network: Option<String>,
    provider_segmentation_id: Option<i32>,
}
