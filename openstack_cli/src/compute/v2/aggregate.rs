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

//! Host Aggregates management
use clap::{Parser, Subcommand};

use openstack_sdk::AsyncOpenStack;

use crate::{Cli, OpenStackCliError};

pub mod add_host;
pub mod create_21;
pub mod delete;
pub mod list;
/// Aggregate image
pub mod image {
    pub mod cache_281;
}
pub mod remove_host;
pub mod set_21;
pub mod set_metadata;
pub mod show;

/// Creates and manages host aggregates. An aggregate assigns metadata to
/// groups of compute nodes.
///
/// Policy defaults enable only users with the administrative role to perform
/// operations with aggregates. Cloud providers can change these permissions
/// through policy file configuration.
#[derive(Parser)]
pub struct AggregateCommand {
    /// subcommand
    #[command(subcommand)]
    command: AggregateCommands,
}

/// Supported subcommands
#[allow(missing_docs)]
#[derive(Subcommand)]
pub enum AggregateCommands {
    /// Adds a host to an aggregate.
    #[command(about = "Add Host")]
    AddHost(add_host::AggregateCommand),
    #[command(visible_alias = "create")]
    Create21(create_21::AggregateCommand),
    CacheImage(image::cache_281::ImageCommand),
    Delete(delete::AggregateCommand),
    List(list::AggregatesCommand),
    /// Removes a host from an aggregate.
    #[command(about = "Remove Host")]
    RemoveHost(remove_host::AggregateCommand),
    Show(show::AggregateCommand),
    #[command(visible_alias = "set")]
    Set21(set_21::AggregateCommand),
    /// Creates or replaces metadata for an aggregate.
    #[command(about = "Create Or Update Aggregate Metadata")]
    SetMetadata(set_metadata::AggregateCommand),
}

impl AggregateCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        session: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        match &self.command {
            AggregateCommands::AddHost(cmd) => cmd.take_action(parsed_args, session).await,
            AggregateCommands::Create21(cmd) => cmd.take_action(parsed_args, session).await,
            AggregateCommands::CacheImage(cmd) => cmd.take_action(parsed_args, session).await,
            AggregateCommands::Delete(cmd) => cmd.take_action(parsed_args, session).await,
            AggregateCommands::List(cmd) => cmd.take_action(parsed_args, session).await,
            AggregateCommands::RemoveHost(cmd) => cmd.take_action(parsed_args, session).await,
            AggregateCommands::Show(cmd) => cmd.take_action(parsed_args, session).await,
            AggregateCommands::Set21(cmd) => cmd.take_action(parsed_args, session).await,
            AggregateCommands::SetMetadata(cmd) => cmd.take_action(parsed_args, session).await,
        }
    }
}
