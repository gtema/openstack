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
//! Response type for the get projects operation

use serde::{Deserialize, Serialize};

/// Project response representation
#[derive(Clone, Deserialize, Serialize)]
struct ProjectResponse {
    /// The description of the project.
    ///
    description: Option<String>,

    /// The ID of the domain for the project.
    ///
    domain_id: Option<String>,

    /// If the user is enabled, this value is `true`. If the user is disabled,
    /// this value is `false`.
    ///
    enabled: Option<bool>,

    /// The ID for the project.
    ///
    id: Option<String>,

    /// If the user is enabled, this value is `true`. If the user is disabled,
    /// this value is `false`.
    ///
    is_domain: Option<bool>,

    /// The name of the project.
    ///
    name: Option<String>,

    /// The resource options for the project. Available resource options are
    /// `immutable`.
    ///
    options: Option<Options>,

    /// The ID of the parent for the project.
    ///
    /// **New in version 3.4**
    ///
    parent_id: Option<String>,

    /// A list of simple strings assigned to a project.
    ///
    tags: Option<Vec<String>>,
}

/// The link to the resources in question.
///
/// `Links` type
#[derive(Clone, Debug, Deserialize, Serialize)]
struct Links {
    _self: Option<String>,
}

/// The resource options for the project. Available resource options are
/// `immutable`.
///
/// `Options` type
#[derive(Clone, Debug, Deserialize, Serialize)]
struct Options {
    immutable: Option<bool>,
}
