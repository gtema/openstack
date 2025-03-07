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
pub struct DscpMarkingRuleCommand {
    /// subcommand
    #[command(subcommand)]
    command: DscpMarkingRuleCommands,
}

/// Supported subcommands
#[allow(missing_docs)]
#[derive(Subcommand)]
pub enum DscpMarkingRuleCommands {
    Create(Box<create::DscpMarkingRuleCommand>),
    Delete(delete::DscpMarkingRuleCommand),
    List(Box<list::DscpMarkingRulesCommand>),
    Set(Box<set::DscpMarkingRuleCommand>),
    Show(Box<show::DscpMarkingRuleCommand>),
}

impl DscpMarkingRuleCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        session: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        match &self.command {
            DscpMarkingRuleCommands::Create(cmd) => cmd.take_action(parsed_args, session).await,
            DscpMarkingRuleCommands::Delete(cmd) => cmd.take_action(parsed_args, session).await,
            DscpMarkingRuleCommands::List(cmd) => cmd.take_action(parsed_args, session).await,
            DscpMarkingRuleCommands::Set(cmd) => cmd.take_action(parsed_args, session).await,
            DscpMarkingRuleCommands::Show(cmd) => cmd.take_action(parsed_args, session).await,
        }
    }
}
