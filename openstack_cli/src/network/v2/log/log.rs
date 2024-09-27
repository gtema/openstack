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

//! Logs

use clap::{Parser, Subcommand};

use openstack_sdk::AsyncOpenStack;

use crate::{Cli, OpenStackCliError};

mod create;
mod delete;
mod list;
mod set;
mod show;

/// Log resource
///
/// The logging extension lists, creates, shows information for, and updates log resource.
#[derive(Parser)]
pub struct LogCommand {
    /// subcommand
    #[command(subcommand)]
    command: LogCommands,
}

/// Supported subcommands
#[allow(missing_docs)]
#[derive(Subcommand)]
pub enum LogCommands {
    Create(create::LogCommand),
    Delete(delete::LogCommand),
    List(list::LogsCommand),
    Set(set::LogCommand),
    Show(show::LogCommand),
}

impl LogCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        session: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        match &self.command {
            LogCommands::Create(cmd) => cmd.take_action(parsed_args, session).await,
            LogCommands::Delete(cmd) => cmd.take_action(parsed_args, session).await,
            LogCommands::List(cmd) => cmd.take_action(parsed_args, session).await,
            LogCommands::Set(cmd) => cmd.take_action(parsed_args, session).await,
            LogCommands::Show(cmd) => cmd.take_action(parsed_args, session).await,
        }
    }
}
