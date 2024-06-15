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

//! Image v2 commands

use clap::{Parser, Subcommand};

use openstack_sdk::{types::ServiceType, AsyncOpenStack};

use crate::{Cli, OpenStackCliError};

mod image;
mod metadef;
mod schema;

/// Image service operations
#[derive(Parser)]
pub struct ImageCommand {
    /// subcommand
    #[command(subcommand)]
    command: ImageCommands,
}

/// Supported subcommands
#[allow(missing_docs)]
#[derive(Subcommand)]
pub enum ImageCommands {
    /// Image commands
    Image(image::ImageCommand),
    /// Metadef commands
    Metadef(metadef::MetadefCommand),
    /// Schema commands
    Schema(schema::SchemaCommand),
}

impl ImageCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        session: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        session
            .discover_service_endpoint(&ServiceType::Image)
            .await?;

        match &self.command {
            ImageCommands::Image(cmd) => cmd.take_action(parsed_args, session).await,
            ImageCommands::Metadef(cmd) => cmd.take_action(parsed_args, session).await,
            ImageCommands::Schema(cmd) => cmd.take_action(parsed_args, session).await,
        }
    }
}
