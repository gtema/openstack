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

use crate::{Cli, OpenStackCliError};

use openstack_sdk::AsyncOpenStack;

pub mod create_311;
pub mod delete;
pub mod group_spec;
pub mod list;
pub mod set_311;
pub mod show;

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
    Delete(Box<delete::GroupTypeCommand>),
    GroupSpec(Box<group_spec::GroupSpecCommand>),
    List(Box<list::GroupTypesCommand>),
    #[command(visible_alias = "set")]
    Set311(Box<set_311::GroupTypeCommand>),
    Show(Box<show::GroupTypeCommand>),
}

impl GroupTypeCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        session: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        match &self.command {
            GroupTypeCommands::Create311(cmd) => cmd.take_action(parsed_args, session).await,
            GroupTypeCommands::Delete(cmd) => cmd.take_action(parsed_args, session).await,
            GroupTypeCommands::GroupSpec(cmd) => cmd.take_action(parsed_args, session).await,
            GroupTypeCommands::List(cmd) => cmd.take_action(parsed_args, session).await,
            GroupTypeCommands::Set311(cmd) => cmd.take_action(parsed_args, session).await,
            GroupTypeCommands::Show(cmd) => cmd.take_action(parsed_args, session).await,
        }
    }
}
