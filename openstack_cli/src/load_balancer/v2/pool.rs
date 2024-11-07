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

//! Octavia `Pool` resource commands

use clap::{Parser, Subcommand};

use openstack_sdk::AsyncOpenStack;

use crate::{Cli, OpenStackCliError};

mod create;
mod delete;
mod list;
mod member;
mod set;
mod show;

/// Pool (Octavia) commands
#[derive(Parser)]
pub struct PoolCommand {
    /// subcommand
    #[command(subcommand)]
    command: PoolCommands,
}

/// Supported subcommands
#[allow(missing_docs)]
#[derive(Subcommand)]
pub enum PoolCommands {
    Create(Box<create::PoolCommand>),
    Delete(Box<delete::PoolCommand>),
    List(Box<list::PoolsCommand>),
    Member(Box<member::MemberCommand>),
    Set(Box<set::PoolCommand>),
    Show(Box<show::PoolCommand>),
}

impl PoolCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        session: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        match &self.command {
            PoolCommands::Create(cmd) => cmd.take_action(parsed_args, session).await,
            PoolCommands::Delete(cmd) => cmd.take_action(parsed_args, session).await,
            PoolCommands::List(cmd) => cmd.take_action(parsed_args, session).await,
            PoolCommands::Member(cmd) => cmd.take_action(parsed_args, session).await,
            PoolCommands::Set(cmd) => cmd.take_action(parsed_args, session).await,
            PoolCommands::Show(cmd) => cmd.take_action(parsed_args, session).await,
        }
    }
}
