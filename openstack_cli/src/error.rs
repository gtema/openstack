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
        source: openstack_sdk::OpenStackError,
    },
    /// Re-scope error.
    #[error("error changing scope to {:?}", scope)]
    ReScope {
        /// Target scope.
        scope: openstack_sdk::auth::authtoken::AuthTokenScope,
        /// The source of the error.
        source: openstack_sdk::OpenStackError,
    },

    /// SDK error.
    #[error(transparent)]
    OpenStackSDK {
        /// The source of the error.
        #[from]
        source: openstack_sdk::OpenStackError,
    },
    /// OpenStack API error.
    #[error(transparent)]
    OpenStackApi {
        /// The source of the error.
        #[from]
        source: openstack_sdk::api::ApiError<openstack_sdk::RestError>,
    },

    /// Configuration error.
    #[error(transparent)]
    ConfigError {
        /// The source of the error.
        #[from]
        source: openstack_sdk::config::ConfigError,
    },

    /// OpenStack Service Catalog error.
    #[error(transparent)]
    OpenStackCatalog {
        /// The source of the error.
        #[from]
        source: openstack_sdk::catalog::CatalogError,
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
}
