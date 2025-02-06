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

//! Placement `resource_class` command with subcommands
use clap::{Parser, Subcommand};

use openstack_sdk::AsyncOpenStack;

use crate::{Cli, OpenStackCliError};

pub mod create;
pub mod delete;
pub mod list;
pub mod set_17;
pub mod show;

/// Resource Class
///
/// Resource classes are entities that indicate standard or deployer-specific resources that can be
/// provided by a resource provider. This group of API calls works with a single resource class
/// identified by name. One resource class can be listed, updated and deleted.
#[derive(Parser)]
pub struct ResourceClassCommand {
    #[command(subcommand)]
    command: ResourceClassCommands,
}

/// Supported subcommands
#[allow(missing_docs)]
#[derive(Subcommand)]
pub enum ResourceClassCommands {
    Create(create::ResourceClassCommand),
    Delete(delete::ResourceClassCommand),
    List(list::ResourceClassesCommand),
    #[command(visible_alias = "set")]
    Set17(set_17::ResourceClassCommand),
    Show(show::ResourceClassCommand),
}

impl ResourceClassCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        session: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        match &self.command {
            ResourceClassCommands::Create(cmd) => cmd.take_action(parsed_args, session).await,
            ResourceClassCommands::Delete(cmd) => cmd.take_action(parsed_args, session).await,
            ResourceClassCommands::List(cmd) => cmd.take_action(parsed_args, session).await,
            ResourceClassCommands::Set17(cmd) => cmd.take_action(parsed_args, session).await,
            ResourceClassCommands::Show(cmd) => cmd.take_action(parsed_args, session).await,
        }
    }
}
