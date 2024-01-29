use clap::{Args, Subcommand};

use crate::{OSCCommand, OpenStackCliError};

use openstack_sdk::AsyncOpenStack;

mod get;

#[derive(Args, Clone)]
#[command(args_conflicts_with_subcommands = true)]
pub struct ImagesArgs {
    #[command(subcommand)]
    command: ImagesCommands,
}

#[derive(Subcommand, Clone)]
pub enum ImagesCommands {
    /// Show Images Schema
    Show(get::ImagesArgs),
}

pub struct ImagesCommand {
    pub args: ImagesArgs,
}

impl OSCCommand for ImagesCommand {
    fn get_subcommand(
        &self,
        _: &mut AsyncOpenStack,
    ) -> Result<Box<dyn OSCCommand + Send + Sync>, OpenStackCliError> {
        match &self.args.command {
            ImagesCommands::Show(args) => Ok(Box::new(get::ImagesCmd { args: args.clone() })),
        }
    }
}
