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

//! AddressScope resource commands

use clap::{Parser, Subcommand};

use openstack_sdk::AsyncOpenStack;

use crate::{Cli, OpenStackCliError};

mod create;
mod delete;
mod list;
mod set;
mod show;

/// Address scopes
///
/// Lists, creates, shows details for, updates, and deletes address scopes.
#[derive(Parser)]
pub struct AddressScopeCommand {
    /// subcommand
    #[command(subcommand)]
    command: AddressScopeCommands,
}

/// Supported subcommands
#[allow(missing_docs)]
#[derive(Subcommand)]
pub enum AddressScopeCommands {
    Create(Box<create::AddressScopeCommand>),
    Delete(Box<delete::AddressScopeCommand>),
    List(Box<list::AddressScopesCommand>),
    Set(Box<set::AddressScopeCommand>),
    Show(Box<show::AddressScopeCommand>),
}

impl AddressScopeCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        session: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        match &self.command {
            AddressScopeCommands::Create(cmd) => cmd.take_action(parsed_args, session).await,
            AddressScopeCommands::Delete(cmd) => cmd.take_action(parsed_args, session).await,
            AddressScopeCommands::List(cmd) => cmd.take_action(parsed_args, session).await,
            AddressScopeCommands::Set(cmd) => cmd.take_action(parsed_args, session).await,
            AddressScopeCommands::Show(cmd) => cmd.take_action(parsed_args, session).await,
        }
    }
}
