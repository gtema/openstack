use clap::{Args, Subcommand};

use crate::common::ServiceApiVersion;
use crate::OpenStackCliError;
use crate::{Command, ResourceCommands};

use openstack_sdk::{types::ServiceType, AsyncOpenStack};

pub mod add_host;
pub mod create_21;
pub mod delete;
pub mod list;
pub mod image {
    pub mod cache_281;
}
pub mod remove_host;
pub mod set_21;
pub mod set_metadata;
pub mod show;

#[derive(Args, Clone, Debug)]
// #[command(args_conflicts_with_subcommands = true)]
pub struct AggregateArgs {
    #[command(subcommand)]
    command: AggregateCommands,
}

#[derive(Subcommand, Clone, Debug)]
pub enum AggregateCommands {
    /// Adds a host to an aggregate.
    #[command(about = "Add Host")]
    AddHost(add_host::AggregateArgs),
    /// Creates an aggregate. If specifying an option availability_zone, the
    /// aggregate is created as an availability zone and the availability zone
    /// is visible to normal users.
    #[command(about = "Create Aggregate")]
    Create(create_21::AggregateArgs),
    /// Requests that a set of images be pre-cached on compute nodes within the
    /// referenced aggregate.
    #[command(about = "Request Image Pre-caching for Aggregate")]
    CacheImage(image::cache_281::ImageArgs),
    /// Deletes an aggregate.
    #[command(about = "Delete Aggregate")]
    Delete(delete::AggregateArgs),
    /// Lists all aggregates. Includes the ID, name, and availability zone for
    /// each aggregate.
    #[command(about = "List Aggregates")]
    List(list::AggregatesArgs),
    /// Removes a host from an aggregate.
    #[command(about = "Remove Host")]
    RemoveHost(remove_host::AggregateArgs),
    /// Shows details for an aggregate. Details include hosts and metadata.
    #[command(about = "Show Aggregate Details")]
    Show(show::AggregateArgs),
    /// Updates either or both the name and availability zone for an aggregate.
    /// If the aggregate to be updated has host that already in the given
    /// availability zone, the request will fail with 400 error.
    #[command(about = "Update Aggregate")]
    Set(set_21::AggregateArgs),
    /// Creates or replaces metadata for an aggregate.
    #[command(about = "Create Or Update Aggregate Metadata")]
    SetMetadata(set_metadata::AggregateArgs),
}

pub struct AggregateCommand {
    pub args: AggregateArgs,
}

impl ResourceCommands for AggregateCommand {
    fn get_command(&self, session: &mut AsyncOpenStack) -> Box<dyn Command> {
        match &self.args.command {
            AggregateCommands::AddHost(args) => {
                Box::new(add_host::AggregateCmd { args: args.clone() })
            }
            AggregateCommands::CacheImage(args) => {
                Box::new(image::cache_281::ImageCmd { args: args.clone() })
            }
            AggregateCommands::Create(args) => {
                Box::new(create_21::AggregateCmd { args: args.clone() })
            }
            AggregateCommands::Delete(args) => {
                Box::new(delete::AggregateCmd { args: args.clone() })
            }
            AggregateCommands::List(args) => Box::new(list::AggregatesCmd { args: args.clone() }),
            AggregateCommands::RemoveHost(args) => {
                Box::new(remove_host::AggregateCmd { args: args.clone() })
            }
            AggregateCommands::Show(args) => Box::new(show::AggregateCmd { args: args.clone() }),
            AggregateCommands::Set(args) => Box::new(set_21::AggregateCmd { args: args.clone() }),
            AggregateCommands::SetMetadata(args) => {
                Box::new(set_metadata::AggregateCmd { args: args.clone() })
            }
        }
    }
}
