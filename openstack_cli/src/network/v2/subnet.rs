use clap::{Args, Subcommand};

use crate::{OSCCommand, OpenStackCliError};

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

impl OSCCommand for SubnetCommand {
    fn get_subcommand(
        &self,
        _: &mut AsyncOpenStack,
    ) -> Result<Box<dyn OSCCommand + Send + Sync>, OpenStackCliError> {
        match &self.args.command {
            SubnetCommands::List(args) => Ok(Box::new(list::SubnetsCmd {
                args: *args.clone(),
            })),
            SubnetCommands::Show(args) => Ok(Box::new(show::SubnetCmd {
                args: *args.clone(),
            })),
            SubnetCommands::Create(args) => Ok(Box::new(create::SubnetCmd {
                args: *args.clone(),
            })),
            SubnetCommands::Delete(args) => Ok(Box::new(delete::SubnetCmd { args: args.clone() })),
        }
    }
}
