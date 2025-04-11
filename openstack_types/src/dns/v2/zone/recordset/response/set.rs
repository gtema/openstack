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
//! Response type for the put zones/{zone_id}/recordsets/{recordset_id} operation

use serde::{Deserialize, Serialize};
use structable_derive::StructTable;

use crate::common::{OutputConfig, StructTable};

/// Recordset response representation
#[derive(Clone, Deserialize, Serialize, StructTable)]
pub struct RecordsetResponse {
    /// current action in progress on the resource
    ///
    #[structable(optional, serialize)]
    pub action: Option<Action>,

    /// Date / Time when resource was created.
    ///
    #[structable(optional)]
    pub created_at: Option<String>,

    /// Description for this recordset
    ///
    #[structable(optional)]
    pub description: Option<String>,

    /// ID for the resource
    ///
    #[structable(optional)]
    pub id: Option<String>,

    /// Links to the resource, and other related resources. When a response has
    /// been broken into pages, we will include a `next` link that should be
    /// followed to retrieve all results
    ///
    #[structable(optional, serialize)]
    pub links: Option<Links>,

    /// DNS Name for the recordset
    ///
    #[structable(optional)]
    pub name: Option<String>,

    /// ID for the project that owns the resource
    ///
    #[structable(optional)]
    pub project_id: Option<String>,

    /// A list of data for this recordset. Each item will be a separate record
    /// in Designate These items should conform to the DNS spec for the record
    /// type - e.g. A records must be IPv4 addresses, CNAME records must be a
    /// hostname.
    ///
    #[structable(optional, serialize)]
    pub records: Option<Vec<String>>,

    /// The status of the resource.
    ///
    #[structable(optional, serialize)]
    pub status: Option<Status>,

    /// TTL (Time to Live) for the recordset.
    ///
    #[structable(optional)]
    pub ttl: Option<i32>,

    /// They RRTYPE of the recordset.
    ///
    #[serde(rename = "type")]
    #[structable(optional, serialize, title = "type")]
    pub _type: Option<Type>,

    /// Date / Time when resource last updated.
    ///
    #[structable(optional, serialize)]
    pub updated_at: Option<String>,

    /// Version of the resource
    ///
    #[structable(optional)]
    pub version: Option<i32>,

    /// ID for the zone that contains this recordset
    ///
    #[structable(optional)]
    pub zone_id: Option<String>,

    /// The name of the zone that contains this recordset
    ///
    #[structable(optional)]
    pub zone_name: Option<String>,
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
    // A
    #[serde(rename = "A")]
    A,

    // Aaaa
    #[serde(rename = "AAAA")]
    Aaaa,

    // Caa
    #[serde(rename = "CAA")]
    Caa,

    // Cert
    #[serde(rename = "CERT")]
    Cert,

    // Cname
    #[serde(rename = "CNAME")]
    Cname,

    // Mx
    #[serde(rename = "MX")]
    Mx,

    // Naptr
    #[serde(rename = "NAPTR")]
    Naptr,

    // Ns
    #[serde(rename = "NS")]
    Ns,

    // Ptr
    #[serde(rename = "PTR")]
    Ptr,

    // Soa
    #[serde(rename = "SOA")]
    Soa,

    // Spf
    #[serde(rename = "SPF")]
    Spf,

    // Srv
    #[serde(rename = "SRV")]
    Srv,

    // Sshfp
    #[serde(rename = "SSHFP")]
    Sshfp,

    // Txt
    #[serde(rename = "TXT")]
    Txt,
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
