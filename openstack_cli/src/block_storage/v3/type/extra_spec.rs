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

//! Block Storage Type Extra Specs commands
use clap::{Parser, Subcommand};

use openstack_sdk::AsyncOpenStack;

use crate::{Cli, OpenStackCliError};

mod create;
mod delete;
mod list;
mod set;
mod show;

/// Type extra specs
#[derive(Parser)]
pub struct ExtraSpecsCommand {
    /// subcommand
    #[command(subcommand)]
    command: ExtraSpecsCommands,
}

/// Supported subcommands
#[allow(missing_docs)]
#[derive(Subcommand)]
pub enum ExtraSpecsCommands {
    Create(create::ExtraSpecCommand),
    Delete(delete::ExtraSpecCommand),
    List(list::ExtraSpecsCommand),
    Show(show::ExtraSpecCommand),
    Set(set::ExtraSpecCommand),
}

impl ExtraSpecsCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        session: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        match &self.command {
            ExtraSpecsCommands::Create(cmd) => cmd.take_action(parsed_args, session).await,
            ExtraSpecsCommands::Delete(cmd) => cmd.take_action(parsed_args, session).await,
            ExtraSpecsCommands::List(cmd) => cmd.take_action(parsed_args, session).await,
            ExtraSpecsCommands::Show(cmd) => cmd.take_action(parsed_args, session).await,
            ExtraSpecsCommands::Set(cmd) => cmd.take_action(parsed_args, session).await,
        }
    }
}
