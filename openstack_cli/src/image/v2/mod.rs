pub mod image;
pub mod schema;

use clap::{Args, Subcommand};

use openstack_sdk::AsyncOpenStack;

use crate::image::v2::image::{ImageArgs, ImageCommand};
use crate::image::v2::schema::{SchemaArgs, SchemaCommand};
use crate::{OSCCommand, OpenStackCliError};

#[derive(Args, Clone)]
#[command(args_conflicts_with_subcommands = true)]
pub struct ImageSrvArgs {
    /// Image service resource
    #[command(subcommand)]
    command: ImageSrvCommands,
}

#[derive(Clone, Subcommand)]
pub enum ImageSrvCommands {
    /// Image commands
    Image(ImageArgs),
    /// Schema commands
    Schema(SchemaArgs),
}

pub struct ImageSrvCommand {
    pub args: ImageSrvArgs,
}

impl OSCCommand for ImageSrvCommand {
    fn get_subcommand(
        &self,
        session: &mut AsyncOpenStack,
    ) -> Result<Box<dyn OSCCommand + Send + Sync>, OpenStackCliError> {
        match &self.args.command {
            ImageSrvCommands::Image(args) => {
                ImageCommand { args: args.clone() }.get_subcommand(session)
            }
            ImageSrvCommands::Schema(args) => {
                SchemaCommand { args: args.clone() }.get_subcommand(session)
            }
        }
    }
}
