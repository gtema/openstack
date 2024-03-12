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

//! Metadef resource_type schema

use clap::{Parser, Subcommand};

use openstack_sdk::AsyncOpenStack;

use crate::{Cli, OpenStackCliError};

mod get;

/// Metadef ResourceType Schema operations
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
    /// Show metadata definition namespace resource type association schema
    Show(get::ResourceTypeCommand),
}

impl ResourceTypeCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        session: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        match &self.command {
            ResourceTypeCommands::Show(cmd) => cmd.take_action(parsed_args, session).await,
        }
    }
}
