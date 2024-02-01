use clap::{Args, Subcommand};

use crate::{OSCCommand, OpenStackCliError};

use openstack_sdk::AsyncOpenStack;

pub mod delete;
pub mod force_complete_222;
pub mod list;
pub mod show;

/// Server migrations (servers, migrations)
///
/// List, show, perform actions on and delete server migrations.
#[derive(Args, Clone)]
#[command(args_conflicts_with_subcommands = true)]
pub struct MigrationArgs {
    #[command(subcommand)]
    command: MigrationCommands,
}

#[derive(Subcommand, Clone)]
pub enum MigrationCommands {
    Delete(delete::MigrationArgs),
    ForceComplete(force_complete_222::MigrationArgs),
    List(list::MigrationsArgs),
    Show(show::MigrationArgs),
}

pub struct MigrationCommand {
    pub args: MigrationArgs,
}

impl OSCCommand for MigrationCommand {
    fn get_subcommand(
        &self,
        _: &mut AsyncOpenStack,
    ) -> Result<Box<dyn OSCCommand + Send + Sync>, OpenStackCliError> {
        match &self.args.command {
            MigrationCommands::Delete(args) => {
                Ok(Box::new(delete::MigrationCmd { args: args.clone() }))
            }
            MigrationCommands::ForceComplete(args) => {
                Ok(Box::new(force_complete_222::MigrationCmd {
                    args: args.clone(),
                }))
            }
            MigrationCommands::List(args) => {
                Ok(Box::new(list::MigrationsCmd { args: args.clone() }))
            }
            MigrationCommands::Show(args) => {
                Ok(Box::new(show::MigrationCmd { args: args.clone() }))
            }
        }
    }
}
