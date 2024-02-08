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

//! Block storage volume metadata commands
use clap::{Parser, Subcommand};

use openstack_sdk::AsyncOpenStack;

use crate::{Cli, OpenStackCliError};

mod create;
mod delete;
mod list;
mod replace;
mod set;
mod show;

/// Volume metadata
///
/// Lists metadata, creates or replaces one or more metadata items, and updates
/// one or more metadata items for a volume.
#[derive(Parser)]
#[command(about = "Volume metadata")]
pub struct MetadataCommand {
    /// subcommand
    #[command(subcommand)]
    command: MetadataCommands,
}

/// Supported subcommands
#[allow(missing_docs)]
#[derive(Subcommand)]
pub enum MetadataCommands {
    Create(Box<create::MetadataCommand>),
    Delete(Box<delete::MetadataCommand>),
    List(Box<list::MetadatasCommand>),
    Replace(Box<replace::MetadataCommand>),
    Set(Box<set::MetadataCommand>),
    Show(Box<show::MetadataCommand>),
}

impl MetadataCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        session: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        match &self.command {
            MetadataCommands::Create(cmd) => cmd.take_action(parsed_args, session).await,
            MetadataCommands::Delete(cmd) => cmd.take_action(parsed_args, session).await,
            MetadataCommands::List(cmd) => cmd.take_action(parsed_args, session).await,
            MetadataCommands::Replace(cmd) => cmd.take_action(parsed_args, session).await,
            MetadataCommands::Set(cmd) => cmd.take_action(parsed_args, session).await,
            MetadataCommands::Show(cmd) => cmd.take_action(parsed_args, session).await,
        }
    }
}
