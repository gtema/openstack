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
//! Response type for the get limits operation

use serde::{Deserialize, Serialize};

/// Limit response representation
#[derive(Clone, Deserialize, Serialize)]
pub struct LimitResponse {
    /// The max amount of items allowed per page
    ///
    pub max_page_limit: Option<i32>,

    /// The max length of a recordset name
    ///
    pub max_recordset_name_length: Option<i32>,

    /// The max amount of records contained in a recordset
    ///
    pub max_recordset_records: Option<i32>,

    /// The max length of a zone name
    ///
    pub max_zone_name_length: Option<i32>,

    /// The max amount of records in a zone
    ///
    pub max_zone_records: Option<i32>,

    /// The max amount of recordsets per zone
    ///
    pub max_zone_recordsets: Option<i32>,

    /// The max amount of zones for this project
    ///
    pub max_zones: Option<i32>,

    /// The lowest ttl allowed on this system
    ///
    pub min_ttl: Option<i32>,
}
