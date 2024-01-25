pub mod extension;
pub mod flavor;
pub mod os_keypair;
pub mod server;

use clap::{Args, Parser, Subcommand};

use openstack_sdk::AsyncOpenStack;

use crate::compute::v2::extension::{ExtensionArgs, ExtensionCommand};
use crate::compute::v2::flavor::{FlavorArgs, FlavorCommand};
use crate::compute::v2::os_keypair::{KeypairArgs, KeypairCommand};
use crate::compute::v2::server::{ServerArgs, ServerCommand};
use crate::{Command, ResourceCommands, ServiceCommands};

/// Compute service (Nova) arguments
#[derive(Args, Clone)]
#[command(args_conflicts_with_subcommands = true)]
pub struct ComputeSrvArgs {
    // /// Compute API microversion
    // #[arg(long, env = "OS_COMPUTE_API_VERSION")]
    // os_compute_api_version: Option<String>,
    /// Compute service resource
    #[command(subcommand)]
    command: ComputeSrvCommands,
}

/// Compute resources commands
#[derive(Clone, Subcommand)]
pub enum ComputeSrvCommands {
    /// Extension commands
    Extension(Box<ExtensionArgs>),
    /// Server (VM) commands
    Server(Box<ServerArgs>),
    /// Flavor commands
    ///
    /// Flavors are a way to describe the basic dimensions of a server to be
    /// created including how much cpu, ram, and disk space are allocated to a
    /// server built with this flavor.
    Flavor(Box<FlavorArgs>),
    /// Keypair commands
    Keypair(Box<KeypairArgs>),
}

pub struct ComputeSrvCommand {
    pub args: ComputeSrvArgs,
}

impl ServiceCommands for ComputeSrvCommand {
    fn get_command(&self, session: &mut AsyncOpenStack) -> Box<dyn Command> {
        match &self.args.command {
            ComputeSrvCommands::Extension(args) => ExtensionCommand {
                args: *args.clone(),
            }
            .get_command(session),
            ComputeSrvCommands::Server(args) => ServerCommand {
                args: *args.clone(),
            }
            .get_command(session),
            ComputeSrvCommands::Flavor(args) => FlavorCommand {
                args: *args.clone(),
            }
            .get_command(session),
            ComputeSrvCommands::Keypair(args) => KeypairCommand {
                args: *args.clone(),
            }
            .get_command(session),
        }
    }
}
