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
//! Response type for the get quotas operation

use serde::{Deserialize, Serialize};

/// Quota response representation
#[derive(Clone, Deserialize, Serialize)]
pub struct QuotaResponse {
    pub created_at: Option<String>,

    pub hard_limit: Option<i32>,

    pub id: Option<i32>,

    pub project_id: Option<String>,

    pub resource: Option<Resource>,

    pub updated_at: Option<String>,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
#[serde(untagged)]
pub enum Resource {
    // Cluster
    #[serde(rename = "Cluster")]
    Cluster,
}
