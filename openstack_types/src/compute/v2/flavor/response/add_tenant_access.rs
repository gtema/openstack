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
//! Response type for the post flavors/{id}/action operation

use serde::{Deserialize, Serialize};

/// Flavor response representation
#[derive(Clone, Deserialize, Serialize)]
struct FlavorResponse {
    /// A list of objects, each with the keys `flavor_id` and `tenant_id`.
    ///
    flavor_access: Vec<FlavorAccess>,
}

/// `FlavorAccess` type
#[derive(Clone, Debug, Deserialize, Serialize)]
struct FlavorAccess {
    flavor_id: String,
    tenant_id: String,
}
