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
//! Response type for the post os-aggregates/{id}/action operation

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use structable::{StructTable, StructTableOptions};

/// Aggregate response representation
#[derive(Clone, Deserialize, Serialize, StructTable)]
pub struct AggregateResponse {
    #[structable(optional, serialize)]
    pub availability_zone: Option<String>,

    #[structable()]
    pub created_at: String,

    #[structable()]
    pub deleted: bool,

    #[structable()]
    pub deleted_at: String,

    #[structable(serialize)]
    pub hosts: Vec<String>,

    #[structable()]
    pub id: i32,

    #[structable(optional, serialize)]
    pub metadata: Option<HashMap<String, Value>>,

    #[structable()]
    pub name: String,

    #[structable()]
    pub updated_at: String,

    #[structable()]
    pub uuid: String,
}
