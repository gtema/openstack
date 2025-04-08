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
//! Response type for the post flavors operation

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

/// Flavor response representation
#[derive(Clone, Deserialize, Serialize)]
pub struct FlavorResponse {
    /// The description of the flavor.
    ///
    /// **New in version 2.55**
    ///
    pub description: Option<String>,

    /// The size of the root disk that will be created in GiB. If 0 the root
    /// disk will be set to exactly the size of the image used to deploy the
    /// instance. However, in this case the scheduler cannot select the compute
    /// host based on the virtual image size. Therefore, 0 should only be used
    /// for volume booted instances or for testing purposes. Volume-backed
    /// instances can be enforced for flavors with zero root disk via the
    /// `os_compute_api:servers:create:zero_disk_flavor` policy rule.
    ///
    pub disk: i32,

    /// A dictionary of the flavor’s extra-specs key-and-value pairs. This will
    /// only be included if the user is allowed by policy to index flavor
    /// extra_specs.
    ///
    /// **New in version 2.61**
    ///
    pub extra_specs: Option<HashMap<String, String>>,

    /// The ID of the flavor. While people often make this look like an int,
    /// this is really a string.
    ///
    pub id: String,

    /// Links to the resources in question. See
    /// [API Guide / Links and References](https://docs.openstack.org/api-guide/compute/links_and_references.html)
    /// for more info.
    ///
    pub links: Vec<Links>,

    /// The display name of a flavor.
    ///
    pub name: String,

    #[serde(rename = "os-flavor-access:is_public")]
    pub os_flavor_access_is_public: Value,

    /// Whether or not the flavor has been administratively disabled. This is
    /// an artifact of the legacy v2 API and will always be set to `false`.
    /// There is currently no way to disable a flavor and set this to `true`.
    ///
    #[serde(rename = "OS-FLV-DISABLED:disabled")]
    pub os_flv_disabled_disabled: bool,

    /// The size of the ephemeral disk that will be created, in GiB. Ephemeral
    /// disks may be written over on server state changes. So should only be
    /// used as a scratch space for applications that are aware of its
    /// limitations. Defaults to 0.
    ///
    #[serde(rename = "OS-FLV-EXT-DATA:ephemeral")]
    pub os_flv_ext_data_ephemeral: i32,

    /// The amount of RAM a flavor has, in MiB.
    ///
    pub ram: i32,

    pub rxtx_factor: Value,

    /// The size of a dedicated swap disk that will be allocated, in MiB. If 0
    /// (the default), no dedicated swap disk will be created. Currently, the
    /// empty string (‘’) is used to represent 0. As of microversion 2.75
    /// default return value of swap is 0 instead of empty string.
    ///
    pub swap: i32,

    /// The number of virtual CPUs that will be allocated to the server.
    ///
    pub vcpus: i32,
}

/// `Links` type
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Links {
    pub href: String,
    pub rel: String,
}
