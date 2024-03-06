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

//! Networking v2 commands

use clap::{Parser, Subcommand};

use openstack_sdk::AsyncOpenStack;

use crate::{Cli, OpenStackCliError};

mod availability_zone;
mod extension;
mod floatingip;
mod network;
mod port;
mod router;
mod subnet;

/// Network (Neutron) commands
#[derive(Parser)]
pub struct NetworkCommand {
    /// Network service resource
    #[command(subcommand)]
    command: NetworkCommands,
}

/// Supported subcommands
#[allow(missing_docs)]
#[derive(Subcommand)]
pub enum NetworkCommands {
    AvailabilityZone(Box<availability_zone::AvailabilityZoneCommand>),
    Extension(Box<extension::ExtensionCommand>),
    FloatingIP(Box<floatingip::FloatingIPCommand>),
    Network(Box<network::NetworkCommand>),
    Port(Box<port::PortCommand>),
    Router(Box<router::RouterCommand>),
    Subnet(Box<subnet::SubnetCommand>),
}

impl NetworkCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        session: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        match &self.command {
            NetworkCommands::AvailabilityZone(cmd) => cmd.take_action(parsed_args, session).await,
            NetworkCommands::Extension(cmd) => cmd.take_action(parsed_args, session).await,
            NetworkCommands::FloatingIP(cmd) => cmd.take_action(parsed_args, session).await,
            NetworkCommands::Network(cmd) => cmd.take_action(parsed_args, session).await,
            NetworkCommands::Port(cmd) => cmd.take_action(parsed_args, session).await,
            NetworkCommands::Router(cmd) => cmd.take_action(parsed_args, session).await,
            NetworkCommands::Subnet(cmd) => cmd.take_action(parsed_args, session).await,
        }
    }
}
