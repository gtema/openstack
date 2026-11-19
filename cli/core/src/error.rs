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
//! CLI Errors

use indicatif;
use reqwest;
use thiserror::Error;

/// CLI error type
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum OpenStackCliError {
    /// Json serialization error.
    #[error("failed to serialize data to json: {}", source)]
    SerializeJson {
        /// The source of the error.
        #[from]
        source: serde_json::Error,
    },

    /// Json deserialization error.
    #[error(
        "failed to deserialize data to json. Try using `-o json` to still see the data. \n\t{}",
        data
    )]
    DeserializeJson {
        /// The source of the error.
        source: serde_json::Error,
        /// Source json data
        data: String,
    },

    /// OpenStack Auth error.
    #[error("authentication error")]
    Auth {
        /// The source of the error.
        source: openstack_sdk_core::OpenStackError,
    },
    /// Re-scope error.
    #[error("error changing scope to {:?}", scope)]
    ReScope {
        /// Target scope.
        scope: openstack_sdk_auth_core::authtoken_scope::AuthTokenScope,
        /// The source of the error.
        source: openstack_sdk_core::OpenStackError,
    },

    /// SDK error.
    #[error(transparent)]
    OpenStackSDK {
        /// The source of the error.
        #[from]
        source: openstack_sdk_core::OpenStackError,
    },
    /// OpenStack API error.
    #[error(transparent)]
    OpenStackApi {
        /// The source of the error.
        #[from]
        source: openstack_sdk_core::api::ApiError<openstack_sdk_core::RestError>,
    },

    /// Configuration error.
    #[error(transparent)]
    CliConfig {
        /// The source of the error.
        #[from]
        source: crate::config::ConfigError,
    },

    /// Configuration error.
    #[error(transparent)]
    CloudConfig {
        /// The source of the error.
        #[from]
        source: openstack_sdk_core::config::ConfigError,
    },

    /// `contexts.yaml` error.
    #[error(transparent)]
    Contexts {
        /// The source of the error.
        #[from]
        source: openstack_sdk_core::config::contexts::ContextsError,
    },

    /// OpenStack Service Catalog error.
    #[error(transparent)]
    OpenStackCatalog {
        /// The source of the error.
        #[from]
        source: openstack_sdk_core::catalog::CatalogError,
    },

    /// No subcommands.
    #[error("command has no subcommands")]
    NoSubcommands,

    /// Resource is not found.
    #[error("resource not found")]
    ResourceNotFound,

    /// Resource identifier is not unique.
    #[error("cannot find resource by identifier")]
    IdNotUnique,

    /// Resource attribute is not present.
    #[error("cannot find resource attribute {0}")]
    ResourceAttributeMissing(String),

    /// Resource attribute is not string.
    #[error("resource attribute {0} is not a string")]
    ResourceAttributeNotString(String),

    /// IO error.
    #[error("IO error: {}", source)]
    IO {
        /// The source of the error.
        #[from]
        source: std::io::Error,
    },
    /// Reqwest library error.
    #[error("reqwest error: {}", source)]
    Reqwest {
        /// The source of the error.
        #[from]
        source: reqwest::Error,
    },
    /// Clap library error.
    #[error("argument parsing error: {}", source)]
    Clap {
        /// The source of the error.
        #[from]
        source: clap::error::Error,
    },
    /// Indicativ library error.
    #[error("indicativ error: {}", source)]
    Idinticatif {
        /// The source of the error.
        #[from]
        source: indicatif::style::TemplateError,
    },
    /// Endpoint builder error.
    #[error("OpenStackSDK endpoint builder error: `{0}`")]
    EndpointBuild(String),

    /// `contexts.yaml` is not found.
    #[error(
        "`contexts.yaml` file is not found in {}",
        openstack_sdk_core::config::contexts::contexts_file_search_dirs()
    )]
    ContextsFileNotFound,

    /// Nothing selects the connection.
    #[error(
        "connection is not selected: use `--os-cloud` (`OS_CLOUD`), `--os-context` (`OS_CONTEXT`) \
         or `--cloud-config-from-env`, or set `current-context` in the `contexts.yaml`"
    )]
    NoConnectionSelected,

    /// No connection is configured.
    #[error(
        "no connection is configured: neither `clouds.yaml` with clouds nor `contexts.yaml` with \
         contexts is found, use `--cloud-config-from-env` to read the environment variables"
    )]
    NoConnectionAvailable,

    /// Connection error.
    #[error("cloud connection `{0:?}` cannot be found")]
    ConnectionNotFound(String),

    /// Invalid header name.
    #[error("invalid header name `{}`", source)]
    InvalidHeaderName {
        /// The source of the error.
        #[from]
        source: http::header::InvalidHeaderName,
    },

    /// Invalid header value.
    #[error("invalid header value `{}`", source)]
    InvalidHeaderValue {
        /// The source of the error.
        #[from]
        source: http::header::InvalidHeaderValue,
    },

    /// Invalid URL.
    #[error("invalid url: {}", source)]
    InvalidUri {
        /// The source of the error.
        #[from]
        source: http::uri::InvalidUri,
    },

    /// User interaction using dialoguer crate failed
    #[error("dialoguer error `{}`", source)]
    DialoguerError {
        /// The source of the error.
        #[from]
        source: dialoguer::Error,
    },

    /// Input parameters
    #[error("input parameters error: {0}")]
    InputParameters(String),

    /// Base64 decoding error.
    #[error(transparent)]
    Base64Decode(#[from] base64::DecodeError),

    /// Re-authorization not possible without active authentication.
    #[error("valid authentication is missing to be able to rescope the session")]
    MissingValidAuthenticationForRescope,

    /// URL parsing error
    #[error(transparent)]
    UrlParse {
        /// The source of the error.
        #[from]
        source: url::ParseError,
    },

    /// Others.
    #[error(transparent)]
    Other(#[from] eyre::Report),
}

impl OpenStackCliError {
    /// Build a deserialization error
    pub fn deserialize(error: serde_json::Error, data: String) -> Self {
        Self::DeserializeJson {
            source: error,
            data,
        }
    }

    /// A short, user-facing hint for errors coming from the SDK's
    /// `wait`/`wait_deleted`/`wait_for_status` combinators, distinguishing
    /// timeout, failure, and vanished-resource cases instead of leaving
    /// them as an opaque, generic API error.
    pub fn wait_hint(&self) -> Option<&'static str> {
        let Self::OpenStackApi { source } = self else {
            return None;
        };
        match source {
            openstack_sdk_core::api::ApiError::WaitTimeout { .. } => Some(
                "the resource may still be transitioning; re-run the command later or check its current status directly",
            ),
            openstack_sdk_core::api::ApiError::WaitFailed { .. } => Some(
                "the resource reported a failure state while waiting for it; inspect it directly for details",
            ),
            openstack_sdk_core::api::ApiError::WaitResourceVanished => Some(
                "the resource disappeared while waiting for it to reach the desired state (it may have been deleted concurrently)",
            ),
            _ => None,
        }
    }
}

impl From<openstack_sdk_core::config::selector::SelectionError> for OpenStackCliError {
    fn from(source: openstack_sdk_core::config::selector::SelectionError) -> Self {
        use openstack_sdk_core::config::selector::SelectionError;
        match source {
            SelectionError::Config(source) => Self::CloudConfig { source },
            SelectionError::Contexts(source) => Self::Contexts { source },
            SelectionError::ContextsFileNotFound => Self::ContextsFileNotFound,
            SelectionError::ConnectionNotFound(name) => Self::ConnectionNotFound(name),
            SelectionError::NoSelection {
                clouds, contexts, ..
            } => {
                if clouds.is_empty() && contexts.is_empty() {
                    Self::NoConnectionAvailable
                } else {
                    Self::NoConnectionSelected
                }
            }
            err => Self::Other(eyre::eyre!("{err}")),
        }
    }
}
