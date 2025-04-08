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
//! Response type for the get servers/{server_id}/os-security-groups operation

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

/// SecurityGroup response representation
#[derive(Clone, Deserialize, Serialize)]
pub struct SecurityGroupResponse {
    /// Security group description.
    ///
    pub description: Option<String>,

    /// The ID of the security group.
    ///
    pub id: String,

    /// The security group name.
    ///
    pub name: String,

    /// The list of security group rules.
    ///
    pub rules: Option<Vec<Rules>>,

    /// The UUID of the tenant in a multi-tenancy cloud.
    ///
    pub tenant_id: Option<String>,
}

/// `Group` type
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Group {
    pub name: Option<String>,
}

/// `Rules` type
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Rules {
    pub from_port: Option<i32>,
    pub group: Option<Group>,
    pub id: Option<String>,
    pub ip_protocol: Option<String>,
    pub ip_range: Option<HashMap<String, Value>>,
    pub parent_group_id: Option<String>,
    pub to_port: Option<i32>,
}
