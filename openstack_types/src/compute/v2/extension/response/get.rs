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
//! Response type for the get extensions/{id} operation

use serde::{Deserialize, Serialize};

/// Extension response representation
#[derive(Clone, Deserialize, Serialize)]
pub struct ExtensionResponse {
    /// A short name by which this extension is also known.
    ///
    alias: String,

    /// Text describing this extension’s purpose.
    ///
    description: Option<String>,

    /// Links pertaining to this extension. This is a list of dictionaries,
    /// each including keys `href` and `rel`.
    ///
    links: Option<Vec<Links>>,

    /// Name of the extension.
    ///
    name: String,

    /// A URL pointing to the namespace for this extension.
    ///
    namespace: Option<String>,

    /// The date and time when the resource was updated. The date and time
    /// stamp format is [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601)
    ///
    /// ```text
    /// CCYY-MM-DDThh:mm:ss±hh:mm
    ///
    /// ```
    ///
    /// For example, `2015-08-27T09:49:58-05:00`. The `±hh:mm` value, if
    /// included, is the time zone as an offset from UTC. In the previous
    /// example, the offset value is `-05:00`.
    ///
    updated: Option<String>,
}

/// Links to the resources in question. See
/// [API Guide / Links and References](https://docs.openstack.org/api-guide/compute/links_and_references.html)
/// for more info.
///
/// `Links` type
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Links {
    href: Option<String>,
    rel: Option<String>,
}
