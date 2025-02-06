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

//! Quality of service commands

use clap::{Parser, Subcommand};

use openstack_sdk::AsyncOpenStack;

use crate::{Cli, OpenStackCliError};

pub mod alias_bandwidth_limit_rule;
pub mod alias_dscp_marking_rule;
pub mod alias_minimum_bandwidth_rule;
pub mod alias_minimum_packet_rate_rule;
pub mod policy;
pub mod rule_type;

/// Quality of Service
#[derive(Parser)]
pub struct QosCommand {
    /// subcommand
    #[command(subcommand)]
    command: QosCommands,
}

/// Supported subcommands
#[allow(missing_docs)]
#[derive(Subcommand)]
pub enum QosCommands {
    AliasBandwidthLimitRule(Box<alias_bandwidth_limit_rule::AliasBandwidthLimitRuleCommand>),
    AliasDscpMarkingRule(Box<alias_dscp_marking_rule::AliasDscpMarkingRuleCommand>),
    AliasMinimumBandwidthRule(Box<alias_minimum_bandwidth_rule::AliasMinimumBandwidthRuleCommand>),
    AliasMinimumPacketRateRule(
        Box<alias_minimum_packet_rate_rule::AliasMinimumPacketRateRuleCommand>,
    ),
    Policy(Box<policy::PolicyCommand>),
    RuleType(Box<rule_type::RuleTypeCommand>),
}

impl QosCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        session: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        match &self.command {
            QosCommands::AliasBandwidthLimitRule(cmd) => {
                cmd.take_action(parsed_args, session).await
            }
            QosCommands::AliasDscpMarkingRule(cmd) => cmd.take_action(parsed_args, session).await,
            QosCommands::AliasMinimumBandwidthRule(cmd) => {
                cmd.take_action(parsed_args, session).await
            }
            QosCommands::AliasMinimumPacketRateRule(cmd) => {
                cmd.take_action(parsed_args, session).await
            }
            QosCommands::Policy(cmd) => cmd.take_action(parsed_args, session).await,
            QosCommands::RuleType(cmd) => cmd.take_action(parsed_args, session).await,
        }
    }
}
