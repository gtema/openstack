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

//! Block storage Manageable Volume commands
//!

use clap::{Parser, Subcommand};

use crate::{Cli, OpenStackCliError};

use openstack_sdk::AsyncOpenStack;

pub mod create_30;
pub mod create_316;
pub mod list;

/// Volume manage extension (manageable_volumes)
///
/// Creates or lists volumes by using existing storage instead of allocating new storage.
#[derive(Parser)]
pub struct VolumeManageCommand {
    /// subcommand
    #[command(subcommand)]
    command: VolumeManageCommands,
}

/// Supported subcommands
#[allow(missing_docs)]
#[derive(Subcommand)]
pub enum VolumeManageCommands {
    #[command(visible_alias = "create")]
    Create316(Box<create_316::VolumeManageCommand>),
    Create30(Box<create_30::VolumeManageCommand>),
    List(Box<list::VolumeManagesCommand>),
}

impl VolumeManageCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        session: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        match &self.command {
            VolumeManageCommands::Create316(cmd) => cmd.take_action(parsed_args, session).await,
            VolumeManageCommands::Create30(cmd) => cmd.take_action(parsed_args, session).await,
            VolumeManageCommands::List(cmd) => cmd.take_action(parsed_args, session).await,
        }
    }
}
