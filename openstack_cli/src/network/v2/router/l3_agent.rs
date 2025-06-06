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

//! Router L3 agent scheduler module
use clap::{Parser, Subcommand};

use openstack_sdk::AsyncOpenStack;

use crate::{Cli, OpenStackCliError};

pub mod list;

/// L3 agent scheduler
///
/// The L3 agent scheduler extension (l3_agent_scheduler) allows administrators to assign Neutron
/// routers to Neutron L3 agents, and retrieve mappings between Neutron routers and L3 agents.
#[derive(Parser)]
pub struct L3AgentCommand {
    /// subcommand
    #[command(subcommand)]
    command: L3AgentCommands,
}

/// Supported subcommands
#[allow(missing_docs)]
#[derive(Subcommand)]
pub enum L3AgentCommands {
    List(list::L3AgentsCommand),
}

impl L3AgentCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        session: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        match &self.command {
            L3AgentCommands::List(cmd) => cmd.take_action(parsed_args, session).await,
        }
    }
}
