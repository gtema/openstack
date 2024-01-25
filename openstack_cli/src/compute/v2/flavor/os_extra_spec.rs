//! Compute Flavor Extra Specs commands
use clap::{Args, Subcommand};

use crate::common::ServiceApiVersion;
use crate::{Command, ResourceCommands};

use openstack_sdk::AsyncOpenStack;

mod create;
mod delete;
mod list;
mod set;
mod show;

#[derive(Args, Clone, Debug)]
pub struct ExtraSpecsArgs {
    #[command(subcommand)]
    command: ExtraSpecsCommands,
}

#[derive(Subcommand, Clone, Debug)]
pub enum ExtraSpecsCommands {
    /// Creates extra specs for a flavor, by ID.
    #[command(about = "Create Extra Specs For A Flavor")]
    Create(create::OsExtraSpecArgs),
    /// Deletes an extra spec, by key, for a flavor, by ID.
    #[command(about = "Delete An Extra Spec For A Flavor")]
    Delete(delete::OsExtraSpecArgs),
    /// Lists all extra specs for a flavor, by ID.
    #[command(about = "List Extra Specs For A Flavor")]
    List(list::OsExtraSpecsArgs),
    /// Shows an extra spec, by key, for a flavor, by ID.
    #[command(about = "Show An Extra Spec For A Flavor")]
    Show(show::OsExtraSpecArgs),
    /// Updates an extra spec, by key, for a flavor, by ID.
    #[command(about = "Update An Extra Spec For A Flavor
")]
    Set(set::OsExtraSpecArgs),
}

pub struct ExtraSpecsCommand {
    pub args: ExtraSpecsArgs,
}

impl ResourceCommands for ExtraSpecsCommand {
    fn get_command(&self, session: &mut AsyncOpenStack) -> Box<dyn Command> {
        match &self.args.command {
            ExtraSpecsCommands::Create(args) => {
                Box::new(create::OsExtraSpecCmd { args: args.clone() })
            }
            ExtraSpecsCommands::Delete(args) => {
                Box::new(delete::OsExtraSpecCmd { args: args.clone() })
            }
            ExtraSpecsCommands::List(args) => {
                Box::new(list::OsExtraSpecsCmd { args: args.clone() })
            }
            ExtraSpecsCommands::Show(args) => Box::new(show::OsExtraSpecCmd { args: args.clone() }),
            ExtraSpecsCommands::Set(args) => Box::new(set::OsExtraSpecCmd { args: args.clone() }),
        }
    }
}
