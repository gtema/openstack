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

//! Block storage Type Encryption commands
//!

use clap::{Parser, Subcommand};

use crate::{Cli, OpenStackCliError};

use openstack_sdk::AsyncOpenStack;

pub mod create;
pub mod delete;
pub mod list;
pub mod set;
pub mod show;

/// Volume Type Encryption commands
///
/// Block Storage volume type assignment provides scheduling to a specific back-end, and can be
/// used to specify actionable information for a back-end storage device.
#[derive(Parser)]
pub struct EncryptionCommand {
    /// sumcommnd
    #[command(subcommand)]
    command: EncryptionCommands,
}

/// Supported subcommands
#[allow(missing_docs)]
#[derive(Subcommand)]
pub enum EncryptionCommands {
    Create(Box<create::EncryptionCommand>),
    Delete(Box<delete::EncryptionCommand>),
    List(Box<list::EncryptionsCommand>),
    Set(Box<set::EncryptionCommand>),
    Show(Box<show::EncryptionCommand>),
}

impl EncryptionCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        session: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        match &self.command {
            EncryptionCommands::Create(cmd) => cmd.take_action(parsed_args, session).await,
            EncryptionCommands::Delete(cmd) => cmd.take_action(parsed_args, session).await,
            EncryptionCommands::List(cmd) => cmd.take_action(parsed_args, session).await,
            EncryptionCommands::Set(cmd) => cmd.take_action(parsed_args, session).await,
            EncryptionCommands::Show(cmd) => cmd.take_action(parsed_args, session).await,
        }
    }
}
