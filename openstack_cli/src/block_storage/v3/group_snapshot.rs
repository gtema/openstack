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

//! Block storage Volume GroupSnapshot commands

use clap::{Parser, Subcommand};

use crate::{Cli, OpenStackCliError};

use openstack_sdk::AsyncOpenStack;

mod create_314;
mod delete;
mod list;
mod reset_status_319;
mod show;

/// GroupSnapshot snapshots (group_snapshots)
///
/// Lists all, lists all with details, shows details for, creates, and deletes group snapshots.
#[derive(Parser)]
pub struct GroupSnapshotCommand {
    /// subcommand
    #[command(subcommand)]
    command: GroupSnapshotCommands,
}

/// Supported subcommands
#[allow(missing_docs)]
#[derive(Subcommand)]
pub enum GroupSnapshotCommands {
    #[command(visible_alias = "create")]
    Create314(Box<create_314::GroupSnapshotCommand>),
    Delete(Box<delete::GroupSnapshotCommand>),
    List(Box<list::GroupSnapshotsCommand>),
    #[command(visible_alias = "reset-status")]
    ResetStatus319(Box<reset_status_319::GroupSnapshotCommand>),
    Show(Box<show::GroupSnapshotCommand>),
}

impl GroupSnapshotCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        session: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        match &self.command {
            GroupSnapshotCommands::Create314(cmd) => cmd.take_action(parsed_args, session).await,
            GroupSnapshotCommands::Delete(cmd) => cmd.take_action(parsed_args, session).await,
            GroupSnapshotCommands::List(cmd) => cmd.take_action(parsed_args, session).await,
            GroupSnapshotCommands::ResetStatus319(cmd) => {
                cmd.take_action(parsed_args, session).await
            }
            GroupSnapshotCommands::Show(cmd) => cmd.take_action(parsed_args, session).await,
        }
    }
}
