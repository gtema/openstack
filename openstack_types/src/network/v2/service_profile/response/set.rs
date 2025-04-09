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
//! Response type for the put service_profiles/{id} operation

use crate::common::BoolString;
use serde::{Deserialize, Serialize};

/// ServiceProfile response representation
#[derive(Clone, Deserialize, Serialize)]
pub struct ServiceProfileResponse {
    /// The human-readable description for the service profile.
    ///
    pub description: Option<String>,

    /// Provider driver to use for this profile.
    ///
    pub driver: Option<String>,

    /// Indicates whether this service profile is enabled or not. Default is
    /// `true`.
    ///
    pub enabled: Option<BoolString>,

    /// The UUID of the service profile.
    ///
    pub id: Option<String>,

    /// JSON-formatted meta information of the service profile.
    ///
    pub metainfo: Option<String>,
}
