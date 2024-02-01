//! Compute Server metadata commands
#![deny(missing_docs)]
use clap::{Args, Subcommand};

use crate::{OSCCommand, OpenStackCliError};

use openstack_sdk::AsyncOpenStack;

mod list;
mod show;

/// Servers actions
///
/// List actions and action details for a server.
#[derive(Args, Clone)]
#[command(args_conflicts_with_subcommands = true)]
pub struct InstanceActionArgs {
    #[command(subcommand)]
    command: InstanceActionCommands,
}

#[derive(Subcommand, Clone)]
pub enum InstanceActionCommands {
    List(Box<list::InstanceActionsArgs>),
    Show(Box<show::InstanceActionArgs>),
}

pub struct InstanceActionCommand {
    pub args: InstanceActionArgs,
}

impl OSCCommand for InstanceActionCommand {
    fn get_subcommand(
        &self,
        _session: &mut AsyncOpenStack,
    ) -> Result<Box<dyn OSCCommand + Send + Sync>, OpenStackCliError> {
        match &self.args.command {
            InstanceActionCommands::List(args) => Ok(Box::new(list::InstanceActionsCmd {
                args: *args.clone(),
            })),
            InstanceActionCommands::Show(args) => Ok(Box::new(show::InstanceActionCmd {
                args: *args.clone(),
            })),
        }
    }
}
