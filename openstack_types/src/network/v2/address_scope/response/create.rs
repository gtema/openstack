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
//! Response type for the post address-scopes operation

use crate::common::BoolString;
use serde::{Deserialize, Serialize};

/// AddressScope response representation
#[derive(Clone, Deserialize, Serialize)]
struct AddressScopeResponse {
    /// The ID of the address scope.
    ///
    id: Option<String>,

    /// The IP protocol version. Valid value is `4` or `6`. Default is `4`.
    ///
    ip_version: Option<i32>,

    /// Human-readable name of the resource.
    ///
    name: Option<String>,

    /// Indicates whether this resource is shared across all projects.
    ///
    shared: Option<BoolString>,

    /// The ID of the project.
    ///
    tenant_id: Option<String>,
}
