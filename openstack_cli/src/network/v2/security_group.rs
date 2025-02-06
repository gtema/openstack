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

//! Security Group commands

use clap::{Parser, Subcommand};

use openstack_sdk::AsyncOpenStack;

use crate::{Cli, OpenStackCliError};

pub mod create;
pub mod delete;
pub mod list;
pub mod set;
pub mod show;
pub mod tag;

/// SecurityGroup commands
#[derive(Parser)]
pub struct SecurityGroupCommand {
    /// subcommand
    #[command(subcommand)]
    command: SecurityGroupCommands,
}

/// Supported subcommands
#[allow(missing_docs)]
#[derive(Subcommand)]
pub enum SecurityGroupCommands {
    Create(Box<create::SecurityGroupCommand>),
    Delete(delete::SecurityGroupCommand),
    List(Box<list::SecurityGroupsCommand>),
    Set(Box<set::SecurityGroupCommand>),
    Show(Box<show::SecurityGroupCommand>),
    Tag(Box<tag::TagCommand>),
}

impl SecurityGroupCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        session: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        match &self.command {
            SecurityGroupCommands::Create(cmd) => cmd.take_action(parsed_args, session).await,
            SecurityGroupCommands::Delete(cmd) => cmd.take_action(parsed_args, session).await,
            SecurityGroupCommands::List(cmd) => cmd.take_action(parsed_args, session).await,
            SecurityGroupCommands::Set(cmd) => cmd.take_action(parsed_args, session).await,
            SecurityGroupCommands::Show(cmd) => cmd.take_action(parsed_args, session).await,
            SecurityGroupCommands::Tag(cmd) => cmd.take_action(parsed_args, session).await,
        }
    }
}
