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

//! Placement `ResourceProviderInventory` command with subcommands
use clap::{Parser, Subcommand};

use openstack_sdk::AsyncOpenStack;

use crate::{Cli, OpenStackCliError};

pub mod create;
pub mod delete;
pub mod delete_all;
pub mod list;
pub mod replace;
pub mod set;
pub mod show;

/// Resource provider inventories
///
/// Each resource provider has inventory records for one or more classes of resources. An inventory
/// record contains information about the total and reserved amounts of the resource and any
/// consumption constraints for that resource against the provider.
#[derive(Parser)]
pub struct InventoryCommand {
    #[command(subcommand)]
    command: InventoryCommands,
}

/// Supported subcommands
#[allow(missing_docs)]
#[derive(Subcommand)]
pub enum InventoryCommands {
    Create(create::InventoryCommand),
    Delete(delete::InventoryCommand),
    Purge(delete_all::InventoryCommand),
    List(list::InventoriesCommand),
    Replace(replace::InventoryCommand),
    Set(set::InventoryCommand),
    Show(show::InventoryCommand),
}

impl InventoryCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        session: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        match &self.command {
            InventoryCommands::Create(cmd) => cmd.take_action(parsed_args, session).await,
            InventoryCommands::Delete(cmd) => cmd.take_action(parsed_args, session).await,
            InventoryCommands::Purge(cmd) => cmd.take_action(parsed_args, session).await,
            InventoryCommands::List(cmd) => cmd.take_action(parsed_args, session).await,
            InventoryCommands::Replace(cmd) => cmd.take_action(parsed_args, session).await,
            InventoryCommands::Set(cmd) => cmd.take_action(parsed_args, session).await,
            InventoryCommands::Show(cmd) => cmd.take_action(parsed_args, session).await,
        }
    }
}
