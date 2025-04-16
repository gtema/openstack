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
//! Response type for the PATCH `endpoints/{endpoint_id}` operation

use serde::{Deserialize, Serialize};
use structable::{StructTable, StructTableOptions};

/// Endpoint response representation
#[derive(Clone, Deserialize, Serialize, StructTable)]
pub struct EndpointResponse {
    /// Indicates whether the endpoint appears in the service catalog: -
    /// `false`. The endpoint does not appear in the service catalog. - `true`.
    /// The endpoint appears in the service catalog.
    #[serde(default)]
    #[structable(optional)]
    pub enabled: Option<bool>,

    /// The endpoint ID.
    #[serde(default)]
    #[structable(optional)]
    pub id: Option<String>,

    /// The interface type, which describes the visibility of the endpoint.
    /// Value is: - `public`. Visible by end users on a publicly available
    /// network interface. - `internal`. Visible by end users on an unmetered
    /// internal network interface. - `admin`. Visible by administrative users
    /// on a secure network interface.
    #[serde(default)]
    #[structable(optional, serialize)]
    pub interface: Option<Interface>,

    /// (Deprecated in v3.2) The geographic location of the service endpoint.
    #[serde(default)]
    #[structable(optional)]
    pub region: Option<String>,

    /// (Since v3.2) The ID of the region that contains the service endpoint.
    #[serde(default)]
    #[structable(optional)]
    pub region_id: Option<String>,

    /// The UUID of the service to which the endpoint belongs.
    #[serde(default)]
    #[structable(optional)]
    pub service_id: Option<String>,

    /// The endpoint URL.
    #[serde(default)]
    #[structable(optional)]
    pub url: Option<String>,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub enum Interface {
    // Admin
    #[serde(rename = "admin")]
    Admin,

    // Internal
    #[serde(rename = "internal")]
    Internal,

    // Public
    #[serde(rename = "public")]
    Public,
}

impl std::str::FromStr for Interface {
    type Err = ();
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "admin" => Ok(Self::Admin),
            "internal" => Ok(Self::Internal),
            "public" => Ok(Self::Public),
            _ => Err(()),
        }
    }
}
