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
//! Response type for the GET `group_snapshots/detail` operation

use serde::{Deserialize, Serialize};
use structable::{StructTable, StructTableOptions};

/// GroupSnapshot response representation
#[derive(Clone, Deserialize, Serialize, StructTable)]
pub struct GroupSnapshotResponse {
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
    #[structable(optional)]
    pub created_at: Option<String>,

    /// The group snapshot description.
    ///
    #[structable(optional, wide)]
    pub description: Option<String>,

    /// The ID of the group.
    ///
    #[structable(optional, wide)]
    pub group_id: Option<String>,

    /// The group type ID.
    ///
    #[structable(optional, wide)]
    pub group_type: Option<String>,

    /// The group type ID.
    ///
    #[structable(optional, wide)]
    pub group_type_id: Option<String>,

    /// The ID of the group snapshot.
    ///
    #[structable()]
    pub id: String,

    /// The group snapshot name.
    ///
    #[structable(optional)]
    pub name: Option<String>,

    /// The UUID of the volume group snapshot project.
    ///
    /// **New in version 3.58**
    ///
    #[structable(optional, wide)]
    pub project_id: Option<String>,

    /// The `status` of the generic group snapshot.
    ///
    #[structable()]
    pub status: String,
}
