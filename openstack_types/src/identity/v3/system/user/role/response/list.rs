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
//! Response type for the get system/users/{user_id}/roles operation

use serde::{Deserialize, Serialize};

/// Role response representation
#[derive(Clone, Deserialize, Serialize)]
struct RoleResponse {
    /// The role description.
    ///
    description: Option<String>,

    /// The ID of the domain.
    ///
    domain_id: Option<String>,

    /// The role ID.
    ///
    id: Option<String>,

    /// The resource name.
    ///
    name: Option<String>,

    /// The resource options for the role. Available resource options are
    /// `immutable`.
    ///
    options: Option<Options>,
}

/// The link to the resources in question.
///
/// `Links` type
#[derive(Clone, Debug, Deserialize, Serialize)]
struct Links {
    _self: Option<String>,
}

/// The resource options for the role. Available resource options are
/// `immutable`.
///
/// `Options` type
#[derive(Clone, Debug, Deserialize, Serialize)]
struct Options {
    immutable: Option<bool>,
}
