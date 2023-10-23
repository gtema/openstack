use bytes::Bytes;
use serde::de::{Deserializer, IntoDeserializer};
use serde::Deserialize;
use serde_json::Value;
use std::collections::HashMap;
use std::str::FromStr;

use anyhow::Context;
use thiserror::Error;

use url::Url;

use tracing::{debug, error, info, trace, Level};

use crate::types::identity::v3::ServiceEndpoints;
use crate::types::{ServiceType, SupportedServiceTypes};

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
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

/// ServiceEndpoint data
#[derive(Debug, Clone)]
pub(crate) struct ServiceEndpoint {
    pub url: Url,
    pub discovered: bool,
    pub versions: Vec<EndpointVersion>,
}

impl ServiceEndpoint {
    /// Process Endpoint version discovery response
    pub fn process_discovery(&mut self, data: &Bytes) -> Result<(), CatalogError> {
        // Unversioned endpoint normally returns: `{versions: []}`
        if let Ok(versions) = serde_json::from_slice::<EndpointVersions>(data) {
            for ver in versions.versions {
                self.process_version_information(&ver)?;
            }
        } else if let Ok(version) = serde_json::from_slice::<EndpointVersion>(data) {
            // Versioned endpoint normally returns: `{version: {}}`
            self.process_version_information(&version)?;
        } else if let Ok(vers) = serde_json::from_slice::<EndpointVersionsValues>(data) {
            // Keystone returns `{versions: {values: []}}}`
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
        if ver.status == EndpointVersionStatus::Current {
            for link in &ver.links {
                if link.rel == "self" {
                    self.url = Url::parse(&link.href)
                        .with_context(|| format!("Wrong endpoint URL: `{}`", link.href))?;
                }
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "lowercase")]
#[serde(remote = "EndpointVersionStatus")]
pub(crate) enum EndpointVersionStatus {
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
pub(crate) struct Link {
    pub href: String,
    pub rel: String,
}

/// Single Endpoint Version structure representation
#[derive(Debug, Deserialize, Clone)]
pub(crate) struct EndpointVersion {
    pub id: String,
    pub status: EndpointVersionStatus,
    pub version: Option<String>,
    pub min_version: Option<String>,
    pub links: Vec<Link>, //    pub url: Url,
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
///
/// Endpoint version values structure
#[derive(Debug, Deserialize, Clone)]
pub(crate) struct EndpointVersionValues {
    pub values: Vec<EndpointVersion>,
}

/// Convert json value to `Option<String>`
fn json_val_to_string(val: &Value) -> Option<String> {
    val.as_str().map(|x| x.to_string())
}

/// Structure representing the ServiceCatalog
#[derive(Debug, Clone, Default)]
pub(crate) struct Catalog {
    /// HashMap containing service endpoints by the service type
    pub(crate) service_endpoints: HashMap<String, ServiceEndpoint>,
    pub(crate) token_catalog: Option<Vec<ServiceEndpoints>>,
}

impl Catalog {
    /// Register single service endpoint
    pub(crate) fn add_service_endpoint(
        &mut self,
        service_type: &str,
        url: &str,
    ) -> Result<(), CatalogError> {
        let mut fixed_url: String = url.into();
        if !fixed_url.ends_with('/') {
            fixed_url.push('/');
        }

        self.service_endpoints.insert(
            service_type.to_string(),
            ServiceEndpoint {
                url: Url::parse(&fixed_url)
                    .with_context(|| format!("Wrong endpoint URL: `{}`", fixed_url))?,
                discovered: false,
                versions: Vec::new(),
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
    pub(crate) fn get_service_endpoint(&self, service_type: &ServiceType) -> Option<Url> {
        trace!("Requested service {} endpoint", service_type);
        for cat_type in service_type.get_supported_catalog_types() {
            if let Some(sep) = self.service_endpoints.get(&cat_type.to_string()) {
                debug!("Service endpoint url = {}", sep.url);
                return Some(sep.url.clone());
            }
        }
        None
    }

    pub fn get_token_catalog(&self) -> Option<Vec<ServiceEndpoints>> {
        self.token_catalog.clone()
    }
}
