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

//! Glance Schemas

use clap::{Parser, Subcommand};

use openstack_sdk::AsyncOpenStack;

use crate::{Cli, OpenStackCliError};

mod image;
mod images;
mod member;
mod members;

/// Schemas
#[derive(Parser)]
pub struct SchemaCommand {
    /// subcommand
    #[command(subcommand)]
    command: SchemaCommands,
}

/// Supported subcommands
#[allow(missing_docs)]
#[derive(Subcommand)]
pub enum SchemaCommands {
    Image(image::ImageCommand),
    Images(images::ImagesCommand),
    Member(member::MemberCommand),
    Members(members::MembersCommand),
}

impl SchemaCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        session: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        match &self.command {
            SchemaCommands::Image(cmd) => cmd.take_action(parsed_args, session).await,
            SchemaCommands::Images(cmd) => cmd.take_action(parsed_args, session).await,
            SchemaCommands::Member(cmd) => cmd.take_action(parsed_args, session).await,
            SchemaCommands::Members(cmd) => cmd.take_action(parsed_args, session).await,
        }
    }
}
