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
//! Response type for the get servers/{server_id}/os-interface operation

use serde::{Deserialize, Serialize};
use structable::{StructTable, StructTableOptions};

/// Interface response representation
#[derive(Clone, Deserialize, Serialize, StructTable)]
pub struct InterfaceResponse {
    /// List of the interface attachments.
    ///
    #[serde(rename = "interfaceAttachments")]
    #[structable(serialize, title = "interfaceAttachments", wide)]
    pub interface_attachments: Vec<InterfaceAttachments>,
}

/// `FixedIps` type
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct FixedIps {
    pub ip_address: Option<String>,
    pub subnet_id: Option<String>,
}

/// The interface attachment.
///
/// `InterfaceAttachments` type
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct InterfaceAttachments {
    pub fixed_ips: Vec<FixedIps>,
    pub mac_addr: Option<String>,
    pub net_id: Option<String>,
    pub port_id: Option<String>,
    pub port_state: Option<String>,
    pub tag: Option<String>,
}
