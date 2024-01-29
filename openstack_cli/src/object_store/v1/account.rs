use clap::{Args, Subcommand};

use crate::{OSCCommand, OpenStackCliError};

use openstack_sdk::AsyncOpenStack;

mod set;
mod show;

#[derive(Args, Clone, Debug)]
pub struct AccountArgs {
    #[command(subcommand)]
    command: AccountCommands,
}

#[derive(Subcommand, Clone, Debug)]
pub enum AccountCommands {
    Show(show::AccountArgs),
    Set(set::AccountArgs),
}

pub struct AccountCommand {
    pub args: AccountArgs,
}

impl OSCCommand for AccountCommand {
    fn get_subcommand(
        &self,
        _: &mut AsyncOpenStack,
    ) -> Result<Box<dyn OSCCommand + Send + Sync>, OpenStackCliError> {
        match &self.args.command {
            AccountCommands::Show(args) => Ok(Box::new(show::AccountCmd { args: args.clone() })),
            AccountCommands::Set(args) => Ok(Box::new(set::AccountCmd { args: args.clone() })),
        }
    }
}
