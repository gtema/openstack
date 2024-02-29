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

//! Identity Project User commands

use clap::{Parser, Subcommand};

use openstack_sdk::AsyncOpenStack;

use crate::{Cli, OpenStackCliError};

mod delete;
mod list;
mod set;
mod show;

/// Identity Project User Role commands
///
/// This command allows managing of the user roles on the `project`
#[derive(Parser)]
pub struct RoleCommand {
    /// subcommand
    #[command(subcommand)]
    command: RoleCommands,
}

/// Supported subcommands
#[allow(missing_docs)]
#[derive(Subcommand)]
pub enum RoleCommands {
    Delete(delete::RoleCommand),
    List(list::RolesCommand),
    Set(set::RoleCommand),
    Show(show::RoleCommand),
}

impl RoleCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        session: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        match &self.command {
            RoleCommands::Delete(cmd) => cmd.take_action(parsed_args, session).await,
            RoleCommands::List(cmd) => cmd.take_action(parsed_args, session).await,
            RoleCommands::Set(cmd) => cmd.take_action(parsed_args, session).await,
            RoleCommands::Show(cmd) => cmd.take_action(parsed_args, session).await,
        }
    }
}
