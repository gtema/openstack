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
//! Response type for the GET `lbaas/l7policies/{l7policy_id}` operation

use serde::{Deserialize, Serialize};
use structable::{StructTable, StructTableOptions};

/// L7policy response representation
#[derive(Clone, Deserialize, Serialize, StructTable)]
pub struct L7policyResponse {
    /// The L7 policy action. One of `REDIRECT_PREFIX`, `REDIRECT_TO_POOL`,
    /// `REDIRECT_TO_URL`, or `REJECT`.
    ///
    #[structable(optional)]
    pub action: Option<String>,

    /// The administrative state of the resource, which is up (`true`) or down
    /// (`false`).
    ///
    #[structable(optional)]
    pub admin_state_up: Option<bool>,

    /// The UTC date and timestamp when the resource was created.
    ///
    #[structable(optional)]
    pub created_at: Option<String>,

    /// A human-readable description for the resource.
    ///
    #[structable(optional)]
    pub description: Option<String>,

    /// The ID of the L7 policy.
    ///
    #[structable(optional)]
    pub id: Option<String>,

    /// The ID of the listener.
    ///
    #[structable(optional)]
    pub listener_id: Option<String>,

    /// Human-readable name of the resource.
    ///
    #[structable(optional)]
    pub name: Option<String>,

    /// The operating status of the resource. See
    /// [Operating Status Codes](#op-status).
    ///
    #[structable(optional)]
    pub operating_status: Option<String>,

    /// The position of this policy on the listener. Positions start at 1.
    ///
    #[structable(optional)]
    pub position: Option<i32>,

    /// The ID of the project owning this resource.
    ///
    #[structable(optional)]
    pub project_id: Option<String>,

    /// The provisioning status of the resource. See
    /// [Provisioning Status Codes](#prov-status).
    ///
    #[structable(optional)]
    pub provisioning_status: Option<String>,

    /// Requests matching this policy will be redirected to the specified URL
    /// or Prefix URL with the HTTP response code. Valid if `action` is
    /// `REDIRECT_TO_URL` or `REDIRECT_PREFIX`. Valid options are: 301, 302,
    /// 303, 307, or 308. Default is 302.
    ///
    /// **New in version 2.9**
    ///
    #[structable(optional)]
    pub redirect_http_code: Option<i32>,

    /// Requests matching this policy will be redirected to the pool with this
    /// ID. Only valid if `action` is `REDIRECT_TO_POOL`. The pool has some
    /// restrictions, See
    /// [Protocol Combinations (Listener/Pool)](#valid-protocol).
    ///
    #[structable(optional)]
    pub redirect_pool_id: Option<String>,

    /// Requests matching this policy will be redirected to this Prefix URL.
    /// Only valid if `action` is `REDIRECT_PREFIX`.
    ///
    #[structable(optional)]
    pub redirect_prefix: Option<String>,

    /// Requests matching this policy will be redirected to this URL. Only
    /// valid if `action` is `REDIRECT_TO_URL`.
    ///
    #[structable(optional)]
    pub redirect_url: Option<String>,

    /// List of associated L7 rule IDs.
    ///
    #[structable(optional, serialize)]
    pub rules: Option<Vec<Rules>>,

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
}

/// Base type for complex types
///
/// `Rules` type
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Rules {
    pub id: String,
}
