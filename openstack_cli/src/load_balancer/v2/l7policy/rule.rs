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

//! Octavia `Rule/Rule` resource commands

use clap::{Parser, Subcommand};

use openstack_sdk::AsyncOpenStack;

use crate::{Cli, OpenStackCliError};

pub mod create;
pub mod delete;
pub mod list;
pub mod set;
pub mod show;

/// `L7Policy Rule` commands
#[derive(Parser)]
pub struct RuleCommand {
    /// subcommand
    #[command(subcommand)]
    command: RuleCommands,
}

/// Supported subcommands
#[allow(missing_docs)]
#[derive(Subcommand)]
pub enum RuleCommands {
    Create(create::RuleCommand),
    Delete(delete::RuleCommand),
    List(list::RulesCommand),
    Set(set::RuleCommand),
    Show(show::RuleCommand),
}

impl RuleCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        session: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        match &self.command {
            RuleCommands::Create(cmd) => cmd.take_action(parsed_args, session).await,
            RuleCommands::Delete(cmd) => cmd.take_action(parsed_args, session).await,
            RuleCommands::List(cmd) => cmd.take_action(parsed_args, session).await,
            RuleCommands::Set(cmd) => cmd.take_action(parsed_args, session).await,
            RuleCommands::Show(cmd) => cmd.take_action(parsed_args, session).await,
        }
    }
}
