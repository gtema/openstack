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
//! Response type for the get versions/{id} operation

use serde::{Deserialize, Serialize};

/// Version response representation
#[derive(Clone, Deserialize, Serialize)]
pub struct VersionResponse {
    /// A common name for the version in question. Informative only, it has no
    /// real semantic meaning.
    ///
    id: String,

    /// Links to the resources in question. See
    /// [API Guide / Links and References](https://docs.openstack.org/api-guide/compute/links_and_references.html)
    /// for more info.
    ///
    links: Vec<Links>,

    #[serde(rename = "media-types")]
    media_types: Option<Vec<MediaTypes>>,

    /// If this version of the API supports microversions, the minimum
    /// microversion that is supported. This will be the empty string if
    /// microversions are not supported.
    ///
    min_version: String,

    /// The status of this API version. This can be one of:
    ///
    /// - `CURRENT`: this is the preferred version of the API to use
    /// - `SUPPORTED`: this is an older, but still supported version of the API
    /// - `DEPRECATED`: a deprecated version of the API that is slated for
    ///   removal
    ///
    status: Status,

    /// This is a fixed string. It is `2011-01-21T11:33:21Z` in version 2.0,
    /// `2013-07-23T11:33:21Z` in version 2.1.
    ///
    /// Note
    ///
    /// It is vestigial and provides no useful information. It will be
    /// deprecated and removed in the future.
    ///
    updated: String,

    version: Option<String>,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
#[serde(untagged)]
pub enum Status {
    // Supported
    #[serde(rename = "SUPPORTED")]
    Supported,

    // Current
    #[serde(rename = "CURRENT")]
    Current,

    // Deprecated
    #[serde(rename = "DEPRECATED")]
    Deprecated,
}

/// `Links` type
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Links {
    href: String,
    rel: String,
    _type: Option<String>,
}

/// `MediaTypes` type
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MediaTypes {
    base: String,
    _type: String,
}
