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

//! DHCP Network Agent commands

use clap::{Parser, Subcommand};

use openstack_sdk::AsyncOpenStack;

use crate::{Cli, OpenStackCliError};

mod create;
mod delete;
mod list;
mod set;
mod show;

/// DHCP agent scheduler
///
/// The DHCP agent scheduler extension (dhcp_agent_scheduler) enables administrators to assign DHCP
/// servers for Neutron networks to given Neutron DHCP agents, and retrieve mappings between
/// Neutron networks and DHCP agents.
#[derive(Parser)]
pub struct DhcpNetworkCommand {
    /// subcommand
    #[command(subcommand)]
    command: DhcpNetworkCommands,
}

/// Supported subcommands
#[allow(missing_docs)]
#[derive(Subcommand)]
pub enum DhcpNetworkCommands {
    Create(Box<create::DhcpNetworkCommand>),
    Delete(Box<delete::DhcpNetworkCommand>),
    List(Box<list::DhcpNetworksCommand>),
    Set(Box<set::DhcpNetworkCommand>),
    Show(Box<show::DhcpNetworkCommand>),
}

impl DhcpNetworkCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        session: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        match &self.command {
            DhcpNetworkCommands::Create(cmd) => cmd.take_action(parsed_args, session).await,
            DhcpNetworkCommands::Delete(cmd) => cmd.take_action(parsed_args, session).await,
            DhcpNetworkCommands::List(cmd) => cmd.take_action(parsed_args, session).await,
            DhcpNetworkCommands::Set(cmd) => cmd.take_action(parsed_args, session).await,
            DhcpNetworkCommands::Show(cmd) => cmd.take_action(parsed_args, session).await,
        }
    }
}
