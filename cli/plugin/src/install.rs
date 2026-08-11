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

//! Install a wasm auth plugin

use std::path::PathBuf;

use clap::Parser;
use serde::{Deserialize, Serialize};
use tracing::info;

use openstack_cli_core::output::{OutputFor, OutputProcessor};
use openstack_cli_core::{cli::CliArgs, error::OpenStackCliError};
use structable::{StructTable, StructTableOptions};

/// Validate and install a wasm auth plugin.
///
/// The plugin is loaded and its ABI is validated before anything is copied,
/// so a malformed `.wasm` file is rejected without touching the plugin
/// directory. Installing a plugin under a name that's already installed
/// fails; remove the existing file from the plugin directory first to
/// replace it.
#[derive(Debug, Parser)]
pub struct InstallCommand {
    /// Path to the `.wasm` auth plugin file to install.
    pub file: PathBuf,
}

/// Information about an installed plugin.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize, StructTable)]
pub struct InstalledPlugin {
    /// Plugin name (its `.wasm` file stem).
    #[structable()]
    pub name: String,

    /// Path the plugin was installed to.
    #[structable()]
    pub source: String,

    /// Auth method names this plugin supports.
    #[structable()]
    pub supported_methods: String,

    /// Guest ABI's declared Identity API version (major.minor).
    #[structable()]
    pub api_version: String,
}

impl InstallCommand {
    /// Perform command action
    pub async fn take_action<C: CliArgs>(&self, parsed_args: &C) -> Result<(), OpenStackCliError> {
        info!("Install wasm auth plugin from {}", self.file.display());

        let op = OutputProcessor::from_args(parsed_args, Some("plugin"), Some("install"));

        let plugin =
            openstack_sdk_plugin_wasm::registry::install(&self.file).map_err(eyre::Report::from)?;
        let (major, minor) = plugin.api_version();

        let info = InstalledPlugin {
            name: plugin.name().to_string(),
            source: plugin.source().display().to_string(),
            supported_methods: plugin.supported_methods().join(", "),
            api_version: format!("{major}.{minor}"),
        };

        match op.target {
            OutputFor::Human => {
                op.output_human(&info)?;
            }
            _ => {
                op.output_machine(serde_json::to_value(&info)?)?;
            }
        }
        Ok(())
    }
}
