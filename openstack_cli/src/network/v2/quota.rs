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

//! Quota commands

use clap::{Parser, Subcommand};

use openstack_sdk::AsyncOpenStack;

use crate::{Cli, OpenStackCliError};

mod defaults;
mod delete;
mod details;
mod list;
mod set;
mod show;

/// Quotas extension (quotas)
///
/// Lists default quotas, current quotas for projects with non-default quota values, and shows,
/// updates, and resets quotas for a project.
///
/// A quota value of -1 means that quota has no limit.
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
    Defaults(defaults::QuotaCommand),
    Delete(delete::QuotaCommand),
    Details(details::QuotaCommand),
    List(list::QuotasCommand),
    Set(set::QuotaCommand),
    Show(show::QuotaCommand),
}

impl QuotaCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        session: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        match &self.command {
            QuotaCommands::Defaults(cmd) => cmd.take_action(parsed_args, session).await,
            QuotaCommands::Delete(cmd) => cmd.take_action(parsed_args, session).await,
            QuotaCommands::Details(cmd) => cmd.take_action(parsed_args, session).await,
            QuotaCommands::List(cmd) => cmd.take_action(parsed_args, session).await,
            QuotaCommands::Set(cmd) => cmd.take_action(parsed_args, session).await,
            QuotaCommands::Show(cmd) => cmd.take_action(parsed_args, session).await,
        }
    }
}
