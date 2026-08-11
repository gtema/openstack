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

//! Show details about an installed wasm auth plugin

use clap::Parser;
use serde::{Deserialize, Serialize};
use tracing::info;

use openstack_cli_core::output::OutputProcessor;
use openstack_cli_core::{cli::CliArgs, error::OpenStackCliError};
use structable::{StructTable, StructTableOptions};

/// Show every installed version of a wasm auth plugin, read from the
/// lockfile.
#[derive(Debug, Parser)]
pub struct InfoCommand {
    /// Plugin name.
    pub name: String,
}

/// Details of a single installed `name@version`.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize, StructTable)]
pub struct PluginVersionInfo {
    /// Plugin name.
    #[structable()]
    pub name: String,

    /// Installed version.
    #[structable()]
    pub version: String,

    /// Whether this is the version currently used for auth-method
    /// resolution.
    #[structable()]
    pub active: bool,

    /// The path this version was originally installed from.
    #[structable()]
    pub source: String,

    /// Lowercase hex-encoded SHA-256 recorded at install time.
    #[structable()]
    pub sha256: String,

    /// When this version was installed.
    #[structable()]
    pub installed_at: String,

    /// Whether the user explicitly confirmed installing this entry.
    #[structable()]
    pub confirmed_by_user: bool,

    /// Whether this entry was allowed to install without a signature.
    #[structable()]
    pub allow_unsigned: bool,

    /// Auth methods declared by this version's ABI. Only populated for the
    /// active version, which is the only one ever loaded.
    #[structable()]
    pub supported_methods: String,

    /// Identity API version (major.minor) declared by this version's ABI.
    /// Only populated for the active version.
    #[structable()]
    pub api_version: String,
}

impl InfoCommand {
    /// Perform command action
    pub async fn take_action<C: CliArgs>(&self, parsed_args: &C) -> Result<(), OpenStackCliError> {
        info!("Show wasm auth plugin info for {}", self.name);

        let op = OutputProcessor::from_args(parsed_args, Some("plugin"), Some("info"));

        let lockfile =
            openstack_sdk_plugin_wasm::registry::installed().map_err(eyre::Report::from)?;
        let mut versions: Vec<_> = lockfile.versions_of(&self.name).cloned().collect();
        if versions.is_empty() {
            return Err(eyre::eyre!("no installed plugin named `{}`", self.name).into());
        }
        versions.sort_by(|a, b| a.version.cmp(&b.version));

        openstack_sdk_plugin_wasm::registry::ensure_loaded().map_err(eyre::Report::from)?;
        let loaded =
            openstack_sdk_plugin_wasm::registry::list_loaded().map_err(eyre::Report::from)?;
        let active_loaded = loaded.iter().find(|p| p.name() == self.name);

        let data: Vec<serde_json::Value> = versions
            .into_iter()
            .map(|entry| {
                let active = lockfile
                    .active
                    .get(&entry.name)
                    .map(|v| v == &entry.version)
                    .unwrap_or(false);
                let (supported_methods, api_version) =
                    match active.then_some(active_loaded).flatten() {
                        Some(p) => {
                            let (major, minor) = p.api_version();
                            (p.supported_methods().join(", "), format!("{major}.{minor}"))
                        }
                        None => (String::new(), String::new()),
                    };
                serde_json::to_value(PluginVersionInfo {
                    name: entry.name,
                    version: entry.version,
                    active,
                    source: entry.source.display().to_string(),
                    sha256: entry.sha256,
                    installed_at: entry.installed_at.to_string(),
                    confirmed_by_user: entry.trust.confirmed_by_user,
                    allow_unsigned: entry.trust.allow_unsigned,
                    supported_methods,
                    api_version,
                })
            })
            .collect::<Result<_, _>>()?;

        op.output_list::<PluginVersionInfo>(data)
    }
}
