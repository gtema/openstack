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

//! Agent resource commands

use clap::{Parser, Subcommand};

use openstack_sdk::AsyncOpenStack;

use crate::{Cli, OpenStackCliError};

mod create;
mod delete;
mod dhcp_network;
mod l3_router;
mod list;
mod set;
mod show;

/// Address scopes
///
/// Lists, creates, shows details for, updates, and deletes address scopes.
#[derive(Parser)]
pub struct AgentCommand {
    /// subcommand
    #[command(subcommand)]
    command: AgentCommands,
}

/// Supported subcommands
#[allow(missing_docs)]
#[derive(Subcommand)]
pub enum AgentCommands {
    Create(Box<create::AgentCommand>),
    Delete(Box<delete::AgentCommand>),
    DhcpNetwork(Box<dhcp_network::DhcpNetworkCommand>),
    L3Router(Box<l3_router::L3RouterCommand>),
    List(Box<list::AgentsCommand>),
    Set(Box<set::AgentCommand>),
    Show(Box<show::AgentCommand>),
}

impl AgentCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        session: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        match &self.command {
            AgentCommands::Create(cmd) => cmd.take_action(parsed_args, session).await,
            AgentCommands::Delete(cmd) => cmd.take_action(parsed_args, session).await,
            AgentCommands::DhcpNetwork(cmd) => cmd.take_action(parsed_args, session).await,
            AgentCommands::L3Router(cmd) => cmd.take_action(parsed_args, session).await,
            AgentCommands::List(cmd) => cmd.take_action(parsed_args, session).await,
            AgentCommands::Set(cmd) => cmd.take_action(parsed_args, session).await,
            AgentCommands::Show(cmd) => cmd.take_action(parsed_args, session).await,
        }
    }
}
