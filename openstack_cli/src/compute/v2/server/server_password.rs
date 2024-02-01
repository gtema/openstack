use clap::{Args, Subcommand};

use crate::{OSCCommand, OpenStackCliError};

use openstack_sdk::AsyncOpenStack;

pub mod delete;
pub mod get;

/// Servers password
///
/// Shows the encrypted administrative password. Also, clears the encrypted
/// administrative password for a server, which removes it from the metadata
/// server.
#[derive(Args, Clone)]
#[command(args_conflicts_with_subcommands = true)]
pub struct PasswordArgs {
    #[command(subcommand)]
    command: PasswordCommands,
}

#[derive(Subcommand, Clone)]
pub enum PasswordCommands {
    Delete(delete::ServerPasswordArgs),
    Show(get::ServerPasswordArgs),
}

pub struct PasswordCommand {
    pub args: PasswordArgs,
}

impl OSCCommand for PasswordCommand {
    fn get_subcommand(
        &self,
        _: &mut AsyncOpenStack,
    ) -> Result<Box<dyn OSCCommand + Send + Sync>, OpenStackCliError> {
        match &self.args.command {
            PasswordCommands::Delete(args) => {
                Ok(Box::new(delete::ServerPasswordCmd { args: args.clone() }))
            }
            PasswordCommands::Show(args) => {
                Ok(Box::new(get::ServerPasswordCmd { args: args.clone() }))
            }
        }
    }
}
