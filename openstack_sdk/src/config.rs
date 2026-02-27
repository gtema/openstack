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

//! OpenStack configuration handling.
//!
//! ```rust
//! let cfg = openstack_sdk::config::ConfigFile::new().unwrap();
//! let profile = cfg
//!     .get_cloud_config("devstack")
//!     .expect("Cloud devstack not found");
//! ```
//!
//! It is possible to create a config by passing paths to a [builder](ConfigFileBuilder).
//!
//! ```no_run
//! let cfg = openstack_sdk::config::ConfigFile::builder()
//!     .add_source("c1.yaml")
//!     .expect("Failed to load 'c1.yaml'")
//!     .add_source("s2.yaml")
//!     .expect("Failed to load 's2.yaml'")
//!     .build();
//! ```
//!
//! It is also possible to create a config with [`ConfigFile::new_with_user_specified_configs`].
//! This is similar to what the python OpenStackSDK does.
//!
//! ```no_run
//! let cfg = openstack_sdk::config::ConfigFile::new_with_user_specified_configs(
//!     Some("c1.yaml"),
//!     Some("s2.yaml"),
//! ).expect("Failed to load the configuration files");
//! ```
//!
//! [CloudConfig] object can be constructed directly from environment variables with the `OS_`
//! prefix:
//!
//! ```rust
//! # use openstack_sdk::config::CloudConfig;
//! let cfg = CloudConfig::from_env().unwrap();
//! ```
//!
pub use openstack_sdk_core::config::*;
