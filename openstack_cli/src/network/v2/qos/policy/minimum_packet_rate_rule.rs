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

mod create;
mod delete;
mod list;
mod set;
mod show;

/// QoS minimum packet rate rules
///
/// Lists, creates, deletes, shows information for, and updates QoS minimum packet rate rules.
#[derive(Parser)]
pub struct MinimumPacketRateRuleCommand {
    /// subcommand
    #[command(subcommand)]
    command: MinimumPacketRateRuleCommands,
}

/// Supported subcommands
#[allow(missing_docs)]
#[derive(Subcommand)]
pub enum MinimumPacketRateRuleCommands {
    Create(Box<create::MinimumPacketRateRuleCommand>),
    Delete(delete::MinimumPacketRateRuleCommand),
    List(Box<list::MinimumPacketRateRulesCommand>),
    Set(Box<set::MinimumPacketRateRuleCommand>),
    Show(Box<show::MinimumPacketRateRuleCommand>),
}

impl MinimumPacketRateRuleCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        session: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        match &self.command {
            MinimumPacketRateRuleCommands::Create(cmd) => {
                cmd.take_action(parsed_args, session).await
            }
            MinimumPacketRateRuleCommands::Delete(cmd) => {
                cmd.take_action(parsed_args, session).await
            }
            MinimumPacketRateRuleCommands::List(cmd) => cmd.take_action(parsed_args, session).await,
            MinimumPacketRateRuleCommands::Set(cmd) => cmd.take_action(parsed_args, session).await,
            MinimumPacketRateRuleCommands::Show(cmd) => cmd.take_action(parsed_args, session).await,
        }
    }
}
