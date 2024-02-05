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

//! FloatingIP commands

use clap::{Parser, Subcommand};

use openstack_sdk::AsyncOpenStack;

use crate::{Cli, OpenStackCliError};

mod create;
mod delete;
mod list;
mod set;
mod show;
mod tag;

/// Floating IP commands
#[derive(Parser)]
pub struct FloatingIPCommand {
    /// subcommand
    #[command(subcommand)]
    command: FloatingIPCommands,
}

/// Supported subcommands
#[allow(missing_docs)]
#[derive(Subcommand)]
pub enum FloatingIPCommands {
    Create(create::FloatingipCommand),
    Delete(delete::FloatingipCommand),
    List(list::FloatingipsCommand),
    Set(set::FloatingipCommand),
    Show(show::FloatingipCommand),
    Tag(tag::TagCommand),
}

impl FloatingIPCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        session: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        match &self.command {
            FloatingIPCommands::Create(cmd) => cmd.take_action(parsed_args, session).await,
            FloatingIPCommands::Delete(cmd) => cmd.take_action(parsed_args, session).await,
            FloatingIPCommands::List(cmd) => cmd.take_action(parsed_args, session).await,
            FloatingIPCommands::Set(cmd) => cmd.take_action(parsed_args, session).await,
            FloatingIPCommands::Show(cmd) => cmd.take_action(parsed_args, session).await,
            FloatingIPCommands::Tag(cmd) => cmd.take_action(parsed_args, session).await,
        }
    }
}
