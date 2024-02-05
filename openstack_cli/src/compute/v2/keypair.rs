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

//! Keypairs (keypairs)
//!
//! Generates, imports, and deletes SSH keys.

use clap::{Parser, Subcommand};

use openstack_sdk::AsyncOpenStack;

use crate::{Cli, OpenStackCliError};

mod create_20;
mod create_21;
mod create_210;
mod create_22;
mod create_292;
mod delete;
mod list;
mod show;

/// Keypairs commands
///
/// Generates, imports, and deletes SSH keys.
#[derive(Parser)]
pub struct KeypairCommand {
    /// subcommand
    #[command(subcommand)]
    command: KeypairCommands,
}

/// Supported subcommands
#[allow(missing_docs)]
#[derive(Subcommand)]
pub enum KeypairCommands {
    #[command(visible_alias = "create")]
    Create292(create_292::KeypairCommand),
    Create210(create_210::KeypairCommand),
    Create22(create_22::KeypairCommand),
    Create21(create_21::KeypairCommand),
    Create20(create_20::KeypairCommand),
    Delete(delete::KeypairCommand),
    List(list::KeypairsCommand),
    Show(show::KeypairCommand),
}

impl KeypairCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        session: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        match &self.command {
            KeypairCommands::Create292(cmd) => cmd.take_action(parsed_args, session).await,
            KeypairCommands::Create210(cmd) => cmd.take_action(parsed_args, session).await,
            KeypairCommands::Create22(cmd) => cmd.take_action(parsed_args, session).await,
            KeypairCommands::Create21(cmd) => cmd.take_action(parsed_args, session).await,
            KeypairCommands::Create20(cmd) => cmd.take_action(parsed_args, session).await,
            KeypairCommands::Delete(cmd) => cmd.take_action(parsed_args, session).await,
            KeypairCommands::List(cmd) => cmd.take_action(parsed_args, session).await,
            KeypairCommands::Show(cmd) => cmd.take_action(parsed_args, session).await,
        }
    }
}
