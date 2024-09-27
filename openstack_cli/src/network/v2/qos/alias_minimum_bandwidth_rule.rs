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

//! QoS minimum bandwidth rule commands

use clap::{Parser, Subcommand};

use openstack_sdk::AsyncOpenStack;

use crate::{Cli, OpenStackCliError};

mod create;
mod delete;
mod list;
mod set;
mod show;

/// QoS minimum bandwidth rules
///
/// Lists, creates, deletes, shows information for, and updates QoS minimum bandwidth rules.
#[derive(Parser)]
pub struct AliasMinimumBandwidthRuleCommand {
    /// subcommand
    #[command(subcommand)]
    command: AliasMinimumBandwidthRuleCommands,
}

/// Supported subcommands
#[allow(missing_docs)]
#[derive(Subcommand)]
pub enum AliasMinimumBandwidthRuleCommands {
    Create(Box<create::AliasMinimumBandwidthRuleCommand>),
    Delete(delete::AliasMinimumBandwidthRuleCommand),
    List(Box<list::AliasMinimumBandwidthRulesCommand>),
    Set(Box<set::AliasMinimumBandwidthRuleCommand>),
    Show(Box<show::AliasMinimumBandwidthRuleCommand>),
}

impl AliasMinimumBandwidthRuleCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        session: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        match &self.command {
            AliasMinimumBandwidthRuleCommands::Create(cmd) => {
                cmd.take_action(parsed_args, session).await
            }
            AliasMinimumBandwidthRuleCommands::Delete(cmd) => {
                cmd.take_action(parsed_args, session).await
            }
            AliasMinimumBandwidthRuleCommands::List(cmd) => {
                cmd.take_action(parsed_args, session).await
            }
            AliasMinimumBandwidthRuleCommands::Set(cmd) => {
                cmd.take_action(parsed_args, session).await
            }
            AliasMinimumBandwidthRuleCommands::Show(cmd) => {
                cmd.take_action(parsed_args, session).await
            }
        }
    }
}
