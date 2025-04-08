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
//! Response type for the get resource_providers/{uuid}/inventories operation

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Inventory response representation
#[derive(Clone, Deserialize, Serialize)]
struct InventoryResponse {
    /// A dictionary of inventories keyed by resource classes.
    ///
    inventories: HashMap<String, InventoriesItem>,

    /// A consistent view marker that assists with the management of concurrent
    /// resource provider updates.
    ///
    resource_provider_generation: i32,
}

/// `InventoriesItem` type
#[derive(Clone, Debug, Deserialize, Serialize)]
struct InventoriesItem {
    allocation_ratio: Option<f32>,
    max_unit: Option<i32>,
    min_unit: Option<i32>,
    reserved: Option<i32>,
    step_size: Option<i32>,
    total: i32,
}
