use clap::{Args, Subcommand};

use crate::{OSCCommand, OpenStackCliError};

use openstack_sdk::AsyncOpenStack;

pub mod list;
pub mod show;

#[derive(Args, Clone, Debug)]
// #[command(args_conflicts_with_subcommands = true)]
pub struct HypervisorArgs {
    #[command(subcommand)]
    command: HypervisorCommands,
}

#[derive(Subcommand, Clone, Debug)]
pub enum HypervisorCommands {
    /// Lists hypervisors details.
    ///
    /// Policy defaults enable only users with the administrative role to
    /// perform this operation. Cloud providers can change these permissions
    /// through the policy.json file.
    #[command(about = "List Hypervisors Details")]
    List(list::HypervisorsArgs),
    /// Shows details for a given hypervisor.
    ///
    /// Policy defaults enable only users with the administrative role to
    /// perform this operation. Cloud providers can change these permissions
    /// through the policy.json file.
    ///
    /// **Note**
    ///
    /// As noted, some of the parameters in the response representing totals do
    /// not take allocation ratios into account. This can result in a disparity
    /// between the totals and the usages. A more accurate representation of
    /// state can be obtained using placement.
    #[command(about = "Show Hypervisor Details")]
    Show(show::HypervisorArgs),
}

pub struct HypervisorCommand {
    pub args: HypervisorArgs,
}

impl OSCCommand for HypervisorCommand {
    fn get_subcommand(
        &self,
        _: &mut AsyncOpenStack,
    ) -> Result<Box<dyn OSCCommand + Send + Sync>, OpenStackCliError> {
        match &self.args.command {
            HypervisorCommands::List(args) => {
                Ok(Box::new(list::HypervisorsCmd { args: args.clone() }))
            }
            HypervisorCommands::Show(args) => {
                Ok(Box::new(show::HypervisorCmd { args: args.clone() }))
            }
        }
    }
}
