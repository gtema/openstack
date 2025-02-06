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

//! Identity Token commands

use clap::{Parser, Subcommand};

use openstack_sdk::AsyncOpenStack;

use crate::{Cli, OpenStackCliError};

//mod create;
pub mod delete;
pub mod get;

/// Identity token commands
///
/// authorization.
#[derive(Parser)]
pub struct TokenCommand {
    /// subcommand
    #[command(subcommand)]
    command: TokenCommands,
}

/// Supported subcommands
#[allow(missing_docs)]
#[derive(Subcommand)]
pub enum TokenCommands {
    //    Create(create::TokenCommand),
    Delete(delete::TokenCommand),
    Get(get::TokenCommand),
}

impl TokenCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        session: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        match &self.command {
            //            TokenCommands::Create(cmd) => cmd.take_action(parsed_args, session).await,
            TokenCommands::Delete(cmd) => cmd.take_action(parsed_args, session).await,
            TokenCommands::Get(cmd) => cmd.take_action(parsed_args, session).await,
        }
    }
}
