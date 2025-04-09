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
//! Response type for the get endpoints/{endpoint_id} operation

use serde::{Deserialize, Serialize};

/// Endpoint response representation
#[derive(Clone, Deserialize, Serialize)]
pub struct EndpointResponse {
    /// Indicates whether the endpoint appears in the service catalog: -
    /// `false`. The endpoint does not appear in the service catalog. - `true`.
    /// The endpoint appears in the service catalog.
    ///
    pub enabled: Option<bool>,

    /// The endpoint ID.
    ///
    pub id: Option<String>,

    /// The interface type, which describes the visibility of the endpoint.
    /// Value is: - `public`. Visible by end users on a publicly available
    /// network interface. - `internal`. Visible by end users on an unmetered
    /// internal network interface. - `admin`. Visible by administrative users
    /// on a secure network interface.
    ///
    pub interface: Option<Interface>,

    /// (Deprecated in v3.2) The geographic location of the service endpoint.
    ///
    pub region: Option<String>,

    /// (Since v3.2) The ID of the region that contains the service endpoint.
    ///
    pub region_id: Option<String>,

    /// The UUID of the service to which the endpoint belongs.
    ///
    pub service_id: Option<String>,

    /// The endpoint URL.
    ///
    pub url: Option<String>,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
#[serde(untagged)]
pub enum Interface {
    // Public
    #[serde(rename = "public")]
    Public,

    // Internal
    #[serde(rename = "internal")]
    Internal,

    // Admin
    #[serde(rename = "admin")]
    Admin,
}
