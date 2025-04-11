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
//! Response type for the get os-hypervisors operation

use serde::{Deserialize, Serialize};

/// Hypervisor response representation
#[derive(Clone, Deserialize, Serialize)]
pub struct HypervisorResponse {
    /// The hypervisor host name provided by the Nova virt driver. For the
    /// Ironic driver, it is the Ironic node uuid.
    ///
    pub hypervisor_hostname: String,

    /// The id of the hypervisor as a UUID.
    ///
    /// **New in version 2.53**
    ///
    pub id: String,

    /// A list of `server` objects. This field has become mandatory in
    /// microversion 2.75. If no servers is on hypervisor then empty list is
    /// returned.
    ///
    /// **New in version 2.53**
    ///
    pub servers: Vec<Servers>,

    /// The state of the hypervisor. One of `up` or `down`.
    ///
    pub state: State,

    /// The status of the hypervisor. One of `enabled` or `disabled`.
    ///
    pub status: Option<Status>,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
#[serde(untagged)]
pub enum State {
    // Down
    #[serde(rename = "down")]
    Down,

    // Up
    #[serde(rename = "up")]
    Up,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
#[serde(untagged)]
pub enum Status {
    // Disabled
    #[serde(rename = "disabled")]
    Disabled,

    // Enabled
    #[serde(rename = "enabled")]
    Enabled,
}

/// `Servers` type
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Servers {
    pub name: String,
    pub uuid: String,
}
