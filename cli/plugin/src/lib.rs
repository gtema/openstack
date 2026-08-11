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

//! WASM auth plugin management operations.
//!
//! Unlike most `osc` subcommands this one never needs a live cloud
//! connection: it only manages `.wasm` auth plugin files in the local
//! plugin directory, so it is dispatched before cloud config resolution
//! (mirroring `osc completion`).

use clap::{Parser, Subcommand};

use openstack_cli_core::{cli::CliArgs, error::OpenStackCliError};

pub mod info;
pub mod install;
pub mod list;
pub mod remove;
pub mod verify;

/// WASM auth plugin management
///
/// This command manages Extism (WASM) auth plugins that extend `osc` with
/// additional authentication methods beyond the ones compiled in. See
/// `osc plugin install --help` for how to add one.
#[derive(Parser)]
pub struct PluginCommand {
    /// Plugin management commands
    #[command(subcommand)]
    pub command: PluginCommands,
}

#[allow(missing_docs)]
#[derive(Subcommand)]
pub enum PluginCommands {
    Install(install::InstallCommand),
    List(list::ListCommand),
    Info(info::InfoCommand),
    Remove(remove::RemoveCommand),
    Verify(verify::VerifyCommand),
}

impl PluginCommand {
    /// Perform command action
    pub async fn take_action<C: CliArgs>(&self, parsed_args: &C) -> Result<(), OpenStackCliError> {
        match &self.command {
            PluginCommands::Install(cmd) => cmd.take_action(parsed_args).await,
            PluginCommands::List(cmd) => cmd.take_action(parsed_args).await,
            PluginCommands::Info(cmd) => cmd.take_action(parsed_args).await,
            PluginCommands::Remove(cmd) => cmd.take_action(parsed_args).await,
            PluginCommands::Verify(cmd) => cmd.take_action(parsed_args).await,
        }
    }
}
