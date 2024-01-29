use clap::{Args, Subcommand};

use crate::{OSCCommand, OpenStackCliError};

use openstack_sdk::AsyncOpenStack;

mod get;

#[derive(Args, Clone)]
#[command(args_conflicts_with_subcommands = true)]
pub struct MemberArgs {
    #[command(subcommand)]
    command: MemberCommands,
}

#[derive(Subcommand, Clone)]
pub enum MemberCommands {
    /// Show Member Schema
    Show(get::MemberArgs),
}

pub struct MemberCommand {
    pub args: MemberArgs,
}

impl OSCCommand for MemberCommand {
    fn get_subcommand(
        &self,
        _: &mut AsyncOpenStack,
    ) -> Result<Box<dyn OSCCommand + Send + Sync>, OpenStackCliError> {
        match &self.args.command {
            MemberCommands::Show(args) => Ok(Box::new(get::MemberCmd { args: args.clone() })),
        }
    }
}
