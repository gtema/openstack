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
//! Response type for the get auth/catalog operation

use serde::{Deserialize, Serialize};

/// Catalog response representation
#[derive(Clone, Deserialize, Serialize)]
pub struct CatalogResponse {
    /// A list of `endpoint` objects.
    ///
    pub endpoints: Option<Vec<Endpoints>>,

    /// The UUID of the service to which the endpoint belongs.
    ///
    pub id: Option<String>,

    /// The service name.
    ///
    pub name: Option<String>,

    /// The service type, which describes the API implemented by the service.
    /// Value is `compute`, `ec2`, `identity`, `image`, `network`, or `volume`.
    ///
    #[serde(rename = "type")]
    pub _type: Option<String>,
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

/// `Endpoints` type
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Endpoints {
    pub id: Option<String>,
    pub interface: Option<Interface>,
    pub region: Option<String>,
    pub url: Option<String>,
}
