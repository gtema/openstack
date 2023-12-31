use clap::{Args, Subcommand};

use crate::{Command, ResourceCommands};

mod delete;
mod list;
mod set;
//mod show;

#[derive(Args, Clone)]
#[command(args_conflicts_with_subcommands = true)]
pub struct TagArgs {
    #[command(subcommand)]
    command: TagCommands,
}

#[derive(Subcommand, Clone)]
pub enum TagCommands {
    Delete(delete::TagArgs),
    List(list::TagsArgs),
    Set(set::TagArgs),
    //    Show(show::TagArgs),
}

pub struct TagCommand {
    pub args: TagArgs,
}

impl ResourceCommands for TagCommand {
    fn get_command(&self) -> Box<dyn Command> {
        match &self.args.command {
            TagCommands::Delete(args) => Box::new(delete::TagCmd { args: args.clone() }),
            TagCommands::List(args) => Box::new(list::TagsCmd { args: args.clone() }),
            TagCommands::Set(args) => Box::new(set::TagCmd { args: args.clone() }),
            //            TagCommands::Show(args) => Box::new(show::TagCmd { args: args.clone() }),
        }
    }
}
