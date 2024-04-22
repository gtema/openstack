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

//! Block storage Snapshot commands
//!

use clap::{Parser, Subcommand};

use crate::{Cli, OpenStackCliError};

use openstack_sdk::AsyncOpenStack;

mod create;
mod delete;
mod list;
mod os_force_delete;
mod os_reset_status;
mod os_unmanage;
mod os_update_snapshot_status;
mod set;
mod show;

/// Volume snapshots (snapshots)
///
/// A snapshot is a point-in-time copy of the data that a volume contains.
///
/// When you create, list, or delete snapshots, these status values are possible:
///
///   - creating: The snapshot is being created.
///
///   - available: The snapshot is ready to use.
///
///   - backing-up: The snapshot is being backed up.
///
///   - deleting: The snapshot is being deleted.
///
///   - error: A snapshot creation error occurred.
///
///   - deleted: The snapshot has been deleted.
///
///   - unmanaging: The snapshot is being unmanaged.
///
///   - restoring: The snapshot is being restored to a volume.
///
///   - error_deleting: A snapshot deletion error occurred.
#[derive(Parser)]
pub struct SnapshotCommand {
    /// subcommand
    #[command(subcommand)]
    command: SnapshotCommands,
}

/// Supported subcommands
#[allow(missing_docs)]
#[derive(Subcommand)]
pub enum SnapshotCommands {
    Create(create::SnapshotCommand),
    Delete(delete::SnapshotCommand),
    ForceDelete(os_force_delete::SnapshotCommand),
    List(list::SnapshotsCommand),
    ResetStatus(os_reset_status::SnapshotCommand),
    Set(set::SnapshotCommand),
    Show(show::SnapshotCommand),
    Unmanage(os_unmanage::SnapshotCommand),
    UpdateStatus(os_update_snapshot_status::SnapshotCommand),
}

impl SnapshotCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        session: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        match &self.command {
            SnapshotCommands::Create(cmd) => cmd.take_action(parsed_args, session).await,
            SnapshotCommands::Delete(cmd) => cmd.take_action(parsed_args, session).await,
            SnapshotCommands::ForceDelete(cmd) => cmd.take_action(parsed_args, session).await,
            SnapshotCommands::List(cmd) => cmd.take_action(parsed_args, session).await,
            SnapshotCommands::ResetStatus(cmd) => cmd.take_action(parsed_args, session).await,
            SnapshotCommands::Set(cmd) => cmd.take_action(parsed_args, session).await,
            SnapshotCommands::Show(cmd) => cmd.take_action(parsed_args, session).await,
            SnapshotCommands::Unmanage(cmd) => cmd.take_action(parsed_args, session).await,
            SnapshotCommands::UpdateStatus(cmd) => cmd.take_action(parsed_args, session).await,
        }
    }
}
