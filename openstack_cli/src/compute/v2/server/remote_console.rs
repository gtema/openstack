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

//! Server remote consoles

use clap::{Parser, Subcommand};

use openstack_sdk::AsyncOpenStack;

use crate::{Cli, OpenStackCliError};

mod create_26;
mod create_28;

/// Server Consoles
///
/// Manage server consoles.
#[derive(Parser)]
pub struct RemoteConsoleCommand {
    #[command(subcommand)]
    command: RemoteConsoleCommands,
}

/// Supported subcommands
#[allow(missing_docs)]
#[derive(Subcommand)]
pub enum RemoteConsoleCommands {
    Create26(create_26::RemoteConsoleCommand),
    #[command(visible_alias = "create")]
    Create28(create_28::RemoteConsoleCommand),
}

impl RemoteConsoleCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        session: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        match &self.command {
            RemoteConsoleCommands::Create26(cmd) => cmd.take_action(parsed_args, session).await,
            RemoteConsoleCommands::Create28(cmd) => cmd.take_action(parsed_args, session).await,
        }
    }
}
