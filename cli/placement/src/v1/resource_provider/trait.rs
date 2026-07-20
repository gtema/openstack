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

//! Placement `ResourceProviderTrait` command with subcommands
use clap::{Parser, Subcommand};

use openstack_cli_core::{cli::CliArgs, error::OpenStackCliError};
use openstack_sdk::AsyncOpenStack;

pub mod delete_16;
pub mod list_16;
pub mod set_16;

/// Resource provider traits
///
/// This group of API requests queries/edits the association between traits and resource providers.
#[derive(Parser)]
pub struct TraitCommand {
    #[command(subcommand)]
    command: TraitCommands,
}

/// Supported subcommands
#[allow(missing_docs)]
#[derive(Subcommand)]
pub enum TraitCommands {
    #[command(visible_alias = "delete")]
    Delete16(delete_16::TraitCommand),
    #[command(visible_alias = "list")]
    List16(list_16::TraitCommand),
    #[command(visible_alias = "set")]
    Set16(set_16::TraitCommand),
}

impl TraitCommand {
    /// Perform command action
    pub async fn take_action<C: CliArgs>(
        &self,
        parsed_args: &C,
        session: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        match &self.command {
            TraitCommands::Delete16(cmd) => cmd.take_action(parsed_args, session).await,
            TraitCommands::List16(cmd) => cmd.take_action(parsed_args, session).await,
            TraitCommands::Set16(cmd) => cmd.take_action(parsed_args, session).await,
        }
    }
}
