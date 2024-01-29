//! Identity Password password commands
//!
use clap::error::{Error, ErrorKind};
use clap::{ArgMatches, Args, Command as ClapCommand, FromArgMatches, Subcommand};

use crate::common::ServiceApiVersion;
use crate::{OSCCommand, OpenStackCliError};

use openstack_sdk::{types::ServiceType, AsyncOpenStack};

mod set;

/// Identity Password password commands
#[derive(Args, Clone, Debug)]
// #[command(args_conflicts_with_subcommands = true)]
pub struct PasswordArgs {
    #[command(subcommand)]
    command: PasswordCommands,
}

#[derive(Subcommand, Clone, Debug)]
pub enum PasswordCommands {
    /// Updates a user password
    #[command(about = "Update user password")]
    Set(set::PasswordArgs),
}

pub struct PasswordCommand {
    pub args: PasswordArgs,
}

impl OSCCommand for PasswordCommand {
    fn get_subcommand(
        &self,
        session: &mut AsyncOpenStack,
    ) -> Result<Box<dyn OSCCommand + Send + Sync>, OpenStackCliError> {
        match &self.args.command {
            PasswordCommands::Set(args) => Ok(Box::new(set::PasswordCmd { args: args.clone() })),
        }
    }
}
