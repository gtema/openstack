pub mod project;
pub mod user;

use clap::{Args, Parser, Subcommand};

use openstack_sdk::AsyncOpenStack;

use crate::identity::v3::project::{ProjectArgs, ProjectCommand};
use crate::identity::v3::user::{UserArgs, UserCommand};
use crate::{Command, ResourceCommands, ServiceCommands};

#[derive(Args, Clone)]
#[command(args_conflicts_with_subcommands = true)]
pub struct IdentitySrvArgs {
    /// Identity service resource
    #[command(subcommand)]
    command: IdentitySrvCommands,
}

#[derive(Clone, Subcommand)]
pub enum IdentitySrvCommands {
    /// Project commands
    Project(ProjectArgs),
    /// User commands
    ///
    /// A user is an individual API consumer that is owned by a domain. A
    /// role explicitly associates a user with projects or domains. A user
    /// with no assigned roles has no access to OpenStack resources.
    ///
    /// You can list, create, show details for, update, delete, and change
    /// the password for users.
    ///
    /// You can also list groups, projects, and role assignments for a
    /// specified user.
    User(UserArgs),
}

pub struct IdentitySrvCommand {
    pub args: IdentitySrvArgs,
}

impl ServiceCommands for IdentitySrvCommand {
    fn get_command(&self, session: &mut AsyncOpenStack) -> Box<dyn Command> {
        match &self.args.command {
            IdentitySrvCommands::Project(args) => {
                ProjectCommand { args: args.clone() }.get_command(session)
            }
            IdentitySrvCommands::User(args) => {
                UserCommand { args: args.clone() }.get_command(session)
            }
        }
    }
}
