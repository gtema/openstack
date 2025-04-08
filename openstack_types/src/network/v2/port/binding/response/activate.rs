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
//! Response type for the put ports/{port_id}/bindings/{id}/activate operation

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

/// Binding response representation
#[derive(Clone, Deserialize, Serialize)]
struct BindingResponse {
    /// The hostname of the system the agent is running on.
    ///
    host: Option<String>,

    /// A dictionary that enables the application running on the specific host
    /// to pass and receive vif port information specific to the networking
    /// back-end. The networking API does not define a specific format of this
    /// field. If the update request is null this response field will be {}.
    ///
    profile: Option<HashMap<String, Value>>,

    project_id: Option<String>,

    status: Option<String>,

    /// A dictionary which contains additional information on the port.
    /// Currently the following fields are defined: `port_filter` and
    /// `ovs_hybrid_plug`. `port_filter` is a boolean indicating the networking
    /// service provides port filtering features such as security group and/or
    /// anti MAC/IP spoofing. `ovs_hybrid_plug` is a boolean used to inform an
    /// API consumer like nova that the hybrid plugging strategy for OVS should
    /// be used.
    ///
    vif_details: Option<String>,

    /// The type of which mechanism is used for the port. An API consumer like
    /// nova can use this to determine an appropriate way to attach a device
    /// (for example an interface of a virtual server) to the port. Available
    /// values currently defined includes `ovs`, `bridge`, `macvtap`, `hw_veb`,
    /// `hostdev_physical`, `vhostuser`, `distributed` and `other`. There are
    /// also special values: `unbound` and `binding_failed`. `unbound` means
    /// the port is not bound to a networking back-end. `binding_failed` means
    /// an error that the port failed to be bound to a networking back-end.
    ///
    vif_type: Option<String>,

    /// The type of vNIC which this port should be attached to. This is used to
    /// determine which mechanism driver(s) to be used to bind the port. The
    /// valid values are `normal`, `macvtap`, `direct`, `baremetal`,
    /// `direct-physical`, `virtio-forwarder`, `smart-nic` and
    /// `remote-managed`. What type of vNIC is actually available depends on
    /// deployments.
    ///
    vnic_type: Option<VnicType>,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
#[serde(untagged)]
pub enum VnicType {
    // AcceleratorDirect
    #[serde(rename = "accelerator-direct")]
    AcceleratorDirect,

    // AcceleratorDirectPhysical
    #[serde(rename = "accelerator-direct-physical")]
    AcceleratorDirectPhysical,

    // RemoteManaged
    #[serde(rename = "remote-managed")]
    RemoteManaged,

    // Normal
    #[serde(rename = "normal")]
    Normal,

    // Direct
    #[serde(rename = "direct")]
    Direct,

    // Vdpa
    #[serde(rename = "vdpa")]
    Vdpa,

    // Baremetal
    #[serde(rename = "baremetal")]
    Baremetal,

    // DirectPhysical
    #[serde(rename = "direct-physical")]
    DirectPhysical,

    // Macvtap
    #[serde(rename = "macvtap")]
    Macvtap,

    // VirtioForwarder
    #[serde(rename = "virtio-forwarder")]
    VirtioForwarder,

    // SmartNic
    #[serde(rename = "smart-nic")]
    SmartNic,
}
