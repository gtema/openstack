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

//! Project cleanup: resource-level dependency graph, discover/apply engine,
//! and the [`provider::CleanupProvider`] extension point.

pub mod engine;
pub mod filters;
pub mod provider;
pub mod providers;
pub mod relations;
pub mod types;

pub use engine::{CleanupPlan, CleanupResult, PlanEdge, ProjectCleanup, ProjectCleanupBuilder};
pub use filters::evaluate_filters;
pub use provider::{
    CleanupContext, CleanupDependency, CleanupError, CleanupProvider, service_order,
};
pub use relations::{Edge, RelationEffect, RelationRule, evaluate_edges};
pub use types::{PlannedResource, ResourceKind};

#[cfg(feature = "network")]
pub use providers::network::NetworkCleanupProvider;
