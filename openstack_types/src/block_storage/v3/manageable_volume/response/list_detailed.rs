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
//! Response type for the GET `manageable_volumes/detail` operation

use serde::{Deserialize, Serialize};
use structable::{StructTable, StructTableOptions};

/// ManageableVolume response representation
#[derive(Clone, Deserialize, Serialize, StructTable)]
pub struct ManageableVolumeResponse {
    /// A list of manageable volumes.
    ///
    #[serde(rename = "manageable-volumes")]
    #[structable(serialize, title = "manageable-volumes", wide)]
    pub manageable_volumes: Vec<ManageableVolumes>,
}

/// Some information for the resource.
///
/// `Reference` type
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Reference {
    pub source_name: Option<String>,
}

/// Manageable volume object.
///
/// `ManageableVolumes` type
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ManageableVolumes {
    pub cinder_id: Option<String>,
    pub extra_info: Option<String>,
    pub reason_not_safe: Option<String>,
    pub reference: Option<Reference>,
    pub safe_to_manage: Option<bool>,
    pub size: Option<i64>,
}
