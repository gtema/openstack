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

//! Conntrack Helper (CT) module
//!
use clap::{Parser, Subcommand};

use openstack_sdk::AsyncOpenStack;

use crate::{Cli, OpenStackCliError};

pub mod create;
pub mod delete;
pub mod list;
pub mod set;
pub mod show;

/// Lists, creates, shows details for, updates, and deletes router conntrack helper (CT) target
/// rules.
#[derive(Parser)]
pub struct ConntrackHelperCommand {
    /// subcommand
    #[command(subcommand)]
    command: ConntrackHelperCommands,
}

/// Supported subcommands
#[allow(missing_docs)]
#[derive(Subcommand)]
pub enum ConntrackHelperCommands {
    Create(create::ConntrackHelperCommand),
    Delete(delete::ConntrackHelperCommand),
    List(list::ConntrackHelpersCommand),
    Set(set::ConntrackHelperCommand),
    Show(show::ConntrackHelperCommand),
}

impl ConntrackHelperCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        session: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        match &self.command {
            ConntrackHelperCommands::Create(cmd) => cmd.take_action(parsed_args, session).await,
            ConntrackHelperCommands::Delete(cmd) => cmd.take_action(parsed_args, session).await,
            ConntrackHelperCommands::List(cmd) => cmd.take_action(parsed_args, session).await,
            ConntrackHelperCommands::Set(cmd) => cmd.take_action(parsed_args, session).await,
            ConntrackHelperCommands::Show(cmd) => cmd.take_action(parsed_args, session).await,
        }
    }
}
