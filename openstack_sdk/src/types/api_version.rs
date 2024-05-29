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

//! Service Api Version
//!
//! Endpoint api version handling

use anyhow::anyhow;
use lazy_static::lazy_static;
use regex::Regex;
use std::fmt;
use thiserror::Error;
use url::Url;

lazy_static! {
    static ref API_VERSION_PREFIXED_REGEX: Regex =
        Regex::new(r"^v(?<major>[0-9]+)(?:\.)?(?<minor>[0-9]+)?$").unwrap();
    static ref API_VERSION_REGEX: Regex =
        Regex::new(r"^(?<major>[0-9]+)(?:\.)?(?<minor>[0-9]+)?$").unwrap();
    static ref ID_LIKE_REGEX: Regex = Regex::new(r"[0-9a-z]{32}$").unwrap();
}

/// ApiVersion error
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum ApiVersionError {
    #[error("failed to parse url: {}", source)]
    UrlParse {
        #[from]
        source: url::ParseError,
    },
    #[error("Not a valid integer: {}", source)]
    ParseInt {
        #[from]
        source: std::num::ParseIntError,
    },
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

/// ApiVersion
///
/// ApiVersion of the Endpoint as described in
/// <https://specs.openstack.org/openstack/api-sig/guidelines/consuming-catalog/version-discovery.html>. It is a subset of a SemVer and only includes `major` and `minor` parts.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ApiVersion {
    /// Major version
    pub major: u8,
    /// Minor version
    pub minor: u8,
}

impl ApiVersion {
    /// Basic constructor
    pub fn new(major: u8, minor: u8) -> Self {
        Self { major, minor }
    }
    /// Determine Api Version based on the Endpoint URL and optional project_id
    pub fn from_url<S: AsRef<str>>(
        url: &Url,
        project_id: Option<S>,
    ) -> Result<Self, ApiVersionError> {
        // Find the version as described under
        // https://specs.openstack.org/openstack/api-sig/guidelines/consuming-catalog/version-discovery.html#inferring-version
        let mut path_segments = url.path_segments().map_or(Vec::new(), |seg| {
            seg.into_iter().filter(|x| !x.is_empty()).collect()
        });
        // Strip last element if it ends with project_id
        if let Some(last) = path_segments.last() {
            match project_id {
                Some(pid) => {
                    if last.ends_with(pid.as_ref()) {
                        path_segments.pop();
                    }
                }
                None => {
                    // Project_id is not known, but path_element contains something that look like
                    // ID. This is safe since this anyway doesn't look like version information.
                    if ID_LIKE_REGEX.is_match(last) {
                        // NOTE(gtema): I don't think it is worth of logging a warning.
                        path_segments.pop();
                    }
                }
            }
        }
        if let Some(last) = path_segments.last() {
            return Self::from_apiver_str(last, true).or(Ok(Self::default()));
        }
        Ok(Self::default())
    }

    /// Determine the api version from a single string (i.e. `v2.3` or `3.4`)
    ///
    /// `prefixed` parameter influences string analysis enforcing presence or absence of the `v`
    /// prefix
    pub fn from_apiver_str<S: AsRef<str>>(
        data: S,
        prefixed: bool,
    ) -> Result<Self, ApiVersionError> {
        let captures = match prefixed {
            true => API_VERSION_PREFIXED_REGEX.captures(data.as_ref()),
            false => API_VERSION_REGEX.captures(data.as_ref()),
        };
        if let Some(cap) = captures {
            let mut res = Self { major: 0, minor: 0 };
            res.major = cap["major"].parse()?;
            if let Some(vmin) = cap.name("minor") {
                res.minor = vmin.as_str().parse()?;
            }
            return Ok(res);
        }
        Err(anyhow!(
            "String {} does not look like a supported version.",
            data.as_ref()
        )
        .into())
    }

    /// Determine the api version from the [RestEndpoint](crate::api::RestEndpoint)
    ///
    /// First element of the url may be containing version information (i.e. `v2.1/servers`).
    pub fn from_endpoint_url<S: AsRef<str>>(url: S) -> Option<Self> {
        url.as_ref()
            .split_once('/')
            .and_then(|(pref, _)| Self::from_apiver_str(pref, true).ok())
    }
}

impl fmt::Display for ApiVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{}", self.major, self.minor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use url::Url;

    #[test]
    fn test_from_url() {
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
            (
                "https://test.com/prefix/v1/AUTH_eed9239eaff6447a95da625e945f1978",
                Some("eed9239eaff6447a95da625e945f1978"),
                1,
                0,
            ),
            (
                "https://test.com/prefix/v1/AUTH_eed9239eaff6447a95da625e945f1978/",
                Some("eed9239eaff6447a95da625e945f1978"),
                1,
                0,
            ),
        ];
        for (url, pid, maj, min) in matrix.iter() {
            let ver = ApiVersion::from_url(&Url::parse(url).unwrap(), pid.as_ref()).unwrap();
            assert_eq!(
                (*maj, *min),
                (ver.major, ver.minor),
                "Major version of {} must be {}.{}",
                url,
                maj,
                min
            );
        }
    }

    #[test]
    fn test_from_apiver_str_prefixed() {
        assert_eq!(
            ApiVersion { major: 1, minor: 0 },
            ApiVersion::from_apiver_str("v1", true).unwrap()
        );
        assert_eq!(
            ApiVersion { major: 2, minor: 3 },
            ApiVersion::from_apiver_str("v2.3", true).unwrap()
        );
        assert!(ApiVersion::from_apiver_str("2.3", true).is_err());
        assert!(ApiVersion::from_apiver_str("vfoo.bar", true).is_err());
    }

    #[test]
    fn test_from_apiver_str_unprefixed() {
        assert_eq!(
            ApiVersion { major: 1, minor: 0 },
            ApiVersion::from_apiver_str("1", false).unwrap()
        );
        assert_eq!(
            ApiVersion { major: 2, minor: 3 },
            ApiVersion::from_apiver_str("2.3", false).unwrap()
        );
        assert!(ApiVersion::from_apiver_str("v2.3", false).is_err());
        assert!(ApiVersion::from_apiver_str("foo.bar", false).is_err());
    }

    #[test]
    fn test_from_endpoint_url() {
        assert_eq!(
            Some(ApiVersion { major: 1, minor: 0 }),
            ApiVersion::from_endpoint_url("v1/foo")
        );
        assert_eq!(
            Some(ApiVersion { major: 2, minor: 1 }),
            ApiVersion::from_endpoint_url("v2.1/foo")
        );
        assert_eq!(None, ApiVersion::from_endpoint_url("v2.1-foo/foo"));
        assert_eq!(None, ApiVersion::from_endpoint_url("vfoo/bar"));
    }
}
