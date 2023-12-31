use clap::{Args, Subcommand};

use crate::{Command, ResourceCommands};

use openstack_sdk::AsyncOpenStack;

mod show;

#[derive(Args, Clone)]
#[command(args_conflicts_with_subcommands = true)]
pub struct ImagesArgs {
    #[command(subcommand)]
    command: ImagesCommands,
}

#[derive(Subcommand, Clone)]
pub enum ImagesCommands {
    /// Show Images Schema
    Show(show::ImagesArgs),
}

pub struct ImagesCommand {
    pub args: ImagesArgs,
}

impl ResourceCommands for ImagesCommand {
    fn get_command(&self, _: &mut AsyncOpenStack) -> Box<dyn Command> {
        match &self.args.command {
            ImagesCommands::Show(args) => Box::new(show::ImagesCmd { args: args.clone() }),
        }
    }
}
