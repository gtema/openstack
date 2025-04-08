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
//! Response type for the get groups/{id} operation

use serde::{Deserialize, Serialize};

/// Group response representation
#[derive(Clone, Deserialize, Serialize)]
pub struct GroupResponse {
    /// The name of the availability zone.
    ///
    pub availability_zone: Option<String>,

    /// The date and time when the resource was created.
    ///
    pub created_at: Option<String>,

    /// The group description.
    ///
    pub description: Option<String>,

    /// The ID of the group snapshot.
    ///
    pub group_snapshot_id: Option<String>,

    /// The group type ID.
    ///
    pub group_type: Option<String>,

    /// The UUID of the group.
    ///
    pub id: String,

    /// The group name.
    ///
    pub name: Option<String>,

    /// The UUID of the volume group project.
    ///
    pub project_id: Option<String>,

    /// The group replication status.
    ///
    pub replication_status: Option<String>,

    /// The UUID of the source group.
    ///
    pub source_group_id: Option<String>,

    /// The status of the generic group.
    ///
    pub status: Option<String>,

    /// The list of volume types. In an environment with multiple-storage back
    /// ends, the scheduler determines where to send the volume based on the
    /// volume type.
    ///
    pub volume_types: Option<Vec<String>>,

    /// A list of volume ids, available only when list_volume set true.
    ///
    pub volumes: Option<Vec<String>>,
}
