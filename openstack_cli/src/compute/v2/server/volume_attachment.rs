// Copyright 2024
//
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

use clap::{Args, Subcommand};

use crate::{OSCCommand, OpenStackCliError};

use openstack_sdk::AsyncOpenStack;

pub mod create_20;
pub mod create_249;
pub mod create_279;
pub mod delete;
pub mod list;
pub mod set_20;
pub mod set_285;
pub mod show;

/// Servers with volume attachments
///
/// Attaches volumes that are created through the volume API to server
/// instances. Also, lists volume attachments for a server, shows details for a
/// volume attachment, and detaches a volume.
#[derive(Args, Clone)]
#[command(args_conflicts_with_subcommands = true)]
pub struct VolumeAttachmentArgs {
    #[command(subcommand)]
    command: VolumeAttachmentCommands,
}

#[derive(Subcommand, Clone)]
pub enum VolumeAttachmentCommands {
    Create20(create_20::VolumeAttachmentArgs),
    Create249(create_249::VolumeAttachmentArgs),
    #[command(visible_alias = "create")]
    Create279(create_279::VolumeAttachmentArgs),
    Delete(delete::VolumeAttachmentArgs),
    List(list::VolumeAttachmentsArgs),
    Set20(set_20::VolumeAttachmentArgs),
    #[command(visible_alias = "set")]
    Set285(set_285::VolumeAttachmentArgs),
    Show(show::VolumeAttachmentArgs),
}

pub struct VolumeAttachmentCommand {
    pub args: VolumeAttachmentArgs,
}

impl OSCCommand for VolumeAttachmentCommand {
    fn get_subcommand(
        &self,
        _: &mut AsyncOpenStack,
    ) -> Result<Box<dyn OSCCommand + Send + Sync>, OpenStackCliError> {
        match &self.args.command {
            VolumeAttachmentCommands::Create20(args) => {
                Ok(Box::new(create_20::VolumeAttachmentCmd {
                    args: args.clone(),
                }))
            }
            VolumeAttachmentCommands::Create249(args) => {
                Ok(Box::new(create_249::VolumeAttachmentCmd {
                    args: args.clone(),
                }))
            }
            VolumeAttachmentCommands::Create279(args) => {
                Ok(Box::new(create_279::VolumeAttachmentCmd {
                    args: args.clone(),
                }))
            }
            VolumeAttachmentCommands::Delete(args) => {
                Ok(Box::new(delete::VolumeAttachmentCmd { args: args.clone() }))
            }
            VolumeAttachmentCommands::List(args) => {
                Ok(Box::new(list::VolumeAttachmentsCmd { args: args.clone() }))
            }
            VolumeAttachmentCommands::Set20(args) => {
                Ok(Box::new(set_20::VolumeAttachmentCmd { args: args.clone() }))
            }
            VolumeAttachmentCommands::Set285(args) => Ok(Box::new(set_285::VolumeAttachmentCmd {
                args: args.clone(),
            })),
            VolumeAttachmentCommands::Show(args) => {
                Ok(Box::new(show::VolumeAttachmentCmd { args: args.clone() }))
            }
        }
    }
}
