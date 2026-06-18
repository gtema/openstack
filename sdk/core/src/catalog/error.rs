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

use std::boxed::Box;
use thiserror::Error;
use url::Url;

use crate::catalog::service_authority::ServiceAuthorityError;
use crate::types::api_version::{ApiVersion, ApiVersionError};

/// Service catalog error
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum CatalogError {
    #[error("Cannot parse catalog data: {}", source)]
    Json {
        #[from]
        source: serde_json::Error,
    },
    #[error("Cannot parse Api Version: {}", source)]
    ApiVersion {
        #[from]
        source: ApiVersionError,
    },

    #[error("Regex parse error: {}", source)]
    Regex {
        #[from]
        source: regex::Error,
    },

    /// Invalid URL
    #[error(
        "Url `{0}` cannot be base. [https://gtema.github.io/openstack/possible_errors.html#version-url-cannot-be-a-base]"
    )]
    UrlCannotBeBase(String),

    /// Invalid URL
    #[error("Failed to parse url: `{}`", source)]
    UrlParse {
        source: url::ParseError,
        url: String,
    },

    /// Invalid URI
    #[error("Invalid URI: `{}`", source)]
    InvalidUri {
        #[from]
        source: http::uri::InvalidUri,
    },

    /// Invalid URL scheme
    #[error("Url must be http/https")]
    UrlScheme(String),

    #[error("Service Authority data cannot be parsed: {}", source)]
    ServiceAuthority {
        #[from]
        source: ServiceAuthorityError,
    },

    #[error("Invalid version discovery document")]
    InvalidDiscoveryDocument,

    /// The service is not configured for the specified type, region and interface.
    #[error(
        "Service `{}` is not configured for interface {:?} at region {:?}",
        srv_type,
        interface,
        region
    )]
    ServiceNotConfigured {
        srv_type: String,
        region: Option<String>,
        interface: Option<String>,
    },

    #[error("Api Version with id `{id}` for service is not defining `self` link")]
    VersionSelfLinkMissing { id: String },

    #[error(
        "Requested/Required Api Version `{}` is not supported by the server side",
        ver
    )]
    VersionUnsupported { ver: ApiVersion },

    /// HTTP error with status code
    #[error("HTTP request failed with status {}: {}", status, body)]
    Http {
        status: http::StatusCode,
        body: String,
    },

    /// Generic API error wrapper
    #[error("API error: {err}")]
    ApiError {
        err: Box<dyn std::error::Error + Send + Sync>,
    },
}

impl CatalogError {
    pub fn url_parse<S: AsRef<str>>(source: url::ParseError, url: S) -> Self {
        Self::UrlParse {
            source,
            url: url.as_ref().into(),
        }
    }

    pub fn cannot_be_base(url: &Url) -> Self {
        Self::UrlCannotBeBase(url.as_str().into())
    }

    /// Wrap an ApiError in CatalogError for generic error types
    pub fn api_error<E>(err: crate::api::ApiError<E>) -> Self
    where
        E: std::error::Error + Send + Sync + 'static,
    {
        Self::ApiError { err: Box::new(err) }
    }
}
