use clap::{Args, Subcommand};

use crate::{Command, ResourceCommands};

mod create;
mod delete;
mod list;
mod show;

#[derive(Args, Clone, Debug)]
// #[command(args_conflicts_with_subcommands = true)]
pub struct KeypairArgs {
    #[command(subcommand)]
    command: KeypairCommands,
}

#[derive(Subcommand, Clone, Debug)]
pub enum KeypairCommands {
    /// List Keypairs
    List(list::KeypairsArgs),
    /// Show single Keypair
    Show(show::KeypairArgs),
    /// Create Keypair
    Create(create::KeypairArgs),
    /// Delete Keypair
    Delete(delete::KeypairArgs),
}

pub struct KeypairCommand {
    pub args: KeypairArgs,
}

impl ResourceCommands for KeypairCommand {
    fn get_command(&self) -> Box<dyn Command> {
        match &self.args.command {
            KeypairCommands::List(args) => Box::new(list::KeypairsCmd { args: args.clone() }),
            KeypairCommands::Show(args) => Box::new(show::KeypairCmd { args: args.clone() }),
            KeypairCommands::Create(args) => Box::new(create::KeypairCmd { args: args.clone() }),
            KeypairCommands::Delete(args) => Box::new(delete::KeypairCmd { args: args.clone() }),
        }
    }
}
