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

//! Server volume attachment commands

use clap::{Parser, Subcommand};

use openstack_sdk::AsyncOpenStack;

use crate::{Cli, OpenStackCliError};

mod create_20;
mod create_249;
mod create_279;
mod delete;
mod list;
mod set_20;
mod set_285;
mod show;

/// Servers with volume attachments
///
/// Attaches volumes that are created through the volume API to server
/// instances. Also, lists volume attachments for a server, shows details for a
/// volume attachment, and detaches a volume.
#[derive(Parser)]
pub struct VolumeAttachmentCommand {
    /// subcommand
    #[command(subcommand)]
    command: VolumeAttachmentCommands,
}

/// Supported subcommands
#[allow(missing_docs)]
#[derive(Subcommand)]
pub enum VolumeAttachmentCommands {
    Create20(create_20::VolumeAttachmentCommand),
    Create249(create_249::VolumeAttachmentCommand),
    #[command(visible_alias = "create")]
    Create279(create_279::VolumeAttachmentCommand),
    Delete(delete::VolumeAttachmentCommand),
    List(list::VolumeAttachmentsCommand),
    Set20(set_20::VolumeAttachmentCommand),
    #[command(visible_alias = "set")]
    Set285(set_285::VolumeAttachmentCommand),
    Show(show::VolumeAttachmentCommand),
}

impl VolumeAttachmentCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        session: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        match &self.command {
            VolumeAttachmentCommands::Create20(cmd) => cmd.take_action(parsed_args, session).await,
            VolumeAttachmentCommands::Create249(cmd) => cmd.take_action(parsed_args, session).await,
            VolumeAttachmentCommands::Create279(cmd) => cmd.take_action(parsed_args, session).await,
            VolumeAttachmentCommands::Delete(cmd) => cmd.take_action(parsed_args, session).await,
            VolumeAttachmentCommands::List(cmd) => cmd.take_action(parsed_args, session).await,
            VolumeAttachmentCommands::Set20(cmd) => cmd.take_action(parsed_args, session).await,
            VolumeAttachmentCommands::Set285(cmd) => cmd.take_action(parsed_args, session).await,
            VolumeAttachmentCommands::Show(cmd) => cmd.take_action(parsed_args, session).await,
        }
    }
}
