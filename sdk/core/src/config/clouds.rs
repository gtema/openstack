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
//! `clouds.yaml` configuration files.
//!
//! Handling of the `clouds.yaml`, `secure.yaml` and `clouds-public.yaml` files (the format known
//! from the python `openstacksdk`). The files are merged into the [`ConfigFile`], which hands out
//! [`CloudConfig`] objects by the cloud name.
//!
//! ```yaml
//! clouds:
//!   devstack:
//!     auth:
//!       auth_url: https://keystone.example.com/v3
//!       username: alice
//!       project_name: demo
//!       user_domain_name: Default
//!       project_domain_name: Default
//!     region_name: RegionOne
//! ```
//!
//! # Search locations
//!
//! `clouds.{yaml,yml,json}`, `secure.{yaml,yml,json}` and `clouds-public.{yaml,yml,json}` are
//! searched in the following directories (in this order):
//!
//! - the current working directory
//! - `$XDG_CONFIG_HOME/openstack`
//! - `~/.config/openstack`
//! - `/etc/openstack`
//!
//! Files found later in the list are merged on top of the earlier ones, with `secure.yaml`
//! loaded after `clouds.yaml` so that it can override the values. In addition the
//! `OS_CLIENT_CONFIG_PATH` environment variable may list files and directories (`:` separated)
//! to be loaded before everything else (see [`find_config_files_specified_in_env`]).
//!
//! Note that unlike [`contexts`](super::contexts) the current working directory **is** searched.
//!
//! # Examples
//!
//! Load the files from the default locations and get a cloud by the name:
//!
//! ```rust
//! use openstack_sdk_core::config::ConfigFile;
//!
//! let cfg = ConfigFile::new().unwrap();
//! let profile = cfg.get_cloud_config("devstack").unwrap();
//! // `None` when there is no such cloud
//! assert!(profile.is_none() || profile.unwrap().name.as_deref() == Some("devstack"));
//! ```
//!
//! List the names of the available clouds (i.e. to let the user choose):
//!
//! ```rust
//! # use openstack_sdk_core::config::ConfigFile;
//! let cfg = ConfigFile::new().unwrap();
//! for name in cfg.get_available_clouds() {
//!     println!("{name}");
//! }
//! ```
//!
//! Explicit files (i.e. coming from the `--os-client-config-file` and `--os-client-secure-file`
//! options) are loaded in addition to the default locations and take precedence over them:
//!
//! ```no_run
//! # use openstack_sdk_core::config::ConfigFile;
//! let cfg = ConfigFile::new_with_user_specified_configs(
//!     Some("c1.yaml"),
//!     Some("s2.yaml"),
//! ).expect("Failed to load the configuration files");
//! ```
//!
//! Only the files that were passed explicitly (nothing is searched for) are used with the
//! [builder](ConfigFileBuilder). Sources added later override the earlier ones:
//!
//! ```no_run
//! # use openstack_sdk_core::config::ConfigFile;
//! let cfg = ConfigFile::builder()
//!     .add_source("clouds.yaml")
//!     .expect("Failed to load 'clouds.yaml'")
//!     .add_source("secure.yaml")
//!     .expect("Failed to load 'secure.yaml'")
//!     .build()
//!     .unwrap();
//! ```
//!
//! The same through the [selector](crate::config::selector), which applies the precedence between
//! `clouds.yaml`, `contexts.yaml` and the environment:
//!
//! ```no_run
//! use openstack_sdk_core::config::selector::ConnectionSelector;
//!
//! let resolved = ConnectionSelector {
//!     cloud: Some("devstack".into()),
//!     config_file: Some("c1.yaml".into()),
//!     ..Default::default()
//! }
//! .resolve()
//! .unwrap();
//! println!("{} comes from {:?}", resolved.name, resolved.source);
//! ```

use std::collections::HashMap;
use std::env;
use std::fmt;
use std::path::{Path, PathBuf};

use serde::Deserialize;
use thiserror::Error;
use tracing::{debug, error, trace, warn};

use super::{CloudConfig, ConfigError};

/// Errors which may occur when adding sources to the [`ConfigFileBuilder`].
#[derive(Error)]
#[non_exhaustive]
pub enum ConfigFileBuilderError {
    /// Failed to parse the configuration file.
    #[error("Failed to parse file {path:?}: {source}")]
    FileParse {
        source: Box<config::ConfigError>,
        builder: ConfigFileBuilder,
        path: PathBuf,
    },

    /// Failed to deserialize the configuration file to the expected structure.
    #[error("Failed to deserialize config {path:?}: {source}")]
    ConfigDeserialize {
        source: Box<config::ConfigError>,
        builder: ConfigFileBuilder,
        path: PathBuf,
    },

    /// Config file deserialization error.
    #[error("Failed to build config: {source}")]
    ConfigBuild { source: Box<config::ConfigError> },
}

impl fmt::Debug for ConfigFileBuilderError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConfigFileBuilderError::FileParse { source, path, .. } => f
                .debug_struct("FileParse")
                .field("source", source)
                .field("path", path)
                .finish_non_exhaustive(),
            ConfigFileBuilderError::ConfigDeserialize { source, path, .. } => f
                .debug_struct("ConfigDeserialize")
                .field("source", source)
                .field("path", path)
                .finish_non_exhaustive(),
            ConfigFileBuilderError::ConfigBuild { source, .. } => f
                .debug_struct("ConfigBuild")
                .field("source", source)
                .finish_non_exhaustive(),
        }
    }
}

/// A builder to create a [`ConfigFile`] by specifying which files to load.
pub struct ConfigFileBuilder {
    /// List of the configuration sources.
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
                    source: Box::new(error),
                    builder: self,
                    path: source.as_ref().to_owned(),
                });
            }
        };

        if let Err(error) = config.clone().try_deserialize::<ConfigFile>() {
            return Err(ConfigFileBuilderError::ConfigDeserialize {
                source: Box::new(error),
                builder: self,
                path: source.as_ref().to_owned(),
            });
        }

        self.sources.push(config);
        Ok(self)
    }

    /// This will build a [`ConfigFile`] with the previously specified sources. Since
    /// the sources have already been checked on errors, this will not fail.
    pub fn build(self) -> Result<ConfigFile, ConfigFileBuilderError> {
        let mut config = config::Config::builder();

        for source in self.sources {
            config = config.add_source(source);
        }

        config
            .build()
            .map_err(|err| ConfigFileBuilderError::ConfigBuild {
                source: Box::new(err),
            })?
            .try_deserialize()
            .map_err(|err| ConfigFileBuilderError::ConfigBuild {
                source: Box::new(err),
            })
    }
}

/// CacheConfig structure.
///
/// A configuration for the built-in authentication caching.
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
#[derive(Deserialize, Debug, Clone)]
pub struct CacheConfig {
    /// Enables/disables authentication caching.
    pub auth: Option<bool>,
}

/// ConfigFile structure.
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
#[derive(Deserialize, Debug, Clone)]
pub struct ConfigFile {
    /// Cache configuration.
    pub cache: Option<CacheConfig>,
    /// clouds configuration.
    pub clouds: Option<HashMap<String, CloudConfig>>,
    /// vendor clouds information (profiles).
    #[serde(rename = "public-clouds")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "public-clouds"))]
    pub public_clouds: Option<HashMap<String, CloudConfig>>,
}

const CONFIG_SUFFIXES: &[&str] = &[".yaml", ".yml", ".json"];

/// Get Paths in which to search for the configuration file.
fn get_config_file_search_paths<S: AsRef<str>>(filename: S) -> Vec<PathBuf> {
    config_file_search_paths(filename, true)
}

/// Get Paths in which to search for the configuration file, optionally including the current
/// working directory.
fn config_file_search_paths<S: AsRef<str>>(filename: S, with_cwd: bool) -> Vec<PathBuf> {
    let paths: Vec<PathBuf> = vec![
        env::current_dir()
            .inspect_err(|e| warn!("Cannot determine current workdir: {}", e))
            .ok()
            .filter(|_| with_cwd),
        dirs::config_dir()
            .or_else(|| {
                warn!("Cannot determine users XDG_CONFIG_HOME");
                None
            })
            .map(|val| val.join("openstack")),
        dirs::home_dir()
            .or_else(|| {
                warn!("Cannot determine users XDG_HOME");
                None
            })
            .map(|val| val.join(".config/openstack")),
        Some(PathBuf::from("/etc/openstack")),
    ]
    .into_iter()
    .flatten()
    .collect();

    paths
        .iter()
        .flat_map(|x| {
            CONFIG_SUFFIXES
                .iter()
                .map(|y| x.join(format!("{}{}", filename.as_ref(), y)))
        })
        .collect()
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

/// Load the vendor profiles (`clouds-public.{yaml,yml,json}`) from the user and system
/// configuration directories only.
///
/// Neither the current working directory nor `OS_CLIENT_CONFIG_PATH` are searched: the profiles
/// are applied to a connection defined in the `contexts.yaml`, which is not allowed to be
/// influenced by the content of an untrusted directory.
pub fn load_trusted_profiles() -> Result<HashMap<String, CloudConfig>, ConfigError> {
    let Some(path) = config_file_search_paths("clouds-public", false)
        .into_iter()
        .find(|path| path.is_file())
    else {
        return Ok(HashMap::new());
    };
    match ConfigFile::builder().add_source(&path) {
        Ok(builder) => Ok(builder.build()?.public_clouds.unwrap_or_default()),
        Err(ConfigFileBuilderError::FileParse { source, .. }) => Err(ConfigError::parse(*source)),
        Err(err) => Err(ConfigError::ConfigFileBuilder { source: err }),
    }
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

/// Returns list of all configuration files pointed at with `OS_CLIENT_CONFIG_PATH` environment
/// variable.
///
/// The variable can point to the concrete file result or be a ":" separated list of the search
/// items. Example: "~/clouds.yaml:~/secure.yaml:/etc/clouds.yaml:~/.config/openstack". When an
/// item is a file it is being added into the resulting list. An item being a directory is used as
/// a base to search regular `clouds.yaml` and `secure.yaml` files which are being add into the list
/// when existing in the directory.
///
pub fn find_config_files_specified_in_env() -> impl IntoIterator<Item = PathBuf> {
    let mut results: Vec<PathBuf> = Vec::new();
    if let Ok(configs) = env::var("OS_CLIENT_CONFIG_PATH") {
        debug!(
            "Searching for the OpenStack client config files in {}.",
            configs
        );
        for candidate in configs.split(":") {
            if let Some(path) = expand_path(candidate) {
                if path.is_file() {
                    results.push(path);
                } else if path.is_dir() {
                    for config_prefix in ["clouds", "secure"] {
                        CONFIG_SUFFIXES
                            .iter()
                            .map(|y| path.join(format!("{}{}", config_prefix, y)))
                            .find(|path| path.is_file())
                            .inspect(|path| results.push(path.to_owned()));
                    }
                }
            }
        }
    }
    results
}

/// Expand the `~` in the path with the HOME environment variable.
fn expand_path(path: &str) -> Option<PathBuf> {
    if path.starts_with("~/") {
        dirs::home_dir().map(|home| home.join(&path[2..]))
    } else {
        Some(PathBuf::from(path))
    }
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

        for path in find_config_files_specified_in_env()
            .into_iter()
            .chain(find_vendor_file())
            .chain(find_clouds_file())
            .chain(clouds.map(|path| path.as_ref().to_owned()))
            .chain(find_secure_file())
            .chain(secure.map(|path| path.as_ref().to_owned()))
        {
            trace!("Try load configuration file {:?}", path);
            builder = match builder.add_source(&path) {
                Ok(builder) => {
                    trace!("Using config file {path:?}");
                    builder
                }
                Err(ConfigFileBuilderError::FileParse { source, .. }) => {
                    return Err(ConfigError::parse(*source));
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
                Err(err @ ConfigFileBuilderError::ConfigBuild { .. }) => {
                    return Err(ConfigError::ConfigFileBuilder { source: err });
                }
            };
        }

        Ok(builder.build()?)
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
        if let Some(clouds) = &self.clouds
            && let Some(cfg) = clouds.get(cloud_name.as_ref())
        {
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
            config.name = Some(cloud_name.as_ref().to_string());

            return Ok(Some(config));
        }
        Ok(None)
    }

    /// Return true if auth caching is enabled.
    pub fn is_auth_cache_enabled(&self) -> bool {
        self.cache.as_ref().and_then(|c| c.auth).unwrap_or(true)
    }

    /// Return list of available cloud connections.
    pub fn get_available_clouds(&self) -> Vec<String> {
        if let Some(clouds) = &self.clouds {
            return clouds.keys().cloned().collect();
        }
        Vec::new()
    }
}
#[cfg(test)]
mod tests {
    use secrecy::ExposeSecret;
    use std::env;
    use std::fs::File;
    use std::io::Write;
    use std::path::PathBuf;
    use tempfile::{Builder, tempdir};

    use super::*;

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
            get_config_file_search_paths(fname)
        );
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

        write!(cloud_file, "{CLOUD_DATA}").unwrap();
        write!(secure_file, "{SECURE_DATA}").unwrap();

        let cfg = ConfigFile::builder()
            .add_source(cloud_file.path())
            .unwrap()
            .add_source(secure_file.path())
            .unwrap()
            .build()
            .unwrap();

        let profile = cfg
            .get_cloud_config("fake_cloud")
            .unwrap()
            .expect("Profile exists");
        let auth = profile.auth.expect("Auth defined");

        assert_eq!(auth.auth_url, Some(String::from("http://fake.com")));
        assert_eq!(auth.username, Some(String::from("foo")));
        assert_eq!(auth.password.unwrap().expose_secret(), String::from("bar"));
        assert_eq!(profile.name, Some(String::from("fake_cloud")));
    }

    #[test]
    fn test_from_os_client_config_path_env() {
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

        write!(cloud_file, "{CLOUD_DATA}").unwrap();
        write!(secure_file, "{SECURE_DATA}").unwrap();
        let cfg = ConfigFile::new().unwrap();

        assert!(cfg.get_cloud_config("fake_cloud").unwrap().is_none());

        // now add both files explicitly into the env var and verify all data is fetched
        unsafe {
            env::set_var(
                "OS_CLIENT_CONFIG_PATH",
                format!(
                    "{}:{}",
                    cloud_file.path().display(),
                    secure_file.path().display()
                ),
            );
        }

        let cfg = ConfigFile::new().unwrap();
        let profile = cfg
            .get_cloud_config("fake_cloud")
            .unwrap()
            .expect("Profile exists");
        let auth = profile.auth.expect("Auth defined");

        assert_eq!(auth.auth_url, Some(String::from("http://fake.com")));
        assert_eq!(auth.username, Some(String::from("foo")));
        assert_eq!(auth.password.unwrap().expose_secret(), String::from("bar"));
        assert_eq!(profile.name, Some(String::from("fake_cloud")));
        // with only directory containing those files they should not be used unless they are named
        // properly
        unsafe {
            env::set_var(
                "OS_CLIENT_CONFIG_PATH",
                format!(
                    "{}:",
                    cloud_file.path().parent().expect("no parent").display(),
                ),
            );
        }
        let cfg = ConfigFile::new().unwrap();
        assert!(
            cfg.get_cloud_config("fake_cloud").unwrap().is_none(),
            "Nothing should be found in {:?}",
            env::var("OS_CLIENT_CONFIG_PATH")
        );

        // env points at the dir and there is clouds.yaml file there
        let tmp_dir = tempdir().unwrap();
        unsafe {
            env::set_var(
                "OS_CLIENT_CONFIG_PATH",
                format!("{}:", tmp_dir.path().display(),),
            );
        }
        let file_path = tmp_dir.path().join("clouds.yml");
        let mut tmp_file = File::create(file_path).unwrap();
        write!(tmp_file, "{CLOUD_DATA}").unwrap();

        let cfg = ConfigFile::new().unwrap();
        let profile = cfg
            .get_cloud_config("fake_cloud")
            .unwrap()
            .expect("Profile exists");
        let auth = profile.auth.expect("Auth defined");

        assert_eq!(auth.auth_url, Some(String::from("http://fake.com")));
        assert_eq!(auth.username, Some(String::from("override_me")));
    }
}
