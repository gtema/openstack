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

//! Network resource commands

use clap::{Parser, Subcommand};

use openstack_sdk::AsyncOpenStack;

use crate::{Cli, OpenStackCliError};

mod create;
mod delete;
mod list;
mod show;
mod tag;

/// Network commands
#[derive(Parser)]
pub struct NetworkCommand {
    /// subcommand
    #[command(subcommand)]
    command: NetworkCommands,
}

/// Supported subcommands
#[allow(missing_docs)]
#[derive(Subcommand)]
pub enum NetworkCommands {
    Create(Box<create::NetworkCommand>),
    Delete(Box<delete::NetworkCommand>),
    List(Box<list::NetworksCommand>),
    Show(Box<show::NetworkCommand>),
    Tag(Box<tag::TagCommand>),
}

impl NetworkCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        session: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        match &self.command {
            NetworkCommands::Create(cmd) => cmd.take_action(parsed_args, session).await,
            NetworkCommands::Delete(cmd) => cmd.take_action(parsed_args, session).await,
            NetworkCommands::List(cmd) => cmd.take_action(parsed_args, session).await,
            NetworkCommands::Show(cmd) => cmd.take_action(parsed_args, session).await,
            NetworkCommands::Tag(cmd) => cmd.take_action(parsed_args, session).await,
        }
    }
}
