use clap::{Args, Subcommand};

use crate::{OSCCommand, OpenStackCliError};

use openstack_sdk::AsyncOpenStack;

pub mod create_26;
pub mod create_28;

/// Server Consoles
///
/// Manage server consoles.
#[derive(Args, Clone)]
#[command(args_conflicts_with_subcommands = true)]
pub struct RemoteConsoleArgs {
    #[command(subcommand)]
    command: RemoteConsoleCommands,
}

#[derive(Subcommand, Clone)]
pub enum RemoteConsoleCommands {
    Create26(create_26::RemoteConsoleArgs),
    #[command(visible_alias = "create")]
    Create28(create_28::RemoteConsoleArgs),
}

pub struct RemoteConsoleCommand {
    pub args: RemoteConsoleArgs,
}

impl OSCCommand for RemoteConsoleCommand {
    fn get_subcommand(
        &self,
        _: &mut AsyncOpenStack,
    ) -> Result<Box<dyn OSCCommand + Send + Sync>, OpenStackCliError> {
        match &self.args.command {
            RemoteConsoleCommands::Create26(args) => {
                Ok(Box::new(create_26::RemoteConsoleCmd { args: args.clone() }))
            }
            RemoteConsoleCommands::Create28(args) => {
                Ok(Box::new(create_28::RemoteConsoleCmd { args: args.clone() }))
            }
        }
    }
}
