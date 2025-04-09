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
//! Response type for the get group_snapshots/{id} operation

use serde::{Deserialize, Serialize};

/// GroupSnapshot response representation
#[derive(Clone, Deserialize, Serialize)]
pub struct GroupSnapshotResponse {
    /// The date and time when the resource was created.
    ///
    pub created_at: Option<String>,

    /// The group snapshot description.
    ///
    pub description: Option<String>,

    /// The ID of the group.
    ///
    pub group_id: Option<String>,

    /// The group type ID.
    ///
    pub group_type: Option<String>,

    /// The group type ID.
    ///
    pub group_type_id: Option<String>,

    /// The ID of the group snapshot.
    ///
    pub id: String,

    /// The group snapshot name.
    ///
    pub name: Option<String>,

    /// The UUID of the volume group project.
    ///
    pub project_id: Option<String>,

    /// The status of the generic group snapshot.
    ///
    pub status: String,
}
