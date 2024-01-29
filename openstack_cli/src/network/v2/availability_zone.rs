use clap::{Args, Subcommand};

use crate::{OSCCommand, OpenStackCliError};

use openstack_sdk::AsyncOpenStack;

mod list;

#[derive(Args, Clone)]
#[command(args_conflicts_with_subcommands = true)]
pub struct AvailabilityZoneArgs {
    #[command(subcommand)]
    command: AvailabilityZoneCommands,
}

#[derive(Subcommand, Clone)]
pub enum AvailabilityZoneCommands {
    /// List AvailabilityZones
    List(list::AvailabilityZonesArgs),
}

pub struct AvailabilityZoneCommand {
    pub args: AvailabilityZoneArgs,
}

impl OSCCommand for AvailabilityZoneCommand {
    fn get_subcommand(
        &self,
        _: &mut AsyncOpenStack,
    ) -> Result<Box<dyn OSCCommand + Send + Sync>, OpenStackCliError> {
        match &self.args.command {
            AvailabilityZoneCommands::List(args) => {
                Ok(Box::new(list::AvailabilityZonesCmd { args: args.clone() }))
            }
        }
    }
}
