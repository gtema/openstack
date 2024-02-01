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

pub mod availability_zone;
pub mod extension;
pub mod floatingip;
pub mod network;
pub mod port;
pub mod router;
pub mod subnet;

use clap::{Args, Subcommand};

use openstack_sdk::AsyncOpenStack;

use crate::network::v2::availability_zone::{AvailabilityZoneArgs, AvailabilityZoneCommand};
use crate::network::v2::extension::{ExtensionArgs, ExtensionCommand};
use crate::network::v2::floatingip::{FloatingIPArgs, FloatingIPCommand};
use crate::network::v2::network::{NetworkArgs, NetworkCommand};
use crate::network::v2::port::{PortArgs, PortCommand};
use crate::network::v2::router::{RouterArgs, RouterCommand};
use crate::network::v2::subnet::{SubnetArgs, SubnetCommand};
use crate::{OSCCommand, OpenStackCliError};

/// Network (Neutron) commands
#[derive(Args, Clone)]
#[command(args_conflicts_with_subcommands = true)]
pub struct NetworkSrvArgs {
    /// Network service resource
    #[command(subcommand)]
    command: NetworkSrvCommands,
}

#[derive(Clone, Subcommand)]
pub enum NetworkSrvCommands {
    AvailabilityZone(Box<AvailabilityZoneArgs>),
    Extension(Box<ExtensionArgs>),
    FloatingIP(Box<FloatingIPArgs>),
    Network(Box<NetworkArgs>),
    Port(Box<PortArgs>),
    Router(Box<RouterArgs>),
    Subnet(Box<SubnetArgs>),
}

pub struct NetworkSrvCommand {
    pub args: NetworkSrvArgs,
}

impl OSCCommand for NetworkSrvCommand {
    fn get_subcommand(
        &self,
        session: &mut AsyncOpenStack,
    ) -> Result<Box<dyn OSCCommand + Send + Sync>, OpenStackCliError> {
        match &self.args.command {
            NetworkSrvCommands::AvailabilityZone(args) => AvailabilityZoneCommand {
                args: *args.clone(),
            }
            .get_subcommand(session),
            NetworkSrvCommands::Extension(args) => ExtensionCommand {
                args: *args.clone(),
            }
            .get_subcommand(session),
            NetworkSrvCommands::FloatingIP(args) => FloatingIPCommand {
                args: *args.clone(),
            }
            .get_subcommand(session),
            NetworkSrvCommands::Network(args) => NetworkCommand {
                args: *args.clone(),
            }
            .get_subcommand(session),
            NetworkSrvCommands::Port(args) => PortCommand {
                args: *args.clone(),
            }
            .get_subcommand(session),
            NetworkSrvCommands::Router(args) => RouterCommand {
                args: *args.clone(),
            }
            .get_subcommand(session),
            NetworkSrvCommands::Subnet(args) => SubnetCommand {
                args: *args.clone(),
            }
            .get_subcommand(session),
        }
    }
}
