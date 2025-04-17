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
//! Response type for the GET `servers/{server_id}/migrations/{id}` operation

use serde::{Deserialize, Serialize};
use structable::{StructTable, StructTableOptions};

/// Migration response representation
#[derive(Clone, Deserialize, Serialize, StructTable)]
pub struct MigrationResponse {
    /// The date and time when the resource was created. The date and time
    /// stamp format is [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601)
    ///
    /// ```text
    /// CCYY-MM-DDThh:mm:ss±hh:mm
    ///
    /// ```
    ///
    /// For example, `2015-08-27T09:49:58-05:00`. The `±hh:mm` value, if
    /// included, is the time zone as an offset from UTC. In the previous
    /// example, the offset value is `-05:00`.
    #[serde(default)]
    #[structable(optional)]
    pub created_at: Option<String>,

    /// The target compute for a migration.
    #[serde(default)]
    #[structable(optional)]
    pub dest_compute: Option<String>,

    /// The target host for a migration.
    #[serde(default)]
    #[structable(optional)]
    pub dest_host: Option<String>,

    /// The target node for a migration.
    #[serde(default)]
    #[structable(optional)]
    pub dest_node: Option<String>,

    /// The amount of disk, in bytes, that has been processed during the
    /// migration.
    #[serde(default)]
    #[structable(optional)]
    pub disk_processed_bytes: Option<i32>,

    /// The amount of disk, in bytes, that still needs to be migrated.
    #[serde(default)]
    #[structable(optional)]
    pub disk_remaining_bytes: Option<i32>,

    /// The total amount of disk, in bytes, that needs to be migrated.
    #[serde(default)]
    #[structable(optional)]
    pub disk_total_bytes: Option<i32>,

    /// The ID of the server migration.
    #[serde(default)]
    #[structable(optional)]
    pub id: Option<i32>,

    /// The amount of memory, in bytes, that has been processed during the
    /// migration.
    #[serde(default)]
    #[structable(optional)]
    pub memory_processed_bytes: Option<i32>,

    /// The amount of memory, in bytes, that still needs to be migrated.
    #[serde(default)]
    #[structable(optional)]
    pub memory_remaining_bytes: Option<i32>,

    /// The total amount of memory, in bytes, that needs to be migrated.
    #[serde(default)]
    #[structable(optional)]
    pub memory_total_bytes: Option<i32>,

    /// The ID of the project which initiated the server migration. The value
    /// may be `null` for older migration records.
    ///
    /// **New in version 2.80**
    #[serde(default)]
    #[structable(optional)]
    pub project_id: Option<String>,

    /// The UUID of the server.
    #[serde(default)]
    #[structable(optional)]
    pub server_uuid: Option<String>,

    /// The source compute for a migration.
    #[serde(default)]
    #[structable(optional)]
    pub source_compute: Option<String>,

    /// The source node for a migration.
    #[serde(default)]
    #[structable(optional)]
    pub source_node: Option<String>,

    /// The current status of the migration.
    #[serde(default)]
    #[structable(optional)]
    pub status: Option<String>,

    /// The date and time when the resource was updated. The date and time
    /// stamp format is [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601)
    ///
    /// ```text
    /// CCYY-MM-DDThh:mm:ss±hh:mm
    ///
    /// ```
    ///
    /// For example, `2015-08-27T09:49:58-05:00`. The `±hh:mm` value, if
    /// included, is the time zone as an offset from UTC. In the previous
    /// example, the offset value is `-05:00`.
    #[serde(default)]
    #[structable(optional)]
    pub updated_at: Option<String>,

    /// The ID of the user which initiated the server migration. The value may
    /// be `null` for older migration records.
    ///
    /// **New in version 2.80**
    #[serde(default)]
    #[structable(optional)]
    pub user_id: Option<String>,

    /// The UUID of the migration.
    ///
    /// **New in version 2.59**
    #[serde(default)]
    #[structable(optional)]
    pub uuid: Option<String>,
}
