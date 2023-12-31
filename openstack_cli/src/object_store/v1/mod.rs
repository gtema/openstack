pub mod account;
pub mod container;
pub mod object;

use clap::{Args, Subcommand};

use openstack_sdk::AsyncOpenStack;

use crate::object_store::v1::account::{AccountArgs, AccountCommand};
use crate::object_store::v1::container::{ContainerArgs, ContainerCommand};
use crate::object_store::v1::object::{ObjectArgs, ObjectCommand};
use crate::{Command, ResourceCommands, ServiceCommands};

#[derive(Args, Clone)]
#[command(args_conflicts_with_subcommands = true)]
pub struct ObjectStoreSrvArgs {
    /// Object store service resource
    #[command(subcommand)]
    command: ObjectStoreSrvCommands,
}

#[derive(Clone, Subcommand)]
pub enum ObjectStoreSrvCommands {
    /// Account commands
    Account(AccountArgs),
    /// Container commands
    Container(ContainerArgs),
    /// Object commands
    Object(ObjectArgs),
}

pub struct ObjectStoreSrvCommand {
    pub args: ObjectStoreSrvArgs,
}

impl ServiceCommands for ObjectStoreSrvCommand {
    fn get_command(&self, session: &mut AsyncOpenStack) -> Box<dyn Command> {
        match &self.args.command {
            ObjectStoreSrvCommands::Account(args) => {
                AccountCommand { args: args.clone() }.get_command(session)
            }
            ObjectStoreSrvCommands::Container(args) => {
                ContainerCommand { args: args.clone() }.get_command(session)
            }
            ObjectStoreSrvCommands::Object(args) => {
                ObjectCommand { args: args.clone() }.get_command(session)
            }
        }
    }
}
