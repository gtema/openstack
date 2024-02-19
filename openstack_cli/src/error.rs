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
    /// Json serialization error
    #[error("failed to serialize data to json: {}", source)]
    SerializeJson {
        /// The source of the error.
        #[from]
        source: serde_json::Error,
    },
    /// SDK error
    #[error("OpenStackSDK error: {}", source)]
    OpenStackSDK {
        /// The source of the error.
        #[from]
        source: openstack_sdk::OpenStackError,
    },
    /// OpenStack API error
    #[error("OpenStackSDK error: {}", source)]
    OpenStackApi {
        /// The source of the error.
        #[from]
        source: openstack_sdk::api::ApiError<openstack_sdk::RestError>,
    },

    /// Configuration error
    #[error("Config error: {}", source)]
    ConfigError {
        /// The source of the error.
        #[from]
        source: openstack_sdk::config::ConfigError,
    },

    /// No subcommands
    #[error("Command has no subcommands")]
    NoSubcommands,

    /// Resource is not found
    #[error("resource not found")]
    ResourceNotFound,

    /// Resource identifier is not unique
    #[error("cannot uniqly findresource by identifier")]
    IdNotUnique,

    /// IO error
    #[error("IO error: {}", source)]
    IO {
        /// The source of the error.
        #[from]
        source: std::io::Error,
    },
    /// Reqwest library error
    #[error("Reqwest error: {}", source)]
    Reqwest {
        /// The source of the error.
        #[from]
        source: reqwest::Error,
    },
    /// Clap library error
    #[error("Argument parsing error: {}", source)]
    Clap {
        /// The source of the error.
        #[from]
        source: clap::error::Error,
    },
    /// Indicativ library error
    #[error("Indicativ error: {}", source)]
    Idinticatif {
        /// The source of the error.
        #[from]
        source: indicatif::style::TemplateError,
    },
    /// Endpoint builder error
    #[error("OpenStackSDK endpoint builder error: `{0}`")]
    EndpointBuild(String),

    /// Connection error
    #[error("Cloud connection for `{0:?}` cannot be found")]
    ConnectionNotFound(String),

    /// Invalid header name
    #[error("Invalid header name `{}`", source)]
    InvalidHeaderName {
        /// The source of the error.
        #[from]
        source: http::header::InvalidHeaderName,
    },

    /// Invalid header value
    #[error("Invalid header value `{}`", source)]
    InvalidHeaderValue {
        /// The source of the error.
        #[from]
        source: http::header::InvalidHeaderValue,
    },

    /// Others
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}
