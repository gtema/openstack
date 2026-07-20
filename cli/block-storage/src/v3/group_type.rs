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

//! Block storage Volume GroupType commands

use clap::{Parser, Subcommand};

use openstack_cli_core::{cli::CliArgs, error::OpenStackCliError};
use openstack_sdk::AsyncOpenStack;

pub mod create_311;
pub mod delete_311;
pub mod group_spec;
pub mod list_311;
pub mod set_311;
pub mod show_311;

/// Group types (group_types)
///
/// To create a generic volume group, you must specify a group type.
#[derive(Parser)]
pub struct GroupTypeCommand {
    /// subcommand
    #[command(subcommand)]
    command: GroupTypeCommands,
}

/// Supported subcommands
#[allow(missing_docs)]
#[derive(Subcommand)]
pub enum GroupTypeCommands {
    #[command(visible_alias = "create")]
    Create311(Box<create_311::GroupTypeCommand>),
    #[command(visible_alias = "delete")]
    Delete311(Box<delete_311::GroupTypeCommand>),
    GroupSpec(Box<group_spec::GroupSpecCommand>),
    #[command(visible_alias = "list")]
    List311(Box<list_311::GroupTypesCommand>),
    #[command(visible_alias = "set")]
    Set311(Box<set_311::GroupTypeCommand>),
    #[command(visible_alias = "show")]
    Show311(Box<show_311::GroupTypeCommand>),
}

impl GroupTypeCommand {
    /// Perform command action
    pub async fn take_action<C: CliArgs>(
        &self,
        parsed_args: &C,
        session: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        match &self.command {
            GroupTypeCommands::Create311(cmd) => cmd.take_action(parsed_args, session).await,
            GroupTypeCommands::Delete311(cmd) => cmd.take_action(parsed_args, session).await,
            GroupTypeCommands::GroupSpec(cmd) => cmd.take_action(parsed_args, session).await,
            GroupTypeCommands::List311(cmd) => cmd.take_action(parsed_args, session).await,
            GroupTypeCommands::Set311(cmd) => cmd.take_action(parsed_args, session).await,
            GroupTypeCommands::Show311(cmd) => cmd.take_action(parsed_args, session).await,
        }
    }
}
