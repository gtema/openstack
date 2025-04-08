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
//! Response type for the get registered_limits/{registered_limit_id} operation

use serde::{Deserialize, Serialize};

/// RegisteredLimit response representation
#[derive(Clone, Deserialize, Serialize)]
struct RegisteredLimitResponse {
    /// The default limit for the registered limit.
    ///
    default_limit: Option<i32>,

    /// The registered limit description.
    ///
    description: Option<String>,

    /// The registered limit ID.
    ///
    id: Option<String>,

    /// The link to the resources in question.
    ///
    links: Option<Links>,

    /// The ID of the region that contains the service endpoint. The value can
    /// be None.
    ///
    region_id: Option<String>,

    /// The resource name.
    ///
    resource_name: Option<String>,

    /// The UUID of the service to which the registered limit belongs.
    ///
    service_id: Option<String>,
}

/// The link to the resources in question.
///
/// `Links` type
#[derive(Clone, Debug, Deserialize, Serialize)]
struct Links {
    _self: Option<String>,
}
