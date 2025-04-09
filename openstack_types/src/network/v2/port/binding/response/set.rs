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
//! Response type for the put ports/{port_id}/bindings/{id} operation

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

/// Binding response representation
#[derive(Clone, Deserialize, Serialize)]
pub struct BindingResponse {
    pub host: Option<String>,

    pub profile: Option<HashMap<String, Value>>,

    pub project_id: Option<String>,

    pub status: Option<String>,

    pub vif_details: Option<String>,

    pub vif_type: Option<String>,

    pub vnic_type: Option<VnicType>,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
#[serde(untagged)]
pub enum VnicType {
    // AcceleratorDirectPhysical
    #[serde(rename = "accelerator-direct-physical")]
    AcceleratorDirectPhysical,

    // AcceleratorDirect
    #[serde(rename = "accelerator-direct")]
    AcceleratorDirect,

    // DirectPhysical
    #[serde(rename = "direct-physical")]
    DirectPhysical,

    // Direct
    #[serde(rename = "direct")]
    Direct,

    // VirtioForwarder
    #[serde(rename = "virtio-forwarder")]
    VirtioForwarder,

    // Baremetal
    #[serde(rename = "baremetal")]
    Baremetal,

    // Macvtap
    #[serde(rename = "macvtap")]
    Macvtap,

    // RemoteManaged
    #[serde(rename = "remote-managed")]
    RemoteManaged,

    // Vdpa
    #[serde(rename = "vdpa")]
    Vdpa,

    // Normal
    #[serde(rename = "normal")]
    Normal,

    // SmartNic
    #[serde(rename = "smart-nic")]
    SmartNic,
}
