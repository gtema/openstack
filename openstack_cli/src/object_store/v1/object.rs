use clap::{Args, Subcommand};

use crate::{OSCCommand, OpenStackCliError};

use openstack_sdk::AsyncOpenStack;

mod delete;
mod download;
mod list;
mod show;
mod upload;

#[derive(Args, Clone, Debug)]
// #[command(args_conflicts_with_subcommands = true)]
pub struct ObjectArgs {
    #[command(subcommand)]
    command: ObjectCommands,
}

#[derive(Subcommand, Clone, Debug)]
pub enum ObjectCommands {
    List(list::ObjectsArgs),
    Download(download::ObjectArgs),
    Upload(upload::ObjectArgs),
    Show(show::ObjectArgs),
    Delete(delete::ObjectArgs),
}

pub struct ObjectCommand {
    pub args: ObjectArgs,
}

impl OSCCommand for ObjectCommand {
    fn get_subcommand(
        &self,
        _: &mut AsyncOpenStack,
    ) -> Result<Box<dyn OSCCommand + Send + Sync>, OpenStackCliError> {
        match &self.args.command {
            ObjectCommands::List(args) => Ok(Box::new(list::ObjectsCmd { args: args.clone() })),
            ObjectCommands::Download(args) => {
                Ok(Box::new(download::ObjectCmd { args: args.clone() }))
            }
            ObjectCommands::Upload(args) => Ok(Box::new(upload::ObjectCmd { args: args.clone() })),
            ObjectCommands::Show(args) => Ok(Box::new(show::ObjectCmd { args: args.clone() })),
            ObjectCommands::Delete(args) => Ok(Box::new(delete::ObjectCmd { args: args.clone() })),
        }
    }
}
