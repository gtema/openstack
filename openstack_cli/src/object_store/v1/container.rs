use clap::{Args, Subcommand};

use crate::{OSCCommand, OpenStackCliError};

use openstack_sdk::AsyncOpenStack;

mod create;
mod delete;
mod list;
mod set;
mod show;

#[derive(Args, Clone, Debug)]
// #[command(args_conflicts_with_subcommands = true)]
pub struct ContainerArgs {
    #[command(subcommand)]
    command: ContainerCommands,
}

#[derive(Subcommand, Clone, Debug)]
pub enum ContainerCommands {
    List(list::ContainersArgs),
    Show(show::ContainerArgs),
    Set(set::ContainerArgs),
    Create(create::ContainerArgs),
    Delete(delete::ContainerArgs),
}

pub struct ContainerCommand {
    pub args: ContainerArgs,
}

impl OSCCommand for ContainerCommand {
    fn get_subcommand(
        &self,
        _: &mut AsyncOpenStack,
    ) -> Result<Box<dyn OSCCommand + Send + Sync>, OpenStackCliError> {
        match &self.args.command {
            ContainerCommands::List(args) => {
                Ok(Box::new(list::ContainersCmd { args: args.clone() }))
            }
            ContainerCommands::Set(args) => Ok(Box::new(set::ContainerCmd { args: args.clone() })),
            ContainerCommands::Show(args) => {
                Ok(Box::new(show::ContainerCmd { args: args.clone() }))
            }
            ContainerCommands::Create(args) => {
                Ok(Box::new(create::ContainerCmd { args: args.clone() }))
            }
            ContainerCommands::Delete(args) => {
                Ok(Box::new(delete::ContainerCmd { args: args.clone() }))
            }
        }
    }
}
