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

//! Glance Metadef namespace properties

use clap::{Parser, Subcommand};

use openstack_sdk::AsyncOpenStack;

use crate::{Cli, OpenStackCliError};

mod create;
mod delete;
mod list;

/// Metadata definition resource types
///
/// Lists resource types. Also, creates, lists, and removes resource type associations in a namespace.
#[derive(Parser)]
pub struct ResourceTypeCommand {
    /// subcommand
    #[command(subcommand)]
    command: ResourceTypeCommands,
}

/// Supported subcommands
#[allow(missing_docs)]
#[derive(Subcommand)]
pub enum ResourceTypeCommands {
    Create(create::ResourceTypeCommand),
    Delete(delete::ResourceTypeCommand),
    List(list::ResourceTypesCommand),
}

impl ResourceTypeCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        session: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        match &self.command {
            ResourceTypeCommands::Create(cmd) => cmd.take_action(parsed_args, session).await,
            ResourceTypeCommands::Delete(cmd) => cmd.take_action(parsed_args, session).await,
            ResourceTypeCommands::List(cmd) => cmd.take_action(parsed_args, session).await,
        }
    }
}
