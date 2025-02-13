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

//! Server group

use clap::{Parser, Subcommand};

use openstack_sdk::AsyncOpenStack;

use crate::{Cli, OpenStackCliError};

pub mod create_20;
pub mod create_215;
pub mod create_264;
pub mod delete;
pub mod list;
pub mod show;

/// Server groups (os-server-groups)
///
/// Lists, shows information for, creates, and deletes server groups.
#[derive(Parser)]
pub struct ServerGroupCommand {
    /// subcommand
    #[command(subcommand)]
    command: ServerGroupCommands,
}

/// Supported subcommands
#[allow(missing_docs)]
#[derive(Subcommand)]
pub enum ServerGroupCommands {
    #[command(visible_alias = "create")]
    Create264(create_264::ServerGroupCommand),
    Create215(create_215::ServerGroupCommand),
    Create20(create_20::ServerGroupCommand),
    Delete(delete::ServerGroupCommand),
    List(list::ServerGroupsCommand),
    Show(show::ServerGroupCommand),
}

impl ServerGroupCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        session: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        match &self.command {
            ServerGroupCommands::Create264(cmd) => cmd.take_action(parsed_args, session).await,
            ServerGroupCommands::Create215(cmd) => cmd.take_action(parsed_args, session).await,
            ServerGroupCommands::Create20(cmd) => cmd.take_action(parsed_args, session).await,
            ServerGroupCommands::Delete(cmd) => cmd.take_action(parsed_args, session).await,
            ServerGroupCommands::List(cmd) => cmd.take_action(parsed_args, session).await,
            ServerGroupCommands::Show(cmd) => cmd.take_action(parsed_args, session).await,
        }
    }
}
