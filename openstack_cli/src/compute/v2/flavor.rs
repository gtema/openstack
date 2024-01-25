//! Compute Flavor commands
use clap::error::{Error, ErrorKind};
use clap::{ArgMatches, Args, Command as ClapCommand, FromArgMatches, Subcommand};

use crate::common::ServiceApiVersion;
use crate::{Command, ResourceCommands};

use openstack_sdk::{types::ServiceType, AsyncOpenStack};

mod add_tenant_access;
mod create_20;
mod create_21;
mod create_255;
mod delete;
mod list;
mod os_extra_spec;
mod os_flavor_access;
mod remove_tenant_access;
mod set;
mod show;

#[derive(Args, Clone, Debug)]
pub struct FlavorArgs {
    #[command(subcommand)]
    command: FlavorCommands,
}

#[derive(Subcommand, Clone, Debug)]
pub enum FlavorCommands {
    /// Lists tenants who have access to a private flavor and adds private
    /// flavor access to and removes private flavor access from tenants. By
    /// default, only administrators can manage private flavor access. A
    /// private flavor has is_public set to false while a public flavor has
    /// is_public set to true.
    #[command(about = "Flavor access commands")]
    Access(Box<os_flavor_access::FlavorAccessArgs>),
    /// **Creates a flavor.**
    ///
    /// Creating a flavor is typically only available to administrators of a
    /// cloud because this has implications for scheduling efficiently in the
    /// cloud.
    ///
    ///
    /// **Note**
    ///
    /// Flavors with special characters in the flavor ID, except the hyphen
    /// ‘-‘, underscore ‘_’, spaces and dots ‘.’, are not permitted.
    ///
    /// Flavor IDs are meant to be UUIDs. Serialized strings separated/grouped
    /// by “-” represent the default flavor ID or UUID. eg:
    /// 01cc74d8-4816-4bef-835b-e36ff188c406.
    ///
    /// Only for backward compatibility, an integer as a flavor ID is
    /// permitted.
    #[command(about = "Create Flavor (with highest possible microversion)")]
    Create(Box<Create>),
    /// Create flavor (microversion 2.55)
    #[command(about = "Create Flavor (microversion = 2.55)")]
    Create255(Box<create_255::FlavorArgs>),
    /// Create flavor (microversion 2.1)
    #[command(about = "Create Flavor (microversion = 2.1)")]
    Create21(Box<create_21::FlavorArgs>),
    /// Create flavor (microversion 2.0)
    #[command(about = "Create Flavor (microversion = 2.0)")]
    Create20(Box<create_20::FlavorArgs>),
    /// Deletes a flavor.
    ///
    /// This is typically an admin only action. Deleting a flavor that is in
    /// use by existing servers is not recommended as it can cause incorrect
    /// data to be returned to the user under some operations.
    #[command(about = "Delete Flavor")]
    Delete(Box<delete::FlavorArgs>),
    /// Lists, creates, deletes, and updates the extra-specs or keys for a
    /// flavor.
    #[command(about = "Flavor extra-specs")]
    Extraspecs(Box<os_extra_spec::ExtraSpecsArgs>),
    /// Lists all flavors accessible to your project.
    #[command(about = "List Flavors")]
    List(Box<list::FlavorsArgs>),
    /// Updates a flavor description.

    /// This API is available starting with microversion 2.55.
    ///
    /// Policy defaults enable only users with the administrative role to
    /// perform this operation. Cloud providers can change these permissions
    /// through the policy.json file.
    #[command(about = "Update Flavor Description")]
    Set(Box<set::FlavorArgs>),
    /// Shows details for a flavor.
    #[command(about = "Show Flavor Details")]
    Show(Box<show::FlavorArgs>),
}

#[derive(Default, Clone, Debug)]
/// Create Flavor arguments structure
pub struct Create {
    create_20: Option<create_20::FlavorArgs>,
    create_21: Option<create_21::FlavorArgs>,
    create_255: Option<create_255::FlavorArgs>,
}

impl FromArgMatches for Create {
    fn from_arg_matches(matches: &ArgMatches) -> Result<Self, Error> {
        Ok(Self {
            create_255: create_255::FlavorArgs::from_arg_matches(matches).ok(),
            create_21: create_21::FlavorArgs::from_arg_matches(matches).ok(),
            create_20: create_20::FlavorArgs::from_arg_matches(matches).ok(),
        })
    }

    fn update_from_arg_matches(&mut self, matches: &ArgMatches) -> Result<(), Error> {
        *self = Self {
            create_255: create_255::FlavorArgs::from_arg_matches(matches).ok(),
            create_21: create_21::FlavorArgs::from_arg_matches(matches).ok(),
            create_20: create_20::FlavorArgs::from_arg_matches(matches).ok(),
        };
        Ok(())
    }
}

impl Args for Create {
    fn augment_args(cmd: ClapCommand) -> ClapCommand {
        create_255::FlavorArgs::augment_args(cmd)
    }
    fn augment_args_for_update(cmd: ClapCommand) -> ClapCommand {
        create_255::FlavorArgs::augment_args(cmd)
    }
}

pub struct FlavorCommand {
    pub args: FlavorArgs,
}

impl ResourceCommands for FlavorCommand {
    fn get_command(&self, session: &mut AsyncOpenStack) -> Box<dyn Command> {
        match &self.args.command {
            FlavorCommands::Access(args) => os_flavor_access::FlavorAccessCommand {
                args: *args.clone(),
            }
            .get_command(session),
            FlavorCommands::Create20(args) => Box::new(create_20::FlavorCmd {
                args: *args.clone(),
            }),
            FlavorCommands::Create21(args) => Box::new(create_21::FlavorCmd {
                args: *args.clone(),
            }),
            FlavorCommands::Create255(args) => Box::new(create_255::FlavorCmd {
                args: *args.clone(),
            }),
            FlavorCommands::Create(args) => {
                if let Some(ep_ver) = session.get_service_endpoint_version(&ServiceType::Compute) {
                    if let Some(vers) = ep_ver.version {
                        if let Ok(ver) = ServiceApiVersion::try_from(vers) {
                            if ver >= ServiceApiVersion(2, 55) {
                                return Box::new(create_255::FlavorCmd {
                                    args: args.create_255.clone().expect("All arguments present"),
                                });
                            } else if ver >= ServiceApiVersion(2, 1) {
                                return Box::new(create_21::FlavorCmd {
                                    args: args.create_21.clone().expect("All arguments present"),
                                });
                            } else if ver >= ServiceApiVersion(2, 0) {
                                return Box::new(create_20::FlavorCmd {
                                    args: args.create_20.clone().expect("All arguments present"),
                                });
                            }
                        }
                    }
                }
                Box::new(create_255::FlavorCmd {
                    args: args.create_255.clone().expect("All arguments present"),
                })
            }
            FlavorCommands::Delete(args) => Box::new(delete::FlavorCmd {
                args: *args.clone(),
            }),
            FlavorCommands::Extraspecs(args) => os_extra_spec::ExtraSpecsCommand {
                args: *args.clone(),
            }
            .get_command(session),
            FlavorCommands::List(args) => Box::new(list::FlavorsCmd {
                args: *args.clone(),
            }),
            FlavorCommands::Set(args) => Box::new(set::FlavorCmd {
                args: *args.clone(),
            }),
            FlavorCommands::Show(args) => Box::new(show::FlavorCmd {
                args: *args.clone(),
            }),
        }
    }
}
