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
//! Response type for the GET `lbaas/quotas` operation

use serde::{Deserialize, Serialize};
use structable::{StructTable, StructTableOptions};

/// Quota response representation
#[derive(Clone, Deserialize, Serialize, StructTable)]
pub struct QuotaResponse {
    #[serde(default)]
    #[structable(optional)]
    pub health_monitor: Option<i32>,

    /// The configured health monitor quota limit. A setting of `null` means it
    /// is using the deployment default quota. A setting of `-1` means
    /// unlimited.
    #[serde(default)]
    #[structable(optional)]
    pub healthmonitor: Option<i32>,

    /// The configured l7policy quota limit. A setting of `null` means it is
    /// using the deployment default quota. A setting of `-1` means unlimited.
    #[serde(default)]
    #[structable(optional)]
    pub l7policy: Option<i32>,

    /// The configured l7rule quota limit. A setting of `null` means it is
    /// using the deployment default quota. A setting of `-1` means unlimited.
    #[serde(default)]
    #[structable(optional)]
    pub l7rule: Option<i32>,

    /// The configured listener quota limit. A setting of `null` means it is
    /// using the deployment default quota. A setting of `-1` means unlimited.
    #[serde(default)]
    #[structable(optional)]
    pub listener: Option<i32>,

    #[serde(default)]
    #[structable(optional)]
    pub load_balancer: Option<i32>,

    /// The configured load balancer quota limit. A setting of `null` means it
    /// is using the deployment default quota. A setting of `-1` means
    /// unlimited.
    #[serde(default)]
    #[structable(optional)]
    pub loadbalancer: Option<i32>,

    /// The configured member quota limit. A setting of `null` means it is
    /// using the deployment default quota. A setting of `-1` means unlimited.
    #[serde(default)]
    #[structable(optional)]
    pub member: Option<i32>,

    /// The configured pool quota limit. A setting of `null` means it is using
    /// the deployment default quota. A setting of `-1` means unlimited.
    #[serde(default)]
    #[structable(optional)]
    pub pool: Option<i32>,

    /// The ID of the project owning this resource.
    #[serde(default)]
    #[structable(optional, wide)]
    pub project_id: Option<String>,

    #[serde(default)]
    #[structable(optional)]
    pub tenant_id: Option<String>,
}
