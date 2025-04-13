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
//! Response type for the get local-ips operation

use serde::{Deserialize, Serialize};
use structable::{StructTable, StructTableOptions};

/// LocalIp response representation
#[derive(Clone, Deserialize, Serialize, StructTable)]
pub struct LocalIpResponse {
    #[structable(optional)]
    pub created_at: Option<String>,

    #[structable(optional, wide)]
    pub description: Option<String>,

    #[structable(optional)]
    pub id: Option<String>,

    #[structable(optional, serialize, wide)]
    pub ip_mode: Option<IpMode>,

    #[structable(optional, wide)]
    pub local_ip_address: Option<String>,

    #[structable(optional, wide)]
    pub local_port_id: Option<String>,

    #[structable(optional)]
    pub name: Option<String>,

    #[structable(optional, wide)]
    pub network_id: Option<String>,

    #[structable(optional, wide)]
    pub project_id: Option<String>,

    #[structable(optional, wide)]
    pub revision_number: Option<i32>,

    #[structable(optional)]
    pub updated_at: Option<String>,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
#[serde(untagged)]
pub enum IpMode {
    // Passthrough
    #[serde(rename = "passthrough")]
    Passthrough,

    // Translate
    #[serde(rename = "translate")]
    Translate,
}
