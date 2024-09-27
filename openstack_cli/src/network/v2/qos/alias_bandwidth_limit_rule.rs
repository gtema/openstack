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

//! Bandwitch Limit rul commands

use clap::{Parser, Subcommand};

use openstack_sdk::AsyncOpenStack;

use crate::{Cli, OpenStackCliError};

mod create;
mod delete;
mod list;
mod set;
mod show;

/// Quality of Service rules alias API
///
/// The purpose of this API extension is to enable callers to execute the requests to delete, show
/// and update QoS rules without specifying the corresponding policy ID. Otherwise, these requests
/// have the exact same behavior as their counterparts described in other parts of this
/// documentation. The requests available in this API extension are:
#[derive(Parser)]
pub struct AliasBandwidthLimitRuleCommand {
    /// subcommand
    #[command(subcommand)]
    command: AliasBandwidthLimitRuleCommands,
}

/// Supported subcommands
#[allow(missing_docs)]
#[derive(Subcommand)]
pub enum AliasBandwidthLimitRuleCommands {
    Create(Box<create::AliasBandwidthLimitRuleCommand>),
    Delete(delete::AliasBandwidthLimitRuleCommand),
    List(Box<list::AliasBandwidthLimitRulesCommand>),
    Set(Box<set::AliasBandwidthLimitRuleCommand>),
    Show(Box<show::AliasBandwidthLimitRuleCommand>),
}

impl AliasBandwidthLimitRuleCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        session: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        match &self.command {
            AliasBandwidthLimitRuleCommands::Create(cmd) => {
                cmd.take_action(parsed_args, session).await
            }
            AliasBandwidthLimitRuleCommands::Delete(cmd) => {
                cmd.take_action(parsed_args, session).await
            }
            AliasBandwidthLimitRuleCommands::List(cmd) => {
                cmd.take_action(parsed_args, session).await
            }
            AliasBandwidthLimitRuleCommands::Set(cmd) => {
                cmd.take_action(parsed_args, session).await
            }
            AliasBandwidthLimitRuleCommands::Show(cmd) => {
                cmd.take_action(parsed_args, session).await
            }
        }
    }
}
