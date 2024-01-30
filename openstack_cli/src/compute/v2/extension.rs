use clap::{Args, Subcommand};

use crate::{OSCCommand, OpenStackCliError};

use openstack_sdk::AsyncOpenStack;

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

impl OSCCommand for ExtensionCommand {
    fn get_subcommand(
        &self,
        _: &mut AsyncOpenStack,
    ) -> Result<Box<dyn OSCCommand + Send + Sync>, OpenStackCliError> {
        match &self.args.command {
            ExtensionCommands::List(args) => {
                Ok(Box::new(list::ExtensionsCmd { args: args.clone() }))
            }
            ExtensionCommands::Show(args) => {
                Ok(Box::new(show::ExtensionCmd { args: args.clone() }))
            }
        }
    }
}
