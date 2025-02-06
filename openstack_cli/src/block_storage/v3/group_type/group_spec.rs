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

//! Group Type group_spec
use clap::{Parser, Subcommand};

use openstack_sdk::AsyncOpenStack;

use crate::{Cli, OpenStackCliError};

pub mod create_311;
pub mod delete;
pub mod list;
pub mod set_311;
pub mod show;

/// Group type specs (group_types, group_specs)
#[derive(Parser)]
#[command(about = "Server metadata")]
pub struct GroupSpecCommand {
    /// subcommand
    #[command(subcommand)]
    command: GroupSpecCommands,
}

/// Supported subcommands
#[allow(missing_docs)]
#[derive(Subcommand)]
pub enum GroupSpecCommands {
    #[command(visible_alias = "create")]
    Create311(Box<create_311::GroupSpecCommand>),
    Delete(Box<delete::GroupSpecCommand>),
    List(Box<list::GroupSpecsCommand>),
    #[command(visible_alias = "set")]
    Set311(Box<set_311::GroupSpecCommand>),
    Show(Box<show::GroupSpecCommand>),
}

impl GroupSpecCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        session: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        match &self.command {
            GroupSpecCommands::Create311(cmd) => cmd.take_action(parsed_args, session).await,
            GroupSpecCommands::Delete(cmd) => cmd.take_action(parsed_args, session).await,
            GroupSpecCommands::List(cmd) => cmd.take_action(parsed_args, session).await,
            GroupSpecCommands::Set311(cmd) => cmd.take_action(parsed_args, session).await,
            GroupSpecCommands::Show(cmd) => cmd.take_action(parsed_args, session).await,
        }
    }
}
