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
//! Response type for the post ports/{port_id}/bindings operation

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

/// Binding response representation
#[derive(Clone, Deserialize, Serialize)]
pub struct BindingResponse {
    host: Option<String>,

    profile: Option<HashMap<String, Value>>,

    project_id: Option<String>,

    status: Option<String>,

    vif_details: Option<String>,

    vif_type: Option<String>,

    vnic_type: Option<VnicType>,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
#[serde(untagged)]
pub enum VnicType {
    // Baremetal
    #[serde(rename = "baremetal")]
    Baremetal,

    // AcceleratorDirect
    #[serde(rename = "accelerator-direct")]
    AcceleratorDirect,

    // DirectPhysical
    #[serde(rename = "direct-physical")]
    DirectPhysical,

    // Vdpa
    #[serde(rename = "vdpa")]
    Vdpa,

    // Macvtap
    #[serde(rename = "macvtap")]
    Macvtap,

    // SmartNic
    #[serde(rename = "smart-nic")]
    SmartNic,

    // AcceleratorDirectPhysical
    #[serde(rename = "accelerator-direct-physical")]
    AcceleratorDirectPhysical,

    // VirtioForwarder
    #[serde(rename = "virtio-forwarder")]
    VirtioForwarder,

    // Direct
    #[serde(rename = "direct")]
    Direct,

    // RemoteManaged
    #[serde(rename = "remote-managed")]
    RemoteManaged,

    // Normal
    #[serde(rename = "normal")]
    Normal,
}
