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

use crate::catalog::service_authority::ServiceAuthorityError;
use crate::types::api_version::ApiVersionError;

/// Service catalog error
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum CatalogError {
    #[error("Cannot parse Api Version: {}", source)]
    ApiVersion {
        #[from]
        source: ApiVersionError,
    },

    #[error("failed to parse url: {}", source)]
    UrlParse {
        #[from]
        source: url::ParseError,
    },

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

    #[error(transparent)]
    Other(#[from] anyhow::Error),
}
