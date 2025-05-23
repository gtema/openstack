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
//! Response type for the GET `agents/{agent_id}/l3-routers` operation

use serde::{Deserialize, Serialize};
use structable::{StructTable, StructTableOptions};

/// L3Router response representation
#[derive(Clone, Deserialize, Serialize, StructTable)]
pub struct L3RouterResponse {
    /// A list of `router` objects.
    #[structable(serialize)]
    pub routers: Vec<Routers>,
}

/// `ExternalFixedIps` type
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ExternalFixedIps {
    #[serde(default)]
    pub ip_address: Option<String>,
    #[serde(default)]
    pub subnet_id: Option<String>,
}

/// The external gateway information of the router. If the router has an
/// external gateway, this would be a dict with `network_id`, `enable_snat`,
/// `external_fixed_ips`, `qos_policy_id`, `enable_default_route_ecmp` and
/// `enable_default_route_bfd`. Otherwise, this would be `null`.
/// `ExternalGatewayInfo` type
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ExternalGatewayInfo {
    #[serde(default)]
    pub enable_snat: Option<bool>,
    #[serde(default)]
    pub external_fixed_ips: Option<Vec<ExternalFixedIps>>,
    #[serde(default)]
    pub network_id: Option<String>,
}

/// `Routes` type
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Routes {
    #[serde(default)]
    pub destination: Option<String>,
    #[serde(default)]
    pub next_hop: Option<String>,
}

/// `Routers` type
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Routers {
    #[serde(default)]
    pub admin_state_up: Option<bool>,
    #[serde(default)]
    pub availability_zone_hints: Option<Vec<String>>,
    #[serde(default)]
    pub availability_zones: Option<Vec<String>>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub distributed: Option<bool>,
    #[serde(default)]
    pub external_gateway_info: Option<ExternalGatewayInfo>,
    #[serde(default)]
    pub flavor_id: Option<String>,
    #[serde(default)]
    pub ha: Option<bool>,
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub project_id: Option<String>,
    #[serde(default)]
    pub revision_number: Option<i32>,
    #[serde(default)]
    pub routes: Option<Vec<Routes>>,
    #[serde(default)]
    pub service_type_id: Option<String>,
    #[serde(default)]
    pub status: Option<String>,
    #[serde(default)]
    pub tenant_id: Option<String>,
}
