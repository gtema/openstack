//! Compute Flavor Extra Specs commands
use clap::{Args, Subcommand};

use crate::{OSCCommand, OpenStackCliError};

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
    Create(create::ExtraSpecArgs),
    /// Deletes an extra spec, by key, for a flavor, by ID.
    #[command(about = "Delete An Extra Spec For A Flavor")]
    Delete(delete::ExtraSpecArgs),
    /// Lists all extra specs for a flavor, by ID.
    #[command(about = "List Extra Specs For A Flavor")]
    List(list::ExtraSpecsArgs),
    /// Shows an extra spec, by key, for a flavor, by ID.
    #[command(about = "Show An Extra Spec For A Flavor")]
    Show(show::ExtraSpecArgs),
    /// Updates an extra spec, by key, for a flavor, by ID.
    #[command(about = "Update An Extra Spec For A Flavor
")]
    Set(set::ExtraSpecArgs),
}

pub struct ExtraSpecsCommand {
    pub args: ExtraSpecsArgs,
}

impl OSCCommand for ExtraSpecsCommand {
    fn get_subcommand(
        &self,
        _: &mut AsyncOpenStack,
    ) -> Result<Box<dyn OSCCommand + Send + Sync>, OpenStackCliError> {
        match &self.args.command {
            ExtraSpecsCommands::Create(args) => {
                Ok(Box::new(create::ExtraSpecCmd { args: args.clone() }))
            }
            ExtraSpecsCommands::Delete(args) => {
                Ok(Box::new(delete::ExtraSpecCmd { args: args.clone() }))
            }
            ExtraSpecsCommands::List(args) => {
                Ok(Box::new(list::ExtraSpecsCmd { args: args.clone() }))
            }
            ExtraSpecsCommands::Show(args) => {
                Ok(Box::new(show::ExtraSpecCmd { args: args.clone() }))
            }
            ExtraSpecsCommands::Set(args) => Ok(Box::new(set::ExtraSpecCmd { args: args.clone() })),
        }
    }
}
