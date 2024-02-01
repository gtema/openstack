use clap::{Args, Subcommand};

use crate::{OSCCommand, OpenStackCliError};

use openstack_sdk::AsyncOpenStack;

pub mod list;
pub mod show;

/// Servers IPs (servers, ips)
///
/// Lists the IP addresses for an instance and shows details for an IP address.
#[derive(Args, Clone)]
#[command(args_conflicts_with_subcommands = true)]
pub struct IpArgs {
    #[command(subcommand)]
    command: IpCommands,
}

#[derive(Subcommand, Clone)]
pub enum IpCommands {
    List(list::IpsArgs),
    Show(show::IpArgs),
}

pub struct IpCommand {
    pub args: IpArgs,
}

impl OSCCommand for IpCommand {
    fn get_subcommand(
        &self,
        _: &mut AsyncOpenStack,
    ) -> Result<Box<dyn OSCCommand + Send + Sync>, OpenStackCliError> {
        match &self.args.command {
            IpCommands::List(args) => Ok(Box::new(list::IpsCmd { args: args.clone() })),
            IpCommands::Show(args) => Ok(Box::new(show::IpCmd { args: args.clone() })),
        }
    }
}
