pub mod volume;

use clap::{Args, Parser, Subcommand};

use openstack_sdk::AsyncOpenStack;

use crate::block_storage::v3::volume::{VolumeArgs, VolumeCommand};
use crate::{Command, ResourceCommands, ServiceCommands};

#[derive(Args, Clone)]
#[command(args_conflicts_with_subcommands = true)]
pub struct BlockStorageSrvArgs {
    /// BlockStorage API microversion
    #[arg(long, env = "OS_VOLUME_API_VERSION")]
    os_volume_api_version: Option<String>,
    /// BlockStorage service resource
    #[command(subcommand)]
    command: BlockStorageSrvCommands,
}

#[derive(Clone, Subcommand)]
pub enum BlockStorageSrvCommands {
    /// Volume commands
    Volume(VolumeArgs),
}

pub struct BlockStorageSrvCommand {
    pub args: BlockStorageSrvArgs,
}

impl ServiceCommands for BlockStorageSrvCommand {
    fn get_command(&self, session: &mut AsyncOpenStack) -> Box<dyn Command> {
        match &self.args.command {
            BlockStorageSrvCommands::Volume(args) => {
                VolumeCommand { args: args.clone() }.get_command(session)
            }
        }
    }
}
