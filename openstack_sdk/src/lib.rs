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

#![doc = include_str!("../README.md")]

pub mod api;
pub mod auth;
pub mod config;
pub mod catalog {
    pub use openstack_sdk_core::catalog::CatalogError;
}

pub mod types;

#[cfg(feature = "async")]
pub use openstack_sdk_core::AsyncOpenStack;
#[cfg(feature = "sync")]
pub use openstack_sdk_core::OpenStack;
pub use openstack_sdk_core::auth::AuthError;
pub use openstack_sdk_core::{OpenStackError, RestError};

#[allow(dead_code)]
pub mod test;
