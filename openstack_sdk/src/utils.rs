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

//! Utilities
//!
use crate::api::identity::v3::auth::token::create as token_v3;
use crate::auth::{AuthError, AuthToken, AuthorizationScope};
use crate::config;
use crate::types::identity::v3::{self as types_v3, AuthResponse, Domain, Project};

/// Build Auth `Identity` from existing `Auth` (use token)
impl TryFrom<&AuthToken> for token_v3::Identity<'_> {
    type Error = AuthError;

    fn try_from(auth: &AuthToken) -> Result<Self, Self::Error> {
        Ok(token_v3::IdentityBuilder::default()
            .methods(Vec::from([token_v3::Methods::Token]))
            .token(
                token_v3::TokenBuilder::default()
                    .id(auth.token.clone())
                    .build()?,
            )
            .build()?)
    }
}

/// Build Auth `Scope` data from `CloudConfig`
impl TryFrom<&config::CloudConfig> for token_v3::Scope<'_> {
    type Error = AuthError;
    fn try_from(config: &config::CloudConfig) -> Result<Self, Self::Error> {
        let auth = config.auth.clone().ok_or(AuthError::MissingAuthData)?;
        let mut scope = token_v3::ScopeBuilder::default();
        if auth.project_id.is_some() || auth.project_name.is_some() {
            // Project scope
            let mut project_scope = token_v3::ProjectBuilder::default();
            if auth.project_domain_name.is_some() || auth.project_domain_id.is_some() {
                let mut project_domain = token_v3::ProjectDomainBuilder::default();
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
            let mut domain_scope = token_v3::ScopeDomainBuilder::default();
            if let Some(val) = auth.domain_id {
                domain_scope.id(val);
            }
            if let Some(val) = auth.domain_name {
                domain_scope.name(val);
            }
            scope.domain(domain_scope.build()?);
        } else {
            return Err(AuthError::MissingScope);
        }

        Ok(scope.build()?)
    }
}

/// Build Auth `Scope` data from existing `AuthorizationScope`
impl TryFrom<&AuthorizationScope> for token_v3::Scope<'_> {
    type Error = AuthError;
    fn try_from(scope: &AuthorizationScope) -> Result<Self, Self::Error> {
        let mut scope_builder = token_v3::ScopeBuilder::default();
        match scope {
            AuthorizationScope::Project(project) => {
                let mut project_builder = token_v3::ProjectBuilder::default();
                if let Some(val) = &project.id {
                    project_builder.id(val.clone());
                }
                if let Some(val) = &project.name {
                    project_builder.name(val.clone());
                }
                if let Some(domain) = &project.domain {
                    let mut domain_builder = token_v3::ProjectDomainBuilder::default();
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
            AuthorizationScope::Domain(domain) => {
                let mut domain_builder = token_v3::ScopeDomainBuilder::default();
                if let Some(val) = &domain.id {
                    domain_builder.id(val.clone());
                }
                if let Some(val) = &domain.name {
                    domain_builder.name(val.clone());
                }
                scope_builder.domain(domain_builder.build()?);
            }
            AuthorizationScope::Unscoped => {}
        }
        Ok(scope_builder.build()?)
    }
}

/// Build `AuthorizationScope` data from `CloudConfig`
impl TryFrom<&config::CloudConfig> for AuthorizationScope {
    type Error = AuthError;
    fn try_from(config: &config::CloudConfig) -> Result<Self, Self::Error> {
        let auth = config.auth.clone().ok_or(AuthError::MissingAuthData)?;
        if auth.project_id.is_some() || auth.project_name.is_some() {
            // Project scope
            Ok(AuthorizationScope::Project(Project {
                id: auth.project_id.clone(),
                name: auth.project_name.clone(),
                domain: types_v3::get_domain(auth.project_domain_id, auth.project_domain_name),
            }))
        } else if auth.domain_id.is_some() || auth.domain_name.is_some() {
            // Domain scope
            Ok(AuthorizationScope::Domain(Domain {
                id: auth.domain_id.clone(),
                name: auth.domain_name.clone(),
            }))
        } else {
            Ok(AuthorizationScope::Unscoped)
        }
    }
}

/// Build `AuthorizationScope` from `AuthResponse`
impl From<&AuthResponse> for AuthorizationScope {
    fn from(auth: &AuthResponse) -> Self {
        if let Some(project) = &auth.token.project {
            Self::Project(project.clone())
        } else if let Some(domain) = &auth.token.domain {
            Self::Domain(domain.clone())
        } else {
            Self::Unscoped
        }
    }
}
