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

pub use openstack_sdk_auth_core::authtoken_scope::{AuthTokenScope, AuthTokenScopeError};
use openstack_sdk_auth_core::{Domain, Project, System};

use crate::config;
use crate::types::identity::v3::get_domain;

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
                domain: get_domain(auth.project_domain_id, auth.project_domain_name),
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
