use clap::{Args, Subcommand};

use crate::{Command, ResourceCommands};

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

impl ResourceCommands for MembersCommand {
    fn get_command(&self, _: &mut AsyncOpenStack) -> Box<dyn Command> {
        match &self.args.command {
            MembersCommands::Show(args) => Box::new(get::MembersCmd { args: args.clone() }),
        }
    }
}
