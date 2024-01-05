use clap::{Args, Subcommand};

use crate::{Command, ResourceCommands};

use openstack_sdk::AsyncOpenStack;

mod list;
mod show;

#[derive(Args, Clone, Debug)]
// #[command(args_conflicts_with_subcommands = true)]
pub struct FlavorArgs {
    #[command(subcommand)]
    command: FlavorCommands,
}

#[derive(Subcommand, Clone, Debug)]
pub enum FlavorCommands {
    /// List Servers
    List(list::FlavorsArgs),
    /// Show single Server
    Show(show::FlavorArgs),
}

pub struct FlavorCommand {
    pub args: FlavorArgs,
}

impl ResourceCommands for FlavorCommand {
    fn get_command(&self, _: &mut AsyncOpenStack) -> Box<dyn Command> {
        match &self.args.command {
            FlavorCommands::List(args) => Box::new(list::FlavorsCmd { args: args.clone() }),
            FlavorCommands::Show(args) => Box::new(show::FlavorCmd { args: args.clone() }),
        }
    }
}
