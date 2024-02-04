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

//! Compute Flavor Extra Specs commands
use clap::{Parser, Subcommand};

use openstack_sdk::AsyncOpenStack;

use crate::{Cli, OpenStackCliError};

mod create;
mod delete;
mod list;
mod set;
mod show;

/// Flavor extra specs
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
    /// Creates extra specs for a flavor, by ID.
    #[command(about = "Create Extra Specs For A Flavor")]
    Create(create::ExtraSpecCommand),
    /// Deletes an extra spec, by key, for a flavor, by ID.
    #[command(about = "Delete An Extra Spec For A Flavor")]
    Delete(delete::ExtraSpecCommand),
    /// Lists all extra specs for a flavor, by ID.
    #[command(about = "List Extra Specs For A Flavor")]
    List(list::ExtraSpecsCommand),
    /// Shows an extra spec, by key, for a flavor, by ID.
    #[command(about = "Show An Extra Spec For A Flavor")]
    Show(show::ExtraSpecCommand),
    /// Updates an extra spec, by key, for a flavor, by ID.
    #[command(about = "Update An Extra Spec For A Flavor
")]
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
