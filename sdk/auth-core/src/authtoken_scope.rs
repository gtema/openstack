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
#[derive(Clone, Deserialize, Eq, Hash, PartialEq, Serialize, Debug, Default)]
#[serde(rename_all = "lowercase")]
pub enum AuthTokenScope {
    /// Project
    Project(Project),
    /// Domain
    Domain(Domain),
    /// System
    System(System),
    /// Unscoped
    #[default]
    Unscoped,
}

impl AuthTokenScope {
    /// Checks if this scope (the requested scope) matches another scope (the cached scope).
    /// This implements "wildcard" matching: if a field in the requested scope is `None`,
    /// it matches any value in the cached scope.
    pub fn matches(&self, cached: &Self) -> bool {
        match (self, cached) {
            (AuthTokenScope::Project(req), AuthTokenScope::Project(cached)) => {
                let id_match = req
                    .id
                    .as_ref()
                    .is_none_or(|id| cached.id.as_ref() == Some(id));
                let name_match = req
                    .name
                    .as_ref()
                    .is_none_or(|name| cached.name.as_ref() == Some(name));
                let domain_match = if let Some(req_domain) = &req.domain {
                    if let Some(cached_domain) = &cached.domain {
                        let d_id_match = req_domain
                            .id
                            .as_ref()
                            .is_none_or(|id| cached_domain.id.as_ref() == Some(id));
                        let d_name_match = req_domain
                            .name
                            .as_ref()
                            .is_none_or(|name| cached_domain.name.as_ref() == Some(name));
                        d_id_match && d_name_match
                    } else {
                        true
                    }
                } else {
                    true
                };
                id_match && name_match && domain_match
            }
            (AuthTokenScope::Domain(req), AuthTokenScope::Domain(cached)) => {
                let id_match = req
                    .id
                    .as_ref()
                    .is_none_or(|id| cached.id.as_ref() == Some(id));
                let name_match = req
                    .name
                    .as_ref()
                    .is_none_or(|name| cached.name.as_ref() == Some(name));
                id_match && name_match
            }
            (AuthTokenScope::Unscoped, AuthTokenScope::Unscoped) => true,
            (AuthTokenScope::System(_), AuthTokenScope::System(_)) => true,
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_project_scope(id: Option<&str>, name: Option<&str>) -> AuthTokenScope {
        AuthTokenScope::Project(Project {
            id: id.map(|s| s.to_string()),
            name: name.map(|s| s.to_string()),
            domain: None,
        })
    }

    fn make_domain_scope(id: Option<&str>, name: Option<&str>) -> AuthTokenScope {
        AuthTokenScope::Domain(Domain {
            id: id.map(|s| s.to_string()),
            name: name.map(|s| s.to_string()),
        })
    }

    // Project scope tests
    #[test]
    fn test_matches_project_exact_id() {
        let req = make_project_scope(Some("proj-1"), None);
        let cached = make_project_scope(Some("proj-1"), None);
        assert!(req.matches(&cached));
    }

    #[test]
    fn test_matches_project_id_mismatch() {
        let req = make_project_scope(Some("proj-1"), None);
        let cached = make_project_scope(Some("proj-2"), None);
        assert!(!req.matches(&cached));
    }

    #[test]
    fn test_matches_project_req_id_none_matches_cached_id_some() {
        let req = make_project_scope(None, Some("myproj"));
        let cached = make_project_scope(Some("proj-1"), Some("myproj"));
        assert!(req.matches(&cached));
    }

    #[test]
    fn test_matches_project_req_name_none_matches_cached_name() {
        let req = make_project_scope(None, None);
        let cached = make_project_scope(Some("proj-1"), Some("myproj"));
        assert!(req.matches(&cached));
    }

    #[test]
    fn test_matches_project_name_mismatch_when_req_name_some() {
        let req = make_project_scope(None, Some("req-name"));
        let cached = make_project_scope(Some("proj-1"), Some("cached-name"));
        assert!(!req.matches(&cached));
    }

    #[test]
    fn test_matches_project_both_none() {
        let req = make_project_scope(None, None);
        let cached = make_project_scope(None, None);
        assert!(req.matches(&cached));
    }

    // Domain scope tests
    #[test]
    fn test_matches_domain_exact_id() {
        let req = make_domain_scope(Some("d1"), None);
        let cached = make_domain_scope(Some("d1"), None);
        assert!(req.matches(&cached));
    }

    #[test]
    fn test_matches_domain_id_mismatch() {
        let req = make_domain_scope(Some("d1"), None);
        let cached = make_domain_scope(Some("d2"), None);
        assert!(!req.matches(&cached));
    }

    #[test]
    fn test_matches_domain_req_id_none_matches_cached() {
        let req = make_domain_scope(None, Some("D"));
        let cached = make_domain_scope(Some("d1"), Some("D"));
        assert!(req.matches(&cached));
    }

    #[test]
    fn test_matches_domain_name_mismatch_when_req_name_some() {
        let req = make_domain_scope(None, Some("req-D"));
        let cached = make_domain_scope(Some("d1"), Some("cached-D"));
        assert!(!req.matches(&cached));
    }

    // Cross-type tests
    #[test]
    fn test_matches_project_does_not_match_domain() {
        let req = make_project_scope(Some("p1"), None);
        let cached = make_domain_scope(Some("p1"), None);
        assert!(!req.matches(&cached));
    }

    #[test]
    fn test_matches_domain_does_not_match_project() {
        let req = make_domain_scope(Some("d1"), None);
        let cached = make_project_scope(Some("d1"), None);
        assert!(!req.matches(&cached));
    }

    #[test]
    fn test_matches_unscoped_matches_unscoped() {
        let req = AuthTokenScope::Unscoped;
        let cached = AuthTokenScope::Unscoped;
        assert!(req.matches(&cached));
    }

    #[test]
    fn test_matches_unscoped_does_not_match_project() {
        let req = AuthTokenScope::Unscoped;
        let cached = make_project_scope(Some("p1"), None);
        assert!(!req.matches(&cached));
    }

    // Domain-within-project tests
    #[test]
    fn test_matches_project_with_domain_exact() {
        let req = AuthTokenScope::Project(Project {
            id: Some("project-id".to_string()),
            name: None,
            domain: Some(Domain {
                id: Some("domain-id".to_string()),
                name: None,
            }),
        });
        let cached = AuthTokenScope::Project(Project {
            id: Some("project-id".to_string()),
            name: None,
            domain: Some(Domain {
                id: Some("domain-id".to_string()),
                name: Some("D".to_string()),
            }),
        });
        assert!(req.matches(&cached));
    }

    #[test]
    fn test_matches_project_with_domain_mismatch() {
        let req = AuthTokenScope::Project(Project {
            id: Some("project-id".to_string()),
            name: None,
            domain: Some(Domain {
                id: Some("domain-1".to_string()),
                name: None,
            }),
        });
        let cached = AuthTokenScope::Project(Project {
            id: Some("project-id".to_string()),
            name: None,
            domain: Some(Domain {
                id: Some("domain-2".to_string()),
                name: Some("D".to_string()),
            }),
        });
        assert!(!req.matches(&cached));
    }

    #[test]
    fn test_matches_project_cached_has_domain_req_has_no_domain() {
        let req = AuthTokenScope::Project(Project {
            id: Some("project-id".to_string()),
            name: None,
            domain: None,
        });
        let cached = AuthTokenScope::Project(Project {
            id: Some("project-id".to_string()),
            name: None,
            domain: Some(Domain {
                id: Some("domain-id".to_string()),
                name: None,
            }),
        });
        assert!(req.matches(&cached));
    }

    #[test]
    fn test_matches_project_req_domain_by_name() {
        let req = AuthTokenScope::Project(Project {
            id: None,
            name: Some("myproj".to_string()),
            domain: Some(Domain {
                id: None,
                name: Some("Default".to_string()),
            }),
        });
        let cached = AuthTokenScope::Project(Project {
            id: Some("proj-123".to_string()),
            name: Some("myproj".to_string()),
            domain: Some(Domain {
                id: Some("d1".to_string()),
                name: Some("Default".to_string()),
            }),
        });
        assert!(req.matches(&cached));
    }
}
