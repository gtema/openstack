pub mod volume;

use clap::{Args, Subcommand};

use openstack_sdk::AsyncOpenStack;

use crate::block_storage::v3::volume::{VolumeArgs, VolumeCommand};
use crate::{OSCCommand, OpenStackCliError};

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

impl OSCCommand for BlockStorageSrvCommand {
    fn get_subcommand(
        &self,
        session: &mut AsyncOpenStack,
    ) -> Result<Box<dyn OSCCommand + Send + Sync>, OpenStackCliError> {
        match &self.args.command {
            BlockStorageSrvCommands::Volume(args) => {
                VolumeCommand { args: args.clone() }.get_subcommand(session)
            }
        }
    }
}
