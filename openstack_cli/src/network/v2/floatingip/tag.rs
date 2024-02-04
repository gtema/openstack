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

//! Floating IP Tag commands

use clap::{Parser, Subcommand};

use openstack_sdk::AsyncOpenStack;

use crate::{Cli, OpenStackCliError};

mod delete;
mod delete_all;
mod list;
mod replace;
mod set;
mod show;

/// Resource tag operations
#[derive(Parser)]
pub struct TagCommand {
    /// subcommand
    #[command(subcommand)]
    command: TagCommands,
}

/// Supported subcommands
#[allow(missing_docs)]
#[derive(Subcommand)]
pub enum TagCommands {
    Add(set::TagCommand),
    Check(show::TagCommand),
    Delete(delete::TagCommand),
    List(list::TagsCommand),
    Purge(delete_all::TagCommand),
    Replace(replace::TagCommand),
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
            TagCommands::Replace(cmd) => cmd.take_action(parsed_args, session).await,
        }
    }
}
