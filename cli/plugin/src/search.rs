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

//! Search the plugin registry index.

use clap::Parser;
use serde::{Deserialize, Serialize};
use tracing::info;

use openstack_cli_core::output::OutputProcessor;
use openstack_cli_core::{cli::CliArgs, error::OpenStackCliError};
use structable::{StructTable, StructTableOptions};

/// Search the plugin registry index for plugins whose name or description
/// matches `query`, or list every published plugin when `query` is omitted.
#[derive(Debug, Parser)]
pub struct SearchCommand {
    /// Case-insensitive substring to match against plugin name/description.
    pub query: Option<String>,

    /// Registry index URL to search. The pinned default is never silently
    /// overridden by anything but this explicit flag.
    #[arg(long)]
    pub registry_url: Option<String>,
}

/// A single matching registry entry.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize, StructTable)]
pub struct PluginSearchResult {
    /// Plugin name.
    #[structable()]
    pub name: String,

    /// Human readable description.
    #[structable()]
    pub description: String,

    /// The version that would be installed by `osc plugin install <name>`.
    #[structable()]
    pub latest_version: String,

    /// The `owner/repo` the latest version claims to be published from.
    #[structable()]
    pub source_repo: String,
}

impl SearchCommand {
    /// Perform command action
    pub async fn take_action<C: CliArgs>(&self, parsed_args: &C) -> Result<(), OpenStackCliError> {
        info!("Search plugin registry for {:?}", self.query);

        let op = OutputProcessor::from_args(parsed_args, Some("plugin"), Some("search"));

        let registry_url = self
            .registry_url
            .as_deref()
            .unwrap_or(openstack_sdk_plugin_wasm::index::DEFAULT_REGISTRY_URL);
        let client = openstack_sdk_plugin_wasm::index::http_client().map_err(eyre::Report::from)?;
        let index = openstack_sdk_plugin_wasm::index::fetch_index(registry_url, &client)
            .await
            .map_err(eyre::Report::from)?;

        let data: Vec<serde_json::Value> =
            openstack_sdk_plugin_wasm::index::search(&index, self.query.as_deref())
                .into_iter()
                .map(|entry| {
                    let latest =
                        openstack_sdk_plugin_wasm::index::resolve_version(entry, None).ok();
                    serde_json::to_value(PluginSearchResult {
                        name: entry.name.clone(),
                        description: entry.description.clone(),
                        latest_version: latest.map(|v| v.version.clone()).unwrap_or_default(),
                        source_repo: latest.map(|v| v.source_repo.clone()).unwrap_or_default(),
                    })
                })
                .collect::<Result<_, _>>()?;

        op.output_list::<PluginSearchResult>(data)
    }
}
