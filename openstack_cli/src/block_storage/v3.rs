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

use openstack_sdk::{AsyncOpenStack, types::ServiceType};

use crate::{Cli, OpenStackCliError};

pub mod attachment;
pub mod availability_zone;
pub mod backup;
pub mod cluster;
pub mod default_type;
pub mod extension;
pub mod group;
pub mod group_snapshot;
pub mod group_type;
pub mod host;
pub mod limit;
pub mod message;
pub mod os_volume_transfer;
pub mod qos_spec;
pub mod resource_filter;
pub mod service;
pub mod snapshot;
pub mod snapshot_manage;
pub mod r#type;
pub mod volume;
pub mod volume_manage;
pub mod volume_transfer;

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
    Attachment(Box<attachment::AttachmentCommand>),
    AvailabilityZone(Box<availability_zone::AvailabilityZoneCommand>),
    Backup(Box<backup::BackupCommand>),
    Cluster(Box<cluster::ClusterCommand>),
    DefaultType(Box<default_type::DefaultTypeCommand>),
    Extension(Box<extension::ExtensionCommand>),
    Group(Box<group::GroupCommand>),
    GroupSnapshot(Box<group_snapshot::GroupSnapshotCommand>),
    GroupType(Box<group_type::GroupTypeCommand>),
    Host(Box<host::HostCommand>),
    Limit(Box<limit::LimitCommand>),
    Message(Box<message::MessageCommand>),
    OsVolumeTransfer(Box<os_volume_transfer::VolumeTransferCommand>),
    QosSpec(Box<qos_spec::QosSpecCommand>),
    Service(Box<service::ServiceCommand>),
    Snapshot(Box<snapshot::SnapshotCommand>),
    SnapshotManage(Box<snapshot_manage::SnapshotManageCommand>),
    ResourceFilter(Box<resource_filter::ResourceFilterCommand>),
    Type(Box<r#type::VolumeTypeCommand>),
    Volume(Box<volume::VolumeCommand>),
    VolumeManage(Box<volume_manage::VolumeManageCommand>),
    VolumeTransfer(Box<volume_transfer::VolumeTransferCommand>),
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
            BlockStorageCommands::Attachment(cmd) => cmd.take_action(parsed_args, session).await,
            BlockStorageCommands::AvailabilityZone(cmd) => {
                cmd.take_action(parsed_args, session).await
            }
            BlockStorageCommands::Backup(cmd) => cmd.take_action(parsed_args, session).await,
            BlockStorageCommands::Cluster(cmd) => cmd.take_action(parsed_args, session).await,
            BlockStorageCommands::DefaultType(cmd) => cmd.take_action(parsed_args, session).await,
            BlockStorageCommands::Extension(cmd) => cmd.take_action(parsed_args, session).await,
            BlockStorageCommands::Group(cmd) => cmd.take_action(parsed_args, session).await,
            BlockStorageCommands::GroupSnapshot(cmd) => cmd.take_action(parsed_args, session).await,
            BlockStorageCommands::GroupType(cmd) => cmd.take_action(parsed_args, session).await,
            BlockStorageCommands::Host(cmd) => cmd.take_action(parsed_args, session).await,
            BlockStorageCommands::Limit(cmd) => cmd.take_action(parsed_args, session).await,
            BlockStorageCommands::Message(cmd) => cmd.take_action(parsed_args, session).await,
            BlockStorageCommands::OsVolumeTransfer(cmd) => {
                cmd.take_action(parsed_args, session).await
            }
            BlockStorageCommands::QosSpec(cmd) => cmd.take_action(parsed_args, session).await,
            BlockStorageCommands::Service(cmd) => cmd.take_action(parsed_args, session).await,
            BlockStorageCommands::Snapshot(cmd) => cmd.take_action(parsed_args, session).await,
            BlockStorageCommands::SnapshotManage(cmd) => {
                cmd.take_action(parsed_args, session).await
            }
            BlockStorageCommands::ResourceFilter(cmd) => {
                cmd.take_action(parsed_args, session).await
            }
            BlockStorageCommands::Type(cmd) => cmd.take_action(parsed_args, session).await,
            BlockStorageCommands::Volume(cmd) => cmd.take_action(parsed_args, session).await,
            BlockStorageCommands::VolumeManage(cmd) => cmd.take_action(parsed_args, session).await,
            BlockStorageCommands::VolumeTransfer(cmd) => {
                cmd.take_action(parsed_args, session).await
            }
        }
    }
}
