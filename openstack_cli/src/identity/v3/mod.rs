pub mod project;

use clap::{Args, Parser, Subcommand};

use openstack_sdk::AsyncOpenStack;

use crate::identity::v3::project::{ProjectArgs, ProjectCommand};
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
        }
    }
}
