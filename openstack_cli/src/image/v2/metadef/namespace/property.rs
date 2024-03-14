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
mod delete_all;
mod list;
mod set;
mod show;

/// Metadata definition properties
///
/// Creates, lists, shows details for, updates, and deletes metadata definition properties.
#[derive(Parser)]
pub struct PropertyCommand {
    /// subcommand
    #[command(subcommand)]
    command: PropertyCommands,
}

/// Supported subcommands
#[allow(missing_docs)]
#[derive(Subcommand)]
pub enum PropertyCommands {
    Create(create::PropertyCommand),
    Delete(delete::PropertyCommand),
    List(list::PropertyCommand),
    Purge(delete_all::PropertyCommand),
    Set(set::PropertyCommand),
    Show(show::PropertyCommand),
}

impl PropertyCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        session: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        match &self.command {
            PropertyCommands::Create(cmd) => cmd.take_action(parsed_args, session).await,
            PropertyCommands::Delete(cmd) => cmd.take_action(parsed_args, session).await,
            PropertyCommands::List(cmd) => cmd.take_action(parsed_args, session).await,
            PropertyCommands::Purge(cmd) => cmd.take_action(parsed_args, session).await,
            PropertyCommands::Set(cmd) => cmd.take_action(parsed_args, session).await,
            PropertyCommands::Show(cmd) => cmd.take_action(parsed_args, session).await,
        }
    }
}
