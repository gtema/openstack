use clap::{Args, Subcommand};

use crate::{Command, ResourceCommands};

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

impl ResourceCommands for HypervisorCommand {
    fn get_command(&self, _: &mut AsyncOpenStack) -> Box<dyn Command> {
        match &self.args.command {
            HypervisorCommands::List(args) => Box::new(list::HypervisorsCmd { args: args.clone() }),
            HypervisorCommands::Show(args) => Box::new(show::HypervisorCmd { args: args.clone() }),
        }
    }
}
