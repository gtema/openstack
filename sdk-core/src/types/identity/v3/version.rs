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
use serde::de::Deserializer;

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
            // Keystone "stable" is same as "current" everywhere else
            "current" | "stable" => Ok(EndpointVersionStatus::Current),
            "supported" => Ok(EndpointVersionStatus::Supported),
            "deprecated" => Ok(EndpointVersionStatus::Deprecated),
            "experimental" => Ok(EndpointVersionStatus::Experimental),
            _ => Ok(EndpointVersionStatus::Unknown),
        }
    }
}

/// Link structure
#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct Link {
    pub href: String,
    pub rel: String,
}

/// Single Endpoint Version structure
#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct EndpointVersionContainer {
    pub version: EndpointVersion,
}

/// Single Endpoint Version structure representation
#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct EndpointVersion {
    pub id: String,
    pub status: EndpointVersionStatus,
    #[serde(default, deserialize_with = "empty_string_as_none")]
    pub version: Option<String>,
    #[serde(default, deserialize_with = "empty_string_as_none")]
    pub min_version: Option<String>,
    #[serde(default, deserialize_with = "empty_string_as_none")]
    pub max_version: Option<String>,
    pub links: Vec<Link>,
}

/// `Versions` array of Endpoint Versions
#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct EndpointVersions {
    pub versions: Vec<EndpointVersion>,
}

/// `Versions.values` structure
#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct EndpointVersionsValues {
    pub versions: EndpointVersionValues,
}

/// Endpoint version values structure
#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct EndpointVersionValues {
    pub values: Vec<EndpointVersion>,
}

/// Deserialize optional string as well as empty string as None
fn empty_string_as_none<'de, D>(de: D) -> Result<Option<String>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let o: Option<String> = Option::deserialize(de)?;
    Ok(o.filter(|s| !s.is_empty()))
}
