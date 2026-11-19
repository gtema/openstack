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

//! Manage the `contexts.yaml` file.

use std::path::PathBuf;

use clap::{Parser, Subcommand};

use openstack_cli_core::{cli::CliArgs, error::OpenStackCliError};
use openstack_sdk_core::config::contexts::find_contexts_file;

pub mod edit;
pub mod list;
pub mod migrate;
pub mod set_current;

/// Manage contexts in `contexts.yaml`
#[derive(Debug, Parser)]
pub struct ContextsCommand {
    /// Context commands
    #[command(subcommand)]
    pub command: ContextsCommands,
}

#[allow(missing_docs)]
#[derive(Debug, Subcommand)]
pub enum ContextsCommands {
    Edit(edit::EditCommand),
    List(list::ListCommand),
    Migrate(migrate::MigrateCommand),
    Use(set_current::UseCommand),
}

impl ContextsCommand {
    /// Perform command action
    pub fn take_action<C: CliArgs>(&self, parsed_args: &C) -> Result<(), OpenStackCliError> {
        match &self.command {
            ContextsCommands::Edit(cmd) => cmd.take_action(parsed_args),
            ContextsCommands::List(cmd) => cmd.take_action(parsed_args),
            ContextsCommands::Migrate(cmd) => cmd.take_action(parsed_args),
            ContextsCommands::Use(cmd) => cmd.take_action(parsed_args),
        }
    }
}

/// Resolve the `contexts.yaml` path: `--file` > `--os-context-file` > the
/// discovered standard file > the XDG default location.
pub(crate) fn resolve_contexts_path(
    file: Option<&std::path::Path>,
    os_context_file: Option<&str>,
) -> PathBuf {
    if let Some(path) = file {
        path.to_path_buf()
    } else if let Some(path) = os_context_file {
        PathBuf::from(path)
    } else if let Some(path) = find_contexts_file() {
        path
    } else {
        crate::clouds::add::default_config_dir().join("contexts.yaml")
    }
}
