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

//! Octavia `Healthmonitor` resource commands

use clap::{Parser, Subcommand};

use openstack_sdk::AsyncOpenStack;

use crate::{Cli, OpenStackCliError};

mod create;
mod delete;
mod list;
mod set;
mod show;

/// Healthmonitor (Octavia) commands
#[derive(Parser)]
pub struct HealthmonitorCommand {
    /// subcommand
    #[command(subcommand)]
    command: HealthmonitorCommands,
}

/// Supported subcommands
#[allow(missing_docs)]
#[derive(Subcommand)]
pub enum HealthmonitorCommands {
    Create(create::HealthmonitorCommand),
    Delete(delete::HealthmonitorCommand),
    List(list::HealthmonitorsCommand),
    Set(set::HealthmonitorCommand),
    Show(show::HealthmonitorCommand),
}

impl HealthmonitorCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        session: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        match &self.command {
            HealthmonitorCommands::Create(cmd) => cmd.take_action(parsed_args, session).await,
            HealthmonitorCommands::Delete(cmd) => cmd.take_action(parsed_args, session).await,
            HealthmonitorCommands::List(cmd) => cmd.take_action(parsed_args, session).await,
            HealthmonitorCommands::Set(cmd) => cmd.take_action(parsed_args, session).await,
            HealthmonitorCommands::Show(cmd) => cmd.take_action(parsed_args, session).await,
        }
    }
}
