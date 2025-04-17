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
//! Response type for the GET `rbac-policies` operation

use serde::{Deserialize, Serialize};
use structable::{StructTable, StructTableOptions};

/// RbacPolicy response representation
#[derive(Clone, Deserialize, Serialize, StructTable)]
pub struct RbacPolicyResponse {
    /// Action for the RBAC policy which is `access_as_external` or
    /// `access_as_shared`.
    #[serde(default)]
    #[structable(optional, wide)]
    pub action: Option<String>,

    /// The ID of the RBAC policy.
    #[serde(default)]
    #[structable(optional)]
    pub id: Option<String>,

    /// The ID of the `object_type` resource. An `object_type` of `network`
    /// returns a network ID, an `object_type` of `qos-policy` returns a QoS
    /// policy ID, an `object_type` of `security-group` returns a security
    /// group ID, an `object_type` of `address-scope` returns a address scope
    /// ID, an `object_type` of `subnetpool` returns a subnetpool ID and an
    /// `object_type` of `address-group` returns an address group ID.
    #[serde(default)]
    #[structable(optional, wide)]
    pub object_id: Option<String>,

    /// The type of the object that the RBAC policy affects. Types include
    /// `qos-policy`, `network`, `security-group`, `address-scope`,
    /// `subnetpool` or `address-group`.
    #[serde(default)]
    #[structable(optional, wide)]
    pub object_type: Option<String>,

    /// The ID of the tenant to which the RBAC policy will be enforced.
    #[serde(default)]
    #[structable(optional, wide)]
    pub target_tenant: Option<String>,

    /// The ID of the project that owns the resource.
    #[serde(default)]
    #[structable(optional, wide)]
    pub tenant_id: Option<String>,
}
