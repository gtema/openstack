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

//! QoS Minimum packet rule commands

use clap::{Parser, Subcommand};

use openstack_sdk::AsyncOpenStack;

use crate::{Cli, OpenStackCliError};

pub mod create;
pub mod delete;
pub mod list;
pub mod set;
pub mod show;

/// QoS minimum packet rate rules
///
/// Lists, creates, deletes, shows information for, and updates QoS minimum packet rate rules.
#[derive(Parser)]
pub struct AliasMinimumPacketRateRuleCommand {
    /// subcommand
    #[command(subcommand)]
    command: AliasMinimumPacketRateRuleCommands,
}

/// Supported subcommands
#[allow(missing_docs)]
#[derive(Subcommand)]
pub enum AliasMinimumPacketRateRuleCommands {
    Create(Box<create::AliasMinimumPacketRateRuleCommand>),
    Delete(delete::AliasMinimumPacketRateRuleCommand),
    List(Box<list::AliasMinimumPacketRateRulesCommand>),
    Set(Box<set::AliasMinimumPacketRateRuleCommand>),
    Show(Box<show::AliasMinimumPacketRateRuleCommand>),
}

impl AliasMinimumPacketRateRuleCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        session: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        match &self.command {
            AliasMinimumPacketRateRuleCommands::Create(cmd) => {
                cmd.take_action(parsed_args, session).await
            }
            AliasMinimumPacketRateRuleCommands::Delete(cmd) => {
                cmd.take_action(parsed_args, session).await
            }
            AliasMinimumPacketRateRuleCommands::List(cmd) => {
                cmd.take_action(parsed_args, session).await
            }
            AliasMinimumPacketRateRuleCommands::Set(cmd) => {
                cmd.take_action(parsed_args, session).await
            }
            AliasMinimumPacketRateRuleCommands::Show(cmd) => {
                cmd.take_action(parsed_args, session).await
            }
        }
    }
}
