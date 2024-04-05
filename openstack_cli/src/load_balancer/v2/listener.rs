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

//! Octavia `Listener` resource commands

use clap::{Parser, Subcommand};

use openstack_sdk::AsyncOpenStack;

use crate::{Cli, OpenStackCliError};

mod create;
mod delete;
mod list;
mod set;
mod show;
mod stats;

/// Listener (Octavia) commands
#[derive(Parser)]
pub struct ListenerCommand {
    /// subcommand
    #[command(subcommand)]
    command: ListenerCommands,
}

/// Supported subcommands
#[allow(missing_docs)]
#[derive(Subcommand)]
pub enum ListenerCommands {
    Create(Box<create::ListenerCommand>),
    Delete(Box<delete::ListenerCommand>),
    List(Box<list::ListenersCommand>),
    Set(Box<set::ListenerCommand>),
    Show(Box<show::ListenerCommand>),
    Stats(Box<stats::ListenerCommand>),
}

impl ListenerCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        session: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        match &self.command {
            ListenerCommands::Create(cmd) => cmd.take_action(parsed_args, session).await,
            ListenerCommands::Delete(cmd) => cmd.take_action(parsed_args, session).await,
            ListenerCommands::List(cmd) => cmd.take_action(parsed_args, session).await,
            ListenerCommands::Set(cmd) => cmd.take_action(parsed_args, session).await,
            ListenerCommands::Show(cmd) => cmd.take_action(parsed_args, session).await,
            ListenerCommands::Stats(cmd) => cmd.take_action(parsed_args, session).await,
        }
    }
}
