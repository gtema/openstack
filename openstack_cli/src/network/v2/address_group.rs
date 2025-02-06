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

//! AddressGroup resource commands

use clap::{Parser, Subcommand};

use openstack_sdk::AsyncOpenStack;

use crate::{Cli, OpenStackCliError};

pub mod add_addresses;
pub mod create;
pub mod delete;
pub mod list;
pub mod remove_addresses;
pub mod set;
pub mod show;

/// Address groups
///
/// Lists, creates, shows details for, updates, and deletes address groups.
#[derive(Parser)]
pub struct AddressGroupCommand {
    /// subcommand
    #[command(subcommand)]
    command: AddressGroupCommands,
}

/// Supported subcommands
#[allow(missing_docs)]
#[derive(Subcommand)]
pub enum AddressGroupCommands {
    AddAddress(Box<add_addresses::AddressGroupCommand>),
    Create(Box<create::AddressGroupCommand>),
    Delete(Box<delete::AddressGroupCommand>),
    List(Box<list::AddressGroupsCommand>),
    RemoveAddress(Box<remove_addresses::AddressGroupCommand>),
    Set(Box<set::AddressGroupCommand>),
    Show(Box<show::AddressGroupCommand>),
}

impl AddressGroupCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        session: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        match &self.command {
            AddressGroupCommands::AddAddress(cmd) => cmd.take_action(parsed_args, session).await,
            AddressGroupCommands::Create(cmd) => cmd.take_action(parsed_args, session).await,
            AddressGroupCommands::Delete(cmd) => cmd.take_action(parsed_args, session).await,
            AddressGroupCommands::List(cmd) => cmd.take_action(parsed_args, session).await,
            AddressGroupCommands::RemoveAddress(cmd) => cmd.take_action(parsed_args, session).await,
            AddressGroupCommands::Set(cmd) => cmd.take_action(parsed_args, session).await,
            AddressGroupCommands::Show(cmd) => cmd.take_action(parsed_args, session).await,
        }
    }
}
