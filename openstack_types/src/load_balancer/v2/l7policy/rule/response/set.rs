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
//! Response type for the PUT `lbaas/l7policies/{l7policy_id}/rules/{rule_id}` operation

use serde::{Deserialize, Serialize};
use structable::{StructTable, StructTableOptions};

/// Rule response representation
#[derive(Clone, Deserialize, Serialize, StructTable)]
pub struct RuleResponse {
    /// The administrative state of the resource, which is up (`true`) or down
    /// (`false`).
    ///
    #[structable(optional)]
    pub admin_state_up: Option<bool>,

    /// The comparison type for the L7 rule. One of `CONTAINS`, `ENDS_WITH`,
    /// `EQUAL_TO`, `REGEX`, or `STARTS_WITH`.
    ///
    #[structable(optional)]
    pub compare_type: Option<String>,

    /// The UTC date and timestamp when the resource was created.
    ///
    #[structable(optional)]
    pub created_at: Option<String>,

    /// The ID of the L7 rule.
    ///
    #[structable(optional)]
    pub id: Option<String>,

    /// When `true` the logic of the rule is inverted. For example, with invert
    /// `true`, equal to would become not equal to.
    ///
    #[structable(optional)]
    pub invert: Option<bool>,

    /// The key to use for the comparison. For example, the name of the cookie
    /// to evaluate.
    ///
    #[structable(optional)]
    pub key: Option<String>,

    /// The operating status of the resource. See
    /// [Operating Status Codes](#op-status).
    ///
    #[structable(optional)]
    pub operating_status: Option<String>,

    /// The ID of the project owning this resource.
    ///
    #[structable(optional)]
    pub project_id: Option<String>,

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

    /// The L7 rule type. One of `COOKIE`, `FILE_TYPE`, `HEADER`, `HOST_NAME`,
    /// `PATH`, `SSL_CONN_HAS_CERT`, `SSL_VERIFY_RESULT`, or `SSL_DN_FIELD`.
    ///
    #[serde(rename = "type")]
    #[structable(optional, title = "type")]
    pub _type: Option<String>,

    /// The UTC date and timestamp when the resource was last updated.
    ///
    #[structable(optional)]
    pub updated_at: Option<String>,

    /// The value to use for the comparison. For example, the file type to
    /// compare.
    ///
    #[structable(optional)]
    pub value: Option<String>,
}
