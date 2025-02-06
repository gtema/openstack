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

//! Block storage Volume Transfer commands
//!

use clap::{Parser, Subcommand};

use crate::{Cli, OpenStackCliError};

use openstack_sdk::AsyncOpenStack;

pub mod accept;
pub mod create_355;
pub mod delete;
pub mod list;
pub mod show;

/// Volume transfers (volume-transfers) (3.55 or later)
///
/// Transfers a volume from one user to another user. This is the new transfer APIs with
/// microversion 3.55.
#[derive(Parser)]
pub struct VolumeTransferCommand {
    /// subcommand
    #[command(subcommand)]
    command: VolumeTransferCommands,
}

/// Supported subcommands
#[allow(missing_docs)]
#[derive(Subcommand)]
pub enum VolumeTransferCommands {
    Accept(Box<accept::VolumeTransferCommand>),
    #[command(visible_alias = "create")]
    Create355(Box<create_355::VolumeTransferCommand>),
    Delete(Box<delete::VolumeTransferCommand>),
    List(Box<list::VolumeTransfersCommand>),
    Show(Box<show::VolumeTransferCommand>),
}

impl VolumeTransferCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        session: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        match &self.command {
            VolumeTransferCommands::Accept(cmd) => cmd.take_action(parsed_args, session).await,
            VolumeTransferCommands::Create355(cmd) => cmd.take_action(parsed_args, session).await,
            VolumeTransferCommands::Delete(cmd) => cmd.take_action(parsed_args, session).await,
            VolumeTransferCommands::List(cmd) => cmd.take_action(parsed_args, session).await,
            VolumeTransferCommands::Show(cmd) => cmd.take_action(parsed_args, session).await,
        }
    }
}
