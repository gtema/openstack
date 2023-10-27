pub mod router;

use clap::{Args, Subcommand};

use crate::network::v2::router::{RouterArgs, RouterCommand};
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
    /// Router commands
    Router(RouterArgs),
}

pub struct NetworkSrvCommand {
    pub args: NetworkSrvArgs,
}

impl ServiceCommands for NetworkSrvCommand {
    fn get_command(&self) -> Box<dyn Command> {
        match &self.args.command {
            NetworkSrvCommands::Router(args) => RouterCommand { args: args.clone() }.get_command(),
        }
    }
}
