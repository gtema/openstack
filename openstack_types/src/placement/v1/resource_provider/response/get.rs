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
//! Response type for the get resource_providers/{uuid} operation

use serde::{Deserialize, Serialize};

/// ResourceProvider response representation
#[derive(Clone, Deserialize, Serialize)]
struct ResourceProviderResponse {
    /// A consistent view marker that assists with the management of concurrent
    /// resource provider updates.
    ///
    generation: i32,

    /// A list of links associated with one resource provider.
    ///
    /// Note
    ///
    /// Aggregates relationship link is available starting from version 1.1.
    /// Traits relationship link is available starting from version 1.6.
    /// Allocations relationship link is available starting from version 1.11.
    ///
    links: Vec<Links>,

    /// The name of one resource provider.
    ///
    name: String,

    /// The UUID of the immediate parent of the resource provider.
    ///
    /// **New in version 1.14**
    ///
    parent_provider_uuid: Option<String>,

    /// Read-only UUID of the top-most provider in this provider tree.
    ///
    /// **New in version 1.14**
    ///
    root_provider_uuid: Option<String>,

    /// The uuid of a resource provider.
    ///
    uuid: String,
}

/// `Links` type
#[derive(Clone, Debug, Deserialize, Serialize)]
struct Links {
    href: Option<String>,
    rel: Option<String>,
}
