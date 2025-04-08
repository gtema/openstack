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
//! Response type for the get resource_providers operation

use serde::{Deserialize, Serialize};

/// ResourceProvider response representation
#[derive(Clone, Deserialize, Serialize)]
pub struct ResourceProviderResponse {
    /// A consistent view marker that assists with the management of concurrent
    /// resource provider updates.
    ///
    pub generation: i32,

    /// The name of one resource provider.
    ///
    pub name: String,

    /// The UUID of the immediate parent of the resource provider.
    ///
    /// **New in version 1.14**
    ///
    pub parent_provider_uuid: Option<String>,

    /// Read-only UUID of the top-most provider in this provider tree.
    ///
    /// **New in version 1.14**
    ///
    pub root_provider_uuid: Option<String>,

    /// The uuid of a resource provider.
    ///
    pub uuid: String,
}

/// `Links` type
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Links {
    pub href: Option<String>,
    pub rel: Option<String>,
}
