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
//! Response type for the patch limits/{limit_id} operation

use serde::{Deserialize, Serialize};

/// Limit response representation
#[derive(Clone, Deserialize, Serialize)]
pub struct LimitResponse {
    /// The limit description.
    ///
    pub description: Option<String>,

    /// The ID of the domain.
    ///
    pub domain_id: Option<String>,

    /// The limit ID.
    ///
    pub id: Option<String>,

    /// The link to the resources in question.
    ///
    pub links: Option<Links>,

    /// The ID for the project.
    ///
    pub project_id: Option<String>,

    /// The ID of the region that contains the service endpoint. The value can
    /// be None.
    ///
    pub region_id: Option<String>,

    /// The override limit.
    ///
    pub resource_limit: Option<i32>,

    /// The resource name.
    ///
    pub resource_name: Option<String>,

    /// The UUID of the service to which the limit belongs.
    ///
    pub service_id: Option<String>,
}

/// The link to the resources in question.
///
/// `Links` type
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Links {
    pub _self: Option<String>,
}
