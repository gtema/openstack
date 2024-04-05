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

//! Octavia `Amphorae` resource commands

use clap::{Parser, Subcommand};

use openstack_sdk::AsyncOpenStack;

use crate::{Cli, OpenStackCliError};

mod config;
mod delete;
mod failover;
mod list;
mod show;
mod stats;

/// Amphorae (Octavia) commands
#[derive(Parser)]
pub struct AmphoraeCommand {
    /// subcommand
    #[command(subcommand)]
    command: AmphoraeCommands,
}

/// Supported subcommands
#[allow(missing_docs)]
#[derive(Subcommand)]
pub enum AmphoraeCommands {
    Config(config::AmphoraeCommand),
    Delete(delete::AmphoraeCommand),
    Failover(failover::AmphoraeCommand),
    List(list::AmphoraesCommand),
    Show(show::AmphoraeCommand),
    Stats(stats::AmphoraeCommand),
}

impl AmphoraeCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        session: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        match &self.command {
            AmphoraeCommands::Config(cmd) => cmd.take_action(parsed_args, session).await,
            AmphoraeCommands::Delete(cmd) => cmd.take_action(parsed_args, session).await,
            AmphoraeCommands::Failover(cmd) => cmd.take_action(parsed_args, session).await,
            AmphoraeCommands::List(cmd) => cmd.take_action(parsed_args, session).await,
            AmphoraeCommands::Show(cmd) => cmd.take_action(parsed_args, session).await,
            AmphoraeCommands::Stats(cmd) => cmd.take_action(parsed_args, session).await,
        }
    }
}
