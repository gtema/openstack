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

//! DSCP marking rule commands

use clap::{Parser, Subcommand};

use openstack_sdk::AsyncOpenStack;

use crate::{Cli, OpenStackCliError};

pub mod create;
pub mod delete;
pub mod list;
pub mod set;
pub mod show;

/// QoS DSCP marking rules
///
/// Lists, creates, deletes, shows information for, and updates QoS DSCP marking rules.
#[derive(Parser)]
pub struct AliasDscpMarkingRuleCommand {
    /// subcommand
    #[command(subcommand)]
    command: AliasDscpMarkingRuleCommands,
}

/// Supported subcommands
#[allow(missing_docs)]
#[derive(Subcommand)]
pub enum AliasDscpMarkingRuleCommands {
    Create(Box<create::AliasDscpMarkingRuleCommand>),
    Delete(delete::AliasDscpMarkingRuleCommand),
    List(Box<list::AliasDscpMarkingRulesCommand>),
    Set(Box<set::AliasDscpMarkingRuleCommand>),
    Show(Box<show::AliasDscpMarkingRuleCommand>),
}

impl AliasDscpMarkingRuleCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        session: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        match &self.command {
            AliasDscpMarkingRuleCommands::Create(cmd) => {
                cmd.take_action(parsed_args, session).await
            }
            AliasDscpMarkingRuleCommands::Delete(cmd) => {
                cmd.take_action(parsed_args, session).await
            }
            AliasDscpMarkingRuleCommands::List(cmd) => cmd.take_action(parsed_args, session).await,
            AliasDscpMarkingRuleCommands::Set(cmd) => cmd.take_action(parsed_args, session).await,
            AliasDscpMarkingRuleCommands::Show(cmd) => cmd.take_action(parsed_args, session).await,
        }
    }
}
