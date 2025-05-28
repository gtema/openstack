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
//! OpenStackClient configuration
//!
//! It is possible to configure different aspects of the OpenStackClient (not the clouds connection
//! credentials) using the configuration file (`$XDG_CONFIG_DIR/osc/config.yaml`). This enables
//! user to configurate which columns should be returned when no corresponding run time arguments
//! on a resource base.
//!
//! ```yaml
//! views:
//!   compute.server:
//!     # Listing compute servers will only return ID, NAME and IMAGE columns unless `-o wide` or
//!     `-f XXX` parameters are being passed
//!     fields: [id, name, image]
//!   dns.zone/recordset:
//!     # DNS zone recordsets are listed in the wide mode by default.
//!     wide: true
//! ```

use eyre::Result;
use serde::Deserialize;
use std::{
    collections::HashMap,
    fmt,
    path::{Path, PathBuf},
};
use thiserror::Error;
use tracing::error;

const CONFIG: &str = include_str!("../.config/config.yaml");

/// Errors which may occur when dealing with OpenStack connection
/// configuration data.
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum ConfigError {
    /// Parsing error
    #[error("failed to parse config: {}", source)]
    Parse {
        /// The source of the error.
        #[from]
        source: config::ConfigError,
    },
    /// Parsing error
    #[error("failed to parse config: {}", source)]
    Builder {
        /// The source of the error.
        #[from]
        source: ConfigBuilderError,
    },
}

impl ConfigError {
    /// Build a `[ConfigError::Parse]` error from `[ConfigError]`
    pub fn parse(source: config::ConfigError) -> Self {
        ConfigError::Parse { source }
    }
    /// Build a `[ConfigError::Builder]` error from `[ConfigBuilderError]`
    pub fn builder(source: ConfigBuilderError) -> Self {
        ConfigError::Builder { source }
    }
}

/// Errors which may occur when adding sources to the [`ConfigBuilder`].
#[derive(Error)]
#[non_exhaustive]
pub enum ConfigBuilderError {
    /// File parsing error
    #[error("failed to parse file {path:?}: {source}")]
    FileParse {
        /// Error source
        source: Box<config::ConfigError>,
        /// Builder object
        builder: ConfigBuilder,
        /// Error file path
        path: PathBuf,
    },
    /// Config file deserialization error
    #[error("failed to deserialize config {path:?}: {source}")]
    ConfigDeserialize {
        /// Error source
        source: Box<config::ConfigError>,
        /// Builder object
        builder: ConfigBuilder,
        /// Error file path
        path: PathBuf,
    },
}
///
/// Output configuration
///
/// This structure is controlling how the table table is being built for a structure.
#[derive(Clone, Debug, Default, Deserialize)]
pub struct ViewConfig {
    /// Limit fields (their titles) to be returned
    #[serde(default)]
    pub default_fields: Vec<String>,
    /// Fields configurations
    #[serde(default)]
    pub fields: Vec<FieldConfig>,
    /// Defaults to wide mode
    #[serde(default)]
    pub wide: Option<bool>,
}

/// Field output configuration
#[derive(Clone, Debug, Default, Deserialize, Eq, Ord, PartialOrd, PartialEq)]
pub struct FieldConfig {
    /// Attribute name
    pub name: String,
    /// Fixed width of the column
    #[serde(default)]
    pub width: Option<usize>,
    /// Min width of the column
    #[serde(default)]
    pub min_width: Option<usize>,
    /// Max width of the column
    #[serde(default)]
    pub max_width: Option<usize>,
    /// [JSON pointer](https://datatracker.ietf.org/doc/html/rfc6901) to extract data from the
    /// field
    #[serde(default)]
    pub json_pointer: Option<String>,
}

const fn _default_true() -> bool {
    true
}

/// OpenStackClient configuration
#[derive(Clone, Debug, Default, Deserialize)]
pub struct Config {
    /// Map of views with the key being the resource key `<SERVICE_TYPE>.<RESOURCE>[/<SUBRESOURCE>]`)
    /// and the value being an `[OutputConfig]`
    #[serde(default)]
    pub views: HashMap<String, ViewConfig>,
    /// List of CLI hints per resource
    #[serde(default)]
    pub command_hints: HashMap<String, HashMap<String, Vec<String>>>,
    /// General hints for the CLI to be used independent on the command
    #[serde(default)]
    pub hints: Vec<String>,
    /// Enable/disable show the hints after successful command execution. Enabled by default
    #[serde(default = "_default_true")]
    pub enable_hints: bool,
}

/// A builder to create a [`ConfigFile`] by specifying which files to load.
pub struct ConfigBuilder {
    /// Config source files
    sources: Vec<config::Config>,
}

impl ConfigBuilder {
    /// Add a source to the builder. This will directly parse the config and check if it is valid.
    /// Values of sources added first will be overridden by later added sources, if the keys match.
    /// In other words, the sources will be merged, with the later taking precedence over the
    /// earlier ones.
    pub fn add_source(mut self, source: impl AsRef<Path>) -> Result<Self, ConfigBuilderError> {
        let config = match config::Config::builder()
            .add_source(config::File::from(source.as_ref()))
            .build()
        {
            Ok(config) => config,
            Err(error) => {
                return Err(ConfigBuilderError::FileParse {
                    source: Box::new(error),
                    builder: self,
                    path: source.as_ref().to_owned(),
                });
            }
        };

        if let Err(error) = config.clone().try_deserialize::<Config>() {
            return Err(ConfigBuilderError::ConfigDeserialize {
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
    pub fn build(self) -> Result<Config, ConfigError> {
        let mut config = config::Config::builder();

        for source in self.sources {
            config = config.add_source(source);
        }

        Ok(config.build()?.try_deserialize::<Config>()?)
    }
}

impl fmt::Debug for ConfigBuilderError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConfigBuilderError::FileParse { source, path, .. } => f
                .debug_struct("FileParse")
                .field("source", source)
                .field("path", path)
                .finish_non_exhaustive(),
            ConfigBuilderError::ConfigDeserialize { source, path, .. } => f
                .debug_struct("ConfigDeserialize")
                .field("source", source)
                .field("path", path)
                .finish_non_exhaustive(),
        }
    }
}

impl Config {
    /// Get the config builder
    pub fn builder() -> ConfigBuilder {
        let default_config: config::Config = config::Config::builder()
            .add_source(config::File::from_str(CONFIG, config::FileFormat::Yaml))
            .build()
            .expect("default config must be valid");

        ConfigBuilder {
            sources: Vec::from([default_config]),
        }
    }

    /// Instantiate new config reading default config updating it with local configuration
    pub fn new() -> Result<Self, ConfigError> {
        let default_config: config::Config = config::Config::builder()
            .add_source(config::File::from_str(CONFIG, config::FileFormat::Yaml))
            .build()?;

        let config_dir = get_config_dir();
        let mut builder = ConfigBuilder {
            sources: Vec::from([default_config]),
        };

        let config_files = [
            ("config.yaml", config::FileFormat::Yaml),
            ("views.yaml", config::FileFormat::Yaml),
        ];
        let mut found_config = false;
        for (file, _format) in &config_files {
            if config_dir.join(file).exists() {
                found_config = true;

                builder = match builder.add_source(config_dir.join(file)) {
                    Ok(builder) => builder,
                    Err(ConfigBuilderError::FileParse { source, .. }) => {
                        return Err(ConfigError::parse(*source));
                    }
                    Err(ConfigBuilderError::ConfigDeserialize {
                        source,
                        builder,
                        path,
                    }) => {
                        error!(
                            "The file {path:?} could not be deserialized and will be ignored: {source}"
                        );
                        builder
                    }
                }
            }
        }
        if !found_config {
            tracing::error!("No configuration file found. Application may not behave as expected");
        }

        builder.build()
    }
}

fn get_config_dir() -> PathBuf {
    dirs::config_dir()
        .expect("Cannot determine users XDG_CONFIG_HOME")
        .join("osc")
}

impl fmt::Display for Config {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::Builder;

    #[test]
    fn test_parse_config() {
        let mut config_file = Builder::new().suffix(".yaml").tempfile().unwrap();

        const CONFIG_DATA: &str = r#"
            views:
              foo:
                default_fields: ["a", "b", "c"]
              bar:
                fields:
                  - name: "b"
                    min_width: 1
            command_hints:
              res:
                cmd:
                  - hint1
                  - hint2
            hints:
              - hint1
              - hint2
            enable_hints: true
        "#;

        write!(config_file, "{}", CONFIG_DATA).unwrap();

        let _cfg = Config::builder()
            .add_source(config_file.path())
            .unwrap()
            .build();
    }
}
