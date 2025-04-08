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
//! Response type for the put routers/{id}/remove_external_gateways operation

use crate::common::BoolString;
use serde::{Deserialize, Serialize};

/// Router response representation
#[derive(Clone, Deserialize, Serialize)]
pub struct RouterResponse {
    /// The administrative state of the resource, which is up (`true`) or down
    /// (`false`).
    ///
    pub admin_state_up: Option<BoolString>,

    /// The availability zone candidates for the router. It is available when
    /// `router_availability_zone` extension is enabled.
    ///
    pub availability_zone_hints: Option<Vec<String>>,

    /// The availability zone(s) for the router. It is available when
    /// `router_availability_zone` extension is enabled.
    ///
    pub availability_zones: Option<Vec<String>>,

    /// The associated conntrack helper resources for the roter. If the router
    /// has multiple conntrack helper resources, this field has multiple
    /// entries. Each entry consists of netfilter conntrack helper (`helper`),
    /// the network protocol (`protocol`), the network port (`port`).
    ///
    pub conntrack_helpers: Option<String>,

    /// Time at which the resource has been created (in UTC ISO8601 format).
    ///
    pub created_at: Option<String>,

    /// A human-readable description for the resource.
    ///
    pub description: Option<String>,

    /// `true` indicates a distributed router. It is available when `dvr`
    /// extension is enabled.
    ///
    pub distributed: Option<BoolString>,

    /// Enable NDP proxy attribute. `true` means NDP proxy is enabled for the
    /// router, the IPv6 address of internal subnets attached to the router can
    /// be published to external by create `ndp_proxy`. `false` means NDP proxy
    /// is disabled, the IPv6 address of internal subnets attached to the
    /// router can not be published to external by `ndp_proxy`. It is available
    /// when `router-extend-ndp-proxy` extension is enabled.
    ///
    pub enable_ndp_proxy: Option<BoolString>,

    /// The external gateway information of the router. If the router has an
    /// external gateway, this would be a dict with `network_id`,
    /// `enable_snat`, `external_fixed_ips`, `qos_policy_id`,
    /// `enable_default_route_ecmp` and `enable_default_route_bfd`. Otherwise,
    /// this would be `null`.
    ///
    pub external_gateway_info: Option<ExternalGatewayInfo>,

    /// The ID of the flavor associated with the router.
    ///
    pub flavor_id: Option<String>,

    /// `true` indicates a highly-available router. It is available when
    /// `l3-ha` extension is enabled.
    ///
    pub ha: Option<BoolString>,

    /// The ID of the router.
    ///
    pub id: Option<String>,

    /// Human-readable name of the resource.
    ///
    pub name: Option<String>,

    /// The revision number of the resource.
    ///
    pub revision_number: Option<i32>,

    /// The extra routes configuration for L3 router. A list of dictionaries
    /// with `destination` and `nexthop` parameters. It is available when
    /// `extraroute` extension is enabled.
    ///
    pub routes: Option<Vec<Routes>>,

    /// The router status.
    ///
    pub status: Option<String>,

    /// The list of tags on the resource.
    ///
    pub tags: Option<Vec<String>>,

    /// The ID of the project.
    ///
    pub tenant_id: Option<String>,

    /// Time at which the resource has been updated (in UTC ISO8601 format).
    ///
    pub updated_at: Option<String>,
}

/// `ExternalFixedIps` type
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ExternalFixedIps {
    pub ip_address: Option<String>,
    pub subnet_id: Option<String>,
}

/// The external gateway information of the router. If the router has an
/// external gateway, this would be a dict with `network_id`, `enable_snat`,
/// `external_fixed_ips`, `qos_policy_id`, `enable_default_route_ecmp` and
/// `enable_default_route_bfd`. Otherwise, this would be `null`.
///
/// `ExternalGatewayInfo` type
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ExternalGatewayInfo {
    pub enable_snat: Option<BoolString>,
    pub external_fixed_ips: Option<Vec<ExternalFixedIps>>,
    pub network_id: String,
    pub qos_policy_id: Option<String>,
}

/// `Routes` type
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Routes {
    pub destination: Option<String>,
    pub nexthop: Option<String>,
}
