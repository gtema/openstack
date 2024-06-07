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

//! Service Version discovery
//!
//! <https://specs.openstack.org/openstack/api-sig/guidelines/consuming-catalog/version-discovery.html>

use bytes::Bytes;
use url::Url;

use crate::catalog::{error::CatalogError, service_endpoint::ServiceEndpoint};
use crate::types::api_version::{ApiVersion, ApiVersionError};
use crate::types::identity::v3::version::{
    EndpointVersion, EndpointVersionContainer, EndpointVersions, EndpointVersionsValues,
};

/// Process Endpoint version discovery response
pub fn extract_discovery_endpoints(
    discovery_url: &Url,
    data: &Bytes,
) -> Result<Vec<ServiceEndpoint>, CatalogError> {
    let mut endpoints: Vec<ServiceEndpoint> = Vec::new();
    // Explicitly add `unversioned` endpoint
    endpoints.push(ServiceEndpoint::new(
        discovery_url.clone(),
        ApiVersion::new(0, 0),
    ));

    if let Ok(versions) = serde_json::from_slice::<EndpointVersions>(data) {
        // Unversioned endpoint normally returns: `{versions: []}`
        for ver in versions.versions {
            endpoints.push(ver.as_endpoint(discovery_url)?);
        }
        return Ok(endpoints);
    } else if let Ok(ver) = serde_json::from_slice::<EndpointVersionContainer>(data) {
        // Versioned endpoint normally returns: `{version: {}}`
        endpoints.push(ver.version.as_endpoint(discovery_url)?);
        return Ok(endpoints);
    } else if let Ok(vers) = serde_json::from_slice::<EndpointVersionsValues>(data) {
        // Keystone returns `{versions: {values: []}}}`
        for ver in vers.versions.values {
            endpoints.push(ver.as_endpoint(discovery_url)?);
        }
        return Ok(endpoints);
    }
    Err(CatalogError::InvalidDiscoveryDocument)
}

/// Expand endpoint link
///
/// Convert link href to the full url according to
/// <https://specs.openstack.org/openstack/api-sig/guidelines/consuming-catalog/version-discovery.html#expanding-endpoints>.
pub fn expand_link<S: AsRef<str>>(link: S, base_url: &Url) -> Result<Url, CatalogError> {
    // Parse link url
    let mut url = match Url::parse(link.as_ref()) {
        Ok(url) => url,
        Err(url::ParseError::RelativeUrlWithoutBase) => base_url
            .clone()
            .join(link.as_ref())
            .map_err(|x| CatalogError::url_parse(x, format!("{}/{}", base_url, link.as_ref())))?,
        Err(err) => {
            return Err(CatalogError::url_parse(err, link.as_ref()));
        }
    };
    if url.cannot_be_a_base() {
        return Err(CatalogError::cannot_be_base(&url));
    }
    url.set_scheme(base_url.scheme())
        .map_err(|_| CatalogError::UrlScheme(base_url.as_ref().to_string()))?;
    url.set_host(base_url.host_str())
        .map_err(|x| CatalogError::url_parse(x, url.as_ref()))?;
    if !url.as_str().ends_with('/') {
        url.path_segments_mut()
            // The error here should not happen since we checked above for cannot_be_base
            .map_err(|_| CatalogError::cannot_be_base(base_url))?
            .push("");
    }
    Ok(url.to_owned())
}

impl EndpointVersion {
    /// Get ApiVersion from `version` or `min_version` or `id`
    pub fn get_api_version(&self) -> Result<ApiVersion, ApiVersionError> {
        if let Some(ver) = &self.version {
            return ApiVersion::from_apiver_str(ver, false);
        }
        if let Some(ver) = &self.min_version {
            return ApiVersion::from_apiver_str(ver, false);
        }
        ApiVersion::from_apiver_str(&self.id, true)
    }

    /// Return `Endpoint` representation of the Version
    pub fn as_endpoint(&self, base_url: &Url) -> Result<ServiceEndpoint, CatalogError> {
        if let Some(link) = self.links.iter().find(|&x| x.rel == "self") {
            return Ok(ServiceEndpoint::new(
                expand_link(&link.href, base_url)?.clone(),
                self.get_api_version()?,
            )
            .set_min_version(self.min_version.clone())
            .set_max_version(self.max_version.clone())
            .set_status(Some(self.status.clone()))
            .to_owned());
        } else {
            Err(CatalogError::VersionSelfLinkMissing {
                id: self.id.clone(),
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::identity::v3::version::{EndpointVersionStatus, Link};
    use crate::types::ApiVersion;
    use serde_json::json;
    use url::Url;

    #[test]
    fn test_endpoint_version() {
        let ev: EndpointVersion = serde_json::from_value(json!({
              "status": "CURRENT",
              "links": [
              ],
              "min_version": "",
              "max_version": "2.38",
              "id": "v2.1"
        }))
        .unwrap();
        assert_eq!("v2.1".to_string(), ev.id);
        assert_eq!(EndpointVersionStatus::Current, ev.status);
        assert_eq!(None, ev.version);
        assert_eq!(None, ev.min_version);
        assert_eq!(Some("2.38".to_string()), ev.max_version);
        assert_eq!(ApiVersion::new(2, 1), ev.get_api_version().unwrap());

        let ev: EndpointVersion = serde_json::from_value(json!({
              "status": "CURRENT",
              "links": [
                {
                  "href": "http://compute.example.com/v2.1/",
                  "rel": "self"
                }
              ],
              "min_version": "2.1",
              "version": "2.38",
              "id": "v2.1"
        }))
        .unwrap();
        assert_eq!("v2.1".to_string(), ev.id);
        assert_eq!(EndpointVersionStatus::Current, ev.status);
        assert_eq!(Some("2.38".to_string()), ev.version);
        assert_eq!(Some("2.1".to_string()), ev.min_version);
        assert_eq!(None, ev.max_version);
        assert_eq!(
            Vec::from([Link {
                href: "http://compute.example.com/v2.1/".to_string(),
                rel: "self".to_string(),
            },]),
            ev.links
        );
        assert_eq!(
            ApiVersion {
                major: 2,
                minor: 38
            },
            ev.get_api_version().unwrap()
        )
    }

    #[test]
    fn test_endpoint_version_normalize() {
        let version = EndpointVersion {
            id: "v2.0".to_string(),
            status: EndpointVersionStatus::Supported,
            version: Some("2.0".to_string()),
            min_version: None,
            max_version: None,
            links: Vec::from([Link {
                href: "http:///localhost/v2/".to_string(),
                rel: "self".to_string(),
            }]),
        };
        let base_url = Url::parse("https://compute.example.com/").unwrap();
        let self_link = version.links.iter().find(|&x| x.rel == "self").unwrap();
        assert_eq!(
            "https://compute.example.com/v2/",
            expand_link(&self_link.href, &base_url).unwrap().as_str(),
            "Scheme and host are from base_url, ends with `/`"
        );
    }

    #[test]
    fn test_endpoint_version_as_endpoint() {
        let version = EndpointVersion {
            id: "v2.0".to_string(),
            status: EndpointVersionStatus::Supported,
            version: Some("2.0".to_string()),
            min_version: None,
            max_version: None,
            links: Vec::from([Link {
                href: "http://compute.example.com/v2/".to_string(),
                rel: "self".to_string(),
            }]),
        };
        let base_url = Url::parse("https://compute.example.com/").unwrap();
        let ep = version.as_endpoint(&base_url).unwrap();
        assert_eq!("https://compute.example.com/v2/", ep.url_str());
    }

    #[test]
    fn test_endpoint_version_as_endpoint_no_self_link() {
        let version = EndpointVersion {
            id: "v2.0".to_string(),
            status: EndpointVersionStatus::Supported,
            version: Some("2.0".to_string()),
            min_version: None,
            max_version: None,
            links: Vec::new(),
        };
        let base_url = Url::parse("https://compute.example.com/").unwrap();
        let err = version.as_endpoint(&base_url).unwrap_err();

        if let CatalogError::VersionSelfLinkMissing { id } = err {
            assert_eq!("v2.0", id);
        } else {
            panic!("Unexpected error: {}", err);
        }
    }

    #[test]
    fn test_endpoint_version_as_endpoint_bad_self_link() {
        let version = EndpointVersion {
            id: "v2.0".to_string(),
            status: EndpointVersionStatus::Supported,
            version: Some("2.0".to_string()),
            min_version: None,
            max_version: None,
            links: Vec::from([Link {
                href: "http://".to_string(),
                rel: "self".to_string(),
            }]),
        };
        let base_url = Url::parse("https://compute.example.com/").unwrap();
        let err = version.as_endpoint(&base_url).unwrap_err();

        if let CatalogError::UrlParse { .. } = err {
        } else {
            panic!("Unexpected error: {}", err);
        }
    }

    #[test]
    fn test_discovery_single_version() {
        let endpoints = extract_discovery_endpoints(
            &Url::parse("http://compute.example.com/").unwrap(),
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
                    ]
                  }
                })
                .to_string(),
            ),
        )
        .unwrap();
        assert_eq!(2, endpoints.len());
        assert_eq!(&ApiVersion::new(0, 0), endpoints[0].version());
        assert_eq!(&ApiVersion::new(2, 0), endpoints[1].version());
    }

    #[test]
    fn test_discovery_multiple_version() {
        let endpoints = extract_discovery_endpoints(
            &Url::parse("http://compute.example.com/").unwrap(),
            &Bytes::from(
                json!({
                  "versions": [
                    {
                      "status": "SUPPORTED",
                      "links": [
                        {
                          "href": "http://compute.example.com/v2/",
                          "rel": "self"
                        }
                      ],
                      "min_version": "",
                      "max_version": "",
                      "id": "v2.0"
                    }, {
                      "status": "CURRENT",
                      "links": [
                        {
                          "href": "http://compute.example.com/v2.1/",
                          "rel": "self"
                        }
                      ],
                      "min_version": "2.1",
                      "version": "2.38",
                      "id": "v2.1"
                    }
                   ]
                })
                .to_string(),
            ),
        )
        .unwrap();
        assert_eq!(3, endpoints.len());
    }

    #[test]
    fn test_discovery_keystone() {
        let endpoints = extract_discovery_endpoints(
            &Url::parse("http://compute.example.com/").unwrap(),
            &Bytes::from(
                json!({
                  "versions": {
                    "values": [
                      {
                        "id": "v3.6",
                        "links": [
                          {
                            "href": "https://identity.example.com/v3/",
                            "rel": "self"
                          }
                        ],
                        "media-types": [
                          {
                            "base": "application/json",
                            "type": "application/vnd.openstack.identity-v3+json"
                          }
                        ],
                        "status": "stable",
                        "updated": "2016-04-04T00:00:00Z"
                      }
                    ]
                  }
                })
                .to_string(),
            ),
        )
        .unwrap();
        assert_eq!(2, endpoints.len());
    }
}
