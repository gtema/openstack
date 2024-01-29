use clap::{Args, Subcommand};

use crate::{OSCCommand, OpenStackCliError};

use openstack_sdk::AsyncOpenStack;

mod list;
mod pause;
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
    List(Box<list::ServersArgs>),
    /// Show single Server
    Show(Box<show::ServerArgs>),
    /// Pause Server
    Pause(pause::ServerArgs),
}

pub struct ServerCommand {
    pub args: ServerArgs,
}

impl OSCCommand for ServerCommand {
    fn get_subcommand(
        &self,
        _: &mut AsyncOpenStack,
    ) -> Result<Box<dyn OSCCommand + Send + Sync>, OpenStackCliError> {
        match &self.args.command {
            ServerCommands::List(args) => Ok(Box::new(list::ServersCmd {
                args: *args.clone(),
            })),
            ServerCommands::Show(args) => Ok(Box::new(show::ServerCmd {
                args: *args.clone(),
            })),
            ServerCommands::Pause(args) => Ok(Box::new(pause::ServerCmd { args: args.clone() })),
        }
    }
}
