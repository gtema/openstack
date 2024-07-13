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

//! Object Store `container` command with subcommands
use clap::{Parser, Subcommand};

use openstack_sdk::AsyncOpenStack;

use crate::{Cli, OpenStackCliError};

mod create;
mod delete;
mod list;
mod prune;
mod set;
mod show;

/// Container commands
#[derive(Parser)]
pub struct ContainerCommand {
    #[command(subcommand)]
    command: ContainerCommands,
}

#[allow(missing_docs)]
#[derive(Subcommand)]
pub enum ContainerCommands {
    Create(create::ContainerCommand),
    Delete(delete::ContainerCommand),
    List(list::ContainersCommand),
    Prune(prune::ContainerCommand),
    Set(set::ContainerCommand),
    Show(show::ContainerCommand),
}

impl ContainerCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        session: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        match &self.command {
            ContainerCommands::Create(cmd) => cmd.take_action(parsed_args, session).await,
            ContainerCommands::Delete(cmd) => cmd.take_action(parsed_args, session).await,
            ContainerCommands::List(cmd) => cmd.take_action(parsed_args, session).await,
            ContainerCommands::Prune(cmd) => cmd.take_action(parsed_args, session).await,
            ContainerCommands::Set(cmd) => cmd.take_action(parsed_args, session).await,
            ContainerCommands::Show(cmd) => cmd.take_action(parsed_args, session).await,
        }
    }
}
