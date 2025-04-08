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
//! Response type for the get servers/{server_id}/shares/{id} operation

use serde::{Deserialize, Serialize};

/// Share response representation
#[derive(Clone, Deserialize, Serialize)]
struct ShareResponse {
    /// The export location used to attach the share to the underlying host.
    ///
    export_location: Option<String>,

    /// The UUID of the attached share.
    ///
    share_id: String,

    /// Status of the Share:
    ///
    /// - attaching: The share is being attached to the VM by the compute node.
    /// - detaching: The share is being detached from the VM by the compute
    ///   node.
    /// - inactive: The share is attached but inactive because the VM is
    ///   stopped.
    /// - active: The share is attached, and the VM is running.
    /// - error: The share is in an error state.
    ///
    status: Status,

    /// The device tag to be used by users to mount the share within the
    /// instance, if not provided then the share UUID will be used
    /// automatically.
    ///
    tag: String,

    /// The UUID of the attached share.
    ///
    uuid: Option<String>,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
#[serde(untagged)]
pub enum Status {
    // Inactive
    #[serde(rename = "inactive")]
    Inactive,

    // Active
    #[serde(rename = "active")]
    Active,

    // Error
    #[serde(rename = "error")]
    Error,

    // Detaching
    #[serde(rename = "detaching")]
    Detaching,

    // Attaching
    #[serde(rename = "attaching")]
    Attaching,
}
