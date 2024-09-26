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

//! L3Router resource commands

use clap::{Parser, Subcommand};

use openstack_sdk::AsyncOpenStack;

use crate::{Cli, OpenStackCliError};

mod create;
mod delete;
mod list;
mod set;
mod show;

/// L3 agent scheduler
///
/// The L3 agent scheduler extension (l3_agent_scheduler) allows administrators to assign Neutron routers to Neutron L3 agents, and retrieve mappings between Neutron routers and L3 agents.
#[derive(Parser)]
pub struct L3RouterCommand {
    /// subcommand
    #[command(subcommand)]
    command: L3RouterCommands,
}

/// Supported subcommands
#[allow(missing_docs)]
#[derive(Subcommand)]
pub enum L3RouterCommands {
    Create(Box<create::L3RouterCommand>),
    Delete(Box<delete::L3RouterCommand>),
    List(Box<list::L3RoutersCommand>),
    Set(Box<set::L3RouterCommand>),
    Show(Box<show::L3RouterCommand>),
}

impl L3RouterCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        session: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        match &self.command {
            L3RouterCommands::Create(cmd) => cmd.take_action(parsed_args, session).await,
            L3RouterCommands::Delete(cmd) => cmd.take_action(parsed_args, session).await,
            L3RouterCommands::List(cmd) => cmd.take_action(parsed_args, session).await,
            L3RouterCommands::Set(cmd) => cmd.take_action(parsed_args, session).await,
            L3RouterCommands::Show(cmd) => cmd.take_action(parsed_args, session).await,
        }
    }
}
