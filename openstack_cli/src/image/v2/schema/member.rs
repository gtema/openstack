use clap::{Args, Subcommand};

use crate::{Command, ResourceCommands};

mod show;

#[derive(Args, Clone)]
#[command(args_conflicts_with_subcommands = true)]
pub struct MemberArgs {
    #[command(subcommand)]
    command: MemberCommands,
}

#[derive(Subcommand, Clone)]
pub enum MemberCommands {
    /// Show Member Schema
    Show(show::MemberArgs),
}

pub struct MemberCommand {
    pub args: MemberArgs,
}

impl ResourceCommands for MemberCommand {
    fn get_command(&self) -> Box<dyn Command> {
        match &self.args.command {
            MemberCommands::Show(args) => Box::new(show::MemberCmd { args: args.clone() }),
        }
    }
}
