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

//! Bandwitch Limit rule commands

use clap::{Parser, Subcommand};

use openstack_sdk::AsyncOpenStack;

use crate::{Cli, OpenStackCliError};

pub mod create;
pub mod delete;
pub mod list;
pub mod set;
pub mod show;

/// QoS bandwidth limit rules
///
/// Lists, creates, deletes, shows information for, and updates QoS bandwidth limit rules.
///
/// Bandwidth limit direction
///
/// The qos-bw-limit-direction extension adds the direction attribute to QoS rule types. The
/// direction attribute allows to configure QoS bandwidth limit rule with specific direction:
/// ingress or egress. Default is egress.
#[derive(Parser)]
pub struct BandwidthLimitRuleCommand {
    /// subcommand
    #[command(subcommand)]
    command: BandwidthLimitRuleCommands,
}

/// Supported subcommands
#[allow(missing_docs)]
#[derive(Subcommand)]
pub enum BandwidthLimitRuleCommands {
    Create(Box<create::BandwidthLimitRuleCommand>),
    Delete(delete::BandwidthLimitRuleCommand),
    List(Box<list::BandwidthLimitRulesCommand>),
    Set(Box<set::BandwidthLimitRuleCommand>),
    Show(Box<show::BandwidthLimitRuleCommand>),
}

impl BandwidthLimitRuleCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        session: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        match &self.command {
            BandwidthLimitRuleCommands::Create(cmd) => cmd.take_action(parsed_args, session).await,
            BandwidthLimitRuleCommands::Delete(cmd) => cmd.take_action(parsed_args, session).await,
            BandwidthLimitRuleCommands::List(cmd) => cmd.take_action(parsed_args, session).await,
            BandwidthLimitRuleCommands::Set(cmd) => cmd.take_action(parsed_args, session).await,
            BandwidthLimitRuleCommands::Show(cmd) => cmd.take_action(parsed_args, session).await,
        }
    }
}
