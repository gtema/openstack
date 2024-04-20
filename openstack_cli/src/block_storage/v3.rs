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

//! Block storage v3 commands
use clap::{Parser, Subcommand};

use openstack_sdk::{types::ServiceType, AsyncOpenStack};

use crate::{Cli, OpenStackCliError};

mod backup;
mod message;
mod resource_filter;
mod r#type;
mod volume;

/// Block Storage (Volume) service (Cinder) commands
#[derive(Parser)]
pub struct BlockStorageCommand {
    /// subcommand
    #[command(subcommand)]
    command: BlockStorageCommands,
}

/// Supported subcommands
#[allow(missing_docs)]
#[derive(Subcommand)]
pub enum BlockStorageCommands {
    Backup(backup::BackupCommand),
    Message(message::MessageCommand),
    ResourceFilter(resource_filter::ResourceFilterCommand),
    Type(r#type::VolumeTypeCommand),
    Volume(volume::VolumeCommand),
}

impl BlockStorageCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        session: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        session
            .discover_service_endpoint(&ServiceType::BlockStorage)
            .await?;

        match &self.command {
            BlockStorageCommands::Backup(cmd) => cmd.take_action(parsed_args, session).await,
            BlockStorageCommands::Message(cmd) => cmd.take_action(parsed_args, session).await,
            BlockStorageCommands::ResourceFilter(cmd) => {
                cmd.take_action(parsed_args, session).await
            }
            BlockStorageCommands::Type(cmd) => cmd.take_action(parsed_args, session).await,
            BlockStorageCommands::Volume(cmd) => cmd.take_action(parsed_args, session).await,
        }
    }
}
