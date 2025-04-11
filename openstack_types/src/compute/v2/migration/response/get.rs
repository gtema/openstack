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
//! Response type for the get os-migrations operation

use serde::{Deserialize, Serialize};

/// Migration response representation
#[derive(Clone, Deserialize, Serialize)]
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
    ///
    pub created_at: Option<String>,

    /// The target compute for a migration.
    ///
    pub dest_compute: Option<String>,

    /// The target host for a migration.
    ///
    pub dest_host: Option<String>,

    /// The target node for a migration.
    ///
    pub dest_node: Option<String>,

    /// The ID of the server migration.
    ///
    pub id: Option<i32>,

    /// The UUID of the server.
    ///
    pub instance_uuid: Option<String>,

    /// The type of the server migration. This is one of `live-migration`,
    /// `migration`, `resize` and `evacuation`.
    ///
    /// **New in version 2.23**
    ///
    pub migration_type: Option<MigrationType>,

    /// In `resize` case, the flavor ID for resizing the server. In the other
    /// cases, this parameter is same as the flavor ID of the server when the
    /// migration was started.
    ///
    /// Note
    ///
    /// This is an internal ID and is not exposed in any other API. In
    /// particular, this is not the ID specified or automatically generated
    /// during flavor creation or returned via the `GET /flavors` API.
    ///
    pub new_instance_type_id: Option<i32>,

    /// The flavor ID of the server when the migration was started.
    ///
    /// Note
    ///
    /// This is an internal ID and is not exposed in any other API. In
    /// particular, this is not the ID specified or automatically generated
    /// during flavor creation or returned via the `GET /flavors` API.
    ///
    pub old_instance_type_id: Option<i32>,

    /// The ID of the project which initiated the server migration. The value
    /// may be `null` for older migration records.
    ///
    /// **New in version 2.80**
    ///
    pub project_id: Option<String>,

    /// The source compute for a migration.
    ///
    pub source_compute: Option<String>,

    /// The source node for a migration.
    ///
    pub source_node: Option<String>,

    /// The current status of the migration.
    ///
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
    ///
    pub updated_at: Option<String>,

    /// The ID of the user which initiated the server migration. The value may
    /// be `null` for older migration records.
    ///
    /// **New in version 2.80**
    ///
    pub user_id: Option<String>,

    /// The UUID of the migration.
    ///
    /// **New in version 2.59**
    ///
    pub uuid: Option<String>,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
#[serde(untagged)]
pub enum MigrationType {
    // LiveMigration
    #[serde(rename = "live-migration")]
    LiveMigration,

    // Migration
    #[serde(rename = "migration")]
    Migration,

    // Resize
    #[serde(rename = "resize")]
    Resize,
}
