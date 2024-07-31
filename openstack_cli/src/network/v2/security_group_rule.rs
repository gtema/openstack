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

//! Security Group Rules commands

use clap::{Parser, Subcommand};

use openstack_sdk::AsyncOpenStack;

use crate::{Cli, OpenStackCliError};

mod create;
mod delete;
mod list;
mod set;
mod show;

/// SecurityGroupRule commands
#[derive(Parser)]
pub struct SecurityGroupRuleCommand {
    /// subcommand
    #[command(subcommand)]
    command: SecurityGroupRuleCommands,
}

/// Supported subcommands
#[allow(missing_docs)]
#[derive(Subcommand)]
pub enum SecurityGroupRuleCommands {
    Create(Box<create::SecurityGroupRuleCommand>),
    Delete(delete::SecurityGroupRuleCommand),
    List(Box<list::SecurityGroupRulesCommand>),
    Set(Box<set::SecurityGroupRuleCommand>),
    Show(Box<show::SecurityGroupRuleCommand>),
}

impl SecurityGroupRuleCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        session: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        match &self.command {
            SecurityGroupRuleCommands::Create(cmd) => cmd.take_action(parsed_args, session).await,
            SecurityGroupRuleCommands::Delete(cmd) => cmd.take_action(parsed_args, session).await,
            SecurityGroupRuleCommands::List(cmd) => cmd.take_action(parsed_args, session).await,
            SecurityGroupRuleCommands::Set(cmd) => cmd.take_action(parsed_args, session).await,
            SecurityGroupRuleCommands::Show(cmd) => cmd.take_action(parsed_args, session).await,
        }
    }
}
