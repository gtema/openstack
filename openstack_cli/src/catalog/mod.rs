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

//! Catalog command

use clap::{Parser, Subcommand};

use openstack_sdk::AsyncOpenStack;

use crate::Cli;
use crate::OpenStackCliError;

mod list;
mod show;

/// Catalog commands args
#[derive(Parser)]
pub struct CatalogCommand {
    /// subcommand
    #[command(subcommand)]
    command: CatalogCommands,
}

/// Catalog command types
#[allow(missing_docs)]
#[derive(Subcommand)]
pub enum CatalogCommands {
    List(list::ListCommand),
    Show(show::ShowCommand),
}

impl CatalogCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        session: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        match &self.command {
            CatalogCommands::List(cmd) => cmd.take_action(parsed_args, session).await,
            CatalogCommands::Show(cmd) => cmd.take_action(parsed_args, session).await,
        }
    }
}
