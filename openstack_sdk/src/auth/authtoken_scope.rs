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

use std::fmt::Debug;

use serde::{Deserialize, Serialize};
use thiserror::Error;
use tracing::error;

use crate::auth::auth_token_endpoint as token_v3;
use crate::auth::authtoken::AuthTokenError;
use crate::config;
use crate::types::identity::v3::{self as types_v3, AuthResponse, Domain, Project, System};

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

    /// Project scope builder error
    #[error("Cannot construct project scope information from config: {}", source)]
    ProjectBuild {
        /// The error source
        #[from]
        source: token_v3::ProjectBuilderError,
    },

    /// Scope Domain cannot be build
    #[error("Cannot construct domain information from config: {}", source)]
    DomainBuild {
        /// The error source
        #[from]
        source: token_v3::DomainBuilderError,
    },

    /// Scope System cannot be build
    #[error("Cannot construct system scope information from config: {}", source)]
    ScopeSystemBuild {
        /// The error source
        #[from]
        source: token_v3::SystemBuilderError,
    },

    /// Scope data cannot be build
    #[error("Cannot construct auth scope information from config: {}", source)]
    ScopeBuild {
        /// The error source
        #[from]
        source: token_v3::ScopeBuilderError,
    },
}

// Implement From to ease error propagation without adding new kinds
impl From<token_v3::ProjectBuilderError> for AuthTokenError {
    fn from(source: token_v3::ProjectBuilderError) -> Self {
        Self::Scope {
            source: source.into(),
        }
    }
}

impl From<token_v3::DomainBuilderError> for AuthTokenError {
    fn from(source: token_v3::DomainBuilderError) -> Self {
        Self::Scope {
            source: source.into(),
        }
    }
}

impl From<token_v3::SystemBuilderError> for AuthTokenError {
    fn from(source: token_v3::SystemBuilderError) -> Self {
        Self::Scope {
            source: source.into(),
        }
    }
}

impl From<token_v3::ScopeBuilderError> for AuthTokenError {
    fn from(source: token_v3::ScopeBuilderError) -> Self {
        Self::Scope {
            source: source.into(),
        }
    }
}

/// Represents AuthToken authorization scope
#[derive(Clone, Deserialize, Eq, Hash, PartialEq, Serialize, Debug)]
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

/// Build [`AuthTokenScope`] data from [`CloudConfig`](config::CloudConfig)
impl TryFrom<&config::CloudConfig> for AuthTokenScope {
    type Error = AuthTokenScopeError;
    fn try_from(config: &config::CloudConfig) -> Result<Self, Self::Error> {
        let auth = config.auth.clone().ok_or(Self::Error::MissingAuthData)?;
        if auth.project_id.is_some() || auth.project_name.is_some() {
            // Project scope
            Ok(AuthTokenScope::Project(Project {
                id: auth.project_id.clone(),
                // Keystone checks for presence of project_name before project_id therefore it fail
                // when project_domain is not set. project_id alone is sufficient.
                name: if auth.project_id.is_none() {
                    auth.project_name.clone()
                } else {
                    None
                },
                domain: types_v3::get_domain(auth.project_domain_id, auth.project_domain_name),
            }))
        } else if auth.domain_id.is_some() || auth.domain_name.is_some() {
            // Domain scope
            Ok(AuthTokenScope::Domain(Domain {
                id: auth.domain_id.clone(),
                name: auth.domain_name.clone(),
            }))
        } else if let Some(system) = auth.system_scope {
            // System scope
            Ok(AuthTokenScope::System(System {
                all: Some(system == "all"),
            }))
        } else {
            Ok(AuthTokenScope::Unscoped)
        }
    }
}

/// Build [`AuthTokenScope`] from [`AuthResponse`]
impl From<&AuthResponse> for AuthTokenScope {
    fn from(auth: &AuthResponse) -> Self {
        if let Some(project) = &auth.token.project {
            Self::Project(project.clone())
        } else if let Some(domain) = &auth.token.domain {
            Self::Domain(domain.clone())
        } else if let Some(system) = &auth.token.system {
            Self::System(system.clone())
        } else {
            Self::Unscoped
        }
    }
}

/// Build Auth [`Scope`](token_v3::Scope) data from existing [`AuthTokenScope`]
impl TryFrom<&AuthTokenScope> for token_v3::Scope<'_> {
    type Error = AuthTokenError;
    fn try_from(scope: &AuthTokenScope) -> Result<Self, Self::Error> {
        let mut scope_builder = token_v3::ScopeBuilder::default();
        match scope {
            AuthTokenScope::Project(project) => {
                let mut project_builder = token_v3::ProjectBuilder::default();
                if let Some(val) = &project.id {
                    project_builder.id(val.clone());
                }
                if let Some(val) = &project.name {
                    project_builder.name(val.clone());
                }
                if let Some(domain) = &project.domain {
                    let mut domain_builder = token_v3::DomainBuilder::default();
                    if let Some(val) = &domain.id {
                        domain_builder.id(val.clone());
                    }
                    if let Some(val) = &domain.name {
                        domain_builder.name(val.clone());
                    }
                    project_builder.domain(domain_builder.build()?);
                }
                scope_builder.project(project_builder.build()?);
            }
            AuthTokenScope::Domain(domain) => {
                let mut domain_builder = token_v3::DomainBuilder::default();
                if let Some(val) = &domain.id {
                    domain_builder.id(val.clone());
                }
                if let Some(val) = &domain.name {
                    domain_builder.name(val.clone());
                }
                scope_builder.domain(domain_builder.build()?);
            }
            AuthTokenScope::System(system) => {
                let mut system_builder = token_v3::SystemBuilder::default();
                if let Some(all) = system.all {
                    system_builder.all(all);
                }
                scope_builder.system(system_builder.build()?);
            }
            AuthTokenScope::Unscoped => {}
        }
        Ok(scope_builder.build()?)
    }
}

/// Build Auth [`Scope`][`token_v3::Scope`] data from [`CloudConfig`][`config::CloudConfig`]
impl TryFrom<&config::CloudConfig> for token_v3::Scope<'_> {
    type Error = AuthTokenError;
    fn try_from(config: &config::CloudConfig) -> Result<Self, Self::Error> {
        let auth = config.auth.clone().ok_or(Self::Error::MissingAuthData)?;
        let mut scope = token_v3::ScopeBuilder::default();
        if auth.project_id.is_some() || auth.project_name.is_some() {
            // Project scope
            let mut project_scope = token_v3::ProjectBuilder::default();
            if auth.project_domain_name.is_some() || auth.project_domain_id.is_some() {
                let mut project_domain = token_v3::DomainBuilder::default();
                if let Some(val) = auth.project_domain_id {
                    project_domain.id(val);
                }
                if let Some(val) = auth.project_domain_name {
                    project_domain.name(val);
                }
                project_scope.domain(project_domain.build()?);
            };
            if let Some(val) = auth.project_id {
                project_scope.id(val);
            }
            if let Some(val) = auth.project_name {
                project_scope.name(val);
            }
            scope.project(project_scope.build()?);
        } else if auth.domain_id.is_some() || auth.domain_name.is_some() {
            // Domain scope
            let mut domain_scope = token_v3::DomainBuilder::default();
            if let Some(val) = auth.domain_id {
                domain_scope.id(val);
            }
            if let Some(val) = auth.domain_name {
                domain_scope.name(val);
            }
            scope.domain(domain_scope.build()?);
        } else if let Some(system) = auth.system_scope {
            // System scope
            let mut system_scope = token_v3::SystemBuilder::default();
            system_scope.all(system == "all");
            scope.system(system_scope.build()?);
        } else {
            return Err(Self::Error::Scope {
                source: AuthTokenScopeError::MissingScope,
            });
        }

        Ok(scope.build()?)
    }
}
