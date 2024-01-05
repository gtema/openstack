use clap::{Args, Subcommand};

use crate::{Command, ResourceCommands};

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

impl ResourceCommands for AvailabilityZoneCommand {
    fn get_command(&self, _: &mut AsyncOpenStack) -> Box<dyn Command> {
        match &self.args.command {
            AvailabilityZoneCommands::List(args) => {
                Box::new(list::AvailabilityZonesCmd { args: args.clone() })
            }
        }
    }
}
