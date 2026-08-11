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

//! Runtime loader and registry for Extism (WASM) OpenStack SDK auth plugins.
//!
//! This crate lets a `.wasm` module implementing the guest ABI documented in
//! [`plugin`] act as an [`openstack_sdk_auth_core::OpenStackAuthType`]
//! alongside the compiled-in, `inventory`-registered auth plugins.

pub mod error;
pub(crate) mod host;
pub mod lockfile;
pub mod plugin;
pub mod registry;

pub use error::WasmPluginError;
pub use lockfile::{PluginEntry, PluginLockfile, TrustInfo};
pub use plugin::WasmAuthPlugin;
