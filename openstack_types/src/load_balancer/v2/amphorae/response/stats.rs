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
//! Response type for the get octavia/amphorae/{amphora_id}/stats operation

use serde::{Deserialize, Serialize};

/// Amphorae response representation
#[derive(Clone, Deserialize, Serialize)]
pub struct AmphoraeResponse {
    /// A list of amphora statistics objects, one per listener.
    ///
    /// **New in version 2.3**
    ///
    amphora_stats: Vec<AmphoraStats>,
}

/// Defines which attributes are to show on stats response.
///
/// `AmphoraStats` type
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AmphoraStats {
    active_connections: Option<i32>,
    bytes_in: Option<i32>,
    bytes_out: Option<i32>,
    id: Option<String>,
    listener_id: Option<String>,
    loadbalancer_id: Option<String>,
    request_errors: Option<i32>,
    total_connections: Option<i32>,
}
