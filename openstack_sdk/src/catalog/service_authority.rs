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

//! Service Authority
//!
//! The [OpenStack Service Types
//! Authority](https://specs.openstack.org/openstack/api-sig/guidelines/consuming-catalog/authority.html)
//! is data about official service type names and historical service type names commonly in use
//! from before there was an official list. It is made available to allow libraries and other
//! client API consumers to be able to provide a consistent interface based on the official list
//! but still support existing names. Providing this support is highly recommended, but is
//! ultimately optional. The first step in the matching process is always to return direct matches
//! between the catalog and the user request, so the existing consumption models from before the
//! existence of the authority should always work.
//!
use serde::Deserialize;
use std::collections::HashMap;
use thiserror::Error;
use tracing::warn;

/// Service authority error
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum ServiceAuthorityError {
    /// JSON deserialization failed.
    #[error("could not parse JSON: {}", source)]
    Json {
        /// The source of the error.
        #[from]
        source: serde_json::Error,
    },
    #[error("unknown service {0}")]
    ServiceUnknown(String),
}

/// ServiceType Authority as provided by <https://service-types.openstack.org/service-types.json>
///
/// This structure lists services with their service types and corresponding aliases.
#[derive(Clone, Debug, Deserialize, Default)]
pub struct ServiceAuthority {
    /// List of services
    pub services: Vec<Service>,
    /// Mapping of official service-type to historical aliases
    pub forward: HashMap<String, Vec<String>>,
    /// Reverse mapping of historical alias to official service-type
    pub reverse: HashMap<String, String>,
    /// Mapping of official service-type to official type and aliases
    pub all_types_by_service_type: Option<HashMap<String, Vec<String>>>,
    /// Mapping of project name to list of service-types for the project
    #[allow(dead_code)]
    pub service_types_by_projects: Option<HashMap<String, Vec<String>>>,
}

/// OpenStack Service metadata
///
/// This structure provides information about the service, such as name, service_type, aliases, etc
#[derive(Clone, Debug, Deserialize)]
pub struct Service {
    /// The unique identifier for the service to be used in the service catalog
    pub service_type: String,
    /// An ordered list of historical aliases for this service type.
    pub aliases: Option<Vec<String>>,
}

impl ServiceAuthority {
    /// Load service types from the official OpenStack authority data
    pub fn from_official_data() -> Result<Self, ServiceAuthorityError> {
        let data = include_str!("../../static/service-types.json");
        let authority: ServiceAuthority = serde_json::from_str(data)?;
        Ok(authority)
    }

    /// Get all known types of the service
    pub fn get_all_types_by_service_type<S: AsRef<str>>(
        &self,
        service_type: S,
    ) -> Result<Vec<String>, ServiceAuthorityError> {
        match &self.all_types_by_service_type {
            Some(data) => data
                .get(service_type.as_ref())
                .ok_or(ServiceAuthorityError::ServiceUnknown(
                    service_type.as_ref().into(),
                ))
                .cloned(),
            None => {
                warn!(
                    "No `all_types_by_service_type` is present in the authority. Reconstructing."
                );
                let service = self
                    .services
                    .iter()
                    .find(|x| x.service_type == service_type.as_ref())
                    .ok_or(ServiceAuthorityError::ServiceUnknown(
                        service_type.as_ref().into(),
                    ))?;
                let mut res = Vec::new();
                res.push(service.service_type.clone());
                if let Some(aliases) = &service.aliases {
                    for alias in aliases {
                        res.push(alias.clone());
                    }
                }
                Ok(res)
            }
        }
    }

    /// Get main service_type by service_type or alias
    #[allow(dead_code)]
    pub fn get_service_type_by_service_type_or_alias<S: AsRef<str>>(
        &self,
        service_type: S,
    ) -> Result<String, ServiceAuthorityError> {
        // Try forward lookup (it is the service itself)
        if self.forward.contains_key(service_type.as_ref()) {
            return Ok(service_type.as_ref().into());
        }

        // Lookup all_types_by_service_type
        if let Some(atbst) = &self.all_types_by_service_type {
            if atbst.contains_key(service_type.as_ref()) {
                return Ok(service_type.as_ref().into());
            }
        }

        // Reverse lookup
        if let Some(srv) = self.reverse.get(service_type.as_ref()) {
            return Ok(srv.into());
        }

        Err(ServiceAuthorityError::ServiceUnknown(
            service_type.as_ref().into(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_official() {
        let foo = ServiceAuthority::from_official_data().unwrap();
        assert!(!foo.services.is_empty());
        assert!(!foo.forward.is_empty());
        assert!(!foo.reverse.is_empty());
    }

    #[test]
    fn test_get_all_types_by_service_type() {
        let mut authority = ServiceAuthority {
            services: Vec::from([
                Service {
                    service_type: "foo".into(),
                    aliases: None,
                },
                Service {
                    service_type: "foo1".into(),
                    aliases: Some(Vec::from(["alias".into()])),
                },
            ]),
            all_types_by_service_type: Some(HashMap::from([
                ("foo".into(), Vec::from(["foo".into()])),
                ("foo1".into(), Vec::from(["foo1".into(), "alias".into()])),
            ])),
            ..Default::default()
        };
        assert_eq!(
            Vec::from(["foo"]),
            authority.get_all_types_by_service_type("foo").unwrap(),
        );
        assert_eq!(
            Vec::from(["foo1", "alias"]),
            authority.get_all_types_by_service_type("foo1").unwrap(),
        );
        assert!(authority.get_all_types_by_service_type("foo2").is_err());
        // And now test the reconstruction path
        authority.all_types_by_service_type = None;
        assert_eq!(
            Vec::from(["foo"]),
            authority.get_all_types_by_service_type("foo").unwrap(),
        );
        assert_eq!(
            Vec::from(["foo1", "alias"]),
            authority.get_all_types_by_service_type("foo1").unwrap(),
        );
    }

    #[test]
    fn test_get_service_type_by_service_type_or_alias() {
        let authority = ServiceAuthority {
            services: Vec::from([
                Service {
                    service_type: "foo".into(),
                    aliases: None,
                },
                Service {
                    service_type: "foo1".into(),
                    aliases: Some(Vec::from(["alias".into()])),
                },
            ]),
            forward: HashMap::from([("foo1".into(), Vec::from(["alias".into()]))]),
            reverse: HashMap::from([("alias".into(), "foo1".into())]),
            all_types_by_service_type: Some(HashMap::from([
                ("foo".into(), Vec::from(["foo".into()])),
                ("foo1".into(), Vec::from(["foo1".into(), "alias".into()])),
            ])),
            ..Default::default()
        };

        assert_eq!(
            "foo",
            authority
                .get_service_type_by_service_type_or_alias("foo")
                .unwrap()
        );
        // Test service_type with alias and using &str instead of String
        assert_eq!(
            "foo1",
            authority
                .get_service_type_by_service_type_or_alias("foo1")
                .unwrap()
        );
        assert_eq!(
            "foo1",
            authority
                .get_service_type_by_service_type_or_alias("alias")
                .unwrap()
        );
    }
}
