//! Block storage Volume commands
//!
use clap::error::Error;
use clap::{ArgMatches, Args, Command, FromArgMatches, Subcommand};

use crate::common::ServiceApiVersion;
use crate::{OSCCommand, OpenStackCliError};

use openstack_sdk::{types::ServiceType, AsyncOpenStack};

mod create_30;
mod create_313;
mod create_347;
mod create_353;
mod delete;
mod list;
mod os_extend;
mod set_30;
mod set_353;
mod show;

/// Block Storage Volume commands
#[derive(Args, Clone, Debug)]
// #[command(args_conflicts_with_subcommands = true)]
pub struct VolumeArgs {
    #[command(subcommand)]
    command: VolumeCommands,
}

#[derive(Subcommand, Clone, Debug)]
pub enum VolumeCommands {
    /// Create volume (with highest possible microversion)
    ///
    /// To create a bootable volume, include the UUID of the image from which
    /// you want to create the volume in the imageRef attribute in the request
    /// body.
    ///
    /// Since the Train release, every volume must have a volume type. It is
    /// optional to specify a volume type as part of your Create a volume
    /// request. If you do not specify one, a default volume type will be
    /// supplied for you. This type may vary according to what project you are
    /// in and how the operator has configured the Block Storage service. Use
    /// the Show default volume type request to determine your effective
    /// default volume type.
    ///
    /// **Preconditions**
    ///
    ///  - You must have enough volume storage quota remaining to create a
    ///  volume of size requested.
    ///
    /// **Asynchronous Postconditions**
    ///
    ///  - With correct permissions, you can see the volume status as available
    ///  through API calls.
    ///
    ///  - With correct access, you can see the created volume in the storage
    ///  system that OpenStack Block Storage manages.
    ///
    /// **Troubleshooting**
    ///
    ///  -  If volume status remains creating or shows another error status,
    ///  the request failed. Ensure you meet the preconditions then investigate
    ///  the storage back end.
    ///
    ///  - Volume is not created in the storage system that OpenStack Block
    ///  Storage manages.
    ///
    ///  - The storage node needs enough free storage space to match the size
    ///  of the volume creation request.
    #[command(about = "Create new volume (with highest possible microversion)")]
    Create(Create),
    /// Create volume (microversion 3.0)
    #[command(about = "Create new volume (microversion = 3.0)")]
    Create30(create_30::VolumeArgs),
    /// Create volume (microversion 3.13)
    #[command(about = "Create new volume (microversion = 3.13)")]
    Create313(create_313::VolumeArgs),
    /// Create volume (microversion 3.47)
    #[command(about = "Create new volume (microversion = 3.47)")]
    Create347(create_347::VolumeArgs),
    /// Create volume (microversion 3.53)
    #[command(about = "Create new volume (microversion = 3.53)")]
    Create353(create_353::VolumeArgs),
    /// Deletes a volume.
    ///
    /// **Preconditions**
    ///
    ///  - Volume status must be available, in-use, error, error_restoring,
    ///  error_extending, error_managing, and must not be migrating, attached,
    ///  awaiting-transfer, belong to a group, have snapshots or be
    ///  disassociated from snapshots after volume transfer.
    ///
    ///  - The cascade option can be passed in the request if you want all
    ///  snapshots of this volume to be deleted automatically, which should
    ///  allow the volume deletion to succeed.
    ///
    ///  - You cannot delete a volume that is in a migration.
    ///
    /// **Asynchronous Postconditions**
    ///
    ///  - The volume is deleted in volume index.
    ///
    ///  - The volume managed by OpenStack Block Storage is deleted in storage
    ///  node.
    ///
    /// **Troubleshooting**
    ///
    ///  - If volume status remains in deleting or becomes error_deleting the
    ///  request failed. Ensure you meet the preconditions then investigate the
    ///  storage back end.
    ///
    ///  - The volume managed by OpenStack Block Storage is not deleted from
    ///  the storage system.
    #[command(about = "Delete volume")]
    Delete(delete::VolumeArgs),
    /// Extends the size of a volume to a requested size, in gibibytes (GiB).
    /// Specify the os-extend action in the request body.
    ///
    /// **Preconditions**
    ///
    ///  - Prior to microversion 3.42 the volume status must be available.
    ///  Starting with microversion 3.42, attached volumes with status in-use
    ///  may be able to be extended depending on policy and backend volume and
    ///  compute driver constraints in the cloud. Note that reserved is not a
    ///  valid state for extend.
    ///
    ///  - Sufficient amount of storage must exist to extend the volume.
    ///
    ///  - The user quota must have sufficient volume storage.
    ///
    /// **Postconditions**
    ///
    ///  - If the request is processed successfully, the volume status will
    ///  change to extending while the volume size is being extended.
    ///
    ///  - Upon successful completion of the extend operation, the volume
    ///  status will go back to its original value.
    ///
    ///  - Starting with microversion 3.42, when extending the size of an
    ///  attached volume, the Block Storage service will notify the Compute
    ///  service that an attached volume has been extended. The Compute service
    ///  will asynchronously process the volume size change for the related
    ///  server instance. This can be monitored using the GET
    ///  /servers/{server_id}/os-instance-actions API in the Compute service.
    ///
    /// **Troubleshooting**
    ///
    ///  - An error_extending volume status indicates that the request failed.
    ///  Ensure that you meet the preconditions and retry the request. If the
    ///  request fails again, investigate the storage back end.
    ///
    #[command(about = "Extend volume")]
    Extend(os_extend::VolumeArgs),
    /// Lists all Block Storage volumes, with details, that the project can
    /// access, since v3.31 if non-admin users specify invalid filters in the
    /// url, API will return bad request.
    #[command(about = "List Volumes")]
    List(list::VolumesArgs),
    /// Updates a volume.
    #[command(about = "Updates a volume (highest possible microversion).")]
    Set(Set),
    /// Updates a volume (microversion 3.53)
    #[command(about = "Updates a volume (microversion = 3.53).")]
    Set353(set_353::VolumeArgs),
    /// Updates a volume (microversion 3.0).
    #[command(about = "Updates a volume (microversion = 3.0).")]
    Set30(set_30::VolumeArgs),
    /// Shows details for a volume.
    ///
    /// **Preconditions**
    ///
    ///  - The volume must exist.
    #[command(about = "Show single volume details")]
    Show(show::VolumeArgs),
}

#[derive(Default, Clone, Debug)]
/// Create Volume arguments structure
pub struct Create {
    create_30: Option<create_30::VolumeArgs>,
    create_313: Option<create_313::VolumeArgs>,
    create_347: Option<create_347::VolumeArgs>,
    create_353: Option<create_353::VolumeArgs>,
}

impl FromArgMatches for Create {
    fn from_arg_matches(matches: &ArgMatches) -> Result<Self, Error> {
        Ok(Self {
            create_353: create_353::VolumeArgs::from_arg_matches(matches).ok(),
            create_347: create_347::VolumeArgs::from_arg_matches(matches).ok(),
            create_313: create_313::VolumeArgs::from_arg_matches(matches).ok(),
            create_30: create_30::VolumeArgs::from_arg_matches(matches).ok(),
        })
    }

    fn update_from_arg_matches(&mut self, matches: &ArgMatches) -> Result<(), Error> {
        *self = Self {
            create_353: create_353::VolumeArgs::from_arg_matches(matches).ok(),
            create_347: create_347::VolumeArgs::from_arg_matches(matches).ok(),
            create_313: create_313::VolumeArgs::from_arg_matches(matches).ok(),
            create_30: create_30::VolumeArgs::from_arg_matches(matches).ok(),
        };
        Ok(())
    }
}

impl Args for Create {
    fn augment_args(cmd: Command) -> Command {
        create_353::VolumeArgs::augment_args(cmd)
    }
    fn augment_args_for_update(cmd: Command) -> Command {
        create_353::VolumeArgs::augment_args(cmd)
    }
}

#[derive(Default, Clone, Debug)]
/// Update Volume arguments structure
pub struct Set {
    set_30: Option<set_30::VolumeArgs>,
    set_353: Option<set_353::VolumeArgs>,
}

impl FromArgMatches for Set {
    fn from_arg_matches(matches: &ArgMatches) -> Result<Self, Error> {
        Ok(Self {
            set_353: set_353::VolumeArgs::from_arg_matches(matches).ok(),
            set_30: set_30::VolumeArgs::from_arg_matches(matches).ok(),
        })
    }

    fn update_from_arg_matches(&mut self, matches: &ArgMatches) -> Result<(), Error> {
        *self = Self {
            set_353: set_353::VolumeArgs::from_arg_matches(matches).ok(),
            set_30: set_30::VolumeArgs::from_arg_matches(matches).ok(),
        };
        Ok(())
    }
}

impl Args for Set {
    fn augment_args(cmd: Command) -> Command {
        set_353::VolumeArgs::augment_args(cmd)
    }
    fn augment_args_for_update(cmd: Command) -> Command {
        set_353::VolumeArgs::augment_args(cmd)
    }
}

pub struct VolumeCommand {
    pub args: VolumeArgs,
}

impl OSCCommand for VolumeCommand {
    fn get_subcommand(
        &self,
        session: &mut AsyncOpenStack,
    ) -> Result<Box<dyn OSCCommand + Send + Sync>, OpenStackCliError> {
        match &self.args.command {
            VolumeCommands::Create30(args) => {
                Ok(Box::new(create_30::VolumeCmd { args: args.clone() }))
            }
            VolumeCommands::Create313(args) => {
                Ok(Box::new(create_313::VolumeCmd { args: args.clone() }))
            }
            VolumeCommands::Create347(args) => {
                Ok(Box::new(create_347::VolumeCmd { args: args.clone() }))
            }
            VolumeCommands::Create353(args) => {
                Ok(Box::new(create_353::VolumeCmd { args: args.clone() }))
            }
            VolumeCommands::Create(args) => {
                if let Some(ep_ver) =
                    session.get_service_endpoint_version(&ServiceType::BlockStorage)
                {
                    if let Some(vers) = ep_ver.version {
                        if let Ok(ver) = ServiceApiVersion::try_from(vers) {
                            if ver >= ServiceApiVersion(3, 53) {
                                return Ok(Box::new(create_353::VolumeCmd {
                                    args: args.create_353.clone().expect("All arguments present"),
                                }));
                            } else if ver >= ServiceApiVersion(3, 47) {
                                return Ok(Box::new(create_347::VolumeCmd {
                                    args: args.create_347.clone().expect("All arguments present"),
                                }));
                            } else if ver >= ServiceApiVersion(3, 13) {
                                return Ok(Box::new(create_313::VolumeCmd {
                                    args: args.create_313.clone().expect("All arguments present"),
                                }));
                            } else if ver >= ServiceApiVersion(3, 0) {
                                return Ok(Box::new(create_30::VolumeCmd {
                                    args: args.create_30.clone().expect("All arguments present"),
                                }));
                            }
                        }
                    }
                }
                Ok(Box::new(create_353::VolumeCmd {
                    args: args.create_353.clone().expect("All arguments present"),
                }))
            }
            VolumeCommands::Delete(args) => Ok(Box::new(delete::VolumeCmd { args: args.clone() })),
            VolumeCommands::Extend(args) => {
                Ok(Box::new(os_extend::VolumeCmd { args: args.clone() }))
            }
            VolumeCommands::List(args) => Ok(Box::new(list::VolumesCmd { args: args.clone() })),
            VolumeCommands::Set30(args) => Ok(Box::new(set_30::VolumeCmd { args: args.clone() })),
            VolumeCommands::Set353(args) => Ok(Box::new(set_353::VolumeCmd { args: args.clone() })),
            VolumeCommands::Set(args) => {
                if let Some(ep_ver) =
                    session.get_service_endpoint_version(&ServiceType::BlockStorage)
                {
                    if let Some(vers) = ep_ver.version {
                        if let Ok(ver) = ServiceApiVersion::try_from(vers) {
                            if ver >= ServiceApiVersion(3, 53) {
                                return Ok(Box::new(set_353::VolumeCmd {
                                    args: args.set_353.clone().expect("All arguments present"),
                                }));
                            } else if ver >= ServiceApiVersion(3, 0) {
                                return Ok(Box::new(set_30::VolumeCmd {
                                    args: args.set_30.clone().expect("All arguments present"),
                                }));
                            }
                        }
                    }
                }
                Ok(Box::new(set_353::VolumeCmd {
                    args: args.set_353.clone().expect("All arguments present"),
                }))
            }
            VolumeCommands::Show(args) => Ok(Box::new(show::VolumeCmd { args: args.clone() })),
        }
    }
}
