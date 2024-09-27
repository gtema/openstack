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

//! QoS policy commands

use clap::{Parser, Subcommand};

use openstack_sdk::AsyncOpenStack;

use crate::{Cli, OpenStackCliError};

mod bandwidth_limit_rule;
mod create;
mod delete;
mod dscp_marking_rule;
mod list;
mod minimum_bandwidth_rule;
mod minimum_packet_rate_rule;
mod set;
mod show;

/// QoS minimum packet rate rules
///
/// Lists, creates, deletes, shows information for, and updates QoS minimum packet rate rules.
#[derive(Parser)]
pub struct PolicyCommand {
    /// subcommand
    #[command(subcommand)]
    command: PolicyCommands,
}

/// Supported subcommands
#[allow(missing_docs)]
#[derive(Subcommand)]
pub enum PolicyCommands {
    BandwidthLimitRule(Box<bandwidth_limit_rule::BandwidthLimitRuleCommand>),
    Create(Box<create::PolicyCommand>),
    Delete(delete::PolicyCommand),
    DscpMarkingRule(dscp_marking_rule::DscpMarkingRuleCommand),
    List(Box<list::PoliciesCommand>),
    MinimumBandwidthRule(Box<minimum_bandwidth_rule::MinimumBandwidthRuleCommand>),
    MinimumPacketRateRule(Box<minimum_packet_rate_rule::MinimumPacketRateRuleCommand>),
    Set(Box<set::PolicyCommand>),
    Show(Box<show::PolicyCommand>),
}

impl PolicyCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        session: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        match &self.command {
            PolicyCommands::BandwidthLimitRule(cmd) => cmd.take_action(parsed_args, session).await,
            PolicyCommands::Create(cmd) => cmd.take_action(parsed_args, session).await,
            PolicyCommands::Delete(cmd) => cmd.take_action(parsed_args, session).await,
            PolicyCommands::DscpMarkingRule(cmd) => cmd.take_action(parsed_args, session).await,
            PolicyCommands::List(cmd) => cmd.take_action(parsed_args, session).await,
            PolicyCommands::MinimumBandwidthRule(cmd) => {
                cmd.take_action(parsed_args, session).await
            }
            PolicyCommands::MinimumPacketRateRule(cmd) => {
                cmd.take_action(parsed_args, session).await
            }
            PolicyCommands::Set(cmd) => cmd.take_action(parsed_args, session).await,
            PolicyCommands::Show(cmd) => cmd.take_action(parsed_args, session).await,
        }
    }
}
