use clap::{Args, Subcommand};

use crate::{Command, ResourceCommands};

use openstack_sdk::AsyncOpenStack;

mod create;
mod list;
mod set;
mod show;

#[derive(Args, Clone, Debug)]
// #[command(args_conflicts_with_subcommands = true)]
pub struct VolumeArgs {
    #[command(subcommand)]
    command: VolumeCommands,
}

#[derive(Subcommand, Clone, Debug)]
pub enum VolumeCommands {
    /// List Volumes
    List(list::VolumesArgs),
    /// Show single volume
    Show(show::VolumeArgs),
    /// Create volume
    Create(create::VolumeArgs),
    /// Update volume
    Set(set::VolumeArgs),
}

pub struct VolumeCommand {
    pub args: VolumeArgs,
}

impl ResourceCommands for VolumeCommand {
    fn get_command(&self, session: &mut AsyncOpenStack) -> Box<dyn Command> {
        match &self.args.command {
            VolumeCommands::List(args) => Box::new(list::VolumesCmd { args: args.clone() }),
            VolumeCommands::Show(args) => Box::new(show::VolumeCmd { args: args.clone() }),
            VolumeCommands::Create(args) => Box::new(create::VolumeCmd { args: args.clone() }),
            VolumeCommands::Set(args) => Box::new(set::VolumeCmd { args: args.clone() }),
        }
    }
}
