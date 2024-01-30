use clap::{Args, Subcommand};

use crate::{OSCCommand, OpenStackCliError};

use openstack_sdk::AsyncOpenStack;

mod get;

#[derive(Args, Clone)]
#[command(args_conflicts_with_subcommands = true)]
pub struct ImageArgs {
    #[command(subcommand)]
    command: ImageCommands,
}

#[derive(Subcommand, Clone)]
pub enum ImageCommands {
    /// Show Image Schema
    Show(get::ImageArgs),
}

pub struct ImageCommand {
    pub args: ImageArgs,
}

impl OSCCommand for ImageCommand {
    fn get_subcommand(
        &self,
        _: &mut AsyncOpenStack,
    ) -> Result<Box<dyn OSCCommand + Send + Sync>, OpenStackCliError> {
        match &self.args.command {
            ImageCommands::Show(args) => Ok(Box::new(get::ImageCmd { args: args.clone() })),
        }
    }
}
