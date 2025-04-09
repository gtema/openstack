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
//! Response type for the put resource_providers/{uuid}/inventories/{resource_class} operation

use serde::{Deserialize, Serialize};

/// Inventory response representation
#[derive(Clone, Deserialize, Serialize)]
pub struct InventoryResponse {
    /// It is used in determining whether consumption of the resource of the
    /// provider can exceed physical constraints.
    ///
    /// For example, for a vCPU resource with:
    ///
    /// ```text
    /// allocation_ratio = 16.0
    /// total = 8
    ///
    /// ```
    ///
    /// Overall capacity is equal to 128 vCPUs.
    ///
    pub allocation_ratio: Option<f32>,

    /// A maximum amount any single allocation against an inventory can have.
    ///
    pub max_unit: Option<i32>,

    /// A minimum amount any single allocation against an inventory can have.
    ///
    pub min_unit: Option<i32>,

    /// The amount of the resource a provider has reserved for its own use.
    ///
    pub reserved: Option<i32>,

    /// A consistent view marker that assists with the management of concurrent
    /// resource provider updates.
    ///
    pub resource_provider_generation: i32,

    /// A representation of the divisible amount of the resource that may be
    /// requested. For example, step_size = 5 means that only values divisible
    /// by 5 (5, 10, 15, etc.) can be requested.
    ///
    pub step_size: Option<i32>,

    /// The actual amount of the resource that the provider can accommodate.
    ///
    pub total: i32,
}
