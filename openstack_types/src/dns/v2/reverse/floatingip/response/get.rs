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
//! Response type for the get reverse/floatingips/{fip_key} operation

use serde::{Deserialize, Serialize};

/// Floatingip response representation
#[derive(Clone, Deserialize, Serialize)]
struct FloatingipResponse {
    /// current action in progress on the resource
    ///
    action: Option<Action>,

    /// The floatingip address for this PTR record.
    ///
    address: Option<String>,

    /// Description for this PTR record
    ///
    description: Option<String>,

    /// ID for PTR record in the format of <region>:\<floatingip_id>
    ///
    id: Option<String>,

    /// Links to the resource, and other related resources. When a response has
    /// been broken into pages, we will include a `next` link that should be
    /// followed to retrieve all results
    ///
    links: Option<Links>,

    /// Domain name for this PTR record
    ///
    ptrdname: Option<String>,

    /// The status of the resource.
    ///
    status: Option<Status>,

    /// Time to live for this PTR record
    ///
    ttl: Option<i32>,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
#[serde(untagged)]
pub enum Status {
    // Active
    #[serde(rename = "ACTIVE")]
    Active,

    // Pending
    #[serde(rename = "PENDING")]
    Pending,

    // Deleted
    #[serde(rename = "DELETED")]
    Deleted,

    // Success
    #[serde(rename = "SUCCESS")]
    Success,

    // Error
    #[serde(rename = "ERROR")]
    Error,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
#[serde(untagged)]
pub enum Action {
    // Delete
    #[serde(rename = "DELETE")]
    Delete,

    // Update
    #[serde(rename = "UPDATE")]
    Update,

    // None
    #[serde(rename = "NONE")]
    None,
}

/// Links to the resource, and other related resources. When a response has
/// been broken into pages, we will include a `next` link that should be
/// followed to retrieve all results
///
/// `Links` type
#[derive(Clone, Debug, Deserialize, Serialize)]
struct Links {
    _self: Option<String>,
}
