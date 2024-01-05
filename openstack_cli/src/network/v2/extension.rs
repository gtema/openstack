use clap::{Args, Subcommand};

use crate::{Command, ResourceCommands};

use openstack_sdk::AsyncOpenStack;

mod list;
mod show;

#[derive(Args, Clone)]
#[command(args_conflicts_with_subcommands = true)]
pub struct ExtensionArgs {
    #[command(subcommand)]
    command: ExtensionCommands,
}

#[derive(Subcommand, Clone)]
pub enum ExtensionCommands {
    /// List Extensions
    List(list::ExtensionsArgs),
    /// show Extensions
    Show(show::ExtensionArgs),
}

pub struct ExtensionCommand {
    pub args: ExtensionArgs,
}

impl ResourceCommands for ExtensionCommand {
    fn get_command(&self, _: &mut AsyncOpenStack) -> Box<dyn Command> {
        match &self.args.command {
            ExtensionCommands::List(args) => Box::new(list::ExtensionsCmd { args: args.clone() }),
            ExtensionCommands::Show(args) => Box::new(show::ExtensionCmd { args: args.clone() }),
        }
    }
}
