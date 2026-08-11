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
use tracing::info;

use openstack_cli_core::output::OutputProcessor;
use openstack_cli_core::{cli::CliArgs, error::OpenStackCliError};

use crate::install::InstalledPlugin;

/// List installed wasm auth plugins.
#[derive(Debug, Parser)]
pub struct ListCommand {}

impl ListCommand {
    /// Perform command action
    pub async fn take_action<C: CliArgs>(&self, parsed_args: &C) -> Result<(), OpenStackCliError> {
        info!("List installed wasm auth plugins");

        let op = OutputProcessor::from_args(parsed_args, Some("plugin"), Some("list"));

        openstack_sdk_plugin_wasm::registry::ensure_loaded().map_err(eyre::Report::from)?;
        let plugins =
            openstack_sdk_plugin_wasm::registry::list_loaded().map_err(eyre::Report::from)?;

        let data: Vec<serde_json::Value> = plugins
            .iter()
            .map(|plugin| {
                let (major, minor) = plugin.api_version();
                serde_json::to_value(InstalledPlugin {
                    name: plugin.name().to_string(),
                    source: plugin.source().display().to_string(),
                    supported_methods: plugin.supported_methods().join(", "),
                    api_version: format!("{major}.{minor}"),
                })
            })
            .collect::<Result<_, _>>()?;

        op.output_list::<InstalledPlugin>(data)
    }
}
