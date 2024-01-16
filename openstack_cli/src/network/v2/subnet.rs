use clap::{Args, Subcommand};

use crate::{Command, ResourceCommands};

use openstack_sdk::AsyncOpenStack;

mod create;
mod delete;
mod list;
mod show;

#[derive(Args, Clone)]
#[command(args_conflicts_with_subcommands = true)]
pub struct SubnetArgs {
    #[command(subcommand)]
    command: SubnetCommands,
}

#[derive(Subcommand, Clone)]
pub enum SubnetCommands {
    /// List Subnets
    List(Box<list::SubnetsArgs>),
    /// Show single Subnet
    Show(Box<show::SubnetArgs>),
    /// Create single Subnet
    Create(Box<create::SubnetArgs>),
    /// Delete single Subnet
    Delete(delete::SubnetArgs),
}

pub struct SubnetCommand {
    pub args: SubnetArgs,
}

impl ResourceCommands for SubnetCommand {
    fn get_command(&self, _: &mut AsyncOpenStack) -> Box<dyn Command> {
        match &self.args.command {
            SubnetCommands::List(args) => Box::new(list::SubnetsCmd {
                args: *args.clone(),
            }),
            SubnetCommands::Show(args) => Box::new(show::SubnetCmd {
                args: *args.clone(),
            }),
            SubnetCommands::Create(args) => Box::new(create::SubnetCmd {
                args: *args.clone(),
            }),
            SubnetCommands::Delete(args) => Box::new(delete::SubnetCmd { args: args.clone() }),
        }
    }
}
