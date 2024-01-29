//! Identity Project commands
//!
use clap::error::{Error, ErrorKind};
use clap::{ArgMatches, Args, Command, FromArgMatches, Subcommand};

use crate::common::ServiceApiVersion;
use crate::{OSCCommand, OpenStackCliError};

use openstack_sdk::{types::ServiceType, AsyncOpenStack};

mod create;
mod delete;
mod list;
mod set;
mod show;

/// Identity Project commands
#[derive(Args, Clone, Debug)]
// #[command(args_conflicts_with_subcommands = true)]
pub struct ProjectArgs {
    #[command(subcommand)]
    command: ProjectCommands,
}

#[derive(Subcommand, Clone, Debug)]
pub enum ProjectCommands {
    /// Creates a project, where the project may act as a domain.
    #[command(about = "Create project")]
    Create(create::ProjectArgs),
    /// Deletes a project.
    #[command(about = "Delete project")]
    Delete(delete::ProjectArgs),
    /// Lists projects.
    #[command(about = "List Projects")]
    List(list::ProjectsArgs),
    /// Updates a project.
    #[command(about = "Update project details")]
    Set(set::ProjectArgs),
    /// Shows details for a project.
    #[command(about = "Show project details")]
    Show(show::ProjectArgs),
}

pub struct ProjectCommand {
    pub args: ProjectArgs,
}

impl OSCCommand for ProjectCommand {
    fn get_subcommand(
        &self,
        session: &mut AsyncOpenStack,
    ) -> Result<Box<dyn OSCCommand + Send + Sync>, OpenStackCliError> {
        match &self.args.command {
            ProjectCommands::Create(args) => {
                Ok(Box::new(create::ProjectCmd { args: args.clone() }))
            }
            ProjectCommands::Delete(args) => {
                Ok(Box::new(delete::ProjectCmd { args: args.clone() }))
            }
            ProjectCommands::List(args) => Ok(Box::new(list::ProjectsCmd { args: args.clone() })),
            ProjectCommands::Set(args) => Ok(Box::new(set::ProjectCmd { args: args.clone() })),
            ProjectCommands::Show(args) => Ok(Box::new(show::ProjectCmd { args: args.clone() })),
        }
    }
}
