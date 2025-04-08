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
//! Response type for the post backups operation

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Backup response representation
#[derive(Clone, Deserialize, Serialize)]
struct BackupResponse {
    /// The name of the availability zone.
    ///
    availability_zone: Option<String>,

    /// The container name or null.
    ///
    container: Option<String>,

    /// The date and time when the resource was created. The date and time
    /// stamp format is ISO 8601
    ///
    created_at: Option<String>,

    /// The time when the data on the volume was first saved. If it is a backup
    /// from volume, it will be the same as created_at for a backup. If it is a
    /// backup from a snapshot, it will be the same as created_at for the
    /// snapshot.
    ///
    data_timestamp: Option<String>,

    /// The backup description or null.
    ///
    description: Option<String>,

    /// If the backup failed, the reason for the failure. Otherwise, null.
    ///
    fail_reason: Option<String>,

    /// If this value is true, there are other backups depending on this
    /// backup.
    ///
    has_dependent_backups: Option<bool>,

    /// The UUID of the backup.
    ///
    id: String,

    /// Indicates whether the backup mode is incremental. If this value is
    /// true, the backup mode is incremental. If this value is false, the
    /// backup mode is full.
    ///
    is_incremental: Option<bool>,

    /// Links for the backup.
    ///
    links: Option<Vec<Links>>,

    /// The backup metadata key value pairs.
    ///
    /// **New in version 3.43**
    ///
    metadata: Option<HashMap<String, String>>,

    /// The backup name.
    ///
    name: Option<String>,

    /// The number of objects in the backup.
    ///
    object_count: Option<i32>,

    /// The size of the volume, in gibibytes (GiB).
    ///
    size: i64,

    /// The UUID of the source volume snapshot.
    ///
    snapshot_id: Option<String>,

    /// The backup status. Refer to Backup statuses table for the possible
    /// status value.
    ///
    status: String,

    /// The date and time when the resource was updated. The date and time
    /// stamp format is ISO 8601
    ///
    updated_at: Option<String>,

    /// The UUID of the volume.
    ///
    volume_id: String,
}

/// Links to the resources in question. See
/// [API Guide / Links and References](https://docs.openstack.org/api-guide/compute/links_and_references.html)
/// for more info.
///
/// `Links` type
#[derive(Clone, Debug, Deserialize, Serialize)]
struct Links {
    href: Option<String>,
    rel: Option<String>,
}
