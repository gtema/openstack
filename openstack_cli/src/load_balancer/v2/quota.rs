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

//! Octavia `Quota` resource commands

use clap::{Parser, Subcommand};

use openstack_sdk::AsyncOpenStack;

use crate::{Cli, OpenStackCliError};

mod delete;
mod list;
mod set;
mod show;

/// Quota commands
#[derive(Parser)]
pub struct QuotaCommand {
    /// subcommand
    #[command(subcommand)]
    command: QuotaCommands,
}

/// Supported subcommands
#[allow(missing_docs)]
#[derive(Subcommand)]
pub enum QuotaCommands {
    Delete(Box<delete::QuotaCommand>),
    List(Box<list::QuotasCommand>),
    Set(Box<set::QuotaCommand>),
    Show(Box<show::QuotaCommand>),
}

impl QuotaCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        session: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        match &self.command {
            QuotaCommands::Delete(cmd) => cmd.take_action(parsed_args, session).await,
            QuotaCommands::List(cmd) => cmd.take_action(parsed_args, session).await,
            QuotaCommands::Set(cmd) => cmd.take_action(parsed_args, session).await,
            QuotaCommands::Show(cmd) => cmd.take_action(parsed_args, session).await,
        }
    }
}
