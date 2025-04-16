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
//! Response type for the GET `registered_limits` operation

use serde::{Deserialize, Serialize};
use structable::{StructTable, StructTableOptions};

/// RegisteredLimit response representation
#[derive(Clone, Deserialize, Serialize, StructTable)]
pub struct RegisteredLimitResponse {
    /// The default limit for the registered limit.
    #[serde(default)]
    #[structable(optional, wide)]
    pub default_limit: Option<i32>,

    /// The registered limit description.
    #[serde(default)]
    #[structable(optional, wide)]
    pub description: Option<String>,

    /// The registered limit ID.
    #[serde(default)]
    #[structable(optional)]
    pub id: Option<String>,

    /// The ID of the region that contains the service endpoint. The value can
    /// be None.
    #[serde(default)]
    #[structable(optional, wide)]
    pub region_id: Option<String>,

    /// The resource name.
    #[serde(default)]
    #[structable(optional, wide)]
    pub resource_name: Option<String>,

    /// The UUID of the service to which the registered limit belongs.
    #[serde(default)]
    #[structable(optional, wide)]
    pub service_id: Option<String>,
}

/// The link to the resources in question.
/// `Links` type
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Links {
    pub _self: Option<String>,
}
