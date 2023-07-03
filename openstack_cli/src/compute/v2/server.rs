use clap::{Args, Subcommand};

use crate::{Command, ResourceCommands};

mod list;
mod show;

#[derive(Args, Clone)]
#[command(args_conflicts_with_subcommands = true)]
pub struct ServerArgs {
    #[command(subcommand)]
    command: ServerCommands,
}

#[derive(Subcommand, Clone)]
pub enum ServerCommands {
    /// List Servers
    List(list::ServersArgs),
    /// Show single Server
    Show(show::ServerArgs),
}

pub struct ServerCommand {
    pub args: ServerArgs,
}

impl ResourceCommands for ServerCommand {
    fn get_command(&self) -> Box<dyn Command> {
        match &self.args.command {
            ServerCommands::List(args) => Box::new(list::ServersCmd { args: args.clone() }),
            ServerCommands::Show(args) => Box::new(show::ServerCmd { args: args.clone() }),
        }
    }
}
