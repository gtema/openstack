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

//! LocalIP commands

use clap::{Parser, Subcommand};

use openstack_sdk::AsyncOpenStack;

use crate::{Cli, OpenStackCliError};

mod create;
mod delete;
mod list;
mod port_association;
mod set;
mod show;

/// Local IPs (local_ips)
///
/// Extension that allows users to create a virtual IP that can later be assigned to multiple
/// ports/VMs (similar to anycast IP) and is guaranteed to only be reachable within the same
/// physical server/node boundaries.
#[derive(Parser)]
pub struct LocalIPCommand {
    /// subcommand
    #[command(subcommand)]
    command: LocalIPCommands,
}

/// Supported subcommands
#[allow(missing_docs)]
#[derive(Subcommand)]
pub enum LocalIPCommands {
    Create(create::LocalIpCommand),
    Delete(delete::LocalIpCommand),
    List(list::LocalIpsCommand),
    PortAssociation(port_association::PortAssociationCommand),
    Set(set::LocalIpCommand),
    Show(show::LocalIpCommand),
}

impl LocalIPCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        session: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        match &self.command {
            LocalIPCommands::Create(cmd) => cmd.take_action(parsed_args, session).await,
            LocalIPCommands::Delete(cmd) => cmd.take_action(parsed_args, session).await,
            LocalIPCommands::List(cmd) => cmd.take_action(parsed_args, session).await,
            LocalIPCommands::PortAssociation(cmd) => cmd.take_action(parsed_args, session).await,
            LocalIPCommands::Set(cmd) => cmd.take_action(parsed_args, session).await,
            LocalIPCommands::Show(cmd) => cmd.take_action(parsed_args, session).await,
        }
    }
}
