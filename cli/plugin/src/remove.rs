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

//! Remove an installed wasm auth plugin

use clap::Parser;
use tracing::info;

use openstack_cli_core::output::{OutputFor, OutputProcessor};
use openstack_cli_core::{cli::CliArgs, error::OpenStackCliError};

use crate::list::PluginListEntry;

/// Remove an installed wasm auth plugin.
///
/// Removes every installed version of `name` unless `--version` narrows it
/// to a single one. If the removed set included the active version and
/// other versions of `name` remain, the most recently installed of those
/// becomes active.
#[derive(Debug, Parser)]
pub struct RemoveCommand {
    /// Plugin name to remove.
    pub name: String,

    /// Remove only this specific version, leaving other installed versions
    /// (if any) in place.
    #[arg(long)]
    pub version: Option<String>,
}

impl RemoveCommand {
    /// Perform command action
    pub async fn take_action<C: CliArgs>(&self, parsed_args: &C) -> Result<(), OpenStackCliError> {
        info!("Remove wasm auth plugin {}", self.name);

        let op = OutputProcessor::from_args(parsed_args, Some("plugin"), Some("remove"));

        openstack_sdk_plugin_wasm::registry::remove(&self.name, self.version.as_deref())
            .map_err(eyre::Report::from)?;

        let lockfile =
            openstack_sdk_plugin_wasm::registry::installed().map_err(eyre::Report::from)?;
        let remaining: Vec<serde_json::Value> = lockfile
            .versions_of(&self.name)
            .map(|entry| {
                let active = lockfile
                    .active
                    .get(&entry.name)
                    .map(|v| v == &entry.version)
                    .unwrap_or(false);
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

        if remaining.is_empty() {
            if let OutputFor::Human = op.target {
                println!("Removed all installed versions of `{}`.", self.name);
            } else {
                op.output_machine(serde_json::Value::Array(Vec::new()))?;
            }
            return Ok(());
        }

        op.output_list::<PluginListEntry>(remaining)
    }
}
