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
//! Response type for the patch OS-EP-FILTER/endpoint_groups/{endpoint_group_id} operation

use serde::{Deserialize, Serialize};
use structable_derive::StructTable;

use crate::common::{OutputConfig, StructTable};

/// EndpointGroup response representation
#[derive(Clone, Deserialize, Serialize, StructTable)]
pub struct EndpointGroupResponse {
    /// The endpoint group description.
    ///
    #[structable(optional, serialize)]
    pub description: Option<String>,

    /// Describes the filtering performed by the endpoint group. The filter
    /// used must be an endpoint property, such as interface, service_id,
    /// region, and enabled. Note that if using interface as a filter, the only
    /// available values are public, internal, and admin.
    ///
    #[structable(optional, serialize)]
    pub filters: Option<Filters>,

    /// The endpoint group ID
    ///
    #[structable(optional)]
    pub id: Option<String>,

    /// The link to the resources in question.
    ///
    #[structable(optional, serialize)]
    pub links: Option<Links>,

    /// The name of the endpoint group.
    ///
    #[structable(optional)]
    pub name: Option<String>,
}

/// The link to the resources in question.
///
/// `Links` type
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Links {
    pub _self: Option<String>,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
#[serde(untagged)]
pub enum Interface {
    // Admin
    #[serde(rename = "admin")]
    Admin,

    // Internal
    #[serde(rename = "internal")]
    Internal,

    // Public
    #[serde(rename = "public")]
    Public,
}

/// Describes the filtering performed by the endpoint group. The filter used
/// must be an endpoint property, such as interface, service_id, region, and
/// enabled. Note that if using interface as a filter, the only available
/// values are public, internal, and admin.
///
/// `Filters` type
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Filters {
    pub enabled: Option<bool>,
    pub interface: Option<Interface>,
    pub region_id: Option<String>,
    pub service_id: Option<String>,
}
