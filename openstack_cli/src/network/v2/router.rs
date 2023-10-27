use clap::{Args, Subcommand};

use crate::{Command, ResourceCommands};

mod create;
mod delete;
mod list;
mod show;

#[derive(Args, Clone)]
#[command(args_conflicts_with_subcommands = true)]
pub struct RouterArgs {
    #[command(subcommand)]
    command: RouterCommands,
}

#[derive(Subcommand, Clone)]
pub enum RouterCommands {
    /// List Routers
    List(list::RoutersArgs),
    /// Show single Router
    Show(show::RouterArgs),
    /// Create single Router
    Create(create::RouterArgs),
    /// Delete single Router
    Delete(delete::RouterArgs),
}

pub struct RouterCommand {
    pub args: RouterArgs,
}

impl ResourceCommands for RouterCommand {
    fn get_command(&self) -> Box<dyn Command> {
        match &self.args.command {
            RouterCommands::List(args) => Box::new(list::RoutersCmd { args: args.clone() }),
            RouterCommands::Show(args) => Box::new(show::RouterCmd { args: args.clone() }),
            RouterCommands::Create(args) => Box::new(create::RouterCmd { args: args.clone() }),
            RouterCommands::Delete(args) => Box::new(delete::RouterCmd { args: args.clone() }),
        }
    }
}
