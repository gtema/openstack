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
//! Response type for the get manageable_snapshots/detail operation

use serde::{Deserialize, Serialize};
use structable_derive::StructTable;

use crate::common::{OutputConfig, StructTable};

/// ManageableSnapshot response representation
#[derive(Clone, Deserialize, Serialize, StructTable)]
pub struct ManageableSnapshotResponse {
    /// A list of manageable snapshots.
    ///
    #[serde(rename = "manageable-snapshots")]
    #[structable(serialize, title = "manageable-snapshots", wide)]
    pub manageable_snapshots: Vec<ManageableSnapshots>,
}

/// The snapshot’s origin volume information.
///
/// `SourceReference` type
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SourceReference {
    pub source_name: Option<String>,
}

/// Some information for the resource.
///
/// `Reference` type
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Reference {
    pub source_name: Option<String>,
}

/// `ManageableSnapshots` type
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ManageableSnapshots {
    pub cinder_id: Option<String>,
    pub extra_info: Option<String>,
    pub reason_not_safe: Option<String>,
    pub reference: Option<Reference>,
    pub safe_to_manage: Option<bool>,
    pub size: Option<i64>,
    pub source_reference: Option<SourceReference>,
}
