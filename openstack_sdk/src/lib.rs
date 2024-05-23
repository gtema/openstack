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
mod auth;
mod catalog;
pub mod config;
mod error;
#[cfg(feature = "sync")]
mod openstack;
#[cfg(feature = "async")]
mod openstack_async;
mod service_authority;
mod state;
mod utils;

pub mod types;

pub use crate::auth::AuthError;
pub use crate::error::{OpenStackError, RestError};
#[cfg(feature = "sync")]
pub use crate::openstack::OpenStack;
#[cfg(feature = "async")]
pub use crate::openstack_async::AsyncOpenStack;

#[cfg(test)]
#[allow(dead_code)]
mod test;
