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

//! List the contexts of `contexts.yaml`.

use std::path::PathBuf;

use clap::Args;
use eyre::eyre;
use serde::{Deserialize, Serialize};
use structable::{StructTable, StructTableOptions};

use openstack_cli_core::cli::CliArgs;
use openstack_cli_core::error::OpenStackCliError;
use openstack_cli_core::output::OutputProcessor;
use openstack_sdk_core::config::contexts::ContextsFile;

/// List the contexts of `contexts.yaml`.
///
/// The `current-context` is marked in the `current` column.
#[derive(Debug, Args)]
pub struct ListCommand {
    /// Path to the `contexts.yaml` (default: `--os-context-file` or the
    /// standard search locations).
    #[arg(long, value_name = "PATH")]
    file: Option<PathBuf>,
}

/// A context as shown by the list.
#[derive(Clone, Debug, Default, Deserialize, Serialize, StructTable)]
pub struct ContextView {
    /// Context name.
    #[structable(title = "Name")]
    pub name: String,
    /// Whether this is the current context.
    #[structable(title = "Current")]
    pub current: bool,
    /// Server name.
    #[structable(title = "Server")]
    pub server: String,
    /// User name.
    #[structable(title = "User")]
    pub user: String,
    /// Scope name.
    #[structable(title = "Scope", optional)]
    pub scope: Option<String>,
    /// Region name.
    #[structable(title = "Region", optional)]
    pub region_name: Option<String>,
}

impl ListCommand {
    /// Perform command action
    pub fn take_action<C: CliArgs>(&self, parsed_args: &C) -> Result<(), OpenStackCliError> {
        let path = super::resolve_contexts_path(
            self.file.as_deref(),
            parsed_args
                .global_opts()
                .connection
                .os_context_file
                .as_deref(),
        );
        if !path.is_file() {
            return Err(eyre!("contexts file {} does not exist", path.display()).into());
        }
        let file = ContextsFile::from_path(&path).map_err(|err| eyre!("{err}"))?;
        for warning in &file.warnings {
            eprintln!("warning: {warning}");
        }

        let data: Vec<serde_json::Value> = file
            .contexts
            .iter()
            .map(|c| {
                serde_json::to_value(ContextView {
                    name: c.name.clone(),
                    current: file.current_context.as_deref() == Some(c.name.as_str()),
                    server: c.context.server.clone(),
                    user: c.context.user.clone(),
                    scope: c.context.scope.clone(),
                    region_name: c.context.region_name.clone(),
                })
            })
            .collect::<Result<_, _>>()
            .map_err(|err| eyre!("{err}"))?;

        let op = OutputProcessor::from_args(parsed_args, Some("config"), Some("contexts list"));
        op.output_list::<ContextView>(data)?;
        op.show_command_hint()?;
        Ok(())
    }
}
