use clap::{Args, Subcommand};

use crate::{Command, ResourceCommands};

use openstack_sdk::AsyncOpenStack;

mod delete;
mod delete_all;
mod list;
mod replace;
mod set;
mod show;

#[derive(Args, Clone)]
#[command(args_conflicts_with_subcommands = true)]
pub struct TagArgs {
    #[command(subcommand)]
    command: TagCommands,
}

#[derive(Subcommand, Clone)]
pub enum TagCommands {
    /// Add a tag
    ///
    /// Adds a tag on the resource.
    Add(set::TagArgs),
    /// Confirm tag presence
    ///
    /// Confirms a given tag is set on the resource.
    /// This method does not return any reasonable
    /// response, but fails with "not found" when tag is
    /// not present.
    Check(show::TagArgs),
    /// Remove a single tag
    ///
    /// Removes a tag on the resource.
    Delete(delete::TagArgs),
    /// List all tags
    ///
    /// Obtains the tags for a resource.
    List(list::TagsArgs),
    /// Remove all tags
    ///
    /// Removes all tags on the resource.
    Purge(delete_all::TagArgs),
    /// Replace all tags
    ///
    /// Replaces all tags on the resource.
    Replace(replace::TagArgs),
}

pub struct TagCommand {
    pub args: TagArgs,
}

impl ResourceCommands for TagCommand {
    fn get_command(&self, _: &mut AsyncOpenStack) -> Box<dyn Command> {
        match &self.args.command {
            TagCommands::Add(args) => Box::new(set::TagCmd { args: args.clone() }),
            TagCommands::Check(args) => Box::new(show::TagCmd { args: args.clone() }),
            TagCommands::Delete(args) => Box::new(delete::TagCmd { args: args.clone() }),
            TagCommands::List(args) => Box::new(list::TagsCmd { args: args.clone() }),
            TagCommands::Purge(args) => Box::new(delete_all::TagCmd { args: args.clone() }),
            TagCommands::Replace(args) => Box::new(replace::TagCmd { args: args.clone() }),
        }
    }
}
