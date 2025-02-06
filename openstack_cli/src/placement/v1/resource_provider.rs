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

//! Placement `resource_provider` command with subcommands
use clap::{Parser, Subcommand};

use openstack_sdk::AsyncOpenStack;

use crate::{Cli, OpenStackCliError};

pub mod aggregate;
pub mod allocation;
pub mod create_10;
pub mod create_114;
pub mod delete;
pub mod inventory;
pub mod list;
pub mod set_10;
pub mod set_114;
pub mod show;
pub mod r#trait;
pub mod usage;

/// Resource Providers
///
/// Resource providers are entities which provide consumable inventory of one or more classes of
/// resource (such as disk or memory). They can be listed (with filters), created, updated and
/// deleted.
#[derive(Parser)]
pub struct ResourceProviderCommand {
    #[command(subcommand)]
    command: ResourceProviderCommands,
}

/// Supported subcommands
#[allow(missing_docs)]
#[derive(Subcommand)]
pub enum ResourceProviderCommands {
    Aggregate(aggregate::AggregateCommand),
    Allocation(allocation::AllocationCommand),
    #[command(visible_alias = "create")]
    Create114(create_114::ResourceProviderCommand),
    Create10(create_10::ResourceProviderCommand),
    Delete(delete::ResourceProviderCommand),
    Inventory(inventory::InventoryCommand),
    List(list::ResourceProvidersCommand),
    #[command(visible_alias = "set")]
    Set114(set_114::ResourceProviderCommand),
    Set10(set_10::ResourceProviderCommand),
    Show(show::ResourceProviderCommand),
    Trait(r#trait::TraitCommand),
    Usage(usage::UsageCommand),
}

impl ResourceProviderCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        session: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        match &self.command {
            ResourceProviderCommands::Aggregate(cmd) => cmd.take_action(parsed_args, session).await,
            ResourceProviderCommands::Allocation(cmd) => {
                cmd.take_action(parsed_args, session).await
            }
            ResourceProviderCommands::Create114(cmd) => cmd.take_action(parsed_args, session).await,
            ResourceProviderCommands::Create10(cmd) => cmd.take_action(parsed_args, session).await,
            ResourceProviderCommands::Delete(cmd) => cmd.take_action(parsed_args, session).await,
            ResourceProviderCommands::Inventory(cmd) => cmd.take_action(parsed_args, session).await,
            ResourceProviderCommands::List(cmd) => cmd.take_action(parsed_args, session).await,
            ResourceProviderCommands::Set114(cmd) => cmd.take_action(parsed_args, session).await,
            ResourceProviderCommands::Set10(cmd) => cmd.take_action(parsed_args, session).await,
            ResourceProviderCommands::Show(cmd) => cmd.take_action(parsed_args, session).await,
            ResourceProviderCommands::Trait(cmd) => cmd.take_action(parsed_args, session).await,
            ResourceProviderCommands::Usage(cmd) => cmd.take_action(parsed_args, session).await,
        }
    }
}
