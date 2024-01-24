//! Identity User Access Rules commands
//!
use clap::error::{Error, ErrorKind};
use clap::{ArgMatches, Args, Command as ClapCommand, FromArgMatches, Subcommand};

use crate::common::ServiceApiVersion;
use crate::{Command, ResourceCommands};

use openstack_sdk::{types::ServiceType, AsyncOpenStack};

mod delete;
mod list;
mod show;

/// Identity User access rules
#[derive(Args, Clone, Debug)]
// #[command(args_conflicts_with_subcommands = true)]
pub struct AccessRuleArgs {
    #[command(subcommand)]
    command: AccessRuleCommands,
}

#[derive(Subcommand, Clone, Debug)]
pub enum AccessRuleCommands {
    /// Delete an access rule. An access rule that is still in use by an
    /// application credential cannot be deleted.
    #[command(about = "Delete access rule")]
    Delete(delete::AccessRuleArgs),
    /// List all access rules for a user.
    #[command(about = "List access rules")]
    List(list::AccessRulesArgs),
    /// Show details of an access rule.
    #[command(about = "Show access rule details")]
    Show(show::AccessRuleArgs),
}

pub struct AccessRuleCommand {
    pub args: AccessRuleArgs,
}

impl ResourceCommands for AccessRuleCommand {
    fn get_command(&self, session: &mut AsyncOpenStack) -> Box<dyn Command> {
        match &self.args.command {
            AccessRuleCommands::Delete(args) => {
                Box::new(delete::AccessRuleCmd { args: args.clone() })
            }
            AccessRuleCommands::List(args) => Box::new(list::AccessRulesCmd { args: args.clone() }),
            AccessRuleCommands::Show(args) => Box::new(show::AccessRuleCmd { args: args.clone() }),
        }
    }
}
