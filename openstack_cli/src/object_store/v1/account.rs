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

//! Object Store `account` command with subcommands
use clap::{Parser, Subcommand};

use openstack_sdk::AsyncOpenStack;

use crate::{Cli, OpenStackCliError};

pub mod set;
pub mod show;

/// Account commands
#[derive(Parser)]
pub struct AccountCommand {
    #[command(subcommand)]
    command: AccountCommands,
}

/// Supported subcommands
#[allow(missing_docs)]
#[derive(Subcommand)]
pub enum AccountCommands {
    Show(show::AccountCommand),
    Set(set::AccountCommand),
}

impl AccountCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        session: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        match &self.command {
            AccountCommands::Show(cmd) => cmd.take_action(parsed_args, session).await,
            AccountCommands::Set(cmd) => cmd.take_action(parsed_args, session).await,
        }
    }
}
