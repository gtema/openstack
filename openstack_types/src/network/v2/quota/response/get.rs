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
//! Response type for the GET `quotas/{id}` operation

use serde::{Deserialize, Serialize};
use structable::{StructTable, StructTableOptions};

/// Quota response representation
#[derive(Clone, Deserialize, Serialize, StructTable)]
pub struct QuotaResponse {
    /// The number of floating IP addresses allowed for each project. A value
    /// of `-1` means no limit.
    ///
    #[structable(optional)]
    pub floatingip: Option<i32>,

    /// The number of networks allowed for each project. A value of `-1` means
    /// no limit.
    ///
    #[structable(optional)]
    pub network: Option<i32>,

    /// The number of ports allowed for each project. A value of `-1` means no
    /// limit.
    ///
    #[structable(optional)]
    pub port: Option<i32>,

    /// The ID of the project.
    ///
    #[structable(optional)]
    pub project_id: Option<String>,

    /// The number of role-based access control (RBAC) policies for each
    /// project. A value of `-1` means no limit.
    ///
    #[structable(optional)]
    pub rbac_policy: Option<i32>,

    /// The number of routers allowed for each project. A value of `-1` means
    /// no limit.
    ///
    #[structable(optional)]
    pub router: Option<i32>,

    /// The number of security groups allowed for each project. A value of `-1`
    /// means no limit.
    ///
    #[structable(optional)]
    pub security_group: Option<i32>,

    /// The number of security group rules allowed for each project. A value of
    /// `-1` means no limit.
    ///
    #[structable(optional)]
    pub security_group_rule: Option<i32>,

    /// The number of subnets allowed for each project. A value of `-1` means
    /// no limit.
    ///
    #[structable(optional)]
    pub subnet: Option<i32>,

    /// The number of subnet pools allowed for each project. A value of `-1`
    /// means no limit.
    ///
    #[structable(optional)]
    pub subnetpool: Option<i32>,
}
