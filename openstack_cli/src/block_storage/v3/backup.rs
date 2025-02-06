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

//! Block storage Backup commands
//!

use clap::{Parser, Subcommand};

use crate::{Cli, OpenStackCliError};

use openstack_sdk::AsyncOpenStack;

pub mod create_30;
pub mod create_343;
pub mod create_351;
pub mod delete;
/// Export backup record
pub mod export_record {
    pub mod get;
}
pub mod list;
/// Import backup records
pub mod import_record {
    pub mod create;
}
pub mod os_force_delete;
pub mod os_reset_status;
pub mod set_343;
pub mod set_39;
pub mod show;

/// Backups
///
/// A backup is a full copy of a volume stored in an external service. The service can be
/// configured. The only supported service is Object Storage. A backup can subsequently be restored
/// from the external service to either the same volume that the backup was originally taken from
/// or to a new volume.
#[derive(Parser)]
pub struct BackupCommand {
    /// subcommand
    #[command(subcommand)]
    command: BackupCommands,
}

/// Supported subcommands
#[allow(missing_docs)]
#[derive(Subcommand)]
pub enum BackupCommands {
    #[command(visible_alias = "create")]
    Create351(Box<create_351::BackupCommand>),
    Create343(Box<create_343::BackupCommand>),
    Create30(Box<create_30::BackupCommand>),
    Delete(Box<delete::BackupCommand>),
    Export(Box<export_record::get::ExportRecordCommand>),
    ForceDelete(Box<os_force_delete::BackupCommand>),
    Import(Box<import_record::create::ImportRecordCommand>),
    List(Box<list::BackupsCommand>),
    ResetStatus(Box<os_reset_status::BackupCommand>),
    #[command(visible_alias = "set")]
    Set343(Box<set_343::BackupCommand>),
    Set39(Box<set_39::BackupCommand>),
    Show(Box<show::BackupCommand>),
}

impl BackupCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        session: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        match &self.command {
            BackupCommands::Create351(cmd) => cmd.take_action(parsed_args, session).await,
            BackupCommands::Create343(cmd) => cmd.take_action(parsed_args, session).await,
            BackupCommands::Create30(cmd) => cmd.take_action(parsed_args, session).await,
            BackupCommands::Delete(cmd) => cmd.take_action(parsed_args, session).await,
            BackupCommands::Export(cmd) => cmd.take_action(parsed_args, session).await,
            BackupCommands::ForceDelete(cmd) => cmd.take_action(parsed_args, session).await,
            BackupCommands::Import(cmd) => cmd.take_action(parsed_args, session).await,
            BackupCommands::List(cmd) => cmd.take_action(parsed_args, session).await,
            BackupCommands::ResetStatus(cmd) => cmd.take_action(parsed_args, session).await,
            BackupCommands::Set343(cmd) => cmd.take_action(parsed_args, session).await,
            BackupCommands::Set39(cmd) => cmd.take_action(parsed_args, session).await,
            BackupCommands::Show(cmd) => cmd.take_action(parsed_args, session).await,
        }
    }
}
