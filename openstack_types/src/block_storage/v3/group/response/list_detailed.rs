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
//! Response type for the get groups/detail operation

use serde::{Deserialize, Serialize};

/// Group response representation
#[derive(Clone, Deserialize, Serialize)]
pub struct GroupResponse {
    /// The name of the availability zone.
    ///
    availability_zone: Option<String>,

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
    created_at: Option<String>,

    /// The group description.
    ///
    description: Option<String>,

    /// The ID of the group snapshot.
    ///
    group_snapshot_id: Option<String>,

    /// The group type ID.
    ///
    group_type: Option<String>,

    /// The UUID of the group.
    ///
    id: String,

    /// The name of the object.
    ///
    name: Option<String>,

    /// The UUID of the volume group project.
    ///
    /// **New in version 3.58**
    ///
    project_id: Option<String>,

    /// The group replication status.
    ///
    /// **New in version 3.38**
    ///
    replication_status: Option<String>,

    /// The UUID of the source group.
    ///
    source_group_id: Option<String>,

    /// The status of the generic group.
    ///
    status: Option<String>,

    /// The list of volume types. In an environment with multiple-storage back
    /// ends, the scheduler determines where to send the volume based on the
    /// volume type. For information about how to use volume types to create
    /// multiple- storage back ends, see
    /// [Configure multiple-storage back ends](https://docs.openstack.org/cinder/latest/admin/blockstorage-multi-backend.html).
    ///
    volume_types: Option<Vec<String>>,

    /// A list of `volume` ids, available only when `list_volume` set true.
    ///
    /// **New in version 3.25**
    ///
    volumes: Option<Vec<String>>,
}
