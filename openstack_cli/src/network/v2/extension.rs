use clap::{Args, Subcommand};

use crate::{OSCCommand, OpenStackCliError};

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

impl OSCCommand for ExtensionCommand {
    fn get_subcommand(
        &self,
        session: &mut AsyncOpenStack,
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
