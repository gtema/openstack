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
//! Response type for the post manageable_snapshots operation

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// ManageableSnapshot response representation
#[derive(Clone, Deserialize, Serialize)]
struct ManageableSnapshotResponse {
    /// Whether this resource consumes quota or not. Resources that not counted
    /// for quota usage are usually temporary internal resources created to
    /// perform an operation.
    ///
    /// **New in version 3.65**
    ///
    consumes_quota: Option<bool>,

    /// The total count of requested resource before pagination is applied.
    ///
    count: Option<i32>,

    /// The date and time when the resource was created.
    ///
    /// The date and time stamp format is
    /// [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601):
    ///
    /// ```text
    /// CCYY-MM-DDThh:mm:ss±hh:mm
    ///
    /// ```
    ///
    /// For example, `2015-08-27T09:49:58-05:00`.
    ///
    /// The `±hh:mm` value, if included, is the time zone as an offset from
    /// UTC.
    ///
    created_at: String,

    /// A description for the snapshot.
    ///
    description: Option<String>,

    /// The ID of the group snapshot.
    ///
    /// **New in version 3.14**
    ///
    group_snapshot_id: Option<String>,

    /// The UUID of the object.
    ///
    id: String,

    /// One or more metadata key and value pairs for the snapshot.
    ///
    metadata: Option<HashMap<String, String>>,

    /// The name of the snapshot. Default is `None`.
    ///
    name: Option<String>,

    /// A percentage value for the build progress.
    ///
    #[serde(rename = "os-extended-snapshot-attributes:progress")]
    os_extended_snapshot_attributes_progress: Option<String>,

    /// The UUID of the owning project.
    ///
    #[serde(rename = "os-extended-snapshot-attributes:project_id")]
    os_extended_snapshot_attributes_project_id: Option<String>,

    /// The size of the volume, in gibibytes (GiB).
    ///
    size: i64,

    /// The status for the snapshot.
    ///
    status: Status,

    /// The date and time when the resource was updated.
    ///
    /// The date and time stamp format is
    /// [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601):
    ///
    /// ```text
    /// CCYY-MM-DDThh:mm:ss±hh:mm
    ///
    /// ```
    ///
    /// For example, `2015-08-27T09:49:58-05:00`.
    ///
    /// The `±hh:mm` value, if included, is the time zone as an offset from
    /// UTC. In the previous example, the offset value is `-05:00`.
    ///
    /// If the `updated_at` date and time stamp is not set, its value is
    /// `null`.
    ///
    updated_at: Option<String>,

    /// The UUID of the volume.
    ///
    volume_id: Option<String>,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
#[serde(untagged)]
pub enum Status {
    // Creating
    #[serde(rename = "creating")]
    Creating,

    // Unmanaging
    #[serde(rename = "unmanaging")]
    Unmanaging,

    // Error
    #[serde(rename = "error")]
    Error,

    // Deleted
    #[serde(rename = "deleted")]
    Deleted,

    // ErrorDeleting
    #[serde(rename = "error_deleting")]
    ErrorDeleting,

    // Deleting
    #[serde(rename = "deleting")]
    Deleting,

    // Restoring
    #[serde(rename = "restoring")]
    Restoring,

    // BackingUp
    #[serde(rename = "backing-up")]
    BackingUp,

    // Available
    #[serde(rename = "available")]
    Available,
}
