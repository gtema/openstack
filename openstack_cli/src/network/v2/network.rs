use clap::{Args, Subcommand};

use crate::{Command, ResourceCommands};

use openstack_sdk::AsyncOpenStack;

mod create;
mod delete;
mod list;
mod show;

#[derive(Args, Clone)]
#[command(args_conflicts_with_subcommands = true)]
pub struct NetworkArgs {
    #[command(subcommand)]
    command: NetworkCommands,
}

#[derive(Subcommand, Clone)]
pub enum NetworkCommands {
    /// List Networks
    List(Box<list::NetworksArgs>),
    /// Show single Network
    Show(Box<show::NetworkArgs>),
    /// Create single Network
    Create(Box<create::NetworkArgs>),
    /// Delete single Network
    Delete(delete::NetworkArgs),
}

pub struct NetworkCommand {
    pub args: NetworkArgs,
}

impl ResourceCommands for NetworkCommand {
    fn get_command(&self, _: &mut AsyncOpenStack) -> Box<dyn Command> {
        match &self.args.command {
            NetworkCommands::List(args) => Box::new(list::NetworksCmd {
                args: *args.clone(),
            }),
            NetworkCommands::Show(args) => Box::new(show::NetworkCmd {
                args: *args.clone(),
            }),
            NetworkCommands::Create(args) => Box::new(create::NetworkCmd {
                args: *args.clone(),
            }),
            NetworkCommands::Delete(args) => Box::new(delete::NetworkCmd { args: args.clone() }),
        }
    }
}
