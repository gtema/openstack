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
//! Response type for the get role_assignments operation

use serde::{Deserialize, Serialize};
use structable_derive::StructTable;

use crate::common::{OutputConfig, StructTable};

/// RoleAssignment response representation
#[derive(Clone, Deserialize, Serialize, StructTable)]
pub struct RoleAssignmentResponse {
    #[structable(optional, serialize)]
    pub group: Option<Group>,

    #[structable(serialize)]
    pub role: Role,

    #[structable(serialize)]
    pub scope: Scope,

    #[structable(optional, serialize)]
    pub user: Option<User>,
}

/// `Links` type
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Links {
    pub assignment: String,
    pub membership: Option<String>,
    pub prior_role: Option<String>,
}

/// `Domain` type
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Domain {
    pub id: String,
    pub name: Option<String>,
}

/// `Role` type
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Role {
    pub domain: Option<Domain>,
    pub id: String,
    pub name: Option<String>,
}

/// `Project` type
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Project {
    pub domain: Option<Domain>,
    pub id: String,
    pub name: Option<String>,
}

/// `ScopeDomain` type
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ScopeDomain {
    pub id: String,
    pub name: Option<String>,
}

/// `System` type
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct System {
    pub all: i32,
}

/// `Scope` type
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Scope {
    pub domain: Option<ScopeDomain>,
    pub os_inherit_inherited_to: Option<String>,
    pub project: Option<Project>,
    pub system: Option<System>,
}

/// `User` type
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct User {
    pub domain: Option<Domain>,
    pub id: String,
    pub name: Option<String>,
}

/// `Group` type
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Group {
    pub domain: Option<Domain>,
    pub id: String,
    pub name: Option<String>,
}
