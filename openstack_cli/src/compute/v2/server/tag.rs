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

//! Resource tag commands

use clap::{Parser, Subcommand};

use openstack_sdk::AsyncOpenStack;

use crate::{Cli, OpenStackCliError};

mod delete;
mod delete_all;
mod list;
mod replace_226;
mod set;
mod show;

/// Lists tags, creates, replaces or deletes one or more tags for a
/// server, checks the existence of a tag for a server.
///
/// Available since version 2.26
///
/// Tags have the following restrictions:
///
///  - Tag is a Unicode bytestring no longer than 60 characters.
///
///  - Tag is a non-empty string.
///
///  - ‘/’ is not allowed to be in a tag name
///
///  - Comma is not allowed to be in a tag name in order to
///    simplify requests that specify lists of tags
///
///  - All other characters are allowed to be in a tag name
///
///  - Each server can have up to 50 tags.
#[derive(Parser)]
pub struct TagCommand {
    /// subcommand
    #[command(subcommand)]
    command: TagCommands,
}

/// Compute resources commands
#[allow(missing_docs)]
#[derive(Subcommand)]
pub enum TagCommands {
    Add(set::TagCommand),
    Check(show::TagCommand),
    Delete(delete::TagCommand),
    List(list::TagsCommand),
    Purge(delete_all::TagCommand),
    #[command(visible_alias = "replace")]
    Replace226(replace_226::TagCommand),
}

impl TagCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        session: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        match &self.command {
            TagCommands::Add(cmd) => cmd.take_action(parsed_args, session).await,
            TagCommands::Check(cmd) => cmd.take_action(parsed_args, session).await,
            TagCommands::Delete(cmd) => cmd.take_action(parsed_args, session).await,
            TagCommands::List(cmd) => cmd.take_action(parsed_args, session).await,
            TagCommands::Purge(cmd) => cmd.take_action(parsed_args, session).await,
            TagCommands::Replace226(cmd) => cmd.take_action(parsed_args, session).await,
        }
    }
}
