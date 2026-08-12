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

//! List installed wasm auth plugins

use clap::Parser;
use serde::{Deserialize, Serialize};
use tracing::info;

use openstack_cli_core::output::{OutputFor, OutputProcessor};
use openstack_cli_core::{cli::CliArgs, error::OpenStackCliError};
use structable::{StructTable, StructTableOptions};

/// A single installed `name@version` entry, as recorded in the plugin
/// lockfile. Listed regardless of whether it's the active version.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize, StructTable)]
pub struct PluginListEntry {
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

    /// Whether this entry was trusted without provenance verification
    /// (`--allow-unsigned` at install/update time).
    #[structable()]
    pub allow_unsigned: bool,
}

/// List installed wasm auth plugins.
#[derive(Debug, Parser)]
pub struct ListCommand {}

impl ListCommand {
    /// Perform command action
    pub async fn take_action<C: CliArgs>(&self, parsed_args: &C) -> Result<(), OpenStackCliError> {
        info!("List installed wasm auth plugins");

        let op = OutputProcessor::from_args(parsed_args, Some("plugin"), Some("list"));

        let lockfile =
            openstack_sdk_plugin_wasm::registry::installed().map_err(eyre::Report::from)?;

        let mut unsigned: Vec<String> = Vec::new();
        let data: Vec<serde_json::Value> = lockfile
            .plugins
            .values()
            .map(|entry| {
                let active = lockfile
                    .active
                    .get(&entry.name)
                    .map(|v| v == &entry.version)
                    .unwrap_or(false);
                if entry.trust.allow_unsigned {
                    unsigned.push(format!("{}@{}", entry.name, entry.version));
                }
                serde_json::to_value(PluginListEntry {
                    name: entry.name.clone(),
                    version: entry.version.clone(),
                    active,
                    source: entry.source.display().to_string(),
                    sha256: entry.sha256.clone(),
                    installed_at: entry.installed_at.to_string(),
                    allow_unsigned: entry.trust.allow_unsigned,
                })
            })
            .collect::<Result<_, _>>()?;

        op.output_list::<PluginListEntry>(data)?;

        if matches!(op.target, OutputFor::Human) && !unsigned.is_empty() {
            println!(
                "\n⚠ {} installed plugin{} running without provenance verification (allow_unsigned): {}",
                unsigned.len(),
                if unsigned.len() == 1 { " is" } else { "s are" },
                unsigned.join(", ")
            );
        }
        Ok(())
    }
}
