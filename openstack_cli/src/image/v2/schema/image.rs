use clap::{Args, Subcommand};

use crate::{Command, ResourceCommands};

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

impl ResourceCommands for ImageCommand {
    fn get_command(&self, _: &mut AsyncOpenStack) -> Box<dyn Command> {
        match &self.args.command {
            ImageCommands::Show(args) => Box::new(get::ImageCmd { args: args.clone() }),
        }
    }
}
