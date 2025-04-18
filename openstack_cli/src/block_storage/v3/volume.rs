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

use clap::{Parser, Subcommand};

use crate::{Cli, OpenStackCliError};

use openstack_sdk::AsyncOpenStack;

pub mod create_30;
pub mod create_313;
pub mod create_347;
pub mod create_353;
pub mod delete;
pub mod list;
pub mod metadata;
pub mod os_extend;
pub mod set_30;
pub mod set_353;
pub mod show;

/// Block Storage Volume commands
#[derive(Parser)]
pub struct VolumeCommand {
    /// subcommand
    #[command(subcommand)]
    command: VolumeCommands,
}

/// Supported subcommands
#[allow(missing_docs)]
#[derive(Subcommand)]
pub enum VolumeCommands {
    #[command(visible_alias = "create")]
    Create353(Box<create_353::VolumeCommand>),
    Create347(Box<create_347::VolumeCommand>),
    Create313(Box<create_313::VolumeCommand>),
    Create30(Box<create_30::VolumeCommand>),
    Delete(Box<delete::VolumeCommand>),
    Extend(Box<os_extend::VolumeCommand>),
    List(Box<list::VolumesCommand>),
    Metadata(Box<metadata::MetadataCommand>),
    #[command(visible_alias = "set")]
    Set353(Box<set_353::VolumeCommand>),
    Set30(Box<set_30::VolumeCommand>),
    Show(Box<show::VolumeCommand>),
}

impl VolumeCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        session: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        match &self.command {
            VolumeCommands::Create353(cmd) => cmd.take_action(parsed_args, session).await,
            VolumeCommands::Create347(cmd) => cmd.take_action(parsed_args, session).await,
            VolumeCommands::Create313(cmd) => cmd.take_action(parsed_args, session).await,
            VolumeCommands::Create30(cmd) => cmd.take_action(parsed_args, session).await,
            VolumeCommands::Delete(cmd) => cmd.take_action(parsed_args, session).await,
            VolumeCommands::Extend(cmd) => cmd.take_action(parsed_args, session).await,
            VolumeCommands::List(cmd) => cmd.take_action(parsed_args, session).await,
            VolumeCommands::Metadata(cmd) => cmd.take_action(parsed_args, session).await,
            VolumeCommands::Set353(cmd) => cmd.take_action(parsed_args, session).await,
            VolumeCommands::Set30(cmd) => cmd.take_action(parsed_args, session).await,
            VolumeCommands::Show(cmd) => cmd.take_action(parsed_args, session).await,
        }
    }
}
