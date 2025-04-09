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
//! Response type for the get quotas/{id}/details operation

use serde::{Deserialize, Serialize};

/// Quota response representation
#[derive(Clone, Deserialize, Serialize)]
pub struct QuotaResponse {
    /// The number of floating IP addresses allowed for each project.
    ///
    pub floatingip: Option<Floatingip>,

    /// The number of networks allowed for each project.
    ///
    pub network: Option<Network>,

    /// The number of ports allowed for each project.
    ///
    pub port: Option<Port>,

    /// The number of role-based access control (RBAC) policies for each
    /// project.
    ///
    pub rbac_policy: Option<RbacPolicy>,

    /// The number of routers allowed for each project.
    ///
    pub router: Option<Router>,

    /// The number of security groups allowed for each project.
    ///
    pub security_group: Option<SecurityGroup>,

    /// The number of security group rules allowed for each project.
    ///
    pub security_group_rule: Option<SecurityGroupRule>,

    /// The number of subnets allowed for each project.
    ///
    pub subnet: Option<Subnet>,

    /// The number of subnet pools allowed for each project.
    ///
    pub subnetpool: Option<Subnetpool>,
}

/// The number of floating IP addresses allowed for each project.
///
/// `Floatingip` type
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Floatingip {
    pub limit: Option<i32>,
    pub reserved: Option<i32>,
    pub used: Option<i32>,
}

/// The number of networks allowed for each project.
///
/// `Network` type
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Network {
    pub limit: Option<i32>,
    pub reserved: Option<i32>,
    pub used: Option<i32>,
}

/// The number of ports allowed for each project.
///
/// `Port` type
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Port {
    pub limit: Option<i32>,
    pub reserved: Option<i32>,
    pub used: Option<i32>,
}

/// The number of role-based access control (RBAC) policies for each project.
///
/// `RbacPolicy` type
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RbacPolicy {
    pub limit: Option<i32>,
    pub reserved: Option<i32>,
    pub used: Option<i32>,
}

/// The number of routers allowed for each project.
///
/// `Router` type
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Router {
    pub limit: Option<i32>,
    pub reserved: Option<i32>,
    pub used: Option<i32>,
}

/// The number of security groups allowed for each project.
///
/// `SecurityGroup` type
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SecurityGroup {
    pub limit: Option<i32>,
    pub reserved: Option<i32>,
    pub used: Option<i32>,
}

/// The number of security group rules allowed for each project.
///
/// `SecurityGroupRule` type
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SecurityGroupRule {
    pub limit: Option<i32>,
    pub reserved: Option<i32>,
    pub used: Option<i32>,
}

/// The number of subnets allowed for each project.
///
/// `Subnet` type
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Subnet {
    pub limit: Option<i32>,
    pub reserved: Option<i32>,
    pub used: Option<i32>,
}

/// The number of subnet pools allowed for each project.
///
/// `Subnetpool` type
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Subnetpool {
    pub limit: Option<i32>,
    pub reserved: Option<i32>,
    pub used: Option<i32>,
}
