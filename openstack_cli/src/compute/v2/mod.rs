pub mod flavor;
pub mod keypair;
pub mod server;

use clap::{Args, Parser, Subcommand};

use crate::compute::v2::flavor::{FlavorArgs, FlavorCommand};
use crate::compute::v2::keypair::{KeypairArgs, KeypairCommand};
use crate::compute::v2::server::{ServerArgs, ServerCommand};
use crate::{Command, ResourceCommands, ServiceCommands};

#[derive(Args, Clone)]
#[command(args_conflicts_with_subcommands = true)]
pub struct ComputeSrvArgs {
    /// Compute API microversion
    #[arg(long, env = "OS_COMPUTE_API_VERSION")]
    os_compute_api_version: Option<String>,
    /// Compute service resource
    #[command(subcommand)]
    command: ComputeSrvCommands,
}

#[derive(Clone, Subcommand)]
pub enum ComputeSrvCommands {
    /// Server (VM) commands
    Server(ServerArgs),
    /// Flavor commands
    Flavor(FlavorArgs),
    /// Keypair commands
    Keypair(KeypairArgs),
}

pub struct ComputeSrvCommand {
    pub args: ComputeSrvArgs,
}

impl ServiceCommands for ComputeSrvCommand {
    fn get_command(&self) -> Box<dyn Command> {
        match &self.args.command {
            ComputeSrvCommands::Server(args) => ServerCommand { args: args.clone() }.get_command(),
            ComputeSrvCommands::Flavor(args) => FlavorCommand { args: args.clone() }.get_command(),
            ComputeSrvCommands::Keypair(args) => {
                KeypairCommand { args: args.clone() }.get_command()
            }
        }
    }
}
