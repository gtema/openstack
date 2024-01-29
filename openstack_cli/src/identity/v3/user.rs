//! Identity User commands
//!

use clap::{Args, Subcommand};

use crate::{OSCCommand, OpenStackCliError};

use openstack_sdk::AsyncOpenStack;

pub mod access_rule;
pub mod application_credential;
mod create;
mod delete;
mod list;
mod password;
mod set;
mod show;
mod project {
    pub(super) mod list;
}
mod group {
    pub(super) mod list;
}

/// Identity User commands
#[derive(Args, Clone, Debug)]
// #[command(args_conflicts_with_subcommands = true)]
pub struct UserArgs {
    #[command(subcommand)]
    command: UserCommands,
}

#[derive(Subcommand, Clone, Debug)]
pub enum UserCommands {
    /// Creates a user.
    #[command(about = "Create user")]
    Create(create::UserArgs),
    /// Deletes a user.
    #[command(about = "Delete user")]
    Delete(delete::UserArgs),
    /// Lists users.
    #[command(about = "List Users")]
    List(list::UsersArgs),
    /// Updates a user.
    #[command(about = "Update user details")]
    Set(set::UserArgs),
    /// Shows details for a user.
    #[command(about = "Show user details")]
    Show(show::UserArgs),
    /// User password commands
    ///
    /// This subcommand allows user to change the password
    #[command(about = "User password operations")]
    Password(password::PasswordArgs),
    /// List projects to which the user has authorization to access.
    #[command(about = "List projects for user")]
    Projects(project::list::ProjectsArgs),
    /// List groups to which a user belongs
    #[command(about = "List groups to which a user belongs")]
    Groups(group::list::GroupsArgs),
}

pub struct UserCommand {
    pub args: UserArgs,
}

impl OSCCommand for UserCommand {
    fn get_subcommand(
        &self,
        session: &mut AsyncOpenStack,
    ) -> Result<Box<dyn OSCCommand + Send + Sync>, OpenStackCliError> {
        match &self.args.command {
            UserCommands::Create(args) => Ok(Box::new(create::UserCmd { args: args.clone() })),
            UserCommands::Delete(args) => Ok(Box::new(delete::UserCmd { args: args.clone() })),
            UserCommands::List(args) => Ok(Box::new(list::UsersCmd { args: args.clone() })),
            UserCommands::Set(args) => Ok(Box::new(set::UserCmd { args: args.clone() })),
            UserCommands::Show(args) => Ok(Box::new(show::UserCmd { args: args.clone() })),
            UserCommands::Password(args) => {
                password::PasswordCommand { args: args.clone() }.get_subcommand(session)
            }
            UserCommands::Projects(args) => {
                Ok(Box::new(project::list::ProjectsCmd { args: args.clone() }))
            }
            UserCommands::Groups(args) => {
                Ok(Box::new(group::list::GroupsCmd { args: args.clone() }))
            }
        }
    }
}
