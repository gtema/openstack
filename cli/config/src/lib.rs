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
//! Local client configuration file operations.
//!
//! This crate is the foundation for `osc config` commands. It currently
//! provides:
//!
//!  * [`show`], displaying the effective local CLI configuration
//!    (`$XDG_CONFIG_HOME/osc/config.yaml`).
//!  * [`yaml_edit`], a comment- and anchor-preserving YAML editor for
//!    `clouds.yaml`/`secure.yaml`; command implementations built on top of
//!    it (e.g. `clouds add`) land separately.

use clap::{Parser, Subcommand};

use openstack_cli_core::{cli::CliArgs, error::OpenStackCliError};

pub mod show;
pub mod yaml_edit;

/// Local `osc` client configuration.
#[derive(Debug, Parser)]
pub struct ConfigCommand {
    /// Config management commands
    #[command(subcommand)]
    pub command: ConfigCommands,
}

#[allow(missing_docs)]
#[derive(Debug, Subcommand)]
pub enum ConfigCommands {
    Show(show::ShowCommand),
}

impl ConfigCommand {
    /// Perform command action.
    pub async fn take_action<C: CliArgs>(&self, parsed_args: &C) -> Result<(), OpenStackCliError> {
        match &self.command {
            ConfigCommands::Show(cmd) => cmd.take_action(parsed_args).await,
        }
    }
}
