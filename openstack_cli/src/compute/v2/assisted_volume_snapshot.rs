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

//! Assisted volume snapshot commands

use clap::{Parser, Subcommand};

use openstack_sdk::AsyncOpenStack;

use crate::{Cli, OpenStackCliError};

pub mod create;
pub mod delete;

/// Assisted volume snapshots (os-assisted-volume-snapshots)
///
/// Creates and deletes snapshots through an emulator/hypervisor. Only qcow2 file format is
/// supported.
///
/// This API is only implemented by the libvirt compute driver.
///
/// An internal snapshot that lacks storage such as NFS can use an emulator/hypervisor to add the
/// snapshot feature. This is used to enable snapshot of volumes on backends such as NFS by storing
/// data as qcow2 files on these volumes.
///
/// This API is only ever called by Cinder, where it is used to create a snapshot for drivers that
/// extend the remotefs Cinder driver.
#[derive(Parser)]
pub struct AssistedVolumeSnapshotCommand {
    /// subcommand
    #[command(subcommand)]
    command: AssistedVolumeSnapshotCommands,
}

/// Supported subcommands
#[allow(missing_docs)]
#[derive(Subcommand)]
pub enum AssistedVolumeSnapshotCommands {
    Create(create::AssistedVolumeSnapshotCommand),
    Delete(delete::AssistedVolumeSnapshotCommand),
}

impl AssistedVolumeSnapshotCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        session: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        match &self.command {
            AssistedVolumeSnapshotCommands::Create(cmd) => {
                cmd.take_action(parsed_args, session).await
            }
            AssistedVolumeSnapshotCommands::Delete(cmd) => {
                cmd.take_action(parsed_args, session).await
            }
        }
    }
}
