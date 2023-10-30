pub mod availability_zone;
pub mod network;
pub mod port;
pub mod router;
pub mod subnet;

use clap::{Args, Subcommand};

use crate::network::v2::availability_zone::{AvailabilityZoneArgs, AvailabilityZoneCommand};
use crate::network::v2::network::{NetworkArgs, NetworkCommand};
use crate::network::v2::port::{PortArgs, PortCommand};
use crate::network::v2::router::{RouterArgs, RouterCommand};
use crate::network::v2::subnet::{SubnetArgs, SubnetCommand};
use crate::{Command, ResourceCommands, ServiceCommands};

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
    AvailabilityZone(AvailabilityZoneArgs),
    /// Network commands
    Network(NetworkArgs),
    /// Port commands
    Port(PortArgs),
    /// Router commands
    Router(RouterArgs),
    /// Subnet commands
    Subnet(SubnetArgs),
}

pub struct NetworkSrvCommand {
    pub args: NetworkSrvArgs,
}

impl ServiceCommands for NetworkSrvCommand {
    fn get_command(&self) -> Box<dyn Command> {
        match &self.args.command {
            NetworkSrvCommands::AvailabilityZone(args) => {
                AvailabilityZoneCommand { args: args.clone() }.get_command()
            }
            NetworkSrvCommands::Network(args) => {
                NetworkCommand { args: args.clone() }.get_command()
            }
            NetworkSrvCommands::Port(args) => PortCommand { args: args.clone() }.get_command(),
            NetworkSrvCommands::Router(args) => RouterCommand { args: args.clone() }.get_command(),
            NetworkSrvCommands::Subnet(args) => SubnetCommand { args: args.clone() }.get_command(),
        }
    }
}
