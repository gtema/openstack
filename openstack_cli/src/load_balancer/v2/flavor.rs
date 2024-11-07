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

//! Octavia `Flavor` resource commands

use clap::{Parser, Subcommand};

use openstack_sdk::AsyncOpenStack;

use crate::{Cli, OpenStackCliError};

mod create;
mod delete;
mod list;
mod set;
mod show;

/// Flavor (Octavia) commands
#[derive(Parser)]
pub struct FlavorCommand {
    /// subcommand
    #[command(subcommand)]
    command: FlavorCommands,
}

/// Supported subcommands
#[allow(missing_docs)]
#[derive(Subcommand)]
pub enum FlavorCommands {
    Create(Box<create::FlavorCommand>),
    Delete(Box<delete::FlavorCommand>),
    List(Box<list::FlavorsCommand>),
    Set(Box<set::FlavorCommand>),
    Show(Box<show::FlavorCommand>),
}

impl FlavorCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        session: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        match &self.command {
            FlavorCommands::Create(cmd) => cmd.take_action(parsed_args, session).await,
            FlavorCommands::Delete(cmd) => cmd.take_action(parsed_args, session).await,
            FlavorCommands::List(cmd) => cmd.take_action(parsed_args, session).await,
            FlavorCommands::Set(cmd) => cmd.take_action(parsed_args, session).await,
            FlavorCommands::Show(cmd) => cmd.take_action(parsed_args, session).await,
        }
    }
}
