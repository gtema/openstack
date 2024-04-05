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

//! Octavia `L7Policy` resource commands

use clap::{Parser, Subcommand};

use openstack_sdk::AsyncOpenStack;

use crate::{Cli, OpenStackCliError};

mod create;
mod delete;
mod list;
mod rule;
mod set;
mod show;

/// L7Policy (Octavia) commands
#[derive(Parser)]
pub struct L7PolicyCommand {
    /// subcommand
    #[command(subcommand)]
    command: L7PolicyCommands,
}

/// Supported subcommands
#[allow(missing_docs)]
#[derive(Subcommand)]
pub enum L7PolicyCommands {
    Create(create::L7PolicyCommand),
    Delete(delete::L7PolicyCommand),
    List(list::L7PoliciesCommand),
    Rule(rule::RuleCommand),
    Set(set::L7PolicyCommand),
    Show(show::L7PolicyCommand),
}

impl L7PolicyCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        session: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        match &self.command {
            L7PolicyCommands::Create(cmd) => cmd.take_action(parsed_args, session).await,
            L7PolicyCommands::Delete(cmd) => cmd.take_action(parsed_args, session).await,
            L7PolicyCommands::List(cmd) => cmd.take_action(parsed_args, session).await,
            L7PolicyCommands::Rule(cmd) => cmd.take_action(parsed_args, session).await,
            L7PolicyCommands::Set(cmd) => cmd.take_action(parsed_args, session).await,
            L7PolicyCommands::Show(cmd) => cmd.take_action(parsed_args, session).await,
        }
    }
}
