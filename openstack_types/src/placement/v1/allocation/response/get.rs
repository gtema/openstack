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
//! Response type for the GET `allocations/{consumer_uuid}` operation

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use structable::{StructTable, StructTableOptions};

/// Allocation response representation
#[derive(Clone, Deserialize, Serialize, StructTable)]
pub struct AllocationResponse {
    /// A dictionary of allocations keyed by resource provider uuid.
    #[structable(serialize)]
    pub allocations: BTreeMap<String, AllocationsItem>,

    /// The generation of the consumer. Will be absent when listing allocations
    /// for a consumer uuid that has no allocations.
    ///
    /// **New in version 1.28**
    #[serde(default)]
    #[structable(optional)]
    pub consumer_generation: Option<i32>,

    /// A string that consists of numbers, `A-Z`, and `_` describing what kind
    /// of consumer is creating, or has created, allocations using a quantity
    /// of inventory. The string is determined by the client when writing
    /// allocations and it is up to the client to ensure correct choices
    /// amongst collaborating services. For example, the compute service may
    /// choose to type some consumers ‘INSTANCE’ and others ‘MIGRATION’.
    ///
    /// **New in version 1.38**
    #[serde(default)]
    #[structable(optional)]
    pub consumer_type: Option<String>,

    #[serde(default)]
    #[structable(optional, serialize)]
    pub mappings: Option<BTreeMap<String, Vec<String>>>,

    /// The uuid of a project. Will be absent when listing allocations for a
    /// consumer uuid that has no allocations.
    ///
    /// **New in version 1.12**
    #[serde(default)]
    #[structable(optional)]
    pub project_id: Option<String>,

    /// The uuid of a user. Will be absent when listing allocations for a
    /// consumer uuid that has no allocations.
    ///
    /// **New in version 1.12**
    #[serde(default)]
    #[structable(optional)]
    pub user_id: Option<String>,
}

/// `AllocationsItem` type
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AllocationsItem {
    #[serde(default)]
    pub generation: Option<i32>,
    pub resources: BTreeMap<String, i32>,
}
