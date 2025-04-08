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
//! Response type for the get os-services operation

use crate::common::IntString;
use serde::{Deserialize, Serialize};

/// Service response representation
#[derive(Clone, Deserialize, Serialize)]
pub struct ServiceResponse {
    /// The binary name of the service.
    ///
    binary: Option<String>,

    /// The reason for disabling a service.
    ///
    disabled_reason: Option<String>,

    /// Whether or not this service was forced down manually by an
    /// administrator after the service was fenced. This value is useful to
    /// know that some 3rd party has verified the service should be marked
    /// down.
    ///
    forced_down: Option<bool>,

    /// The name of the host.
    ///
    host: String,

    /// The id of the service as a uuid.
    ///
    id: IntString,

    /// Service name
    ///
    name: Option<String>,

    /// The state of the service. One of `up` or `down`.
    ///
    state: String,

    /// The status of the service. One of `enabled` or `disabled`.
    ///
    status: Status,

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
    updated_at: Option<String>,

    /// The availability zone name.
    ///
    zone: String,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
#[serde(untagged)]
pub enum Status {
    // Disabled
    #[serde(rename = "disabled")]
    Disabled,

    // Enabled
    #[serde(rename = "enabled")]
    Enabled,
}
