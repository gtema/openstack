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

//! OpenStack API types
//!
//! Types used in the OpenStack API communications. This crate defines the types that are reused
//! between `openstack_sdk`, `openstack_cli` and `openstack_tui` crates.

#[cfg(feature = "block_storage")]
pub mod block_storage;
pub mod common;
#[cfg(feature = "compute")]
pub mod compute;
#[cfg(feature = "container_infra")]
pub mod container_infrastructure_management;
#[cfg(feature = "dns")]
pub mod dns;
#[cfg(feature = "identity")]
pub mod identity;
#[cfg(feature = "image")]
pub mod image;
#[cfg(feature = "load_balancer")]
pub mod load_balancer;
#[cfg(feature = "network")]
pub mod network;
#[cfg(feature = "object_store")]
pub mod object_store;
#[cfg(feature = "placement")]
pub mod placement;
