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
//! Response type for the post clusters/nodegroups operation

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use structable::{StructTable, StructTableOptions};

/// Nodegroup response representation
#[derive(Clone, Deserialize, Serialize, StructTable)]
pub struct NodegroupResponse {
    #[structable(optional)]
    pub cluster_id: Option<String>,

    #[structable(optional)]
    pub created_at: Option<String>,

    #[structable(optional)]
    pub docker_volume_size: Option<i32>,

    #[structable(optional)]
    pub flavor_id: Option<String>,

    #[structable(optional)]
    pub id: Option<i32>,

    #[structable(optional)]
    pub image_id: Option<String>,

    #[structable(optional)]
    pub is_default: Option<String>,

    #[structable(optional, serialize)]
    pub labels: Option<HashMap<String, String>>,

    #[structable(optional, serialize)]
    pub labels_added: Option<HashMap<String, String>>,

    #[structable(optional, serialize)]
    pub labels_overridden: Option<HashMap<String, String>>,

    #[structable(optional, serialize)]
    pub labels_skipped: Option<HashMap<String, String>>,

    #[structable(optional, serialize)]
    pub links: Option<Vec<Links>>,

    #[structable(optional)]
    pub max_node_count: Option<i32>,

    #[structable(optional)]
    pub merge_labels: Option<String>,

    #[structable(optional)]
    pub min_node_count: Option<i32>,

    #[structable(optional)]
    pub name: Option<String>,

    #[structable(optional, serialize)]
    pub node_addresses: Option<Vec<String>>,

    #[structable(optional)]
    pub node_count: Option<i32>,

    #[structable(optional)]
    pub project_id: Option<String>,

    #[structable(optional)]
    pub role: Option<String>,

    #[structable(optional)]
    pub stack_id: Option<String>,

    #[structable(optional, serialize)]
    pub status: Option<Status>,

    #[structable(optional)]
    pub status_reason: Option<String>,

    #[structable(optional)]
    pub updated_at: Option<String>,

    #[structable(optional)]
    pub uuid: Option<String>,

    #[structable(optional)]
    pub version: Option<String>,
}

/// A link representation.
///
/// `Links` type
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Links {
    pub created_at: Option<String>,
    pub href: Option<String>,
    pub rel: Option<String>,
    pub _type: Option<String>,
    pub updated_at: Option<String>,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
#[serde(untagged)]
pub enum Status {
    // AdoptComplete
    #[serde(rename = "ADOPT_COMPLETE")]
    AdoptComplete,

    // CheckComplete
    #[serde(rename = "CHECK_COMPLETE")]
    CheckComplete,

    // CreateComplete
    #[serde(rename = "CREATE_COMPLETE")]
    CreateComplete,

    // CreateFailed
    #[serde(rename = "CREATE_FAILED")]
    CreateFailed,

    // CreateInProgress
    #[serde(rename = "CREATE_IN_PROGRESS")]
    CreateInProgress,

    // DeleteComplete
    #[serde(rename = "DELETE_COMPLETE")]
    DeleteComplete,

    // DeleteFailed
    #[serde(rename = "DELETE_FAILED")]
    DeleteFailed,

    // DeleteInProgress
    #[serde(rename = "DELETE_IN_PROGRESS")]
    DeleteInProgress,

    // RestoreComplete
    #[serde(rename = "RESTORE_COMPLETE")]
    RestoreComplete,

    // ResumeComplete
    #[serde(rename = "RESUME_COMPLETE")]
    ResumeComplete,

    // ResumeFailed
    #[serde(rename = "RESUME_FAILED")]
    ResumeFailed,

    // RollbackComplete
    #[serde(rename = "ROLLBACK_COMPLETE")]
    RollbackComplete,

    // RollbackFailed
    #[serde(rename = "ROLLBACK_FAILED")]
    RollbackFailed,

    // RollbackInProgress
    #[serde(rename = "ROLLBACK_IN_PROGRESS")]
    RollbackInProgress,

    // SnapshotComplete
    #[serde(rename = "SNAPSHOT_COMPLETE")]
    SnapshotComplete,

    // UpdateComplete
    #[serde(rename = "UPDATE_COMPLETE")]
    UpdateComplete,

    // UpdateFailed
    #[serde(rename = "UPDATE_FAILED")]
    UpdateFailed,

    // UpdateInProgress
    #[serde(rename = "UPDATE_IN_PROGRESS")]
    UpdateInProgress,
}
