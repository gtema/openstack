//! Keypairs (keypairs)
//!
//! Generates, imports, and deletes SSH keys.
//!
use clap::error::{Error, ErrorKind};
use clap::{ArgMatches, Args, Command as ClapCommand, FromArgMatches, Subcommand};
use tracing::{debug, info};

use crate::common::ServiceApiVersion;
use crate::{Command, ResourceCommands};

use openstack_sdk::{types::ServiceType, AsyncOpenStack};

mod create_20;
mod create_21;
mod create_210;
mod create_22;
mod create_292;
mod delete;
mod list;
mod show;

/// Keypairs commands
#[derive(Args, Clone, Debug)]
// #[command(args_conflicts_with_subcommands = true)]
pub struct KeypairArgs {
    #[command(subcommand)]
    command: KeypairCommands,
}

#[derive(Subcommand, Clone, Debug)]
pub enum KeypairCommands {
    /// List keypairs
    List(list::OsKeypairsArgs),
    /// Show single keypair details
    ///
    /// Shows details for a keypair that is associated with the account.
    Show(show::OsKeypairArgs),
    /// Imports (or generates) Keypair (with highest possible microversion)
    Create(Create),
    /// Import keypair (microversion >= 2.92)
    Create292(create_292::OsKeypairArgs),
    /// Import (or generate) keypair (2.10 <= microversion < 2.92)
    Create210(create_210::OsKeypairArgs),
    /// Import (or generate) keypair (2.2 <= microversion < 2.10)
    Create22(create_22::OsKeypairArgs),
    /// Import (or generate) keypair (2.1 <= microversion < 2.2)
    Create21(create_21::OsKeypairArgs),
    /// Import (or generate) keypair (microversion == 2.0)
    Create20(create_20::OsKeypairArgs),
    /// Delete keypair
    Delete(delete::OsKeypairArgs),
}

#[derive(Default, Clone, Debug)]
/// Create Keypair arguments structure
pub struct Create {
    create_20: Option<create_20::OsKeypairArgs>,
    create_21: Option<create_21::OsKeypairArgs>,
    create_22: Option<create_22::OsKeypairArgs>,
    create_210: Option<create_210::OsKeypairArgs>,
    create_292: Option<create_292::OsKeypairArgs>,
}

impl FromArgMatches for Create {
    fn from_arg_matches(matches: &ArgMatches) -> Result<Self, Error> {
        Ok(Self {
            create_292: create_292::OsKeypairArgs::from_arg_matches(matches).ok(),
            create_210: create_210::OsKeypairArgs::from_arg_matches(matches).ok(),
            create_22: create_22::OsKeypairArgs::from_arg_matches(matches).ok(),
            create_21: create_21::OsKeypairArgs::from_arg_matches(matches).ok(),
            create_20: create_20::OsKeypairArgs::from_arg_matches(matches).ok(),
        })
    }

    fn update_from_arg_matches(&mut self, matches: &ArgMatches) -> Result<(), Error> {
        *self = Self {
            create_292: create_292::OsKeypairArgs::from_arg_matches(matches).ok(),
            create_210: create_210::OsKeypairArgs::from_arg_matches(matches).ok(),
            create_22: create_22::OsKeypairArgs::from_arg_matches(matches).ok(),
            create_21: create_21::OsKeypairArgs::from_arg_matches(matches).ok(),
            create_20: create_20::OsKeypairArgs::from_arg_matches(matches).ok(),
        };
        Ok(())
    }
}

impl Args for Create {
    fn augment_args(cmd: ClapCommand) -> ClapCommand {
        create_292::OsKeypairArgs::augment_args(cmd)
    }
    fn augment_args_for_update(cmd: ClapCommand) -> ClapCommand {
        create_292::OsKeypairArgs::augment_args(cmd)
    }
}

pub struct KeypairCommand {
    pub args: KeypairArgs,
}

impl ResourceCommands for KeypairCommand {
    fn get_command(&self, session: &mut AsyncOpenStack) -> Box<dyn Command> {
        match &self.args.command {
            KeypairCommands::List(args) => Box::new(list::OsKeypairsCmd { args: args.clone() }),
            KeypairCommands::Show(args) => Box::new(show::OsKeypairCmd { args: args.clone() }),
            KeypairCommands::Create292(args) => {
                Box::new(create_292::OsKeypairCmd { args: args.clone() })
            }
            KeypairCommands::Create210(args) => {
                Box::new(create_210::OsKeypairCmd { args: args.clone() })
            }
            KeypairCommands::Create22(args) => {
                Box::new(create_22::OsKeypairCmd { args: args.clone() })
            }
            KeypairCommands::Create21(args) => {
                Box::new(create_21::OsKeypairCmd { args: args.clone() })
            }
            KeypairCommands::Create20(args) => {
                Box::new(create_20::OsKeypairCmd { args: args.clone() })
            }
            KeypairCommands::Create(args) => {
                if let Some(ep_ver) = session.get_service_endpoint_version(&ServiceType::Compute) {
                    if let Some(vers) = ep_ver.version {
                        if let Ok(ver) = ServiceApiVersion::try_from(vers) {
                            if ver >= ServiceApiVersion(2, 92) {
                                return Box::new(create_292::OsKeypairCmd {
                                    args: args.create_292.clone().expect("All arguments present"),
                                });
                            } else if ver >= ServiceApiVersion(2, 10) {
                                return Box::new(create_210::OsKeypairCmd {
                                    args: args.create_210.clone().expect("All arguments present"),
                                });
                            } else if ver >= ServiceApiVersion(2, 2) {
                                return Box::new(create_22::OsKeypairCmd {
                                    args: args.create_22.clone().expect("All arguments present"),
                                });
                            } else if ver >= ServiceApiVersion(2, 1) {
                                return Box::new(create_21::OsKeypairCmd {
                                    args: args.create_21.clone().expect("All arguments present"),
                                });
                            } else if ver >= ServiceApiVersion(2, 0) {
                                return Box::new(create_20::OsKeypairCmd {
                                    args: args.create_20.clone().expect("All arguments present"),
                                });
                            }
                        }
                    }
                }
                Box::new(create_292::OsKeypairCmd {
                    args: args.create_292.clone().expect("All arguments present"),
                })
            }
            KeypairCommands::Delete(args) => Box::new(delete::OsKeypairCmd { args: args.clone() }),
        }
    }
}
