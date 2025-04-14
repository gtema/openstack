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
//! Response type for the GET `os-availability-zone/detail` operation

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use structable::{StructTable, StructTableOptions};

/// AvailabilityZone response representation
#[derive(Clone, Deserialize, Serialize, StructTable)]
pub struct AvailabilityZoneResponse {
    /// An object containing a list of host information. The host information
    /// is comprised of host and service objects. The service object returns
    /// three parameters representing the states of the service: `active`,
    /// `available`, and `updated_at`.
    ///
    #[structable(optional, serialize)]
    pub hosts: Option<HashMap<String, Value>>,

    /// The availability zone name.
    ///
    #[serde(rename = "zoneName")]
    #[structable(title = "zoneName")]
    pub zone_name: String,

    /// The current state of the availability zone.
    ///
    #[serde(rename = "zoneState")]
    #[structable(serialize, title = "zoneState")]
    pub zone_state: ZoneState,
}

/// The current state of the availability zone.
///
/// `ZoneState` type
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ZoneState {
    pub available: Option<bool>,
}
