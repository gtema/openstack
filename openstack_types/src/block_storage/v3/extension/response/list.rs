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
//! Response type for the get extensions operation

use serde::{Deserialize, Serialize};
use structable::{StructTable, StructTableOptions};

/// Extension response representation
#[derive(Clone, Deserialize, Serialize, StructTable)]
pub struct ExtensionResponse {
    /// The alias for the extension. For example, “FOXNSOX”, “os-
    /// availability-zone”, “os-extended-quotas”, “os- share-unmanage” or
    /// “os-used-limits.”
    ///
    #[structable()]
    pub alias: String,

    /// The extension description.
    ///
    #[structable()]
    pub description: String,

    /// The name of the object.
    ///
    #[structable()]
    pub name: String,

    /// The date and time when the resource was updated.
    ///
    /// The date and time stamp format is
    /// [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601):
    ///
    /// ```text
    /// CCYY-MM-DDThh:mm:ss±hh:mm
    ///
    /// ```
    ///
    /// For example, `2015-08-27T09:49:58-05:00`.
    ///
    /// The `±hh:mm` value, if included, is the time zone as an offset from
    /// UTC. In the previous example, the offset value is `-05:00`.
    ///
    /// If the `updated_at` date and time stamp is not set, its value is
    /// `null`.
    ///
    #[structable()]
    pub updated: String,
}
