use clap::{Args, Subcommand};

use crate::{Command, ResourceCommands};

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

impl ResourceCommands for ObjectCommand {
    fn get_command(&self) -> Box<dyn Command> {
        match &self.args.command {
            ObjectCommands::List(args) => Box::new(list::ObjectsCmd { args: args.clone() }),
            ObjectCommands::Download(args) => Box::new(download::ObjectCmd { args: args.clone() }),
            ObjectCommands::Upload(args) => Box::new(upload::ObjectCmd { args: args.clone() }),
            ObjectCommands::Show(args) => Box::new(show::ObjectCmd { args: args.clone() }),
            ObjectCommands::Delete(args) => Box::new(delete::ObjectCmd { args: args.clone() }),
        }
    }
}
