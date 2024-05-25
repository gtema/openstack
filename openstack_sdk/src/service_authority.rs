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

/// ServiceType Authority as provided by https://service-types.openstack.org/service-types.json
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
        let data = include_str!("../static/service-types.json");
        let authority: ServiceAuthority = serde_json::from_str(data)?;
        Ok(authority)
    }

    /// Get all known types of the service
    pub fn get_all_types_by_service_type(
        &self,
        service_type: &String,
    ) -> Result<Vec<String>, ServiceAuthorityError> {
        match &self.all_types_by_service_type {
            Some(data) => data
                .get(service_type)
                .ok_or(ServiceAuthorityError::ServiceUnknown(service_type.clone()))
                .cloned(),
            None => {
                warn!(
                    "No `all_types_by_service_type` is present in the authority. Reconstructing."
                );
                let service = self
                    .services
                    .iter()
                    .find(|x| x.service_type == *service_type)
                    .ok_or(ServiceAuthorityError::ServiceUnknown(service_type.clone()))?;
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
                    service_type: "foo".to_string(),
                    aliases: None,
                },
                Service {
                    service_type: "foo1".to_string(),
                    aliases: Some(Vec::from(["alias".to_string()])),
                },
            ]),
            all_types_by_service_type: Some(HashMap::from([
                ("foo".to_string(), Vec::from(["foo".to_string()])),
                (
                    "foo1".to_string(),
                    Vec::from(["foo1".to_string(), "alias".to_string()]),
                ),
            ])),
            ..Default::default()
        };
        assert_eq!(
            authority
                .get_all_types_by_service_type(&"foo".to_string())
                .unwrap(),
            Vec::from(["foo"])
        );
        assert_eq!(
            authority
                .get_all_types_by_service_type(&"foo1".to_string())
                .unwrap(),
            Vec::from(["foo1".to_string(), "alias".to_string()])
        );
        assert!(authority
            .get_all_types_by_service_type(&"foo2".to_string())
            .is_err());
        // And now test the reconstruction path
        authority.all_types_by_service_type = None;
        assert_eq!(
            authority
                .get_all_types_by_service_type(&"foo".to_string())
                .unwrap(),
            Vec::from(["foo"])
        );
        assert_eq!(
            authority
                .get_all_types_by_service_type(&"foo1".to_string())
                .unwrap(),
            Vec::from(["foo1".to_string(), "alias".to_string()])
        );
    }
}
