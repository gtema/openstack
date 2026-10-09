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

#[cfg(test)]
mod tests {
    use secrecy::{ExposeSecret, SecretString};

    use super::*;

    #[test]
    fn test_split_sensitive_uses_plugin_schema() {
        let cfg = CloudConfig {
            auth: Some(Auth {
                username: Some("u".into()),
                password: Some(SecretString::from("pass")),
                ..Default::default()
            }),
            auth_type: Some("password".into()),
            ..Default::default()
        };
        let split = cfg.split_sensitive().unwrap();
        let public = split.public.auth.unwrap();
        assert_eq!(public.username.as_deref(), Some("u"));
        assert!(public.password.is_none());
        assert_eq!(
            split.secure.auth.unwrap().password.unwrap().expose_secret(),
            "pass"
        );
    }

    #[test]
    fn test_plugin_sensitive_fields() {
        let cfg = CloudConfig {
            auth_type: Some("v3totp".into()),
            ..Default::default()
        };
        let fields = openstack_sdk_core::auth::plugin_sensitive_fields(&cfg);
        assert!(fields.contains("passcode"));
        assert!(!fields.contains("username"));

        // plugin not linked in: nothing is reported
        let cfg = CloudConfig {
            auth_type: Some("unknown".into()),
            ..Default::default()
        };
        assert!(openstack_sdk_core::auth::plugin_sensitive_fields(&cfg).is_empty());
    }
}
