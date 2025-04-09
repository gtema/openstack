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
//! Response type for the get lbaas/pools/{pool_id}/members/{member_id} operation

use serde::{Deserialize, Serialize};

/// Member response representation
#[derive(Clone, Deserialize, Serialize)]
pub struct MemberResponse {
    /// The IP address of the backend member server.
    ///
    pub address: Option<String>,

    /// The administrative state of the resource, which is up (`true`) or down
    /// (`false`).
    ///
    pub admin_state_up: Option<bool>,

    /// Is the member a backup? Backup members only receive traffic when all
    /// non-backup members are down.
    ///
    /// **New in version 2.1**
    ///
    pub backup: Option<bool>,

    /// The UTC date and timestamp when the resource was created.
    ///
    pub created_at: Option<String>,

    /// The ID of the member.
    ///
    pub id: Option<String>,

    /// An alternate IP address used for health monitoring a backend member.
    /// Default is `null` which monitors the member `address`.
    ///
    pub monitor_address: Option<String>,

    /// An alternate protocol port used for health monitoring a backend member.
    /// Default is `null` which monitors the member `protocol_port`.
    ///
    pub monitor_port: Option<i32>,

    /// Human-readable name of the resource.
    ///
    pub name: Option<String>,

    /// The operating status of the resource. See
    /// [Operating Status Codes](#op-status).
    ///
    pub operating_status: Option<String>,

    /// The ID of the project owning this resource.
    ///
    pub project_id: Option<String>,

    /// The protocol port number the backend member server is listening on.
    ///
    pub protocol_port: Option<i32>,

    /// The provisioning status of the resource. See
    /// [Provisioning Status Codes](#prov-status).
    ///
    pub provisioning_status: Option<String>,

    /// The subnet ID the member service is accessible from.
    ///
    pub subnet_id: Option<String>,

    /// A list of simple strings assigned to the resource.
    ///
    /// **New in version 2.5**
    ///
    pub tags: Option<Vec<String>>,

    pub tenant_id: Option<String>,

    /// The UTC date and timestamp when the resource was last updated.
    ///
    pub updated_at: Option<String>,

    /// The member vNIC type used for the member port. One of `normal` or
    /// `direct`.
    ///
    /// **New in version 2.29**
    ///
    pub vnic_type: Option<String>,

    /// The weight of a member determines the portion of requests or
    /// connections it services compared to the other members of the pool. For
    /// example, a member with a weight of 10 receives five times as many
    /// requests as a member with a weight of 2. A value of 0 means the member
    /// does not receive new connections but continues to service existing
    /// connections. A valid value is from `0` to `256`. Default is `1`.
    ///
    pub weight: Option<i32>,
}
