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

//! Octavia `Member` resource commands

use clap::{Parser, Subcommand};

use openstack_sdk::AsyncOpenStack;

use crate::{Cli, OpenStackCliError};

mod create;
mod delete;
mod list;
mod set;
mod show;

/// Pool Member commands
#[derive(Parser)]
pub struct MemberCommand {
    /// subcommand
    #[command(subcommand)]
    command: MemberCommands,
}

/// Supported subcommands
#[allow(missing_docs)]
#[derive(Subcommand)]
pub enum MemberCommands {
    Create(create::MemberCommand),
    Delete(delete::MemberCommand),
    List(list::MembersCommand),
    Set(set::MemberCommand),
    Show(show::MemberCommand),
}

impl MemberCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        session: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        match &self.command {
            MemberCommands::Create(cmd) => cmd.take_action(parsed_args, session).await,
            MemberCommands::Delete(cmd) => cmd.take_action(parsed_args, session).await,
            MemberCommands::List(cmd) => cmd.take_action(parsed_args, session).await,
            MemberCommands::Set(cmd) => cmd.take_action(parsed_args, session).await,
            MemberCommands::Show(cmd) => cmd.take_action(parsed_args, session).await,
        }
    }
}
