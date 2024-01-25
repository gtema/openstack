//! Compute Flavor Access commands
use clap::{Args, Subcommand};

use crate::common::ServiceApiVersion;
use crate::{Command, ResourceCommands};

use openstack_sdk::AsyncOpenStack;

use crate::compute::v2::flavor::add_tenant_access;
use crate::compute::v2::flavor::remove_tenant_access;

mod list;

#[derive(Args, Clone, Debug)]
pub struct FlavorAccessArgs {
    #[command(subcommand)]
    command: FlavorAccessCommands,
}

#[derive(Subcommand, Clone, Debug)]
pub enum FlavorAccessCommands {
    /// Adds flavor access to a tenant and flavor.
    #[command(about = "Add Flavor Access To Tenant (addTenantAccess Action)")]
    Add(add_tenant_access::FlavorArgs),
    /// Lists flavor access information.
    #[command(about = "List Flavor Access Information For Given Flavor")]
    List(list::OsFlavorAccesesArgs),
    /// Removes flavor access from a tenant and flavor.
    #[command(about = "Remove Flavor Access From Tenant (removeTenantAccess Action)")]
    Remove(remove_tenant_access::FlavorArgs),
}

pub struct FlavorAccessCommand {
    pub args: FlavorAccessArgs,
}

impl ResourceCommands for FlavorAccessCommand {
    fn get_command(&self, session: &mut AsyncOpenStack) -> Box<dyn Command> {
        match &self.args.command {
            FlavorAccessCommands::Add(args) => {
                Box::new(add_tenant_access::FlavorCmd { args: args.clone() })
            }
            FlavorAccessCommands::List(args) => {
                Box::new(list::OsFlavorAccesesCmd { args: args.clone() })
            }
            FlavorAccessCommands::Remove(args) => {
                Box::new(remove_tenant_access::FlavorCmd { args: args.clone() })
            }
        }
    }
}
