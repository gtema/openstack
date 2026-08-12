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

//! Client for the HTTPS-hosted plugin registry index (`index.json`), and the
//! download step that turns a resolved [`IndexVersion`] into checksummed
//! bytes ready for [`crate::registry::plan_remote_install`].
//!
//! The default index lives in this repository at
//! `plugins/registry/index.json`, served over plain HTTPS via
//! `raw.githubusercontent.com`. Nothing about the schema or this client is
//! tied to that location: any URL matching the schema documented in
//! `plugins/registry/README.md` works, and `--registry-url` on the CLI lets
//! a different one be used explicitly. The default is never silently
//! overridden by an environment variable — only an explicit flag changes it,
//! so a plugin never ends up resolved against an unexpected index by
//! accident.

use std::time::Duration;

use serde::Deserialize;
use url::Url;

use crate::error::WasmPluginError;

/// The registry index consulted when no `--registry-url` is given.
pub const DEFAULT_REGISTRY_URL: &str =
    "https://raw.githubusercontent.com/gtema/openstack/main/plugins/registry/index.json";

/// The only `schema_version` this client understands.
const SUPPORTED_SCHEMA_VERSION: u32 = 1;

/// The full registry index.
#[derive(Clone, Debug, Deserialize)]
pub struct PluginIndex {
    /// Schema version of this index document.
    pub schema_version: u32,
    /// Published plugins.
    #[serde(default)]
    pub plugins: Vec<IndexEntry>,
}

/// A single plugin's registry entry: its name and every version published
/// for it.
#[derive(Clone, Debug, Deserialize)]
pub struct IndexEntry {
    /// Plugin name.
    pub name: String,
    /// Human readable one-line description.
    #[serde(default)]
    pub description: String,
    /// Published versions.
    pub versions: Vec<IndexVersion>,
}

/// A single published version of a plugin.
#[derive(Clone, Debug, Deserialize)]
pub struct IndexVersion {
    /// Version string. Compared as [`semver::Version`] when possible for
    /// "latest" resolution, falling back to exact string match otherwise.
    pub version: String,
    /// URL the `.wasm` artifact is downloaded from.
    pub download_url: Url,
    /// Lowercase hex-encoded sha256 the downloaded bytes must match before
    /// anything is written to disk.
    pub sha256: String,
    /// The `owner/repo` GitHub repository whose CI is expected to have
    /// published (and attested) this artifact.
    pub source_repo: String,
    /// Informational: the guest ABI version this build declares.
    #[serde(default)]
    pub abi_version: Option<String>,
    /// Minimum `osc` version required to install this version, if any.
    #[serde(default)]
    pub min_cli_version: Option<String>,
}

/// Build a `reqwest::Client` suitable for registry/attestation fetches:
/// rustls, a bounded timeout, and an identifying user agent. Shared by
/// [`fetch_index`]/[`download`] and `crate::provenance`.
pub fn http_client() -> Result<reqwest::Client, WasmPluginError> {
    reqwest::Client::builder()
        .timeout(Duration::from_secs(15))
        .user_agent(concat!("osc-plugin-manager/", env!("CARGO_PKG_VERSION")))
        .build()
        .map_err(|source| WasmPluginError::RegistryFetch {
            url: "<client construction>".into(),
            source,
        })
}

/// Fetch and parse the registry index at `url`.
pub async fn fetch_index(
    url: &str,
    client: &reqwest::Client,
) -> Result<PluginIndex, WasmPluginError> {
    let response = client
        .get(url)
        .send()
        .await
        .map_err(|source| WasmPluginError::RegistryFetch {
            url: url.to_string(),
            source,
        })?
        .error_for_status()
        .map_err(|source| WasmPluginError::RegistryFetch {
            url: url.to_string(),
            source,
        })?;
    let bytes = response
        .bytes()
        .await
        .map_err(|source| WasmPluginError::RegistryFetch {
            url: url.to_string(),
            source,
        })?;
    let index: PluginIndex =
        serde_json::from_slice(&bytes).map_err(|e| WasmPluginError::RegistryFormat {
            url: url.to_string(),
            reason: e.to_string(),
        })?;
    if index.schema_version != SUPPORTED_SCHEMA_VERSION {
        return Err(WasmPluginError::RegistryFormat {
            url: url.to_string(),
            reason: format!(
                "unsupported schema_version {} (expected {SUPPORTED_SCHEMA_VERSION})",
                index.schema_version
            ),
        });
    }
    Ok(index)
}

/// Entries whose name or description contains `query` (case-insensitive),
/// or every entry when `query` is `None`.
pub fn search<'a>(index: &'a PluginIndex, query: Option<&str>) -> Vec<&'a IndexEntry> {
    match query {
        None => index.plugins.iter().collect(),
        Some(q) => {
            let q = q.to_lowercase();
            index
                .plugins
                .iter()
                .filter(|e| {
                    e.name.to_lowercase().contains(&q) || e.description.to_lowercase().contains(&q)
                })
                .collect()
        }
    }
}

/// Resolve `version` (or, when `None`, the highest by semver, falling back
/// to the lexicographically greatest version string when any published
/// version doesn't parse as semver) within `entry`.
pub fn resolve_version<'a>(
    entry: &'a IndexEntry,
    version: Option<&str>,
) -> Result<&'a IndexVersion, WasmPluginError> {
    match version {
        Some(v) => entry
            .versions
            .iter()
            .find(|iv| iv.version == v)
            .ok_or_else(|| WasmPluginError::NotInIndex {
                name: entry.name.clone(),
                version: Some(v.to_string()),
            }),
        None => {
            if entry.versions.is_empty() {
                return Err(WasmPluginError::NotInIndex {
                    name: entry.name.clone(),
                    version: None,
                });
            }
            let all_semver: Option<Vec<(semver::Version, &IndexVersion)>> = entry
                .versions
                .iter()
                .map(|iv| semver::Version::parse(&iv.version).ok().map(|sv| (sv, iv)))
                .collect();
            let latest = match all_semver {
                Some(mut parsed) => {
                    parsed.sort_by(|a, b| a.0.cmp(&b.0));
                    parsed.pop().map(|(_, iv)| iv)
                }
                None => entry
                    .versions
                    .iter()
                    .max_by(|a, b| a.version.cmp(&b.version)),
            };
            latest.ok_or_else(|| WasmPluginError::NotInIndex {
                name: entry.name.clone(),
                version: None,
            })
        }
    }
}

/// Download `version`'s artifact and verify its sha256 matches what the
/// index declared, before returning the bytes to the caller. Nothing is
/// written to disk here or by any caller before this check passes.
pub async fn download(
    entry_name: &str,
    version: &IndexVersion,
    client: &reqwest::Client,
) -> Result<Vec<u8>, WasmPluginError> {
    let response = client
        .get(version.download_url.clone())
        .send()
        .await
        .map_err(|source| WasmPluginError::RegistryFetch {
            url: version.download_url.to_string(),
            source,
        })?
        .error_for_status()
        .map_err(|source| WasmPluginError::RegistryFetch {
            url: version.download_url.to_string(),
            source,
        })?;
    let bytes = response
        .bytes()
        .await
        .map_err(|source| WasmPluginError::RegistryFetch {
            url: version.download_url.to_string(),
            source,
        })?
        .to_vec();

    let actual = sha256_hex_bytes(&bytes);
    if actual != version.sha256.to_lowercase() {
        return Err(WasmPluginError::ChecksumMismatch {
            name: entry_name.to_string(),
            version: version.version.clone(),
            expected: version.sha256.clone(),
            actual,
        });
    }
    Ok(bytes)
}

/// Lowercase hex-encoded sha256 of an in-memory byte slice. Mirrors
/// [`crate::lockfile::sha256_hex`]'s streaming file-based variant for bytes
/// that are already in memory (downloaded content, not yet written to disk).
fn sha256_hex_bytes(bytes: &[u8]) -> String {
    use sha2::{Digest, Sha256};
    let digest = Sha256::digest(bytes);
    digest.iter().map(|b| format!("{b:02x}")).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_index() -> PluginIndex {
        PluginIndex {
            schema_version: 1,
            plugins: vec![IndexEntry {
                name: "example_auth".into(),
                description: "Example auth plugin".into(),
                versions: vec![
                    IndexVersion {
                        version: "1.0.0".into(),
                        download_url: "https://example.invalid/v1.wasm".parse().unwrap(),
                        sha256: "aaaa".into(),
                        source_repo: "gtema/example-auth-plugin".into(),
                        abi_version: Some("1".into()),
                        min_cli_version: None,
                    },
                    IndexVersion {
                        version: "1.2.0".into(),
                        download_url: "https://example.invalid/v1.2.wasm".parse().unwrap(),
                        sha256: "bbbb".into(),
                        source_repo: "gtema/example-auth-plugin".into(),
                        abi_version: Some("1".into()),
                        min_cli_version: None,
                    },
                ],
            }],
        }
    }

    #[test]
    fn search_matches_name_and_description_case_insensitively() {
        let index = sample_index();
        assert_eq!(search(&index, None).len(), 1);
        assert_eq!(search(&index, Some("EXAMPLE")).len(), 1);
        assert_eq!(search(&index, Some("nope")).len(), 0);
    }

    #[test]
    fn resolve_version_picks_highest_semver_by_default() -> Result<(), Box<dyn std::error::Error>> {
        let index = sample_index();
        let entry = &index.plugins[0];
        let latest = resolve_version(entry, None)?;
        assert_eq!(latest.version, "1.2.0");
        let exact = resolve_version(entry, Some("1.0.0"))?;
        assert_eq!(exact.sha256, "aaaa");
        assert!(resolve_version(entry, Some("9.9.9")).is_err());
        Ok(())
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn fetch_index_parses_a_served_document() -> Result<(), Box<dyn std::error::Error>> {
        let server = httpmock::MockServer::start();
        let mock = server.mock(|when, then| {
            when.method(httpmock::Method::GET).path("/index.json");
            then.status(200).json_body(serde_json::json!({
                "schema_version": 1,
                "plugins": [
                    {
                        "name": "example_auth",
                        "description": "Example auth plugin",
                        "versions": [
                            {
                                "version": "1.0.0",
                                "download_url": "https://example.invalid/v1.wasm",
                                "sha256": "aaaa",
                                "source_repo": "gtema/example-auth-plugin"
                            }
                        ]
                    }
                ]
            }));
        });
        let client = http_client()?;
        let index = fetch_index(&format!("{}/index.json", server.base_url()), &client).await?;
        mock.assert();
        assert_eq!(index.plugins.len(), 1);
        assert_eq!(index.plugins[0].name, "example_auth");
        Ok(())
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn fetch_index_rejects_unsupported_schema_version() {
        let server = httpmock::MockServer::start();
        server.mock(|when, then| {
            when.method(httpmock::Method::GET).path("/index.json");
            then.status(200)
                .json_body(serde_json::json!({"schema_version": 2, "plugins": []}));
        });
        let client = http_client().expect("client builds");
        let result = fetch_index(&format!("{}/index.json", server.base_url()), &client).await;
        assert!(matches!(
            result,
            Err(WasmPluginError::RegistryFormat { .. })
        ));
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn download_rejects_checksum_mismatch_before_any_caller_sees_the_bytes()
    -> Result<(), Box<dyn std::error::Error>> {
        let server = httpmock::MockServer::start();
        server.mock(|when, then| {
            when.method(httpmock::Method::GET).path("/plugin.wasm");
            then.status(200).body(b"not what the index promised");
        });
        let iv = IndexVersion {
            version: "1.0.0".into(),
            download_url: format!("{}/plugin.wasm", server.base_url()).parse()?,
            sha256: "0".repeat(64),
            source_repo: "gtema/example-auth-plugin".into(),
            abi_version: None,
            min_cli_version: None,
        };
        let client = http_client()?;
        let result = download("example_auth", &iv, &client).await;
        assert!(matches!(
            result,
            Err(WasmPluginError::ChecksumMismatch { .. })
        ));
        Ok(())
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn download_returns_bytes_matching_the_declared_checksum()
    -> Result<(), Box<dyn std::error::Error>> {
        let payload = b"a fake but checksum-consistent wasm module";
        let server = httpmock::MockServer::start();
        server.mock(|when, then| {
            when.method(httpmock::Method::GET).path("/plugin.wasm");
            then.status(200).body(payload.as_slice());
        });
        let iv = IndexVersion {
            version: "1.0.0".into(),
            download_url: format!("{}/plugin.wasm", server.base_url()).parse()?,
            sha256: sha256_hex_bytes(payload),
            source_repo: "gtema/example-auth-plugin".into(),
            abi_version: None,
            min_cli_version: None,
        };
        let client = http_client()?;
        let bytes = download("example_auth", &iv, &client).await?;
        assert_eq!(bytes, payload);
        Ok(())
    }
}
