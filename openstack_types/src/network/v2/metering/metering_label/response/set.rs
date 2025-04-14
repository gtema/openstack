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
//! Response type for the PUT `metering/metering-labels/{id}` operation

use crate::common::deser_bool_str_opt;
use serde::{Deserialize, Serialize};
use structable::{StructTable, StructTableOptions};

/// MeteringLabel response representation
#[derive(Clone, Deserialize, Serialize, StructTable)]
pub struct MeteringLabelResponse {
    #[structable(optional)]
    pub description: Option<String>,

    #[structable(optional)]
    pub id: Option<String>,

    #[structable(optional)]
    pub name: Option<String>,

    #[serde(deserialize_with = "deser_bool_str_opt")]
    #[structable(optional)]
    pub shared: Option<bool>,

    #[structable(optional)]
    pub tenant_id: Option<String>,
}
