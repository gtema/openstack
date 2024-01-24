//! Identity Password password commands
//!
use clap::error::{Error, ErrorKind};
use clap::{ArgMatches, Args, Command as ClapCommand, FromArgMatches, Subcommand};

use crate::common::ServiceApiVersion;
use crate::{Command, ResourceCommands};

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

impl ResourceCommands for PasswordCommand {
    fn get_command(&self, session: &mut AsyncOpenStack) -> Box<dyn Command> {
        match &self.args.command {
            PasswordCommands::Set(args) => Box::new(set::PasswordCmd { args: args.clone() }),
        }
    }
}
