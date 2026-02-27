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
//!
//! OpenStack has a concept of `Service Catalog` which allows the client to discover list of
//! services and their versions on the cloud.
//!
//! The process is described on
//! <https://specs.openstack.org/openstack/api-sig/guidelines/consuming-catalog.html>

use bytes::Bytes;
use std::collections::{HashMap, HashSet};
use tracing::{debug, error, trace, warn};
use url::Url;

pub use crate::catalog::error::CatalogError;
pub use crate::catalog::service_endpoint::ServiceEndpoint;
use crate::catalog::{service_authority::ServiceAuthority, service_endpoint::ServiceEndpoints};
use crate::config::CloudConfig;
use crate::types::{
    ServiceType, api_version::ApiVersion, identity::v3::ServiceEndpoints as ApiServiceEndpoints,
};

mod discovery;
mod error;
mod service_authority;
mod service_endpoint;

/// Structure representing session ServiceCatalog
#[derive(Debug, Clone)]
pub struct Catalog {
    /// Current project_id
    project_id: Option<String>,

    /// Current region
    region: Option<String>,

    /// HashMap containing service endpoints by the service type
    pub(crate) service_endpoints: HashMap<String, ServiceEndpoints>,

    /// Catalog information as presented in the token
    token_catalog: Option<Vec<ApiServiceEndpoints>>,

    /// Configured endpoint overrides
    endpoint_overrides: HashMap<String, ServiceEndpoint>,

    /// Service Authority data
    service_authority: ServiceAuthority,

    /// Service endpoints as configured by the service catalog
    catalog_endpoints: HashMap<String, ServiceEndpoints>,

    /// Skip discovery configuration
    skip_discovery: HashSet<String>,
}

impl Default for Catalog {
    fn default() -> Self {
        Self {
            project_id: None,
            region: None,
            service_endpoints: HashMap::new(),
            token_catalog: None,
            endpoint_overrides: HashMap::new(),
            service_authority: ServiceAuthority::from_official_data().unwrap_or_default(),
            catalog_endpoints: HashMap::new(),
            skip_discovery: HashSet::from(["object-store".into()]),
        }
    }
}

impl Catalog {
    pub fn discovery_allowed<S: AsRef<str>>(&self, service_type: S) -> bool {
        !self.skip_discovery.contains(service_type.as_ref())
    }
    /// Set project_id for the catalog scope knowledge. This influences certain discovery
    /// mechanisms but should be also safe to skip.
    pub fn set_project_id<S: AsRef<str>>(&mut self, project_id: Option<S>) -> &mut Self {
        self.project_id = project_id.map(|x| x.as_ref().into());
        self
    }

    /// Register single service endpoint as catalog endpoint
    pub(crate) fn register_catalog_endpoint<S1: AsRef<str>, S2: AsRef<str>, S3: AsRef<str>>(
        &mut self,
        service_type: S1,
        url: &str,
        region: Option<S2>,
        interface: Option<S3>,
    ) -> Result<&mut Self, CatalogError> {
        self.catalog_endpoints
            .entry(service_type.as_ref().into())
            .or_default()
            .push(
                ServiceEndpoint::from_url_string(url, self.project_id.as_ref())?
                    .set_region(region)
                    .set_interface(interface)
                    .to_owned(),
            );
        Ok(self)
    }

    /// Process catalog information (usually from the token)
    pub(crate) fn process_catalog_endpoints(
        &mut self,
        srv_endpoints: &Vec<ApiServiceEndpoints>,
        interface: Option<&str>,
    ) -> Result<&mut Self, CatalogError> {
        trace!("Start processing ServiceCatalog response");
        let mut token_catalog = Vec::new();
        let intf = interface.unwrap_or("public");
        // Reset all previously discovered information. If we re-authed we may get different
        // project_ids
        for (srv, val) in self.service_endpoints.iter_mut() {
            // We needed to discover identity before we ever tried to connect to the cloud. It does
            // not support project_id part of the URL by nature so we skip resetting it, otherwise
            // we need to again discover identity endpoint.
            if srv != "identity" {
                val.clear();
            }
        }
        for srv in srv_endpoints {
            trace!("Processing catalog service {:?}", srv);
            token_catalog.push(srv.clone());
            if let Some(cat_service) = self.catalog_endpoints.get_mut(&srv.service_type) {
                // Clear all endpoints processed in previous catalog processing
                cat_service.clear();
            }
            for ep in &srv.endpoints {
                trace!("Processing endpoint {:?}", ep);
                if ep.interface == intf {
                    self.register_catalog_endpoint(
                        &srv.service_type,
                        &ep.url,
                        ep.region.clone(),
                        Some(&ep.interface),
                    )?;
                }
            }
        }
        self.token_catalog = Some(token_catalog);
        Ok(self)
    }

    /// Get endpoint
    pub(crate) fn get_service_endpoint<S1: AsRef<str>, S2: AsRef<str>>(
        &self,
        service_type: S1,
        api_version: Option<&ApiVersion>,
        region_name: Option<S2>,
    ) -> Result<&ServiceEndpoint, CatalogError> {
        // Check for endpoint_override -> return override directly
        if let Some(ep) = &self.endpoint_overrides.get(service_type.as_ref()) {
            debug!(
                "Using `{}_endpoint_override` [`{}`] as endpoint for version `{:?}`",
                service_type.as_ref(),
                ep.url_str(),
                api_version
            );
            return Ok(ep);
        }

        // Find main service_type
        let main_service_type = self
            .service_authority
            .get_service_type_by_service_type_or_alias(&service_type)?;

        // Determine service_type for which there is catalog entry
        let catalog_service_type = match self.catalog_endpoints.contains_key(service_type.as_ref())
        {
            true => service_type.as_ref(),
            false => &main_service_type,
        };

        // Get catalog endpoint
        let catalog_endpoint: Option<&ServiceEndpoint> = self
            .catalog_endpoints
            .get(catalog_service_type)
            .and_then(|eps| match region_name {
                Some(_) => eps.get_by_region(region_name.as_ref()),
                None => eps.get_by_region(self.region.as_ref()),
            });
        // TODO: check the API version

        // Get discovered information for the main service
        if let Some(discovered_endpoints) = self.service_endpoints.get(&main_service_type) {
            if let Some(ep) =
                discovered_endpoints.get_by_version_and_region(api_version, region_name.as_ref())
            {
                debug!(
                    "Using discovered endpoint `{:?}` for service_type: `{}` and version `{:?}`",
                    ep,
                    service_type.as_ref(),
                    api_version
                );
                return Ok(ep);
            }
            if let Some(ver) = api_version {
                // Specific version requested, version discovery info available, but version is not
                // present - error
                return Err(CatalogError::VersionUnsupported { ver: ver.clone() });
            }
        }

        // No override and no discovered service info -> just return what is in the service catalog
        if let Some(ep) = catalog_endpoint {
            debug!(
                "Using catalog endpoint `{:?}` for service_type: `{}` and version `{:?}`",
                ep,
                service_type.as_ref(),
                api_version
            );
            return Ok(ep);
        }
        // No direct matches can be i.e. explain by absent discovery info (at start), catalog
        // containing `alias` and request for the main service
        for cat_type in self
            .service_authority
            .get_all_types_by_service_type(main_service_type)?
        {
            if let Some(catalog_eps) = &self.catalog_endpoints.get(&cat_type) {
                if let Some(catalog_ep) =
                    catalog_eps.get_by_version_and_region(api_version, region_name.as_ref())
                {
                    debug!(
                        "Using catalog endpoint `{:?}` for service_type: `{}` (requested: `{}`) and version `{:?}`",
                        catalog_ep,
                        cat_type,
                        service_type.as_ref(),
                        api_version
                    );
                    return Ok(catalog_ep);
                }
            }

            if let Some(ep_override) = &self.endpoint_overrides.get(&cat_type) {
                if let Some(requested_ver) = api_version {
                    // Entry in the overrides with version match
                    if ep_override.version() == requested_ver {
                        debug!(
                            "Using `{}_endpoint_override` as endpoint for service_type: `{}` and version `{:?}`",
                            cat_type,
                            service_type.as_ref(),
                            api_version
                        );
                        return Ok(ep_override);
                    }
                } else {
                    // No version was requested, take first configured alias
                    debug!(
                        "Using `{}_endpoint_override` as endpoint for service_type: `{}`",
                        cat_type,
                        service_type.as_ref()
                    );
                    return Ok(ep_override);
                }
            }
        }

        Err(CatalogError::ServiceNotConfigured(
            service_type.as_ref().into(),
        ))
    }

    /// Process version discovery response for the service returning list of extracted endpoints
    pub(crate) fn parse_endpoint_discovery<S: AsRef<str>>(
        &self,
        service_type: &ServiceType,
        url: &Url,
        data: &Bytes,
        region: Option<S>,
    ) -> Result<ServiceEndpoints, CatalogError> {
        let mut service_entry = ServiceEndpoints::default();
        // Get main service type
        let main_service_type = self
            .service_authority
            .get_service_type_by_service_type_or_alias(service_type.to_string())?;
        // Get all known service aliases
        let catalog_types = self
            .service_authority
            .get_all_types_by_service_type(main_service_type)?;
        for ep in
            discovery::extract_discovery_endpoints(url, data, service_type.to_string())?.iter_mut()
        {
            ep.set_region(region.as_ref());
            // Rounded (to the major) version so that we respect catalog_endpoints and
            // endpoint_overrides for a higher minor versions
            let ep_maj_ver = ApiVersion::new(ep.version().major, 0);

            // iterate over all known endpoint overrides and catalog endpoints for the service for
            // the corresponding version to identify whether it should end with project_id
            for cat_type in catalog_types.iter() {
                if let Some(epo) = &self.endpoint_overrides.get(cat_type) {
                    if epo.version().major == ep.version().major {
                        // Endpoint override applies to all microversions of the service of the
                        // major version
                        ep.set_last_segment_with_project_id(
                            epo.last_segment_with_project_id().clone(),
                        );
                    }
                }
                if let Some(ep_cat) = &self.catalog_endpoints.get(cat_type).and_then(|srv| {
                    srv.get_by_version_and_region(Some(&ep_maj_ver), region.as_ref())
                }) {
                    ep.set_last_segment_with_project_id(
                        ep_cat.last_segment_with_project_id().clone(),
                    );
                }
            }
            if !ep.build_request_url("./")?.path().starts_with(url.path()) {
                warn!(
                    "Discovered url [{}] does not have the same prefix as the url it was discovered from [{}]. This may indicate misconfiguration [https://gtema.github.io/openstack/possible_errors.html#discovered-url-has-different-prefix]",
                    ep.build_request_url("./")?.as_str(),
                    url.as_str()
                );
            }

            service_entry.push(ep.to_owned());
        }
        Ok(service_entry)
    }

    /// Consume service endpoints as discovered
    pub(crate) fn consume_discovered_endpoints(
        &mut self,
        service_type: &ServiceType,
        endpoints: ServiceEndpoints,
    ) -> Result<&mut Self, CatalogError> {
        let service_entry = self
            .service_endpoints
            .entry(service_type.to_string())
            .or_default();
        for ep in endpoints.get_all().iter() {
            service_entry.push(ep.to_owned());
        }
        Ok(self)
    }

    /// Parse service endpoint discovery document and set discovered endpoints.
    pub(crate) fn process_endpoint_discovery<S: AsRef<str>>(
        &mut self,
        service_type: &ServiceType,
        url: &Url,
        data: &Bytes,
        region: Option<S>,
    ) -> Result<&mut Self, CatalogError> {
        let discovered_endpoints =
            self.parse_endpoint_discovery(service_type, url, data, region)?;
        self.consume_discovered_endpoints(service_type, discovered_endpoints)
    }

    /// Return catalog endpoints as returned in the authorization response
    pub fn get_token_catalog(&self) -> Option<Vec<ApiServiceEndpoints>> {
        self.token_catalog.clone()
    }

    // Save endpoint overrides given in the config
    pub fn configure(&mut self, config: &CloudConfig) -> Result<&mut Self, CatalogError> {
        for (name, val) in config.options.iter() {
            if name.ends_with("_endpoint_override") {
                let len = name.len();
                let srv_type = &name[..(len - 18)];
                let service_type = &srv_type.replace('_', "-");

                // If URL is not valid not raise an error, but instead only log an error and ignore
                match ServiceEndpoint::from_url_string(val.to_string(), self.project_id.as_ref()) {
                    Ok(ep) => {
                        self.endpoint_overrides.insert(service_type.to_string(), ep);
                    }
                    Err(err) => {
                        error!("Error processing {}: {}", name, err);
                    }
                }
            } else if name.ends_with("_skip_discovery") {
                let len = name.len();
                let srv_type = &name[..(len - 15)];
                let service_type = &srv_type.replace('_', "-");
                match val.clone().into_bool() {
                    Ok(true) => {
                        self.skip_discovery.insert(service_type.to_string());
                    }
                    Ok(false) => {
                        self.skip_discovery.remove(service_type);
                    }
                    _ => {}
                };
            }
            if name == "region_name" {
                self.region = Some(val.to_string());
            }
        }
        Ok(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use url::Url;

    use crate::types::api_version::ApiVersion;

    #[test]
    fn test_set_project_id() {
        let mut cat = Catalog::default();
        assert!(cat.project_id.is_none());
        assert_eq!(
            Some("foo"),
            cat.set_project_id(Some("foo")).project_id.as_deref()
        );
        assert_eq!(
            Some("bar"),
            cat.set_project_id(Some("bar")).project_id.as_deref()
        );
        assert!(cat.set_project_id(None::<String>).project_id.is_none());
    }

    #[test]
    fn test_configure() {
        let mut cat = Catalog::default();
        let conf = CloudConfig {
            options: HashMap::from([
                (
                    "s1_endpoint_override".into(),
                    config::Value::from("http://foo.bar/v3/wrong"),
                ),
                (
                    "s2_endpoint_override".into(),
                    config::Value::from("http://foo.bar/v3"),
                ),
                (
                    "s3_endpoint_override".into(),
                    config::Value::from("http://foo.bar"),
                ),
                (
                    "s4_endpoint_override".into(),
                    config::Value::from("uni://foo/bar"),
                ),
                (
                    "s5_endpoint_override".into(),
                    config::Value::from("http://foo.bar/z_PROJECT_ID"),
                ),
            ]),
            ..Default::default()
        };
        cat.set_project_id(Some("PROJECT_ID"));
        cat.configure(&conf).unwrap();
        let val = cat.endpoint_overrides.get("s1").unwrap();
        assert_eq!("http://foo.bar/v3/wrong", val.url_str());
        assert_eq!(&ApiVersion { major: 0, minor: 0 }, val.version());
        let val = cat.endpoint_overrides.get("s2").unwrap();
        assert_eq!("http://foo.bar/v3", val.url_str());
        assert_eq!(&ApiVersion { major: 3, minor: 0 }, val.version());
        let val = cat.endpoint_overrides.get("s3").unwrap();
        assert_eq!("http://foo.bar/", val.url_str());
        assert_eq!(&ApiVersion { major: 0, minor: 0 }, val.version());
        assert!(
            !cat.endpoint_overrides.contains_key("s4"),
            "Ensure bad URL is simply ignored"
        );
        let val = cat.endpoint_overrides.get("s5").unwrap();
        assert_eq!("http://foo.bar/z_PROJECT_ID", val.url_str());
        assert_eq!(
            &Some("z_PROJECT_ID".into()),
            val.last_segment_with_project_id()
        );
    }

    #[test]
    fn test_register_catalog_endpoint() {
        let mut cat = Catalog::default();
        cat.register_catalog_endpoint("s1", "http://s1.foo.bar", Some("public"), Some("default"))
            .unwrap();
        let check = cat.catalog_endpoints.get("s1").unwrap();
        assert_eq!(
            "http://s1.foo.bar/",
            check.get_by_region(None::<String>).unwrap().url_str()
        );
    }

    #[test]
    fn test_process_catalog_endpoints() {
        let mut cat = Catalog::default();
        cat.register_catalog_endpoint("s1", "http://s1.foo.bar", Some("public"), Some("default"))
            .unwrap();
        cat.process_catalog_endpoints(
            &Vec::from([
                serde_json::from_value(json!({
                    "type": "s2",
                    "name": "s2",
                    "endpoints": [
                        {"id": "dummy", "interface": "public", "region": "r1", "url": "http://r1.foo.bar/s2"},
                        {"id": "dummy", "interface": "public", "region": "r2", "url": "http://r2.foo.bar/s2"}
                    ]
                })).unwrap(),
                serde_json::from_value(json!({
                    "type": "s3",
                    "name": "s3",
                    "endpoints": [
                        {"id": "dummy", "interface": "public", "region": "r1", "url": "http://r2.foo.bar/s3"}
                    ]
                })).unwrap()
            ]),
            Some("public"),
        ).unwrap();
        let check = cat.catalog_endpoints.get("s1").unwrap();
        assert_eq!(
            "http://s1.foo.bar/",
            check.get_by_region(None::<String>).unwrap().url_str()
        );
        let check = cat.catalog_endpoints.get("s2").unwrap();
        assert_eq!(
            Vec::from(["http://r1.foo.bar/s2", "http://r2.foo.bar/s2"]),
            check
                //.0
                .get_all()
                .iter()
                .map(|x| x.url_str())
                .collect::<Vec<_>>()
        );
        // TODO: Check token_catalog
    }

    #[test]
    fn test_process_endpoint_discovery() {
        let mut cat = Catalog::default();
        cat.process_endpoint_discovery(
            &ServiceType::Compute,
            &Url::parse("http://foo.bar/v2").unwrap(),
            &Bytes::from(
                json!({
                  "version": {
                    "status": "SUPPORTED",
                    "id": "v2.0",
                    "links": [
                      {
                        "href": "http://compute.example.com/v2/",
                        "rel": "self"
                      },
                      {
                        "href": "http://compute.example.com/",
                        "rel": "collection"
                      }
                    ]
                  }
                })
                .to_string(),
            ),
            Some("default"),
        )
        .unwrap();
        assert!(cat.service_endpoints.contains_key("compute"));
    }

    #[test]
    fn test_process_endpoint_discovery_with_endpoint_override() {
        let mut cat = Catalog::default();
        let conf = CloudConfig {
            options: HashMap::from([(
                "volumev3_endpoint_override".into(),
                config::Value::from("http://another.foo.bar/v3/z_PROJECT_ID"),
            )]),
            ..Default::default()
        };
        cat.set_project_id(Some("PROJECT_ID"));
        cat.configure(&conf).unwrap();
        cat.register_catalog_endpoint(
            "volumev2",
            "http://another.foo.bar/v2/y_PROJECT_ID",
            Some("default"),
            Some("public"),
        )
        .unwrap();
        cat.process_endpoint_discovery(
            &ServiceType::BlockStorage,
            &Url::parse("http://foo.bar").unwrap(),
            &Bytes::from(
                json!({
                  "versions": [
                    {
                      "status": "SUPPORTED",
                      "links": [
                        {
                          "href": "http://foo.bar/v2/",
                          "rel": "self"
                        }
                      ],
                      "id": "v2"
                    }, {
                      "status": "CURRENT",
                      "links": [
                        {
                          "href": "http://foo.bar/v3/",
                          "rel": "self"
                        }
                      ],
                      "id": "v3"
                    }
                  ]
                })
                .to_string(),
            ),
            Some("default"),
        )
        .unwrap();
        let srv = cat.service_endpoints.get("block-storage").unwrap();
        let ep = srv
            .get_by_version_and_region(Some(&ApiVersion::new(3, 0)), Some("default"))
            .unwrap();
        assert_eq!("http://foo.bar/v3/", ep.url_str(), "Versioned endpoint");
        assert_eq!(
            &Some("z_PROJECT_ID".into()),
            ep.last_segment_with_project_id(),
            "Contains project_id suffix of the endpoint override"
        );
        let ep = srv
            .get_by_version_and_region(Some(&ApiVersion::new(2, 0)), Some("default"))
            .unwrap();
        assert_eq!("http://foo.bar/v2/", ep.url_str());
        assert_eq!(
            &Some("y_PROJECT_ID".into()),
            ep.last_segment_with_project_id(),
            "Contains project_id suffix of the catalog endpoint"
        );
        let ep = srv
            .get_by_version_and_region(Some(&ApiVersion::new(0, 0)), Some("default"))
            .unwrap();
        assert_eq!("http://foo.bar/", ep.url_str());
        assert_eq!(
            &None,
            ep.last_segment_with_project_id(),
            "Base service endpoint does not contain project_id suffix"
        );
    }

    #[test]
    fn test_process_endpoint_discovery_with_multiversion() {
        let mut cat = Catalog::default();
        cat.set_project_id(Some("PROJECT_ID"));
        cat.register_catalog_endpoint(
            "compute",
            "http://foo.bar/v2/y_PROJECT_ID",
            Some("default"),
            Some("public"),
        )
        .unwrap();
        cat.process_endpoint_discovery(
            &ServiceType::Compute,
            &Url::parse("http://foo.bar").unwrap(),
            &Bytes::from(
                json!({
                  "versions": [
                    {
                      "status": "SUPPORTED",
                      "links": [
                        {
                          "href": "http://foo.bar/v2/",
                          "rel": "self"
                        }
                      ],
                      "min_version": "",
                      "id": "v2.0"
                    }, {
                      "status": "CURRENT",
                      "links": [
                        {
                          "href": "http://foo.bar/v2.1/",
                          "rel": "self"
                        }
                      ],
                      "min_version": "2.1",
                      "version": "2.90",
                      "id": "v2.1"
                    }
                  ]
                })
                .to_string(),
            ),
            Some("default"),
        )
        .unwrap();
        let srv = cat.service_endpoints.get("compute").unwrap();
        let ep = srv
            .get_by_version_and_region(Some(&ApiVersion::new(2, 0)), Some("default"))
            .unwrap();
        assert_eq!("http://foo.bar/v2/", ep.url_str());
        assert_eq!(
            &Some("y_PROJECT_ID".into()),
            ep.last_segment_with_project_id(),
            "base version"
        );
        let ep = srv
            .get_by_version_and_region(Some(&ApiVersion::new(2, 10)), Some("default"))
            .unwrap();
        assert_eq!("http://foo.bar/v2.1/", ep.url_str());
        assert_eq!(
            &Some("y_PROJECT_ID".into()),
            ep.last_segment_with_project_id(),
            "newest microversion"
        );
    }

    #[test]
    fn test_get_service_endpoint() {
        let mut cat = Catalog::default();
        cat.process_endpoint_discovery(
            &ServiceType::BlockStorage,
            &Url::parse("http://foo.bar/").unwrap(),
            &Bytes::from(
                json!({
                  "versions": [
                    {
                      "status": "SUPPORTED",
                      "links": [
                        {
                          "href": "http://foo.bar/v2/",
                          "rel": "self"
                        }
                      ],
                      "id": "v2"
                    }, {
                      "status": "CURRENT",
                      "links": [
                        {
                          "href": "http://foo.bar/v3/",
                          "rel": "self"
                        }
                      ],
                      "id": "v3"
                    }
                  ]
                })
                .to_string(),
            ),
            Some("default"),
        )
        .unwrap();
        assert_eq!(
            "http://foo.bar/v2/",
            cat.get_service_endpoint("volume", Some(&ApiVersion::new(2, 0)), Some("default"))
                .unwrap()
                .url_str(),
            "alias service type versioned"
        );
        assert_eq!(
            "http://foo.bar/v2/",
            cat.get_service_endpoint(
                "block-storage",
                Some(&ApiVersion::new(2, 0)),
                Some("default")
            )
            .unwrap()
            .url_str(),
            "main service type versioned"
        );
        assert_eq!(
            "http://foo.bar/v3/",
            cat.get_service_endpoint(
                "block-storage",
                Some(&ApiVersion::new(3, 0)),
                Some("default")
            )
            .unwrap()
            .url_str(),
            "main service type versioned"
        );
        assert_eq!(
            "http://foo.bar/",
            cat.get_service_endpoint(
                "block-storage",
                Some(&ApiVersion::new(0, 0)),
                Some("default")
            )
            .unwrap()
            .url_str(),
            "main service type unversioned"
        );
    }

    #[test]
    fn test_get_service_endpoint_overrides() {
        let mut cat = Catalog::default();
        let conf = CloudConfig {
            options: HashMap::from([(
                "volumev3_endpoint_override".into(),
                config::Value::from("http://another.foo.bar/v3/z_PROJECT_ID"),
            )]),
            ..Default::default()
        };
        cat.set_project_id(Some("PROJECT_ID"));
        cat.configure(&conf).unwrap();
        cat.register_catalog_endpoint(
            "volumev2",
            "http://another.foo.bar/v2/y_PROJECT_ID",
            Some("default"),
            Some("public"),
        )
        .unwrap();

        cat.process_endpoint_discovery(
            &ServiceType::BlockStorage,
            &Url::parse("http://foo.bar/").unwrap(),
            &Bytes::from(
                json!({
                  "versions": [
                    {
                      "status": "SUPPORTED",
                      "links": [
                        {
                          "href": "http://foo.bar/v2/",
                          "rel": "self"
                        }
                      ],
                      "id": "v2"
                    }, {
                      "status": "CURRENT",
                      "links": [
                        {
                          "href": "http://foo.bar/v3/",
                          "rel": "self"
                        }
                      ],
                      "min_version": "3.0",
                      "max_version": "3.15",
                      "id": "v3"
                    }
                  ]
                })
                .to_string(),
            ),
            Some("default"),
        )
        .unwrap();
        assert_eq!(
            "http://foo.bar/v2/",
            cat.get_service_endpoint("volume", Some(&ApiVersion::new(2, 0)), Some("default"))
                .unwrap()
                .url_str(),
            "alias service type versioned"
        );
        assert_eq!(
            "http://foo.bar/v2/",
            cat.get_service_endpoint(
                "block-storage",
                Some(&ApiVersion::new(2, 0)),
                Some("default")
            )
            .unwrap()
            .url_str(),
            "main service type versioned"
        );
        assert_eq!(
            "http://foo.bar/v3/",
            cat.get_service_endpoint(
                "block-storage",
                Some(&ApiVersion::new(3, 0)),
                Some("default")
            )
            .unwrap()
            .url_str(),
            "main service type versioned"
        );
        assert_eq!(
            "http://foo.bar/",
            cat.get_service_endpoint(
                "block-storage",
                Some(&ApiVersion::new(0, 0)),
                Some("default")
            )
            .unwrap()
            .url_str(),
            "main service type unversioned"
        );
        assert_eq!(
            "http://another.foo.bar/v3/z_PROJECT_ID",
            cat.get_service_endpoint("volumev3", None, Some("default"))
                .unwrap()
                .url_str(),
            "endpoint as in endpoint_override"
        );
        // TODO:
        assert_eq!(
            "http://foo.bar/v3/",
            cat.get_service_endpoint("volumev2", None, Some("default"))
                .unwrap()
                .url_str(),
            "This must start failing"
        );

        let ep = cat
            .get_service_endpoint("volumev2", Some(&ApiVersion::new(2, 0)), Some("default"))
            .unwrap();
        assert_eq!(
            "http://foo.bar/v2/",
            ep.url_str(),
            "endpoint as in catalog but with discovery url"
        );
        assert_eq!(
            &Some("y_PROJECT_ID".into()),
            ep.last_segment_with_project_id(),
            "discovered service url with discovery url and project_id as in catalog"
        );
        let ep = cat
            .get_service_endpoint(
                "block-storage",
                Some(&ApiVersion::new(3, 0)),
                Some("default"),
            )
            .unwrap();
        assert_eq!(
            "http://foo.bar/v3/",
            ep.url_str(),
            "endpoint as in catalog but with discovery url"
        );
        assert_eq!(
            &Some("z_PROJECT_ID".into()),
            ep.last_segment_with_project_id(),
            "discovered service url with discovery url and project_id as in catalog"
        );
        assert_eq!(
            &Some("3.15".into()),
            ep.max_version(),
            "discovered service from catalog contain microversion info"
        );
    }

    #[test]
    fn test_get_service_endpoint_no_discovery() {
        let mut cat = Catalog::default();
        cat.set_project_id(Some("PROJECT_ID"));
        cat.register_catalog_endpoint(
            "volume",
            "http://foo.bar/v1/PROJECT_ID",
            Some("default"),
            Some("public"),
        )
        .unwrap();
        cat.register_catalog_endpoint(
            "volumev2",
            "http://foo.bar/v2/PROJECT_ID",
            Some("default"),
            Some("public"),
        )
        .unwrap();
        cat.register_catalog_endpoint(
            "volumev3",
            "http://foo.bar/v3/PROJECT_ID",
            Some("default"),
            Some("public"),
        )
        .unwrap();

        assert_eq!(
            "http://foo.bar/v1/PROJECT_ID",
            cat.get_service_endpoint("volume", Some(&ApiVersion::new(1, 0)), Some("default"))
                .unwrap()
                .url_str(),
        );
        assert_eq!(
            "http://foo.bar/v3/PROJECT_ID",
            cat.get_service_endpoint("block-storage", None, Some("default"))
                .unwrap()
                .url_str(),
        );
        assert_eq!(
            "http://foo.bar/v2/PROJECT_ID",
            cat.get_service_endpoint(
                "block-storage",
                Some(&ApiVersion::new(2, 0)),
                Some("default")
            )
            .unwrap()
            .url_str(),
        );
    }
}
