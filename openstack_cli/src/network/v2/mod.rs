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

#[derive(Args, Clone)]
#[command(args_conflicts_with_subcommands = true)]
pub struct NetworkSrvArgs {
    /// Network service resource
    #[command(subcommand)]
    command: NetworkSrvCommands,
}

#[derive(Clone, Subcommand)]
pub enum NetworkSrvCommands {
    /// Availability Zones commands
    AvailabilityZone(Box<AvailabilityZoneArgs>),
    /// Extensions commands
    Extension(Box<ExtensionArgs>),
    /// Floating IP commands
    FloatingIP(Box<FloatingIPArgs>),
    /// Network commands
    Network(Box<NetworkArgs>),
    /// Port commands
    Port(Box<PortArgs>),
    /// Router commands
    Router(Box<RouterArgs>),
    /// Subnet commands
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
