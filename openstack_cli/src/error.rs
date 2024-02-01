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

use indicatif;
use reqwest;
use thiserror::Error;

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum OpenStackCliError {
    #[error("failed to serialize data to json: {}", source)]
    SerializeJson {
        /// The source of the error.
        #[from]
        source: serde_json::Error,
    },
    // #[error("failed to serialize data to yaml: {}", source)]
    // SerializeYaml {
    //     /// The source of the error.
    //     #[from]
    //     source: serde_yaml::Error,
    // },
    #[error("OpenStackSDK error: {}", source)]
    OpenStackSDK {
        /// The source of the error.
        #[from]
        source: openstack_sdk::OpenStackError,
    },
    #[error("OpenStackSDK error: {}", source)]
    OpenStackApi {
        /// The source of the error.
        #[from]
        source: openstack_sdk::api::ApiError<openstack_sdk::RestError>,
    },

    #[error("Config error: {}", source)]
    ConfigError {
        #[from]
        source: openstack_sdk::config::ConfigError,
    },

    #[error("Command has no subcommands")]
    NoSubcommands,

    #[error("resource not found")]
    ResourceNotFound,

    #[error("cannot uniqly findresource by identifier")]
    IdNotUnique,

    #[error("IO error: {}", source)]
    IO {
        #[from]
        source: std::io::Error,
    },
    #[error("Reqwest error: {}", source)]
    Reqwest {
        #[from]
        source: reqwest::Error,
    },
    #[error("Argument parsing error: {}", source)]
    Clap {
        #[from]
        source: clap::error::Error,
    },
    #[error("Indicativ error: {}", source)]
    Idinticatif {
        #[from]
        source: indicatif::style::TemplateError,
    },
    #[error("OpenStackSDK endpoint builder error: `{0}`")]
    EndpointBuild(String),

    #[error("Cloud connection for `{0:?}` cannot be found")]
    ConnectionNotFound(String),

    #[error(transparent)]
    Other(#[from] anyhow::Error),
}
