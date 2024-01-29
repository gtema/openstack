use clap::{Args, Subcommand};

use crate::{OSCCommand, OpenStackCliError};

use openstack_sdk::AsyncOpenStack;

mod get;

#[derive(Args, Clone)]
#[command(args_conflicts_with_subcommands = true)]
pub struct MembersArgs {
    #[command(subcommand)]
    command: MembersCommands,
}

#[derive(Subcommand, Clone)]
pub enum MembersCommands {
    /// Show Members Schema
    Show(get::MembersArgs),
}

pub struct MembersCommand {
    pub args: MembersArgs,
}

impl OSCCommand for MembersCommand {
    fn get_subcommand(
        &self,
        session: &mut AsyncOpenStack,
    ) -> Result<Box<dyn OSCCommand + Send + Sync>, OpenStackCliError> {
        match &self.args.command {
            MembersCommands::Show(args) => Ok(Box::new(get::MembersCmd { args: args.clone() })),
        }
    }
}
