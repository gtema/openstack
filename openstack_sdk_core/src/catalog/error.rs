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

use thiserror::Error;
use url::Url;

use crate::catalog::service_authority::ServiceAuthorityError;
use crate::types::api_version::{ApiVersion, ApiVersionError};

/// Service catalog error
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum CatalogError {
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
    #[error("Url `{0}` cannot be base. [https://gtema.github.io/openstack/possible_errors.html#version-url-cannot-be-a-base]")]
    UrlCannotBeBase(String),

    /// Invalid URL
    #[error("Failed to parse url: `{}`", source)]
    UrlParse {
        source: url::ParseError,
        url: String,
    },

    /// A transparent url::ParseError error for unwrapped cases
    #[error("Failed to parse url: `{}`", source)]
    UrlParseError {
        #[from]
        source: url::ParseError,
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

    #[error("Service `{0}` is not configured")]
    ServiceNotConfigured(String),

    #[error("Api Version with id `{id}` for service is not defining `self` link")]
    VersionSelfLinkMissing { id: String },

    #[error(
        "Requested/Required Api Version `{}` is not supported by the server side",
        ver
    )]
    VersionUnsupported { ver: ApiVersion },
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
}
