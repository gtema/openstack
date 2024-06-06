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

//! Service endpoint module
//!
//! Represent the Service ServiceEndpoint.

use anyhow::{anyhow, Context};
use std::fmt;
use url::Url;

use crate::catalog::CatalogError;
use crate::types::identity::v3::version::EndpointVersionStatus;
use crate::types::ApiVersion;

/// Service endpoint
///
#[derive(Clone, PartialEq)]
pub struct ServiceEndpoint {
    /// ServiceEndpoint URL
    url: Url,
    /// Service endpoint region
    region: Option<String>,
    /// Service endpoint interface
    interface: Option<String>,
    /// API version determined from the url
    version: ApiVersion,
    /// lowest supported microversion
    min_version: Option<String>,
    /// Highest supported microversion
    max_version: Option<String>,
    /// Service type as used in the ServiceCatalog
    service_type: Option<String>,
    // Last url segment if it ends with project_id
    last_segment_with_project_id: Option<String>,
    /// ServiceEndpoint status
    status: Option<EndpointVersionStatus>,
}

impl ServiceEndpoint {
    /// Build new ServiceEndpoint from URL and `[ApiVersion]`
    pub fn new(url: Url, api_version: ApiVersion) -> Self {
        Self {
            url,
            version: api_version,
            region: None,
            interface: None,
            min_version: None,
            max_version: None,
            service_type: None,
            last_segment_with_project_id: None,
            status: None,
        }
    }
    /// Build new ServiceEndpoint from string URL
    ///
    /// optional project_id is used to locate version information in the url
    pub fn from_url_string<S1: AsRef<str>, S2: AsRef<str>>(
        url: S1,
        project_id: Option<S2>,
    ) -> Result<Self, CatalogError> {
        let url = Url::parse(url.as_ref())
            .with_context(|| format!("Wrong endpoint URL: `{}`", url.as_ref()))?;
        assert!(!url.cannot_be_a_base());
        if url.cannot_be_a_base() {
            return Err(anyhow!("URL `{}` cannot be a base url", url.as_ref()).into());
        }
        if !["http", "https"].contains(&url.scheme()) {
            return Err(anyhow!("URL `{}` must be http/https", url.as_ref()).into());
        }
        let version = ApiVersion::from_url(&url, project_id.as_ref())?;

        let mut last_segment_with_project_id: Option<String> = None;
        if let Some(pid) = &project_id {
            let path_segments = url.path_segments().map_or(Vec::new(), |seg| {
                seg.into_iter().filter(|x| !x.is_empty()).collect()
            });
            // Save last element if it ends with project_id
            if let Some(last) = path_segments.last() {
                if last.ends_with(pid.as_ref()) {
                    last_segment_with_project_id = Some(last.to_string().clone());
                }
            }
        }

        let res = Self {
            url,
            version,
            region: None,
            interface: None,
            min_version: None,
            max_version: None,
            service_type: None,
            last_segment_with_project_id,
            status: None,
        };
        Ok(res)
    }

    /// Set service type
    pub fn set_service_type<S: AsRef<str>>(&mut self, service_type: Option<S>) -> &mut Self {
        self.service_type = service_type.map(|x| x.as_ref().to_string());
        self
    }

    /// Set endpoint region
    pub fn set_region<S: AsRef<str>>(&mut self, region: Option<S>) -> &mut Self {
        self.region = region.map(|x| x.as_ref().to_string());
        self
    }

    /// Set endpoint interface
    pub fn set_interface<S: AsRef<str>>(&mut self, interface: Option<S>) -> &mut Self {
        self.interface = interface.map(|x| x.as_ref().to_string());
        self
    }

    /// Set min_version
    pub fn set_min_version<S: AsRef<str>>(&mut self, version: Option<S>) -> &mut Self {
        self.min_version = version.map(|x| x.as_ref().to_string());
        self
    }
    /// Set min_version
    pub fn set_max_version<S: AsRef<str>>(&mut self, version: Option<S>) -> &mut Self {
        self.max_version = version.map(|x| x.as_ref().to_string());
        self
    }
    /// Set status
    pub fn set_status(&mut self, status: Option<EndpointVersionStatus>) -> &mut Self {
        self.status = status;
        self
    }

    /// Returns a reference to Url
    pub fn url(&self) -> &Url {
        &self.url
    }

    /// Returns a reference to Url as &str
    pub fn url_str(&self) -> &str {
        self.url.as_str()
    }

    /// Returns a reference to ApiVersion
    pub fn version(&self) -> &ApiVersion {
        &self.version
    }

    /// Returns a reference to region
    pub fn region(&self) -> &Option<String> {
        &self.region
    }

    pub fn last_segment_with_project_id(&self) -> &Option<String> {
        &self.last_segment_with_project_id
    }

    pub fn set_last_segment_with_project_id(&mut self, val: Option<String>) -> &mut Self {
        self.last_segment_with_project_id = val;
        self
    }

    /// Retrieve ServiceServiceEndpoint status
    pub fn status(&self) -> &Option<EndpointVersionStatus> {
        &self.status
    }

    /// Retrieve ServiceServiceEndpoint min_version
    pub fn min_version(&self) -> &Option<String> {
        &self.min_version
    }

    /// Retrieve ServiceServiceEndpoint max_version
    pub fn max_version(&self) -> &Option<String> {
        &self.max_version
    }

    pub fn build_request_url(&self, endpoint: &str) -> Result<Url, CatalogError> {
        let mut base_url = self.url().clone();
        if let Some(pid_suffix) = self.last_segment_with_project_id() {
            if !(base_url.path().trim_end_matches('/').ends_with(pid_suffix)) {
                base_url
                    .path_segments_mut()
                    .map_err(|_| anyhow!("cannot be base"))?
                    // Strip trailing slash
                    .pop_if_empty()
                    // Append segment with project_id
                    .push(pid_suffix)
                    // Append trailing slash
                    .push("");
            }
        }
        if !base_url.path().ends_with('/') {
            // Ensure base url ends with "/"
            base_url
                .path_segments_mut()
                .map_err(|_| anyhow!("cannot be base"))?
                // Append trailing slash
                .push("");
        }
        //let work_service_url = url;
        let mut work_endpoint = endpoint;
        if let Some(segments) = base_url.path_segments() {
            // Service catalog may point to /v2.1/ and target endpoint start
            // with v2.1/servers. The same may happen also for project_id being
            // used in the service catalog while rest endpoint also contain it.
            // In order to construct proper url look in the path elements of
            // the service catalog and for each entry ensure target url does
            // not start with that value.
            let mut overlap: bool = false;
            for part in segments.filter(|x| !x.is_empty()) {
                if work_endpoint.starts_with(part) {
                    work_endpoint = work_endpoint
                        .get(part.len() + 1..)
                        .expect("Cannot remove prefix from url");
                    overlap = true;
                } else if overlap {
                    break;
                }
            }
        }
        Ok(base_url.join(work_endpoint)?)
    }
}

impl fmt::Debug for ServiceEndpoint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ServiceEndpoint")
            .field("url", &self.url.as_str())
            .field("region", &self.region)
            .field("interface", &self.interface)
            .field("version", &self.version)
            .field("min_version", &self.min_version)
            .field("max_version", &self.max_version)
            .field("pid_segment", &self.last_segment_with_project_id)
            .finish()
    }
}

#[derive(Clone, Debug, Default)]
pub struct ServiceEndpoints(Vec<ServiceEndpoint>);

impl ServiceEndpoints {
    /// Add endpoint into the list
    pub fn push(&mut self, ep: ServiceEndpoint) -> &mut Self {
        self.0.push(ep);
        self
    }

    /// Get the endpoint by region name. If no reion_name is passed first endpoint is being
    /// returned.
    pub fn get_by_region<S: AsRef<str>>(&self, region_name: Option<S>) -> Option<&ServiceEndpoint> {
        if let Some(region) = &region_name {
            return self
                .0
                .iter()
                .find(|&ep| ep.region == Some(region.as_ref().to_string()));
        } else {
            return self.0.first();
        }
    }

    /// Get the endpoint by region name. If no reion_name is passed first endpoint is being
    /// returned.
    pub fn get_by_version_and_region<S: AsRef<str>>(
        &self,
        api_version: Option<&ApiVersion>,
        region_name: Option<S>,
    ) -> Option<&ServiceEndpoint> {
        for candidate in self.0.iter() {
            if let Some(requested_version) = &api_version {
                let cver = candidate.version();
                if !(cver.major == requested_version.major
                    && (cver.minor >= requested_version.minor))
                {
                    continue;
                }
            } else if *candidate.status() != Some(EndpointVersionStatus::Current) {
                continue;
            }
            match (&region_name, candidate.region()) {
                (Some(requested_region), Some(candidate_region)) => {
                    if candidate_region.as_str() == requested_region.as_ref() {
                        return Some(candidate);
                    }
                }
                (None, _) => {
                    return Some(candidate);
                }
                _ => {}
            };
        }
        // Maybe there is no entry with `status=Current` (i.e. before we do version discovery)
        if api_version.is_none() {
            return self.get_by_region(region_name);
        }
        None
    }

    #[cfg(test)]
    pub fn get_all(&self) -> &Vec<ServiceEndpoint> {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use url::Url;

    #[test]
    fn test_endpoint() {
        let matrix = [
            ("http://test.com/foo", None, 0, 0),
            ("https://test.com/foo", None, 0, 0),
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
            (
                "https://test.com/prefix/v1/AUTH_eed9239eaff6447a95da625e945f1978",
                None,
                1,
                0,
            ),
        ];
        for (url, pid, _maj, _min) in matrix.iter() {
            let sep = ServiceEndpoint::from_url_string(url, pid.as_ref()).unwrap();
            assert_eq!(sep.url, Url::parse(url).unwrap());
        }
        let val = ServiceEndpoint::from_url_string("unix://foo.bar", None::<String>);
        assert!(val.is_err());
    }

    #[test]
    fn test_endpoints_by_region() {
        let e1 = ServiceEndpoint::from_url_string("http://r1.foo.bar/s1", None::<String>)
            .unwrap()
            .set_region(Some("r1"))
            .to_owned();
        let e2 = ServiceEndpoint::from_url_string("http://r2.foo.bar/s1", None::<String>)
            .unwrap()
            .set_region(Some("r2"))
            .to_owned();
        let endpoints = ServiceEndpoints(Vec::from([e1, e2]));
        assert_eq!(
            "http://r1.foo.bar/s1",
            endpoints.get_by_region(Some("r1")).unwrap().url_str()
        );
        assert_eq!(
            "http://r1.foo.bar/s1",
            endpoints.get_by_region(None::<String>).unwrap().url_str()
        );
        assert_eq!(
            "http://r2.foo.bar/s1",
            endpoints.get_by_region(Some("r2")).unwrap().url_str()
        );
        assert!(endpoints.get_by_region(Some("r3")).is_none());
    }

    #[test]
    fn test_endpoints_by_version_and_region() {
        let endpoints = ServiceEndpoints(Vec::from([
            ServiceEndpoint::new(
                Url::parse("http://r1.foo.bar/s1/").unwrap(),
                ApiVersion::new(0, 0),
            )
            .set_region(Some("r1"))
            .to_owned(),
            ServiceEndpoint::new(
                Url::parse("http://r1.foo.bar/s1/v1/").unwrap(),
                ApiVersion::new(1, 0),
            )
            .set_region(Some("r1"))
            .to_owned(),
            ServiceEndpoint::new(
                Url::parse("http://r1.foo.bar/s1/v2/").unwrap(),
                ApiVersion::new(2, 0),
            )
            .set_region(Some("r1"))
            .set_status(Some(EndpointVersionStatus::Current))
            .to_owned(),
            ServiceEndpoint::new(
                Url::parse("http://r1.foo.bar/s1/v3/").unwrap(),
                ApiVersion::new(3, 0),
            )
            .set_region(Some("r1"))
            .set_status(Some(EndpointVersionStatus::Experimental))
            .to_owned(),
            ServiceEndpoint::new(
                Url::parse("http://r2.foo.bar/s1/").unwrap(),
                ApiVersion::new(0, 0),
            )
            .set_region(Some("r2"))
            .to_owned(),
            ServiceEndpoint::new(
                Url::parse("http://r2.foo.bar/s1/v1/").unwrap(),
                ApiVersion::new(1, 0),
            )
            .set_region(Some("r2"))
            .to_owned(),
            ServiceEndpoint::new(
                Url::parse("http://r2.foo.bar/s1/v2/").unwrap(),
                ApiVersion::new(2, 90),
            )
            .set_region(Some("r2"))
            .set_status(Some(EndpointVersionStatus::Current))
            .to_owned(),
        ]));
        assert_eq!(
            "http://r1.foo.bar/s1/",
            endpoints
                .get_by_version_and_region(Some(&ApiVersion::new(0, 0)), Some("r1"))
                .unwrap()
                .url_str(),
            "Requesting unversioned endpoint"
        );
        assert_eq!(
            "http://r1.foo.bar/s1/v1/",
            endpoints
                .get_by_version_and_region(Some(&ApiVersion::new(1, 0)), Some("r1"))
                .unwrap()
                .url_str(),
            "Requesting versioned endpoint"
        );
        assert_eq!(
            "http://r1.foo.bar/s1/v2/",
            endpoints
                .get_by_version_and_region(None, Some("r1"))
                .unwrap()
                .url_str(),
            "Requesting current endpoint"
        );
        assert_eq!(
            "http://r2.foo.bar/s1/v2/",
            endpoints
                .get_by_version_and_region(None, Some("r2"))
                .unwrap()
                .url_str(),
            "Requesting current endpoint"
        );
        assert_eq!(
            "http://r2.foo.bar/s1/v2/",
            endpoints
                .get_by_version_and_region(Some(&ApiVersion::new(2, 0)), Some("r2"))
                .unwrap()
                .url_str(),
            "Requesting versioned endpoint with min 'lower' then available"
        );
        assert!(
            endpoints
                .get_by_version_and_region(Some(&ApiVersion::new(2, 100)), Some("r2"))
                .is_none(),
            "Requesting versioned endpoint with min 'higher' then available"
        );
    }

    #[test]
    fn test_construct_request() {
        let map = [
            ("http://foo.bar/", 0, 0, None, "", "http://foo.bar/"),
            ("http://foo.bar/", 0, 0, None, "info", "http://foo.bar/info"),
            ("http://foo.bar/", 0, 0, None, "v1", "http://foo.bar/v1"),
            (
                "http://foo.bar/",
                0,
                0,
                None,
                "v1/resource",
                "http://foo.bar/v1/resource",
            ),
            (
                "http://foo.bar/v1/",
                1,
                0,
                None,
                "v1/resource",
                "http://foo.bar/v1/resource",
            ),
            (
                "http://foo.bar/v1/PROJECT_ID/",
                1,
                0,
                None,
                "resources",
                "http://foo.bar/v1/PROJECT_ID/resources",
            ),
            (
                "http://foo.bar/v1/",
                1,
                0,
                Some("PROJECT_ID".to_string()),
                "resources",
                "http://foo.bar/v1/PROJECT_ID/resources",
            ),
            (
                "http://foo.bar/v1/PROJECT_ID/",
                1,
                0,
                None,
                "v1/resource",
                "http://foo.bar/v1/PROJECT_ID/resource",
            ),
            (
                "http://foo.bar/v1/PROJECT_ID/",
                1,
                0,
                Some("PROJECT_ID".to_string()),
                "v1/resource",
                "http://foo.bar/v1/PROJECT_ID/resource",
            ),
            (
                "http://foo.bar/v1/PROJECT_ID",
                1,
                0,
                Some("PROJECT_ID".to_string()),
                "v1/resource",
                "http://foo.bar/v1/PROJECT_ID/resource",
            ),
            (
                "http://foo.bar/v1/PROJECT_ID/",
                1,
                0,
                None,
                "v1/PROJECT_ID/resource",
                "http://foo.bar/v1/PROJECT_ID/resource",
            ),
            (
                "http://foo.bar/prefix/",
                0,
                0,
                None,
                "info",
                "http://foo.bar/prefix/info",
            ),
            (
                "http://foo.bar/prefix/v1/",
                1,
                0,
                None,
                "v1/info",
                "http://foo.bar/prefix/v1/info",
            ),
        ];
        for (service_url, major, minor, pid, endpoint, expected) in map {
            assert_eq!(
                Url::parse(expected).unwrap().as_str(),
                ServiceEndpoint::new(
                    Url::parse(service_url).unwrap(),
                    ApiVersion::new(major, minor)
                )
                .set_last_segment_with_project_id(pid)
                .build_request_url(endpoint)
                .unwrap()
                .as_str(),
                "ServiceEndpoint: {} with URL: {} results in {}",
                service_url,
                endpoint,
                expected
            );
        }
    }
}
