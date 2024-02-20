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

//! Errors module

use std::any;
use thiserror::Error;

use futures::io::Error as IoError;

use crate::api;
use crate::auth::AuthError;
use crate::catalog::CatalogError;
use crate::config::ConfigError;

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum RestError {
    #[error("error setting auth header: {}", source)]
    AuthError {
        #[from]
        source: AuthError,
    },

    #[error("communication with openstack: {}", source)]
    Communication {
        #[from]
        source: reqwest::Error,
    },

    #[error("`http` error: {}", source)]
    Http {
        #[from]
        source: http::Error,
    },
    #[error("`IO` error: {}", source)]
    IO {
        #[from]
        source: IoError,
    },
    #[error("`Catalog` error: {}", source)]
    Catalog {
        #[from]
        source: CatalogError,
    },
}

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum OpenStackError {
    #[error("failed to parse url: {}", source)]
    UrlParse {
        #[from]
        source: url::ParseError,
    },

    #[error("error setting auth header: {}", source)]
    AuthError {
        #[from]
        source: AuthError,
    },

    #[error("communication with cloud: {}", source)]
    Communication {
        #[from]
        source: reqwest::Error,
    },

    #[error("openstack HTTP error: {}", status)]
    Http { status: reqwest::StatusCode },

    #[error("no response from API")]
    NoResponse {},

    #[error("could not parse {} data from JSON: {}", typename, source)]
    DataType {
        #[source]
        source: serde_json::Error,
        typename: &'static str,
    },

    #[error("api error: {}", source)]
    Api {
        #[from]
        source: api::ApiError<RestError>,
    },
    #[error("config error: {}", msg)]
    Config { msg: String },

    #[error("service_catalog error: {}", source)]
    Catalog {
        #[from]
        source: CatalogError,
    },

    #[error("configuration error: {}", source)]
    ConfigError {
        #[from]
        source: ConfigError,
    },

    #[error("Endpoint version discovery error: {}", msg)]
    Discovery { msg: String },

    #[error(
        "Interactive mode is required but not available (running `echo foo | osc`?). {}",
        msg
    )]
    NonInteractiveMode { msg: String },

    #[error(transparent)]
    Other(#[from] anyhow::Error), // source and Display delegate to anyhow::Error
}

impl OpenStackError {
    pub fn http(status: reqwest::StatusCode) -> Self {
        OpenStackError::Http { status }
    }

    pub fn no_response() -> Self {
        OpenStackError::NoResponse {}
    }

    pub fn data_type<T>(source: serde_json::Error) -> Self {
        OpenStackError::DataType {
            source,
            typename: any::type_name::<T>(),
        }
    }
}

pub type OpenStackResult<T> = Result<T, OpenStackError>;
