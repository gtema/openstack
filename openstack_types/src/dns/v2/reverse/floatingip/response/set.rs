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
//! Response type for the patch reverse/floatingips/{fip_key} operation

use serde::{Deserialize, Serialize};
use structable::{StructTable, StructTableOptions};

/// Floatingip response representation
#[derive(Clone, Deserialize, Serialize, StructTable)]
pub struct FloatingipResponse {
    /// current action in progress on the resource
    ///
    #[structable(optional, serialize)]
    pub action: Option<Action>,

    /// The floatingip address for this PTR record.
    ///
    #[structable(optional)]
    pub address: Option<String>,

    /// Description for this PTR record
    ///
    #[structable(optional)]
    pub description: Option<String>,

    /// ID for PTR record in the format of <region>:\<floatingip_id>
    ///
    #[structable(optional)]
    pub id: Option<String>,

    /// Links to the resource, and other related resources. When a response has
    /// been broken into pages, we will include a `next` link that should be
    /// followed to retrieve all results
    ///
    #[structable(optional, serialize)]
    pub links: Option<Links>,

    /// Domain name for this PTR record
    ///
    #[structable(optional, serialize)]
    pub ptrdname: Option<String>,

    /// The status of the resource.
    ///
    #[structable(optional, serialize)]
    pub status: Option<Status>,

    /// Time to live for this PTR record
    ///
    #[structable(optional)]
    pub ttl: Option<i32>,
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
}

#[derive(Debug, Deserialize, Clone, Serialize)]
#[serde(untagged)]
pub enum Action {
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

/// Links to the resource, and other related resources. When a response has
/// been broken into pages, we will include a `next` link that should be
/// followed to retrieve all results
///
/// `Links` type
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Links {
    pub _self: Option<String>,
}
