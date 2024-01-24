//! Identity User Access Credentials commands
//!
use clap::error::{Error, ErrorKind};
use clap::{ArgMatches, Args, Command as ClapCommand, FromArgMatches, Subcommand};

use crate::common::ServiceApiVersion;
use crate::{Command, ResourceCommands};

use openstack_sdk::{types::ServiceType, AsyncOpenStack};

mod create;
mod delete;
mod list;
mod show;

/// Identity Application Credentials
#[derive(Args, Clone, Debug)]
// #[command(args_conflicts_with_subcommands = true)]
pub struct ApplicationCredentialArgs {
    #[command(subcommand)]
    command: ApplicationCredentialCommands,
}

#[derive(Subcommand, Clone, Debug)]
pub enum ApplicationCredentialCommands {
    /// Creates an application credential for a user on
    /// the project to which the current token is scoped.
    #[command(about = "Create application credential")]
    Create(create::ApplicationCredentialArgs),
    /// Delete an application credential.
    #[command(about = "Delete application credential")]
    Delete(delete::ApplicationCredentialArgs),
    /// List all application credentials for a user.
    #[command(about = "List application credentials")]
    List(list::ApplicationCredentialsArgs),
    /// Show details of an application credential.
    #[command(about = "Show application credential details")]
    Show(show::ApplicationCredentialArgs),
}

pub struct ApplicationCredentialCommand {
    pub args: ApplicationCredentialArgs,
}

impl ResourceCommands for ApplicationCredentialCommand {
    fn get_command(&self, session: &mut AsyncOpenStack) -> Box<dyn Command> {
        match &self.args.command {
            ApplicationCredentialCommands::Create(args) => {
                Box::new(create::ApplicationCredentialCmd { args: args.clone() })
            }
            ApplicationCredentialCommands::Delete(args) => {
                Box::new(delete::ApplicationCredentialCmd { args: args.clone() })
            }
            ApplicationCredentialCommands::List(args) => {
                Box::new(list::ApplicationCredentialsCmd { args: args.clone() })
            }
            ApplicationCredentialCommands::Show(args) => {
                Box::new(show::ApplicationCredentialCmd { args: args.clone() })
            }
        }
    }
}
