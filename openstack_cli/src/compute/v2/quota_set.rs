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

//! Quota Set

use clap::{Parser, Subcommand};

use openstack_sdk::AsyncOpenStack;

use crate::{Cli, OpenStackCliError};

pub mod defaults;
pub mod delete;
pub mod details;
pub mod set_236;
pub mod set_257;
pub mod show;

/// Quota sets (os-quota-sets)
///
/// Permits administrators, depending on policy settings, to view default quotas, view details for
/// quotas, revert quotas to defaults, and update the quotas for a project or a project and user.
#[derive(Parser)]
pub struct QuotaSetCommand {
    /// subcommand
    #[command(subcommand)]
    command: QuotaSetCommands,
}

/// Supported subcommands
#[allow(missing_docs)]
#[derive(Subcommand)]
pub enum QuotaSetCommands {
    Defaults(defaults::QuotaSetCommand),
    Delete(delete::QuotaSetCommand),
    Details(details::QuotaSetCommand),
    #[command(visible_alias = "set")]
    Set257(set_257::QuotaSetCommand),
    Set236(set_236::QuotaSetCommand),
    Show(show::QuotaSetCommand),
}

impl QuotaSetCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        session: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        match &self.command {
            QuotaSetCommands::Defaults(cmd) => cmd.take_action(parsed_args, session).await,
            QuotaSetCommands::Delete(cmd) => cmd.take_action(parsed_args, session).await,
            QuotaSetCommands::Details(cmd) => cmd.take_action(parsed_args, session).await,
            QuotaSetCommands::Set257(cmd) => cmd.take_action(parsed_args, session).await,
            QuotaSetCommands::Set236(cmd) => cmd.take_action(parsed_args, session).await,
            QuotaSetCommands::Show(cmd) => cmd.take_action(parsed_args, session).await,
        }
    }
}
