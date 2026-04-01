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

//! OpenStack AuthToken Scope handling
//!
//! When authenticating with AuthToken user is able to explicitly request scope (authorization)
//!
//! - `project` - intention to work with a certain project
//! - `domain` - intention to work with a certain domain
//! - `unscoped` - authenticate without any explicit roles

use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::{Domain, Project, System};

//use crate::auth::auth_token_endpoint as token_v3;

/// AuthToken (X-Auth-Token) Scope based auth errors
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum AuthTokenScopeError {
    /// Auth data is missing in the config
    #[error("Auth data is missing")]
    MissingAuthData,

    /// Scope cannot be built
    #[error("Cannot determine authorization scope from config")]
    MissingScope,

    /// Scope cannot be built
    #[error(transparent)]
    //"Cannot determine authorization scope from config")]
    Builder {
        #[from]
        source: crate::BuilderError,
    },
}

/// Represents AuthToken authorization scope
#[derive(Clone, Deserialize, Eq, Hash, PartialEq, Serialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum AuthTokenScope {
    /// Project
    Project(Project),
    /// Domain
    Domain(Domain),
    /// System
    System(System),
    /// Unscoped
    Unscoped,
}
