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

pub use openstack_sdk_auth_core::{
    authtoken::AuthTokenError,
    authtoken_scope::{AuthTokenScope, AuthTokenScopeError},
};
use openstack_sdk_core::config;

use crate::auth::auth_token_endpoint as token_v3;

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
                    project_builder.domain(domain_builder.build().map_err(AuthTokenError::plugin)?);
                }
                scope_builder.project(project_builder.build().map_err(AuthTokenError::plugin)?);
            }
            AuthTokenScope::Domain(domain) => {
                let mut domain_builder = token_v3::DomainBuilder::default();
                if let Some(val) = &domain.id {
                    domain_builder.id(val.clone());
                }
                if let Some(val) = &domain.name {
                    domain_builder.name(val.clone());
                }
                scope_builder.domain(domain_builder.build().map_err(AuthTokenError::plugin)?);
            }
            AuthTokenScope::System(system) => {
                let mut system_builder = token_v3::SystemBuilder::default();
                if let Some(all) = system.all {
                    system_builder.all(all);
                }
                scope_builder.system(system_builder.build().map_err(AuthTokenError::plugin)?);
            }
            AuthTokenScope::Unscoped => {}
        }
        scope_builder.build().map_err(AuthTokenError::plugin)
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
                project_scope.domain(project_domain.build().map_err(AuthTokenError::plugin)?);
            };
            if let Some(val) = auth.project_id {
                project_scope.id(val);
            }
            if let Some(val) = auth.project_name {
                project_scope.name(val);
            }
            scope.project(project_scope.build().map_err(AuthTokenError::plugin)?);
        } else if auth.domain_id.is_some() || auth.domain_name.is_some() {
            // Domain scope
            let mut domain_scope = token_v3::DomainBuilder::default();
            if let Some(val) = auth.domain_id {
                domain_scope.id(val);
            }
            if let Some(val) = auth.domain_name {
                domain_scope.name(val);
            }
            scope.domain(domain_scope.build().map_err(AuthTokenError::plugin)?);
        } else if let Some(system) = auth.system_scope {
            // System scope
            let mut system_scope = token_v3::SystemBuilder::default();
            system_scope.all(system == "all");
            scope.system(system_scope.build().map_err(AuthTokenError::plugin)?);
        } else {
            return Err(Self::Error::Scope {
                source: AuthTokenScopeError::MissingScope,
            });
        }

        scope.build().map_err(AuthTokenError::plugin)
    }
}
