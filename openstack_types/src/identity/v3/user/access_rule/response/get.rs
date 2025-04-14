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
//! Response type for the GET `users/{user_id}/access_rules/{access_rule_id}` operation

use serde::{Deserialize, Serialize};
use structable::{StructTable, StructTableOptions};

/// AccessRule response representation
#[derive(Clone, Deserialize, Serialize, StructTable)]
pub struct AccessRuleResponse {
    /// The UUID of the access rule
    ///
    #[structable(optional)]
    pub id: Option<String>,

    /// The link to the resources in question.
    ///
    #[structable(optional, serialize)]
    pub links: Option<Links>,

    /// The request method that the application credential is permitted to use
    /// for a given API endpoint.
    ///
    #[structable(optional, serialize)]
    pub method: Option<Method>,

    /// The API path that the application credential is permitted to access.
    ///
    #[structable(optional)]
    pub path: Option<String>,

    /// The service type identifier for the service that the application
    /// credential is permitted to access. Must be a service type that is
    /// listed in the service catalog and not a code name for a service.
    ///
    #[structable(optional)]
    pub service: Option<String>,
}

/// The link to the resources in question.
///
/// `Links` type
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Links {
    pub _self: Option<String>,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub enum Method {
    // Delete
    #[serde(rename = "DELETE")]
    Delete,

    // Get
    #[serde(rename = "GET")]
    Get,

    // Head
    #[serde(rename = "HEAD")]
    Head,

    // Patch
    #[serde(rename = "PATCH")]
    Patch,

    // Post
    #[serde(rename = "POST")]
    Post,

    // Put
    #[serde(rename = "PUT")]
    Put,
}
