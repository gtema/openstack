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
//! Response type for the get zones operation

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use structable::{StructTable, StructTableOptions};

/// Zone response representation
#[derive(Clone, Deserialize, Serialize, StructTable)]
pub struct ZoneResponse {
    /// current action in progress on the resource
    ///
    #[structable(optional, serialize, wide)]
    pub action: Option<Action>,

    /// Key:Value pairs of information about this zone, and the pool the user
    /// would like to place the zone in. This information can be used by the
    /// scheduler to place zones on the correct pool.
    ///
    #[structable(optional, serialize, wide)]
    pub attributes: Option<HashMap<String, String>>,

    /// Date / Time when resource was created.
    ///
    #[structable(optional)]
    pub created_at: Option<String>,

    /// Description for this zone
    ///
    #[structable(optional, wide)]
    pub description: Option<String>,

    /// e-mail for the zone. Used in SOA records for the zone
    ///
    #[structable(optional, wide)]
    pub email: Option<String>,

    /// ID for the resource
    ///
    #[structable(optional)]
    pub id: Option<String>,

    /// Mandatory for secondary zones. The servers to slave from to get DNS
    /// information
    ///
    #[structable(optional, serialize, wide)]
    pub masters: Option<Vec<String>>,

    /// DNS Name for the zone
    ///
    #[structable(optional)]
    pub name: Option<String>,

    /// ID for the pool hosting this zone
    ///
    #[structable(optional, wide)]
    pub pool_id: Option<String>,

    /// ID for the project that owns the resource
    ///
    #[structable(optional, wide)]
    pub project_id: Option<String>,

    /// current serial number for the zone
    ///
    #[structable(optional, wide)]
    pub serial: Option<i32>,

    /// True if the zone is shared with another project.
    ///
    /// **New in version 2.1**
    ///
    #[structable(optional, wide)]
    pub shared: Option<bool>,

    /// The status of the resource.
    ///
    #[structable(optional, serialize)]
    pub status: Option<Status>,

    /// For secondary zones. The last time an update was retrieved from the
    /// master servers
    ///
    #[structable(optional, serialize, wide)]
    pub transferred_at: Option<String>,

    /// TTL (Time to Live) for the zone.
    ///
    #[structable(optional, wide)]
    pub ttl: Option<i32>,

    /// Type of zone. PRIMARY is controlled by Designate, SECONDARY zones are
    /// slaved from another DNS Server. Defaults to PRIMARY
    ///
    #[serde(rename = "type")]
    #[structable(optional, serialize, title = "type", wide)]
    pub _type: Option<Type>,

    /// Date / Time when resource last updated.
    ///
    #[structable(optional, serialize)]
    pub updated_at: Option<String>,

    /// Version of the resource
    ///
    #[structable(optional, wide)]
    pub version: Option<i32>,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
#[serde(untagged)]
pub enum Status {
    // Active
    #[serde(rename = "ACTIVE")]
    Active,

    // Deleted
    #[serde(rename = "DELETED")]
    Deleted,

    // Error
    #[serde(rename = "ERROR")]
    Error,

    // Pending
    #[serde(rename = "PENDING")]
    Pending,

    // Success
    #[serde(rename = "SUCCESS")]
    Success,

    // Zone
    #[serde(rename = "ZONE")]
    Zone,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
#[serde(untagged)]
pub enum Action {
    // Create
    #[serde(rename = "CREATE")]
    Create,

    // Delete
    #[serde(rename = "DELETE")]
    Delete,

    // None
    #[serde(rename = "NONE")]
    None,

    // Update
    #[serde(rename = "UPDATE")]
    Update,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
#[serde(untagged)]
pub enum Type {
    // Catalog
    #[serde(rename = "CATALOG")]
    Catalog,

    // Primary
    #[serde(rename = "PRIMARY")]
    Primary,

    // Secondary
    #[serde(rename = "SECONDARY")]
    Secondary,
}

/// Links to the resource, and other related resources. When a response has
/// been broken into pages, we will include a `next` link that should be
/// followed to retrieve all results
///
/// `Links` type
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Links {
    pub _self: Option<String>,
}
