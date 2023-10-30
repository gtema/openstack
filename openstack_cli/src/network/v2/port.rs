use clap::{Args, Subcommand};

use crate::{Command, ResourceCommands};

mod create;
mod delete;
mod list;
mod show;

#[derive(Args, Clone)]
#[command(args_conflicts_with_subcommands = true)]
pub struct PortArgs {
    #[command(subcommand)]
    command: PortCommands,
}

#[derive(Subcommand, Clone)]
pub enum PortCommands {
    /// List Ports
    List(list::PortsArgs),
    /// Show single Port
    Show(show::PortArgs),
    /// Create single Port
    Create(create::PortArgs),
    /// Delete single Port
    Delete(delete::PortArgs),
}

pub struct PortCommand {
    pub args: PortArgs,
}

impl ResourceCommands for PortCommand {
    fn get_command(&self) -> Box<dyn Command> {
        match &self.args.command {
            PortCommands::List(args) => Box::new(list::PortsCmd { args: args.clone() }),
            PortCommands::Show(args) => Box::new(show::PortCmd { args: args.clone() }),
            PortCommands::Create(args) => Box::new(create::PortCmd { args: args.clone() }),
            PortCommands::Delete(args) => Box::new(delete::PortCmd { args: args.clone() }),
        }
    }
}
