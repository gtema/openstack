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
//! Response type for the get servers/{server_id}/os-interface/{id} operation

use serde::{Deserialize, Serialize};

/// Interface response representation
#[derive(Clone, Deserialize, Serialize)]
pub struct InterfaceResponse {
    /// Fixed IP addresses with subnet IDs.
    ///
    pub fixed_ips: Vec<FixedIps>,

    /// The MAC address.
    ///
    pub mac_addr: Option<String>,

    /// The network ID.
    ///
    pub net_id: Option<String>,

    /// The port ID.
    ///
    pub port_id: Option<String>,

    /// The port state.
    ///
    pub port_state: Option<String>,

    /// The device tag applied to the virtual network interface or `null`.
    ///
    /// **New in version 2.70**
    ///
    pub tag: Option<String>,
}

/// `FixedIps` type
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct FixedIps {
    pub ip_address: Option<String>,
    pub subnet_id: Option<String>,
}
