use clap::{Args, Subcommand};

use crate::{Command, ResourceCommands};

mod list;
mod show;

#[derive(Args, Clone, Debug)]
// #[command(args_conflicts_with_subcommands = true)]
pub struct ExtensionArgs {
    #[command(subcommand)]
    command: ExtensionCommands,
}

#[derive(Subcommand, Clone, Debug)]
pub enum ExtensionCommands {
    /// List Extensions
    List(list::ExtensionsArgs),
    /// Show single extension
    Show(show::ExtensionArgs),
}

pub struct ExtensionCommand {
    pub args: ExtensionArgs,
}

impl ResourceCommands for ExtensionCommand {
    fn get_command(&self) -> Box<dyn Command> {
        match &self.args.command {
            ExtensionCommands::List(args) => Box::new(list::ExtensionsCmd { args: args.clone() }),
            ExtensionCommands::Show(args) => Box::new(show::ExtensionCmd { args: args.clone() }),
        }
    }
}
