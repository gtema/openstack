//! Authorization operations
//!
//!
mod show;
use clap::{Args, Parser, Subcommand};

use openstack_sdk::AsyncOpenStack;

use crate::{Command, ResourceCommands, ServiceCommands};

#[derive(Args, Clone)]
#[command(args_conflicts_with_subcommands = true)]
pub struct AuthArgs {
    /// Authentication commands
    #[command(subcommand)]
    command: AuthCommands,
}

#[derive(Clone, Subcommand)]
pub enum AuthCommands {
    /// Show current authorization information for the cloud
    ///
    /// This command returns authentication and
    /// authorization information for the currently
    /// active connection. It includes issue and
    /// expiration information, user data, list of
    /// granted roles and project/domain information.
    ///
    /// **NOTE**: The command does not support selecting
    /// individual fields in the output, but it supports
    /// `-o json` command and returns full available
    /// information in json format what allows further
    /// processing with `jq`
    #[command(about = "Show current auth information")]
    Show(show::AuthArgs),
}

pub struct AuthCommand {
    pub args: AuthArgs,
}

impl ServiceCommands for AuthCommand {
    fn get_command(&self, session: &mut AsyncOpenStack) -> Box<dyn Command> {
        match &self.args.command {
            AuthCommands::Show(args) => Box::new(show::AuthCmd { args: args.clone() }),
        }
    }
}
