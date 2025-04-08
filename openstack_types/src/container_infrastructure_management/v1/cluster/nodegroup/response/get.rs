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
//! Response type for the get clusters/nodegroups/{nodegroup_id} operation

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Nodegroup response representation
#[derive(Clone, Deserialize, Serialize)]
pub struct NodegroupResponse {
    cluster_id: Option<String>,

    created_at: Option<String>,

    docker_volume_size: Option<i32>,

    flavor_id: Option<String>,

    id: Option<i32>,

    image_id: Option<String>,

    is_default: Option<String>,

    labels: Option<HashMap<String, String>>,

    labels_added: Option<HashMap<String, String>>,

    labels_overridden: Option<HashMap<String, String>>,

    labels_skipped: Option<HashMap<String, String>>,

    links: Option<Vec<Links>>,

    max_node_count: Option<i32>,

    merge_labels: Option<String>,

    min_node_count: Option<i32>,

    name: Option<String>,

    node_addresses: Option<Vec<String>>,

    node_count: Option<i32>,

    project_id: Option<String>,

    role: Option<String>,

    stack_id: Option<String>,

    status: Option<Status>,

    status_reason: Option<String>,

    updated_at: Option<String>,

    uuid: Option<String>,

    version: Option<String>,
}

/// A link representation.
///
/// `Links` type
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Links {
    created_at: Option<String>,
    href: Option<String>,
    rel: Option<String>,
    _type: Option<String>,
    updated_at: Option<String>,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
#[serde(untagged)]
pub enum Status {
    // UpdateComplete
    #[serde(rename = "UPDATE_COMPLETE")]
    UpdateComplete,

    // CreateComplete
    #[serde(rename = "CREATE_COMPLETE")]
    CreateComplete,

    // RestoreComplete
    #[serde(rename = "RESTORE_COMPLETE")]
    RestoreComplete,

    // DeleteInProgress
    #[serde(rename = "DELETE_IN_PROGRESS")]
    DeleteInProgress,

    // CreateFailed
    #[serde(rename = "CREATE_FAILED")]
    CreateFailed,

    // UpdateFailed
    #[serde(rename = "UPDATE_FAILED")]
    UpdateFailed,

    // SnapshotComplete
    #[serde(rename = "SNAPSHOT_COMPLETE")]
    SnapshotComplete,

    // ResumeComplete
    #[serde(rename = "RESUME_COMPLETE")]
    ResumeComplete,

    // AdoptComplete
    #[serde(rename = "ADOPT_COMPLETE")]
    AdoptComplete,

    // DeleteFailed
    #[serde(rename = "DELETE_FAILED")]
    DeleteFailed,

    // RollbackFailed
    #[serde(rename = "ROLLBACK_FAILED")]
    RollbackFailed,

    // ResumeFailed
    #[serde(rename = "RESUME_FAILED")]
    ResumeFailed,

    // UpdateInProgress
    #[serde(rename = "UPDATE_IN_PROGRESS")]
    UpdateInProgress,

    // DeleteComplete
    #[serde(rename = "DELETE_COMPLETE")]
    DeleteComplete,

    // RollbackInProgress
    #[serde(rename = "ROLLBACK_IN_PROGRESS")]
    RollbackInProgress,

    // CheckComplete
    #[serde(rename = "CHECK_COMPLETE")]
    CheckComplete,

    // RollbackComplete
    #[serde(rename = "ROLLBACK_COMPLETE")]
    RollbackComplete,

    // CreateInProgress
    #[serde(rename = "CREATE_IN_PROGRESS")]
    CreateInProgress,
}
