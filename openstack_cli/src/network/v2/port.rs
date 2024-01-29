use clap::{Args, Subcommand};

use crate::{OSCCommand, OpenStackCliError};

use openstack_sdk::AsyncOpenStack;

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
    List(Box<list::PortsArgs>),
    /// Show single Port
    Show(Box<show::PortArgs>),
    /// Create single Port
    Create(Box<create::PortArgs>),
    /// Delete single Port
    Delete(delete::PortArgs),
}

pub struct PortCommand {
    pub args: PortArgs,
}

impl OSCCommand for PortCommand {
    fn get_subcommand(
        &self,
        _: &mut AsyncOpenStack,
    ) -> Result<Box<dyn OSCCommand + Send + Sync>, OpenStackCliError> {
        match &self.args.command {
            PortCommands::List(args) => Ok(Box::new(list::PortsCmd {
                args: *args.clone(),
            })),
            PortCommands::Show(args) => Ok(Box::new(show::PortCmd {
                args: *args.clone(),
            })),
            PortCommands::Create(args) => Ok(Box::new(create::PortCmd {
                args: *args.clone(),
            })),
            PortCommands::Delete(args) => Ok(Box::new(delete::PortCmd { args: args.clone() })),
        }
    }
}
