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
//! Response type for the post services operation

use serde::{Deserialize, Serialize};

/// Service response representation
#[derive(Clone, Deserialize, Serialize)]
pub struct ServiceResponse {
    /// The service description.
    ///
    description: Option<String>,

    /// Defines whether the service and its endpoints appear in the service
    /// catalog: - `false`. The service and its endpoints do not appear in the
    /// service catalog. - `true`. The service and its endpoints appear in the
    /// service catalog.
    ///
    enabled: Option<bool>,

    /// The UUID of the service to which the endpoint belongs.
    ///
    id: Option<String>,

    /// The service name.
    ///
    name: Option<String>,

    /// The service type, which describes the API implemented by the service.
    /// Value is `compute`, `ec2`, `identity`, `image`, `network`, or `volume`.
    ///
    #[serde(rename = "type")]
    _type: Option<String>,
}
