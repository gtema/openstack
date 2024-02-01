// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//
// SPDX-License-Identifier: Apache-2.0

//! Block storage Volume commands
//!

use clap::{Args, Subcommand};

use crate::{OSCCommand, OpenStackCliError};

use openstack_sdk::AsyncOpenStack;

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
pub struct VolumeArgs {
    #[command(subcommand)]
    command: VolumeCommands,
}

#[derive(Subcommand, Clone, Debug)]
pub enum VolumeCommands {
    Create30(create_30::VolumeArgs),
    Create313(create_313::VolumeArgs),
    Create347(create_347::VolumeArgs),
    #[command(visible_alias = "create")]
    Create353(create_353::VolumeArgs),
    Delete(delete::VolumeArgs),
    Extend(os_extend::VolumeArgs),
    List(list::VolumesArgs),
    #[command(visible_alias = "set")]
    Set353(set_353::VolumeArgs),
    Set30(set_30::VolumeArgs),
    Show(show::VolumeArgs),
}

pub struct VolumeCommand {
    /// Command arguments
    pub args: VolumeArgs,
}

impl OSCCommand for VolumeCommand {
    fn get_subcommand(
        &self,
        _session: &mut AsyncOpenStack,
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
            VolumeCommands::Delete(args) => Ok(Box::new(delete::VolumeCmd { args: args.clone() })),
            VolumeCommands::Extend(args) => {
                Ok(Box::new(os_extend::VolumeCmd { args: args.clone() }))
            }
            VolumeCommands::List(args) => Ok(Box::new(list::VolumesCmd { args: args.clone() })),
            VolumeCommands::Set30(args) => Ok(Box::new(set_30::VolumeCmd { args: args.clone() })),
            VolumeCommands::Set353(args) => Ok(Box::new(set_353::VolumeCmd { args: args.clone() })),
            VolumeCommands::Show(args) => Ok(Box::new(show::VolumeCmd { args: args.clone() })),
        }
    }
}
