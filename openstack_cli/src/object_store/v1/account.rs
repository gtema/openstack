use clap::{Args, Subcommand};

use crate::{Command, ResourceCommands};

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

impl ResourceCommands for AccountCommand {
    fn get_command(&self) -> Box<dyn Command> {
        match &self.args.command {
            AccountCommands::Show(args) => Box::new(show::AccountCmd { args: args.clone() }),
            AccountCommands::Set(args) => Box::new(set::AccountCmd { args: args.clone() }),
        }
    }
}
