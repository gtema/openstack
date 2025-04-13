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
//! Response type for the post servers/{id}/action operation

use serde::{Deserialize, Serialize};
use structable::{StructTable, StructTableOptions};

/// Server response representation
#[derive(Clone, Deserialize, Serialize, StructTable)]
pub struct ServerResponse {
    /// The type of the remote console
    ///
    #[serde(rename = "type")]
    #[structable(serialize, title = "type")]
    pub _type: Type,

    /// The URL used to connect to the console.
    ///
    #[structable()]
    pub url: String,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
#[serde(untagged)]
pub enum Type {
    // Novnc
    #[serde(rename = "novnc")]
    Novnc,

    // RdpHtml5
    #[serde(rename = "rdp-html5")]
    RdpHtml5,

    // Serial
    #[serde(rename = "serial")]
    Serial,

    // SpiceHtml5
    #[serde(rename = "spice-html5")]
    SpiceHtml5,
}
