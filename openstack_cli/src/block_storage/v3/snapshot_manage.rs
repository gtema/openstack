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

//! Block storage Manageable Snapshot commands
//!

use clap::{Parser, Subcommand};

use crate::{Cli, OpenStackCliError};

use openstack_sdk::AsyncOpenStack;

pub mod create;
pub mod list;

/// SnapshotManage manage extension (manageable_snapshots)
///
/// Creates or lists snapshots by using existing storage instead of allocating new storage.
#[derive(Parser)]
pub struct SnapshotManageCommand {
    /// subcommand
    #[command(subcommand)]
    command: SnapshotManageCommands,
}

/// Supported subcommands
#[allow(missing_docs)]
#[derive(Subcommand)]
pub enum SnapshotManageCommands {
    Create(Box<create::SnapshotManageCommand>),
    List(Box<list::SnapshotManagesCommand>),
}

impl SnapshotManageCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        session: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        match &self.command {
            SnapshotManageCommands::Create(cmd) => cmd.take_action(parsed_args, session).await,
            SnapshotManageCommands::List(cmd) => cmd.take_action(parsed_args, session).await,
        }
    }
}
