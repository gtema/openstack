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

//! Module to handle OpenStack config
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

use std::fmt;
use std::path::{Path, PathBuf};
use tracing::{error, warn};

use serde::Deserialize;
use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::collections::HashSet;
use std::env;
use std::hash::{Hash, Hasher};

use thiserror::Error;

/// Errors which may occur when dealing with OpenStack connection
/// configuration data.
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum ConfigError {
    #[error("Cloud {0} not found")]
    CloudNotFound(String),

    #[error("Profile {} not found", profile_name)]
    MissingProfile { profile_name: String },

    #[error("unknown error")]
    Unknown,

    #[error("failed to deserialize config: {}", source)]
    Parse {
        /// The source of the error.
        #[from]
        source: config::ConfigError,
    },
}

impl ConfigError {
    pub fn parse(source: config::ConfigError) -> Self {
        ConfigError::Parse { source }
    }
}

/// Errors which may occur when adding sources to the [`ConfigFileBuilder`].
#[derive(Error)]
#[non_exhaustive]
pub enum ConfigFileBuilderError {
    #[error("Failed to parse file {path:?}: {source}")]
    FileParse {
        source: config::ConfigError,
        builder: ConfigFileBuilder,
        path: PathBuf,
    },
    #[error("Failed to deserialize config {path:?}: {source}")]
    ConfigDeserialize {
        source: config::ConfigError,
        builder: ConfigFileBuilder,
        path: PathBuf,
    },
}

impl fmt::Debug for ConfigFileBuilderError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConfigFileBuilderError::FileParse {
                ref source,
                ref path,
                ..
            } => f
                .debug_struct("FileParse")
                .field("source", source)
                .field("path", path)
                .finish_non_exhaustive(),
            ConfigFileBuilderError::ConfigDeserialize {
                ref source,
                ref path,
                ..
            } => f
                .debug_struct("ConfigDeserialize")
                .field("source", source)
                .field("path", path)
                .finish_non_exhaustive(),
        }
    }
}

/// A builder to create a [`ConfigFile`] by specifying which files to load.
pub struct ConfigFileBuilder {
    sources: Vec<config::Config>,
}

impl ConfigFileBuilder {
    /// Add a source to the builder. This will directly parse the config and check if it is valid.
    /// Values of sources added first will be overridden by later added sources, if the keys match.
    /// In other words, the sources will be merged, with the later taking precedence over the
    /// earlier ones.
    pub fn add_source(mut self, source: impl AsRef<Path>) -> Result<Self, ConfigFileBuilderError> {
        let config = match config::Config::builder()
            .add_source(config::File::from(source.as_ref()))
            .build()
        {
            Ok(config) => config,
            Err(error) => {
                return Err(ConfigFileBuilderError::FileParse {
                    source: error,
                    builder: self,
                    path: source.as_ref().to_owned(),
                })
            }
        };

        if let Err(error) = config.clone().try_deserialize::<ConfigFile>() {
            return Err(ConfigFileBuilderError::ConfigDeserialize {
                source: error,
                builder: self,
                path: source.as_ref().to_owned(),
            });
        }

        self.sources.push(config);
        Ok(self)
    }

    /// This will build a [`ConfigFile`] with the previously specified sources. Since
    /// the sources have already been checked on errors, this will not fail.
    pub fn build(self) -> ConfigFile {
        let mut config = config::Config::builder();

        for source in self.sources {
            config = config.add_source(source);
        }

        config.build().unwrap().try_deserialize().unwrap()
    }
}

/// CacheConfig structure
#[derive(Deserialize, Debug, Clone)]
pub struct CacheConfig {
    pub auth: Option<bool>,
}

/// ConfigFile structure
#[derive(Deserialize, Debug, Clone)]
pub struct ConfigFile {
    /// Cache configuration
    pub cache: Option<CacheConfig>,
    /// clouds configuration
    pub clouds: Option<HashMap<String, CloudConfig>>,
    /// vendor clouds information (profiles)
    #[serde(rename = "public-clouds")]
    pub public_clouds: Option<HashMap<String, CloudConfig>>,
}

/// Authentication data
#[derive(Clone, Default, Deserialize)]
pub struct Auth {
    /// Authentication URL
    pub(crate) auth_url: Option<String>,
    /// Authentication endpoint type (public/internal/admin)
    pub(crate) endpoint: Option<String>,
    /// Auth Token
    pub(crate) token: Option<String>,

    /// Auth User.Name
    pub(crate) username: Option<String>,
    /// Auth User.ID
    pub(crate) user_id: Option<String>,
    /// Auth User.Domain.Name
    pub(crate) user_domain_name: Option<String>,
    /// Auth User.Domain.ID
    pub(crate) user_domain_id: Option<String>,
    /// Auth User password
    pub(crate) password: Option<String>,

    /// Auth (totp) MFA passcode
    pub(crate) passcode: Option<String>,

    /// `Domain` scope Domain.ID
    pub(crate) domain_id: Option<String>,
    /// `Domain` scope Domain.Name
    pub(crate) domain_name: Option<String>,
    /// `Project` scope Project.ID
    pub(crate) project_id: Option<String>,
    /// `Project` scope Project.Name
    pub(crate) project_name: Option<String>,
    /// `Project` scope Project.Domain.ID
    pub(crate) project_domain_id: Option<String>,
    /// `Project` scope Project.Domain.Name
    pub(crate) project_domain_name: Option<String>,

    /// `Federation` protocol
    pub(crate) protocol: Option<String>,
    /// `Federation` identity provider
    pub(crate) identity_provider: Option<String>,

    /// `Application Credential` ID
    pub(crate) application_credential_id: Option<String>,
    /// `Application Credential` Name
    pub(crate) application_credential_name: Option<String>,
    /// `Application Credential` Secret
    pub(crate) application_credential_secret: Option<String>,
}

impl fmt::Debug for Auth {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Auth")
            .field("auth_url", &self.auth_url)
            .field("domain_id", &self.domain_id)
            .field("domain_name", &self.domain_name)
            .field("project_id", &self.project_id)
            .field("project_name", &self.project_name)
            .field("project_domain_id", &self.project_domain_id)
            .field("project_domain_name", &self.project_domain_name)
            .field("username", &self.username)
            .field("user_domain_id", &self.user_domain_id)
            .field("user_domain_name", &self.user_domain_name)
            .field("protocol", &self.protocol)
            .field("identity_provider", &self.identity_provider)
            .field("application_credential_id", &self.application_credential_id)
            .field(
                "application_credential_name",
                &self.application_credential_name,
            )
            .field(
                "application_credential_secret",
                &self.application_credential_secret,
            )
            .finish()
    }
}

/// CloudConfig structure
#[derive(Deserialize, Default, Debug, Clone)]
pub struct CloudConfig {
    /// Authorization data
    pub(crate) auth: Option<Auth>,
    /// Authorization type. While it can be enum it would make hard to extend SDK with custom implementations
    pub auth_type: Option<String>,
    /// Authorization methods (in the case when auth_type = `multifactor`.
    pub auth_methods: Option<Vec<String>>,

    /// Vendor Profile (by name from clouds-public.yaml or TBD: URL)
    pub profile: Option<String>,
    /// Interface name to be used for endpoints selection
    pub interface: Option<String>,
    /// Region name
    pub region_name: Option<String>,

    /// Custom CA Certificate
    pub cacert: Option<String>,
    /// Verify SSL Certificates
    pub verify: Option<bool>,

    /// All other options
    #[serde(flatten)]
    pub options: HashMap<String, config::Value>,
}

/// Get a user authentication hash
pub fn get_config_identity_hash(config: &CloudConfig) -> u64 {
    // Calculate hash of the auth information
    let mut s = DefaultHasher::new();
    if let Some(auth) = &config.auth {
        if let Some(data) = &auth.auth_url {
            data.hash(&mut s);
        }
        if let Some(data) = &auth.username {
            data.hash(&mut s);
        }
        if let Some(data) = &auth.user_id {
            data.hash(&mut s);
        }
        if let Some(data) = &auth.user_domain_id {
            data.hash(&mut s);
        }
        if let Some(data) = &auth.user_domain_name {
            data.hash(&mut s);
        }
        if let Some(data) = &auth.identity_provider {
            data.hash(&mut s);
        }
        if let Some(data) = &auth.protocol {
            data.hash(&mut s);
        }
        if let Some(data) = &auth.application_credential_name {
            data.hash(&mut s);
        }
        if let Some(data) = &auth.application_credential_id {
            data.hash(&mut s);
        }
    }
    if let Some(data) = &config.profile {
        data.hash(&mut s);
    }
    s.finish()
}

/// CloudConfig struct implementation
impl CloudConfig {
    /// Update unset CloudConfig with values from the `update` var
    pub fn update(&mut self, update: &CloudConfig) {
        if let Some(update_auth) = &update.auth {
            let auth = self.auth.get_or_insert(Auth::default());
            if auth.auth_url.is_none() && update_auth.auth_url.is_some() {
                auth.auth_url.clone_from(&update_auth.auth_url);
            }
            if auth.domain_id.is_none() && update_auth.domain_id.is_some() {
                auth.domain_id.clone_from(&update_auth.domain_id);
            }
            if auth.domain_name.is_none() && update_auth.domain_name.is_some() {
                auth.domain_name.clone_from(&update_auth.domain_name);
            }
            if auth.endpoint.is_none() && update_auth.endpoint.is_some() {
                auth.endpoint.clone_from(&update_auth.endpoint);
            }
            if auth.password.is_none() && update_auth.password.is_some() {
                auth.password.clone_from(&update_auth.password);
            }
            if auth.project_id.is_none() && update_auth.project_id.is_some() {
                auth.project_id.clone_from(&update_auth.project_id);
            }
            if auth.project_name.is_none() && update_auth.project_name.is_some() {
                auth.project_name.clone_from(&update_auth.project_name);
            }
            if auth.project_domain_id.is_none() && update_auth.project_domain_id.is_some() {
                auth.project_domain_id
                    .clone_from(&update_auth.project_domain_id);
            }
            if auth.project_domain_name.is_none() && update_auth.project_domain_name.is_some() {
                auth.project_domain_name
                    .clone_from(&update_auth.project_domain_name);
            }
            if auth.token.is_none() && update_auth.token.is_some() {
                auth.token.clone_from(&update_auth.token);
            }
            if auth.username.is_none() && update_auth.username.is_some() {
                auth.username.clone_from(&update_auth.username);
            }
            if auth.user_domain_name.is_none() && update_auth.user_domain_name.is_some() {
                auth.user_domain_name
                    .clone_from(&update_auth.user_domain_name);
            }
            if auth.user_domain_id.is_none() && update_auth.user_domain_id.is_some() {
                auth.user_domain_id.clone_from(&update_auth.user_domain_id);
            }
            if auth.protocol.is_none() && update_auth.protocol.is_some() {
                auth.protocol.clone_from(&update_auth.protocol);
            }
            if auth.identity_provider.is_none() && update_auth.identity_provider.is_some() {
                auth.identity_provider
                    .clone_from(&update_auth.identity_provider);
            }
            if auth.application_credential_id.is_none()
                && update_auth.application_credential_id.is_some()
            {
                auth.application_credential_id
                    .clone_from(&update_auth.application_credential_id);
            }
            if auth.application_credential_name.is_none()
                && update_auth.application_credential_name.is_some()
            {
                auth.application_credential_name
                    .clone_from(&update_auth.application_credential_name);
            }
            if auth.application_credential_secret.is_none()
                && update_auth.application_credential_secret.is_some()
            {
                auth.application_credential_secret
                    .clone_from(&update_auth.application_credential_secret);
            }
        }
        if self.auth_type.is_none() && update.auth_type.is_some() {
            self.auth_type.clone_from(&update.auth_type);
        }
        if self.profile.is_none() && update.profile.is_some() {
            self.profile.clone_from(&update.profile);
        }
        if self.interface.is_none() && update.interface.is_some() {
            self.interface.clone_from(&update.interface);
        }
        if self.region_name.is_none() && update.region_name.is_some() {
            self.region_name.clone_from(&update.region_name);
        }
        if self.cacert.is_none() && update.cacert.is_some() {
            self.cacert.clone_from(&update.cacert);
        }
        if self.verify.is_none() && update.verify.is_some() {
            self.verify.clone_from(&update.verify);
        }
        let current_keys: HashSet<String> = self.options.keys().cloned().collect();
        self.options.extend(
            update
                .options
                .clone()
                .into_iter()
                .filter(|x| !current_keys.contains(&x.0)),
        );
    }
}

const CONFIG_SUFFIXES: &[&str] = &[".yaml", ".yml", ".json"];

/// Get Paths in which to search for the configuration file
fn get_config_file_search_paths<S: AsRef<str>>(filename: S) -> Vec<PathBuf> {
    let paths: Vec<PathBuf> = vec![
        env::current_dir().expect("Cannot determine current workdir"),
        dirs::config_dir()
            .expect("Cannot determine users XDG_CONFIG_HOME")
            .join("openstack"),
        dirs::home_dir()
            .expect("Cannot determine users XDG_HOME")
            .join(".config/openstack"),
        PathBuf::from("/etc/openstack"),
    ];

    return paths
        .iter()
        .flat_map(|x| {
            CONFIG_SUFFIXES
                .iter()
                .map(|y| x.join(format!("{}{}", filename.as_ref(), y)))
        })
        .collect();
}

/// Searches for a `clouds-public.{yaml,yml,json}` config file.
///
/// The following locations will be tried in order:
///
/// - `./clouds-public.{yaml,yml,json}` (current working directory)
/// - `$XDG_CONFIG_HOME/openstack/clouds-public.{yaml,yml,json}`
/// - `$XDG_HOME/.config/openstack/clouds-public.{yaml,yml,json}`
/// - `/etc/openstack/clouds-public.{yaml,yml,json}`
pub fn find_vendor_file() -> Option<PathBuf> {
    get_config_file_search_paths("clouds-public")
        .into_iter()
        .find(|path| path.is_file())
}

/// Searches for a `clouds.{yaml,yml,json}` config file.
///
/// The following locations will be tried in order:
///
/// - `./clouds.{yaml,yml,json}` (current working directory)
/// - `$XDG_CONFIG_HOME/openstack/clouds.{yaml,yml,json}`
/// - `$XDG_HOME/.config/openstack/clouds.{yaml,yml,json}`
/// - `/etc/openstack/clouds.{yaml,yml,json}`
pub fn find_clouds_file() -> Option<PathBuf> {
    get_config_file_search_paths("clouds")
        .into_iter()
        .find(|path| path.is_file())
}

/// Searches for a `secure.{yaml,yml,json}` config file.
///
/// The following locations will be tried in order:
///
/// - `./secure.{yaml,yml,json}` (current working directory)
/// - `$XDG_CONFIG_HOME/openstack/secure.{yaml,yml,json}`
/// - `$XDG_HOME/.config/openstack/secure.{yaml,yml,json}`
/// - `/etc/openstack/secure.{yaml,yml,json}`
pub fn find_secure_file() -> Option<PathBuf> {
    get_config_file_search_paths("secure")
        .into_iter()
        .find(|path| path.is_file())
}

impl ConfigFile {
    /// A builder to create a `ConfigFile` by specifying which files to load.
    pub fn builder() -> ConfigFileBuilder {
        ConfigFileBuilder {
            sources: Vec::new(),
        }
    }

    /// Create a `ConfigFile` which also loads the default sources in the following order:
    ///
    /// - `clouds-public.{yaml,yml,json}` (see [`find_vendor_file`] for search paths)
    /// - `clouds.{yaml,yml,json}` (see [`find_clouds_file`] for search paths)
    /// - the provided clouds file
    /// - `secure.{yaml,yml,json}` (see [`find_secure_file`] for search paths)
    /// - the provided secure file
    ///
    /// If a source is not a valid config it will be ignored, but if one of the sources
    /// has syntax errors (YAML/JSON) or one of the user specified configs does not
    /// exist, a [`ConfigError`] will be returned.
    pub fn new_with_user_specified_configs(
        clouds: Option<impl AsRef<Path>>,
        secure: Option<impl AsRef<Path>>,
    ) -> Result<Self, ConfigError> {
        let mut builder = Self::builder();

        for path in find_vendor_file()
            .into_iter()
            .chain(find_clouds_file())
            .chain(clouds.map(|path| path.as_ref().to_owned()))
            .chain(find_secure_file())
            .chain(secure.map(|path| path.as_ref().to_owned()))
        {
            builder = match builder.add_source(path) {
                Ok(builder) => builder,
                Err(ConfigFileBuilderError::FileParse { source, .. }) => {
                    return Err(ConfigError::parse(source));
                }
                Err(ConfigFileBuilderError::ConfigDeserialize {
                    source,
                    builder,
                    path,
                }) => {
                    error!(
                        "The file {path:?} could not be deserialized and will be ignored: {source}"
                    );
                    builder
                }
            };
        }

        Ok(builder.build())
    }

    /// A convenience function which calls [`new_with_user_specified_configs`](ConfigFile::new_with_user_specified_configs) without an
    /// additional clouds or secret file.
    pub fn new() -> Result<Self, ConfigError> {
        Self::new_with_user_specified_configs(None::<PathBuf>, None::<PathBuf>)
    }

    /// Get cloud connection configuration by name.
    ///
    /// This method does not raise exception when the cloud is
    /// not found.
    pub fn get_cloud_config<S: AsRef<str>>(
        &self,
        cloud_name: S,
    ) -> Result<Option<CloudConfig>, ConfigError> {
        if let Some(clouds) = &self.clouds {
            if let Some(cfg) = clouds.get(cloud_name.as_ref()) {
                let mut config = cfg.clone();
                if let Some(ref profile_name) = config.profile {
                    let mut profile_definition: Option<&CloudConfig> = None;
                    // TODO: profile may be URL to .well_known
                    // Merge profile
                    match &self.public_clouds {
                        Some(profiles) => {
                            profile_definition = profiles.get(profile_name);
                        }
                        None => {
                            warn!("Cannot find profiles definition");
                        }
                    }
                    if let Some(profile) = profile_definition {
                        config.update(profile);
                    }
                }

                return Ok(Some(config));
            }
        }
        Ok(None)
    }

    /// Return true if auth caching is enabled
    pub fn is_auth_cache_enabled(&self) -> bool {
        self.cache.as_ref().and_then(|c| c.auth).unwrap_or(true)
    }

    /// Return list of available cloud connections
    pub fn get_available_clouds(&self) -> Vec<String> {
        if let Some(clouds) = &self.clouds {
            return clouds.keys().cloned().collect();
        }
        Vec::new()
    }
}

#[cfg(test)]
mod tests {
    use crate::config;
    use std::env;
    use std::io::Write;
    use std::path::PathBuf;
    use tempfile::Builder;

    use super::ConfigFile;

    #[test]
    fn test_get_search_paths() {
        let fname = "clouds";
        let cwd = env::current_dir().unwrap();
        let conf_dir = dirs::config_dir().unwrap().join("openstack");
        let unix_conf_home = dirs::home_dir().unwrap().join(".config/openstack");
        let site_conf = PathBuf::from("/etc/openstack");
        assert_eq!(
            vec![
                PathBuf::from(format!("{}/{}.yaml", cwd.display(), fname)),
                PathBuf::from(format!("{}/{}.yml", cwd.display(), fname)),
                PathBuf::from(format!("{}/{}.json", cwd.display(), fname)),
                PathBuf::from(format!("{}/{}.yaml", conf_dir.display(), fname)),
                PathBuf::from(format!("{}/{}.yml", conf_dir.display(), fname)),
                PathBuf::from(format!("{}/{}.json", conf_dir.display(), fname)),
                PathBuf::from(format!("{}/{}.yaml", unix_conf_home.display(), fname)),
                PathBuf::from(format!("{}/{}.yml", unix_conf_home.display(), fname)),
                PathBuf::from(format!("{}/{}.json", unix_conf_home.display(), fname)),
                PathBuf::from(format!("{}/{}.yaml", site_conf.display(), fname)),
                PathBuf::from(format!("{}/{}.yml", site_conf.display(), fname)),
                PathBuf::from(format!("{}/{}.json", site_conf.display(), fname)),
            ],
            config::get_config_file_search_paths(fname)
        );
    }

    #[test]
    fn test_default_auth_cache_enabled() {
        let cfg = ConfigFile::new().unwrap();
        assert!(cfg.is_auth_cache_enabled());
    }

    #[test]
    fn test_get_available_clouds() {
        let cfg = ConfigFile::new().unwrap();
        let _ = cfg.get_available_clouds();
    }

    #[test]
    fn test_from_custom_files() {
        let mut cloud_file = Builder::new().suffix(".yaml").tempfile().unwrap();
        let mut secure_file = Builder::new().suffix(".yaml").tempfile().unwrap();

        const CLOUD_DATA: &str = r#"
            clouds:
              fake_cloud:
                auth:
                  auth_url: http://fake.com
                  username: override_me
        "#;
        const SECURE_DATA: &str = r#"
            clouds:
              fake_cloud:
                auth:
                  username: foo
                  password: bar
        "#;

        write!(cloud_file, "{}", CLOUD_DATA).unwrap();
        write!(secure_file, "{}", SECURE_DATA).unwrap();

        let cfg = ConfigFile::builder()
            .add_source(cloud_file.path())
            .unwrap()
            .add_source(secure_file.path())
            .unwrap()
            .build();

        let profile = cfg
            .get_cloud_config("fake_cloud")
            .unwrap()
            .expect("Profile exists");
        let auth = profile.auth.expect("Auth defined");

        assert_eq!(auth.auth_url, Some(String::from("http://fake.com")));
        assert_eq!(auth.username, Some(String::from("foo")));
        assert_eq!(auth.password, Some(String::from("bar")));
    }
}
