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

use crate::api;
#[cfg(feature = "keystone_ng")]
use crate::auth::v4federation::FederationError;
use crate::auth::{
    authtoken::AuthTokenError, authtoken_scope::AuthTokenScopeError,
    v3oidcaccesstoken::OidcAccessTokenError, v3websso::WebSsoError, AuthError,
};
use crate::catalog::CatalogError;
use crate::config::ConfigError;

/// Rest errors that may happen during API communication
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum RestError {
    /// Auth error
    #[error("error setting auth header: {}", source)]
    AuthError {
        /// The source of the error.
        #[from]
        source: AuthError,
    },

    /// API communication error
    #[error("communication with openstack: {}", source)]
    Communication {
        /// The source of the error.
        #[from]
        source: reqwest::Error,
    },

    /// HTTP error
    #[error("`http` error: {}", source)]
    Http {
        /// The source of the error.
        #[from]
        source: http::Error,
    },
}

/// OpenStack Client error
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum OpenStackError {
    /// URL parse error
    #[error("failed to parse url: {}", source)]
    UrlParse {
        /// The source of the error.
        #[from]
        source: url::ParseError,
    },

    /// Authentication error
    #[error("No authentication information available")]
    NoAuth,

    /// Authentication error
    #[error("error setting auth header: {}", source)]
    AuthError {
        /// The source of the error.
        #[from]
        source: AuthError,
    },

    /// API Communication error
    #[error("communication with cloud: {}", source)]
    Communication {
        /// The source of the error.
        #[from]
        source: reqwest::Error,
    },

    /// HTTP error
    #[error("openstack HTTP error: {}", status)]
    Http { status: reqwest::StatusCode },

    /// No response
    #[error("no response from API")]
    NoResponse {},

    /// Json deserialization error
    #[error("could not parse {} data from JSON: {}", typename, source)]
    DataType {
        /// The source of the error.
        #[source]
        source: serde_json::Error,
        /// type name that could not be parsed
        typename: &'static str,
    },

    /// API error
    #[error("api error: {}", source)]
    Api {
        /// The source of the error.
        #[from]
        source: api::ApiError<RestError>,
    },

    /// Service catalog error
    #[error("service_catalog error: {}", source)]
    Catalog {
        /// The source of the error.
        #[from]
        /// error source
        source: CatalogError,
    },

    #[error("configuration error: {}", source)]
    ConfigError {
        /// The source of the error.
        #[from]
        source: ConfigError,
    },

    /// Dialoguer error
    #[error("error reading the user input: {}", source)]
    Dialoguer {
        /// The source of the error
        #[from]
        source: dialoguer::Error,
    },

    /// Service version discovery error
    #[error(
        "`{}` endpoint version discovery error:\n\tUrl: {}\n\tMessage: {}",
        service,
        url,
        msg
    )]
    Discovery {
        service: String,
        url: String,
        msg: String,
    },

    /// Interactive mode required
    #[error(
        "Interactive mode is required but not available (running `echo foo | osc`?). {}",
        msg
    )]
    NonInteractiveMode { msg: String },

    /// JSON deserialization from OpenStack failed.
    #[error("could not parse JSON response: {}", source)]
    Json {
        /// The source of the error.
        #[from]
        source: serde_json::Error,
    },
    /// IO error.
    #[error("IO error: {}\n\tPath: {}", source, path)]
    IO {
        /// The source of the error.
        source: std::io::Error,
        path: String,
    },

    /// Endpoint builder error
    #[error("endpoint builder error: `{0}`")]
    EndpointBuild(String),
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

    pub fn catalog(source: CatalogError) -> Self {
        OpenStackError::Catalog { source }
    }
}

// Explicitly implement From to easier propagate nested errors
impl From<AuthTokenError> for OpenStackError {
    fn from(source: AuthTokenError) -> Self {
        Self::AuthError {
            source: AuthError::AuthToken { source },
        }
    }
}

impl From<AuthTokenScopeError> for OpenStackError {
    fn from(source: AuthTokenScopeError) -> Self {
        Self::AuthError {
            source: source.into(),
        }
    }
}

impl From<OidcAccessTokenError> for OpenStackError {
    fn from(source: OidcAccessTokenError) -> Self {
        Self::AuthError {
            source: source.into(),
        }
    }
}

impl From<WebSsoError> for OpenStackError {
    fn from(source: WebSsoError) -> Self {
        Self::AuthError {
            source: source.into(),
        }
    }
}

#[cfg(feature = "keystone_ng")]
impl From<FederationError> for OpenStackError {
    fn from(source: FederationError) -> Self {
        Self::AuthError {
            source: source.into(),
        }
    }
}

pub type OpenStackResult<T> = Result<T, OpenStackError>;
