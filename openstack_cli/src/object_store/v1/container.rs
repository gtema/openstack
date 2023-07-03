use clap::{Args, Subcommand};

use crate::{Command, ResourceCommands};

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

impl ResourceCommands for ContainerCommand {
    fn get_command(&self) -> Box<dyn Command> {
        match &self.args.command {
            ContainerCommands::List(args) => Box::new(list::ContainersCmd { args: args.clone() }),
            ContainerCommands::Set(args) => Box::new(set::ContainerCmd { args: args.clone() }),
            ContainerCommands::Show(args) => Box::new(show::ContainerCmd { args: args.clone() }),
            ContainerCommands::Create(args) => {
                Box::new(create::ContainerCmd { args: args.clone() })
            }
            ContainerCommands::Delete(args) => {
                Box::new(delete::ContainerCmd { args: args.clone() })
            }
        }
    }
}
