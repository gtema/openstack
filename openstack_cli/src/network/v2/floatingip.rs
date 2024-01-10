use clap::{Args, Subcommand};

use crate::{Command, ResourceCommands};

use openstack_sdk::AsyncOpenStack;

mod create;
mod delete;
mod list;
mod set;
mod show;
mod tag;

#[derive(Args, Clone)]
#[command(args_conflicts_with_subcommands = true)]
pub struct FloatingIPArgs {
    #[command(subcommand)]
    command: FloatingIPCommands,
}

#[derive(Subcommand, Clone)]
pub enum FloatingIPCommands {
    /// Create single FloatingIP
    Create(create::FloatingipArgs),
    /// Delete single FloatingIP
    Delete(delete::FloatingipArgs),
    /// List FloatingIPs
    List(list::FloatingipsArgs),
    /// Update FloatingIP attributes
    Set(set::FloatingipArgs),
    /// Show single FloatingIP
    Show(show::FloatingipArgs),
    /// FloatingIP Tags management
    ///
    /// Shows details for, updates, and deletes tags.
    /// The maximum number of characters allowed in a tag
    /// is 60.
    Tag(tag::TagArgs),
}

pub struct FloatingIPCommand {
    pub args: FloatingIPArgs,
}

impl ResourceCommands for FloatingIPCommand {
    fn get_command(&self, session: &mut AsyncOpenStack) -> Box<dyn Command> {
        match &self.args.command {
            FloatingIPCommands::Create(args) => {
                Box::new(create::FloatingipCmd { args: args.clone() })
            }
            FloatingIPCommands::Delete(args) => {
                Box::new(delete::FloatingipCmd { args: args.clone() })
            }
            FloatingIPCommands::List(args) => Box::new(list::FloatingipsCmd { args: args.clone() }),
            FloatingIPCommands::Set(args) => Box::new(set::FloatingipCmd { args: args.clone() }),
            FloatingIPCommands::Show(args) => Box::new(show::FloatingipCmd { args: args.clone() }),
            FloatingIPCommands::Tag(args) => {
                tag::TagCommand { args: args.clone() }.get_command(session)
            }
        }
    }
}
