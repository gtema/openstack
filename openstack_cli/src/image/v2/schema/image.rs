use clap::{Args, Subcommand};

use crate::{Command, ResourceCommands};

mod show;

#[derive(Args, Clone)]
#[command(args_conflicts_with_subcommands = true)]
pub struct ImageArgs {
    #[command(subcommand)]
    command: ImageCommands,
}

#[derive(Subcommand, Clone)]
pub enum ImageCommands {
    /// Show Image Schema
    Show(show::ImageArgs),
}

pub struct ImageCommand {
    pub args: ImageArgs,
}

impl ResourceCommands for ImageCommand {
    fn get_command(&self) -> Box<dyn Command> {
        match &self.args.command {
            ImageCommands::Show(args) => Box::new(show::ImageCmd { args: args.clone() }),
        }
    }
}
