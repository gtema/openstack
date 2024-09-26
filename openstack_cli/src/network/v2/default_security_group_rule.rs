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

//! DefaultSecurityGroupRule resource commands

use clap::{Parser, Subcommand};

use openstack_sdk::AsyncOpenStack;

use crate::{Cli, OpenStackCliError};

mod create;
mod delete;
mod list;
mod set;
mod show;

/// Security group default rules (security-group-default-rules)
///
/// Lists, creates, shows information for, and deletes security group default rules.
#[derive(Parser)]
pub struct DefaultSecurityGroupRuleCommand {
    /// subcommand
    #[command(subcommand)]
    command: DefaultSecurityGroupRuleCommands,
}

/// Supported subcommands
#[allow(missing_docs)]
#[derive(Subcommand)]
pub enum DefaultSecurityGroupRuleCommands {
    Create(Box<create::DefaultSecurityGroupRuleCommand>),
    Delete(Box<delete::DefaultSecurityGroupRuleCommand>),
    List(Box<list::DefaultSecurityGroupRulesCommand>),
    Set(Box<set::DefaultSecurityGroupRuleCommand>),
    Show(Box<show::DefaultSecurityGroupRuleCommand>),
}

impl DefaultSecurityGroupRuleCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        session: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        match &self.command {
            DefaultSecurityGroupRuleCommands::Create(cmd) => {
                cmd.take_action(parsed_args, session).await
            }
            DefaultSecurityGroupRuleCommands::Delete(cmd) => {
                cmd.take_action(parsed_args, session).await
            }
            DefaultSecurityGroupRuleCommands::List(cmd) => {
                cmd.take_action(parsed_args, session).await
            }
            DefaultSecurityGroupRuleCommands::Set(cmd) => {
                cmd.take_action(parsed_args, session).await
            }
            DefaultSecurityGroupRuleCommands::Show(cmd) => {
                cmd.take_action(parsed_args, session).await
            }
        }
    }
}
