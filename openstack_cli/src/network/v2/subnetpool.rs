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

//! SubnetPool pool commands

use clap::{Parser, Subcommand};

use openstack_sdk::AsyncOpenStack;

use crate::{Cli, OpenStackCliError};

pub mod add_prefixes;
pub mod create;
pub mod delete;
pub mod list;
pub mod remove_prefixes;
pub mod set;
pub mod show;
pub mod tag;

/// SubnetPool commands
#[derive(Parser)]
pub struct SubnetPoolCommand {
    /// subcommand
    #[command(subcommand)]
    command: SubnetPoolCommands,
}

/// Supported subcommands
#[allow(missing_docs)]
#[derive(Subcommand)]
pub enum SubnetPoolCommands {
    AddPrefixes(Box<add_prefixes::SubnetpoolCommand>),
    Create(Box<create::SubnetpoolCommand>),
    Delete(delete::SubnetpoolCommand),
    List(Box<list::SubnetpoolsCommand>),
    RemovePrefixes(Box<remove_prefixes::SubnetpoolCommand>),
    Set(Box<set::SubnetpoolCommand>),
    Show(Box<show::SubnetpoolCommand>),
    Tag(Box<tag::TagCommand>),
}

impl SubnetPoolCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        session: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        match &self.command {
            SubnetPoolCommands::AddPrefixes(cmd) => cmd.take_action(parsed_args, session).await,
            SubnetPoolCommands::Create(cmd) => cmd.take_action(parsed_args, session).await,
            SubnetPoolCommands::Delete(cmd) => cmd.take_action(parsed_args, session).await,
            SubnetPoolCommands::List(cmd) => cmd.take_action(parsed_args, session).await,
            SubnetPoolCommands::RemovePrefixes(cmd) => cmd.take_action(parsed_args, session).await,
            SubnetPoolCommands::Set(cmd) => cmd.take_action(parsed_args, session).await,
            SubnetPoolCommands::Show(cmd) => cmd.take_action(parsed_args, session).await,
            SubnetPoolCommands::Tag(cmd) => cmd.take_action(parsed_args, session).await,
        }
    }
}
