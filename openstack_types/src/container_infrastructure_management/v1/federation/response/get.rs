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
//! Response type for the GET `federations/{federation_id}` operation

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use structable::{StructTable, StructTableOptions};

/// Federation response representation
#[derive(Clone, Deserialize, Serialize, StructTable)]
pub struct FederationResponse {
    #[structable(optional)]
    pub created_at: Option<String>,

    #[structable(optional)]
    pub hostcluster_id: Option<String>,

    #[structable(optional, serialize)]
    pub links: Option<Vec<Links>>,

    #[structable(optional, serialize)]
    pub member_ids: Option<Vec<String>>,

    #[structable(optional)]
    pub name: Option<String>,

    #[structable(optional, serialize)]
    pub properties: Option<HashMap<String, String>>,

    #[structable(optional, serialize)]
    pub status: Option<Status>,

    #[structable(optional)]
    pub status_reason: Option<String>,

    #[structable(optional)]
    pub updated_at: Option<String>,

    #[structable(optional)]
    pub uuid: Option<String>,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub enum Status {
    // CreateComplete
    #[serde(rename = "CREATE_COMPLETE")]
    CreateComplete,

    // CreateFailed
    #[serde(rename = "CREATE_FAILED")]
    CreateFailed,

    // CreateInProgress
    #[serde(rename = "CREATE_IN_PROGRESS")]
    CreateInProgress,

    // DeleteComplete
    #[serde(rename = "DELETE_COMPLETE")]
    DeleteComplete,

    // DeleteFailed
    #[serde(rename = "DELETE_FAILED")]
    DeleteFailed,

    // DeleteInProgress
    #[serde(rename = "DELETE_IN_PROGRESS")]
    DeleteInProgress,

    // UpdateComplete
    #[serde(rename = "UPDATE_COMPLETE")]
    UpdateComplete,

    // UpdateFailed
    #[serde(rename = "UPDATE_FAILED")]
    UpdateFailed,

    // UpdateInProgress
    #[serde(rename = "UPDATE_IN_PROGRESS")]
    UpdateInProgress,
}

/// A link representation.
///
/// `Links` type
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Links {
    pub created_at: Option<String>,
    pub href: Option<String>,
    pub rel: Option<String>,
    pub _type: Option<String>,
    pub updated_at: Option<String>,
}
