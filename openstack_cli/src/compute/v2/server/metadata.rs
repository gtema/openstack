//! Compute Server metadata commands
#![deny(missing_docs)]
use clap::{Args, Subcommand};

use crate::{OSCCommand, OpenStackCliError};

use openstack_sdk::AsyncOpenStack;

mod create;
mod delete;
mod list;
mod replace;
mod set;
mod show;

/// Lists metadata, creates or replaces one or more metadata items, and updates
/// one or more metadata items for a server.
///
/// Shows details for, creates or replaces, and updates a metadata item, by
/// key, for a server.
#[derive(Args, Clone)]
#[command(args_conflicts_with_subcommands = true)]
#[command(about = "Server metadata")]
pub struct MetadataArgs {
    #[command(subcommand)]
    command: MetadataCommands,
}

#[derive(Subcommand, Clone)]
pub enum MetadataCommands {
    Create(Box<create::MetadataArgs>),
    Delete(Box<delete::MetadataArgs>),
    List(Box<list::MetadatasArgs>),
    Replace(Box<replace::MetadataArgs>),
    Set(Box<set::MetadataArgs>),
    Show(Box<show::MetadataArgs>),
}

pub struct MetadataCommand {
    pub args: MetadataArgs,
}

impl OSCCommand for MetadataCommand {
    fn get_subcommand(
        &self,
        _session: &mut AsyncOpenStack,
    ) -> Result<Box<dyn OSCCommand + Send + Sync>, OpenStackCliError> {
        match &self.args.command {
            MetadataCommands::Create(args) => Ok(Box::new(create::MetadataCmd {
                args: *args.clone(),
            })),
            MetadataCommands::Delete(args) => Ok(Box::new(delete::MetadataCmd {
                args: *args.clone(),
            })),
            MetadataCommands::List(args) => Ok(Box::new(list::MetadatasCmd {
                args: *args.clone(),
            })),
            MetadataCommands::Replace(args) => Ok(Box::new(replace::MetadataCmd {
                args: *args.clone(),
            })),
            MetadataCommands::Set(args) => Ok(Box::new(set::MetadataCmd {
                args: *args.clone(),
            })),
            MetadataCommands::Show(args) => Ok(Box::new(show::MetadataCmd {
                args: *args.clone(),
            })),
        }
    }
}
