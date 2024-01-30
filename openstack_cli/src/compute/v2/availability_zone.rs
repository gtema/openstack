use clap::{Args, Subcommand};

use crate::{OSCCommand, OpenStackCliError};

use openstack_sdk::AsyncOpenStack;

mod list;

#[derive(Args, Clone, Debug)]
// #[command(args_conflicts_with_subcommands = true)]
pub struct AvailabilityZoneArgs {
    #[command(subcommand)]
    command: AvailabilityZoneCommands,
}

#[derive(Subcommand, Clone, Debug)]
pub enum AvailabilityZoneCommands {
    /// Gets detailed availability zone information. Policy defaults enable
    /// only users with the administrative role to perform this operation.
    /// Cloud providers can change these permissions through the policy.json
    /// file.
    #[command(about = "Get Detailed Availability Zone Information")]
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
