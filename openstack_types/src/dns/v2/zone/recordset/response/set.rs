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

/// Recordset response representation
#[derive(Clone, Deserialize, Serialize)]
pub struct RecordsetResponse {
    /// current action in progress on the resource
    ///
    pub action: Option<Action>,

    /// Date / Time when resource was created.
    ///
    pub created_at: Option<String>,

    /// Description for this recordset
    ///
    pub description: Option<String>,

    /// ID for the resource
    ///
    pub id: Option<String>,

    /// Links to the resource, and other related resources. When a response has
    /// been broken into pages, we will include a `next` link that should be
    /// followed to retrieve all results
    ///
    pub links: Option<Links>,

    /// DNS Name for the recordset
    ///
    pub name: Option<String>,

    /// ID for the project that owns the resource
    ///
    pub project_id: Option<String>,

    /// A list of data for this recordset. Each item will be a separate record
    /// in Designate These items should conform to the DNS spec for the record
    /// type - e.g. A records must be IPv4 addresses, CNAME records must be a
    /// hostname.
    ///
    pub records: Option<Vec<String>>,

    /// The status of the resource.
    ///
    pub status: Option<Status>,

    /// TTL (Time to Live) for the recordset.
    ///
    pub ttl: Option<i32>,

    /// They RRTYPE of the recordset.
    ///
    #[serde(rename = "type")]
    pub _type: Option<Type>,

    /// Date / Time when resource last updated.
    ///
    pub updated_at: Option<String>,

    /// Version of the resource
    ///
    pub version: Option<i32>,

    /// ID for the zone that contains this recordset
    ///
    pub zone_id: Option<String>,

    /// The name of the zone that contains this recordset
    ///
    pub zone_name: Option<String>,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
#[serde(untagged)]
pub enum Status {
    // Active
    #[serde(rename = "ACTIVE")]
    Active,

    // Success
    #[serde(rename = "SUCCESS")]
    Success,

    // Error
    #[serde(rename = "ERROR")]
    Error,

    // Pending
    #[serde(rename = "PENDING")]
    Pending,

    // Deleted
    #[serde(rename = "DELETED")]
    Deleted,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
#[serde(untagged)]
pub enum Action {
    // None
    #[serde(rename = "NONE")]
    None,

    // Create
    #[serde(rename = "CREATE")]
    Create,

    // Update
    #[serde(rename = "UPDATE")]
    Update,

    // Delete
    #[serde(rename = "DELETE")]
    Delete,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
#[serde(untagged)]
pub enum Type {
    // Cname
    #[serde(rename = "CNAME")]
    Cname,

    // Spf
    #[serde(rename = "SPF")]
    Spf,

    // Naptr
    #[serde(rename = "NAPTR")]
    Naptr,

    // Ns
    #[serde(rename = "NS")]
    Ns,

    // Txt
    #[serde(rename = "TXT")]
    Txt,

    // Srv
    #[serde(rename = "SRV")]
    Srv,

    // Sshfp
    #[serde(rename = "SSHFP")]
    Sshfp,

    // Cert
    #[serde(rename = "CERT")]
    Cert,

    // Aaaa
    #[serde(rename = "AAAA")]
    Aaaa,

    // Ptr
    #[serde(rename = "PTR")]
    Ptr,

    // Soa
    #[serde(rename = "SOA")]
    Soa,

    // Caa
    #[serde(rename = "CAA")]
    Caa,

    // A
    #[serde(rename = "A")]
    A,

    // Mx
    #[serde(rename = "MX")]
    Mx,
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
