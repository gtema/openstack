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

//! Service Catalog processing
#![allow(dead_code)]

use bytes::Bytes;
use lazy_static::lazy_static;
use regex::Regex;
use serde::de::Deserializer;
use serde::Deserialize;
use std::collections::HashMap;
use std::num::ParseIntError;

use anyhow::Context;
use thiserror::Error;

use url::Url;

use tracing::{debug, error, info, trace};

use crate::config::CloudConfig;
use crate::service_authority::{ServiceAuthority, ServiceAuthorityError};
use crate::types::identity::v3::ServiceEndpoints;
use crate::types::ServiceType;

lazy_static! {
    static ref API_VERSION_REGEX: Regex =
        Regex::new(r"^v(?<major>[0-9]+)(?:\.)?(?<minor>[0-9]+)?$").unwrap();
}

#[derive(Debug, Clone, Default)]
pub struct ServiceEndpointInformation {}

/// Service catalog error
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum CatalogError {
    #[error("failed to parse url: {}", source)]
    UrlParse {
        #[from]
        source: url::ParseError,
    },
    #[error("Not a valid integer: {}", source)]
    ParseInt {
        #[from]
        source: ParseIntError,
    },
    #[error("Service Authority data cannot be parsed: {}", source)]
    ServiceAuthority {
        #[from]
        source: ServiceAuthorityError,
    },
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

/// Catalog endpoint
#[derive(Clone, Debug)]
pub struct Endpoint {
    pub url: Url,
    pub version: ApiVersion,
}

impl Endpoint {
    /// Build new Endpoint from string URL
    ///
    /// optional project_id is used to locate version information in the url
    pub fn from_url_string<S1: AsRef<str>, S2: AsRef<str>>(
        url: S1,
        project_id: &Option<S2>,
    ) -> Result<Self, CatalogError> {
        let url = Url::parse(url.as_ref())
            .with_context(|| format!("Wrong endpoint URL: `{}`", url.as_ref()))?;
        let version = ApiVersion::from_url(&url, project_id)?;
        let res = Self { url, version };
        Ok(res)
    }
}

/// ApiVersion
///
/// ApiVersion of the Endpoint as described in
/// https://specs.openstack.org/openstack/api-sig/guidelines/consuming-catalog/version-discovery.html
#[derive(Clone, Debug)]
pub struct ApiVersion {
    pub major: u8,
    pub minor: u8,
}

impl ApiVersion {
    /// Determine Api Version based on the Endpoint URL and optional project_id
    pub fn from_url<S: AsRef<str>>(
        url: &Url,
        project_id: &Option<S>,
    ) -> Result<Self, CatalogError> {
        let mut res = Self { major: 0, minor: 0 };
        // Find the version as described under
        // https://specs.openstack.org/openstack/api-sig/guidelines/consuming-catalog/version-discovery.html#inferring-version
        let mut path_segments = url.path_segments().map_or(Vec::new(), |x| {
            x.into_iter().filter(|x| !x.is_empty()).collect()
        });
        if let Some(pid) = project_id {
            if let Some(last) = path_segments.last() {
                if last.ends_with(pid.as_ref()) {
                    path_segments.pop();
                }
            }
        }
        if let Some(last) = path_segments.last() {
            if let Some(cap) = API_VERSION_REGEX.captures(last) {
                res.major = cap["major"].parse()?;
                if let Some(vmin) = cap.name("minor") {
                    res.minor = vmin.as_str().parse()?;
                }
            }
        }
        Ok(res)
    }
}

/// ServiceEndpoint data
#[derive(Debug, Clone)]
pub struct ServiceEndpoint {
    pub url: Url,
    pub discovered: bool,
    pub versions: Vec<EndpointVersion>,
    pub current_version: Option<EndpointVersion>,
}

impl ServiceEndpoint {
    /// Build new ServiceEndpoint from string URL
    pub fn from_url_string(fixed_url: &str) -> Result<Self, CatalogError> {
        Ok(Self {
            url: Url::parse(fixed_url)
                .with_context(|| format!("Wrong endpoint URL: `{}`", fixed_url))?,
            discovered: false,
            versions: Vec::new(),
            current_version: None,
        })
    }

    /// Process Endpoint version discovery response
    pub fn process_discovery(&mut self, data: &Bytes) -> Result<(), CatalogError> {
        // Unversioned endpoint normally returns: `{versions: []}`
        if let Ok(versions) = serde_json::from_slice::<EndpointVersions>(data) {
            self.versions.clear();
            for ver in versions.versions {
                self.process_version_information(&ver)?;
            }
        } else if let Ok(version) = serde_json::from_slice::<EndpointVersionContainer>(data) {
            // Versioned endpoint normally returns: `{version: {}}`
            self.versions.clear();
            self.process_version_information(&version.version)?;
        } else if let Ok(vers) = serde_json::from_slice::<EndpointVersionsValues>(data) {
            // Keystone returns `{versions: {values: []}}}`
            self.versions.clear();
            for ver in vers.versions.values {
                self.process_version_information(&ver)?;
            }
        }
        self.discovered = true;
        debug!("Endpoint info {:?}", self);
        Ok(())
    }

    /// Process single Endpoint Version information
    fn process_version_information(&mut self, ver: &EndpointVersion) -> Result<(), CatalogError> {
        self.versions.push(ver.clone());
        if ver.status == EndpointVersionStatus::Current {
            self.current_version = Some(ver.clone());
            for link in &ver.links {
                if link.rel == "self" && link.href.starts_with(self.url.as_str()) {
                    // When link.rel is "self" and link.href is more preciese
                    // than what we've got from catalog - take it as our endpoint
                    self.url = Url::parse(&link.href)
                        .with_context(|| format!("Wrong endpoint URL: `{}`", link.href))?;
                    // Path MUST end with "/"
                    if !self.url.path().ends_with('/') {
                        let mut new_path: String = self.url.path().into();
                        new_path.push('/');
                        self.url.set_path(new_path.as_str());
                    }

                    info!("Using discovered URL `{:?}` as the base", self.url);
                } else {
                    info!("discovery {:?} vs {:?}", link.href, self.url);
                }
            }
        }
        Ok(())
    }
}

/// Endpoint version status
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "lowercase")]
#[serde(remote = "EndpointVersionStatus")]
pub enum EndpointVersionStatus {
    Current,
    Supported,
    Deprecated,
    Experimental,
    #[serde(other)]
    Unknown,
}

impl<'de> Deserialize<'de> for EndpointVersionStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match s.to_lowercase().as_str() {
            "current" => Ok(EndpointVersionStatus::Current),
            // Keystone "stable" is same as "current" everywhere else
            "stable" => Ok(EndpointVersionStatus::Current),
            "supported" => Ok(EndpointVersionStatus::Supported),
            "deprecated" => Ok(EndpointVersionStatus::Deprecated),
            "experimental" => Ok(EndpointVersionStatus::Experimental),
            _ => Ok(EndpointVersionStatus::Unknown),
        }
    }
}

/// Link structure
#[derive(Debug, Deserialize, Clone)]
pub struct Link {
    pub href: String,
    pub rel: String,
}

/// Single Endpoint Version structure
#[derive(Debug, Deserialize, Clone)]
pub struct EndpointVersionContainer {
    pub version: EndpointVersion,
}

/// Single Endpoint Version structure representation
#[derive(Debug, Deserialize, Clone)]
pub struct EndpointVersion {
    pub id: String,
    pub status: EndpointVersionStatus,
    pub version: Option<String>,
    pub min_version: Option<String>,
    pub links: Vec<Link>,
}

/// `Versions` array of Endpoint Versions
#[derive(Debug, Deserialize, Clone)]
pub(crate) struct EndpointVersions {
    pub versions: Vec<EndpointVersion>,
}

/// `Versions.values` structure
#[derive(Debug, Deserialize, Clone)]
pub(crate) struct EndpointVersionsValues {
    pub versions: EndpointVersionValues,
}

/// Endpoint version values structure
#[derive(Debug, Deserialize, Clone)]
pub(crate) struct EndpointVersionValues {
    pub values: Vec<EndpointVersion>,
}

/// Structure representing session ServiceCatalog
#[derive(Debug, Clone)]
pub(crate) struct Catalog {
    /// HashMap containing service endpoints by the service type
    service_endpoints: HashMap<String, ServiceEndpoint>,
    token_catalog: Option<Vec<ServiceEndpoints>>,
    /// Configured endpoint overrides
    endpoint_overrides: HashMap<String, ServiceEndpoint>,
    /// Service Authority data
    service_authority: ServiceAuthority,
}

impl Default for Catalog {
    fn default() -> Self {
        Self {
            service_endpoints: HashMap::new(),
            token_catalog: None,
            endpoint_overrides: HashMap::new(),
            service_authority: ServiceAuthority::from_official_data().unwrap_or_default(),
        }
    }
}

impl Catalog {
    /// Build service endpoint instance from parameters
    fn build_service_endpoint(&self, url: &str) -> Result<ServiceEndpoint, CatalogError> {
        let mut fixed_url: String = url.into();
        if !fixed_url.ends_with('/') {
            fixed_url.push('/');
        }

        ServiceEndpoint::from_url_string(&fixed_url)
    }

    /// Register single service endpoint
    pub(crate) fn add_service_endpoint(
        &mut self,
        service_type: &str,
        url: &str,
    ) -> Result<(), CatalogError> {
        //// Set the URL from catalog/input respecting known overrides
        self.service_endpoints.insert(
            service_type.to_string(),
            match self.endpoint_overrides.get(service_type) {
                Some(ep) => ep.clone(),
                None => self.build_service_endpoint(url)?,
            },
        );
        Ok(())
    }

    /// Process catalog information from the token
    pub(crate) fn process_catalog_endpoints(
        &mut self,
        srv_endpoints: &Vec<ServiceEndpoints>,
        interface: Option<&str>,
    ) -> Result<(), CatalogError> {
        trace!("Start processing ServiceCatalog response");
        let mut token_catalog = Vec::new();
        let intf = interface.unwrap_or("public");
        for srv in srv_endpoints {
            trace!("Processing service {:?}", srv);
            token_catalog.push(srv.clone());
            for ep in &srv.endpoints {
                trace!("Processing endpoint {:?}", ep);
                if ep.interface == intf {
                    self.add_service_endpoint(&srv.service_type, &ep.url)
                        .with_context(|| {
                            format!(
                                "While processing service catalog response for service {}",
                                srv.service_type
                            )
                        })?;
                }
            }
        }
        self.token_catalog = Some(token_catalog);
        Ok(())
    }

    /// Get URL for the endpoint by the service_type
    pub(crate) fn get_service_endpoint(
        &self,
        service_type: &ServiceType,
    ) -> Option<ServiceEndpoint> {
        let srv_type_string: String = service_type.to_string();
        for cat_type in self
            .service_authority
            .get_all_types_by_service_type(&srv_type_string)
            .unwrap_or(Vec::from([srv_type_string.clone()]))
        {
            if let Some(sep) = self.service_endpoints.get(&cat_type) {
                debug!("Service endpoint url = {}", sep.url);
                info!("Service info = {:?}", sep);
                return Some(sep.clone());
            }
        }
        // There is nothing in the altered catalog, but what if the service is not present in the
        // catalog while being set as endpoint_override
        self.endpoint_overrides.get(&srv_type_string).cloned()
    }

    /// Invoke process_discovery of the endpoint by the service type
    ///
    /// This is implemented this way since it is hard to get mutable entry
    /// from the mutable catalog in the main client while invoking non
    /// mutable methods (in openstack::discover_service_endpoint).
    pub(crate) fn process_endpoint_discovery(
        &mut self,
        service_type: &ServiceType,
        data: &Bytes,
    ) -> Result<(), CatalogError> {
        let srv_type_string: String = service_type.to_string();
        for cat_type in self
            .service_authority
            .get_all_types_by_service_type(&srv_type_string)
            .unwrap_or(Vec::from([srv_type_string.clone()]))
        {
            if let Some(sep) = self.service_endpoints.get_mut(&cat_type) {
                return sep.process_discovery(data);
            }
        }

        Ok(())
    }

    /// Return catalog endpoints as returned in the authorization response
    pub fn get_token_catalog(&self) -> Option<Vec<ServiceEndpoints>> {
        self.token_catalog.clone()
    }

    // Save endpoint overrides given in the config
    pub fn set_endpoint_overrides(
        &mut self,
        config: &CloudConfig,
    ) -> Result<&mut Self, CatalogError> {
        for (name, val) in config.options.iter() {
            if name.ends_with("_endpoint_override") {
                let len = name.len();
                let srv_type = &name[..(len - 18)];
                let service_type = &srv_type.replace('_', "-");

                self.endpoint_overrides.insert(
                    service_type.to_string(),
                    self.build_service_endpoint(&val.to_string())?,
                );
            }
        }
        Ok(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use url::Url;

    #[test]
    fn test_service_endpoint() {
        let matrix = [
            ("http://test.com/foo", None, 0, 0),
            ("http://test.com/v1", None, 1, 0),
            ("http://test.com/prefix/v1", None, 1, 0),
            ("http://test.com/prefix/v1", Some("project"), 1, 0),
            ("http://test.com/prefix/v1/project", Some("project"), 1, 0),
            ("http://test.com/prefix/v2.3/project", Some("project"), 2, 3),
            (
                "http://test.com/prefix/v1/AUTH_project",
                Some("project"),
                1,
                0,
            ),
        ];
        for (url, pid, maj, min) in matrix.iter() {
            let sep = Endpoint::from_url_string(url, pid).unwrap();
            assert_eq!(sep.url, Url::parse(url).unwrap());
            assert_eq!(
                (*maj, *min),
                (sep.version.major, sep.version.minor),
                "Major version of {} must be {}.{}",
                url,
                maj,
                min
            );
        }
    }
}
