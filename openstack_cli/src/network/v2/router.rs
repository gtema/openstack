use clap::{Args, Subcommand};

use crate::{OSCCommand, OpenStackCliError};

use openstack_sdk::AsyncOpenStack;

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

impl OSCCommand for RouterCommand {
    fn get_subcommand(
        &self,
        _: &mut AsyncOpenStack,
    ) -> Result<Box<dyn OSCCommand + Send + Sync>, OpenStackCliError> {
        match &self.args.command {
            RouterCommands::List(args) => Ok(Box::new(list::RoutersCmd { args: args.clone() })),
            RouterCommands::Show(args) => Ok(Box::new(show::RouterCmd { args: args.clone() })),
            RouterCommands::Create(args) => Ok(Box::new(create::RouterCmd { args: args.clone() })),
            RouterCommands::Delete(args) => Ok(Box::new(delete::RouterCmd { args: args.clone() })),
        }
    }
}
