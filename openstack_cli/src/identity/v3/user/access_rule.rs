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

//! Identity User Access Rules commands
//!

use clap::{Args, Subcommand};

use crate::{OSCCommand, OpenStackCliError};

use openstack_sdk::AsyncOpenStack;

mod delete;
mod list;
mod show;

/// Identity User access rules
#[derive(Args, Clone, Debug)]
// #[command(args_conflicts_with_subcommands = true)]
pub struct AccessRuleArgs {
    #[command(subcommand)]
    command: AccessRuleCommands,
}

#[derive(Subcommand, Clone, Debug)]
pub enum AccessRuleCommands {
    /// Delete an access rule. An access rule that is still in use by an
    /// application credential cannot be deleted.
    #[command(about = "Delete access rule")]
    Delete(delete::AccessRuleArgs),
    /// List all access rules for a user.
    #[command(about = "List access rules")]
    List(list::AccessRulesArgs),
    /// Show details of an access rule.
    #[command(about = "Show access rule details")]
    Show(show::AccessRuleArgs),
}

pub struct AccessRuleCommand {
    pub args: AccessRuleArgs,
}

impl OSCCommand for AccessRuleCommand {
    fn get_subcommand(
        &self,
        _: &mut AsyncOpenStack,
    ) -> Result<Box<dyn OSCCommand + Send + Sync>, OpenStackCliError> {
        match &self.args.command {
            AccessRuleCommands::Delete(args) => {
                Ok(Box::new(delete::AccessRuleCmd { args: args.clone() }))
            }
            AccessRuleCommands::List(args) => {
                Ok(Box::new(list::AccessRulesCmd { args: args.clone() }))
            }
            AccessRuleCommands::Show(args) => {
                Ok(Box::new(show::AccessRuleCmd { args: args.clone() }))
            }
        }
    }
}
