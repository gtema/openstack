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
//! Response type for the GET `volume-transfers/detail` operation

use serde::{Deserialize, Serialize};
use structable::{StructTable, StructTableOptions};

/// VolumeTransfer response representation
#[derive(Clone, Deserialize, Serialize, StructTable)]
pub struct VolumeTransferResponse {
    /// Records if this transfer was accepted or not.
    ///
    /// **New in version 3.57**
    #[serde(default)]
    #[structable(optional, wide)]
    pub accepted: Option<bool>,

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
    #[serde(default)]
    #[structable(optional)]
    pub created_at: Option<String>,

    /// Records the destination project_id after volume transfer.
    ///
    /// **New in version 3.57**
    #[serde(default)]
    #[structable(optional, wide)]
    pub destination_project_id: Option<String>,

    /// The UUID of the object.
    #[serde(default)]
    #[structable(optional)]
    pub id: Option<String>,

    /// The name of the object.
    #[serde(default)]
    #[structable(optional)]
    pub name: Option<String>,

    /// Transfer volume without snapshots. Defaults to False if not specified.
    ///
    /// **New in version 3.55**
    #[serde(default)]
    #[structable(optional, wide)]
    pub no_snapshots: Option<bool>,

    /// Records the source project_id before volume transfer.
    ///
    /// **New in version 3.57**
    #[serde(default)]
    #[structable(optional, wide)]
    pub source_project_id: Option<String>,

    /// The UUID of the volume.
    #[serde(default)]
    #[structable(optional, wide)]
    pub volume_id: Option<String>,
}

/// Links to the resources in question. See
/// [API Guide / Links and References](https://docs.openstack.org/api-guide/compute/links_and_references.html)
/// for more info.
/// `Links` type
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Links {
    #[serde(default)]
    pub href: Option<String>,
    #[serde(default)]
    pub rel: Option<String>,
}
