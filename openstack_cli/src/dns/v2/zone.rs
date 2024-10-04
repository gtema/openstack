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

//! DNS Zone management
use clap::{Parser, Subcommand};

use openstack_sdk::AsyncOpenStack;

use crate::{Cli, OpenStackCliError};

mod create;
mod delete;
mod list;
mod nameserver;
mod recordset;
mod set;
mod show;

/// DNS Zone operations
#[derive(Parser)]
pub struct ZoneCommand {
    /// subcommand
    #[command(subcommand)]
    command: ZoneCommands,
}

/// Supported subcommands
#[allow(missing_docs)]
#[derive(Subcommand)]
pub enum ZoneCommands {
    Create(create::ZoneCommand),
    Delete(delete::ZoneCommand),
    List(list::ZonesCommand),
    Nameserver(nameserver::NameserverCommand),
    Recordset(recordset::RecordsetCommand),
    Show(show::ZoneCommand),
    Set(set::ZoneCommand),
}

impl ZoneCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        session: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        match &self.command {
            ZoneCommands::Create(cmd) => cmd.take_action(parsed_args, session).await,
            ZoneCommands::Delete(cmd) => cmd.take_action(parsed_args, session).await,
            ZoneCommands::List(cmd) => cmd.take_action(parsed_args, session).await,
            ZoneCommands::Nameserver(cmd) => cmd.take_action(parsed_args, session).await,
            ZoneCommands::Recordset(cmd) => cmd.take_action(parsed_args, session).await,
            ZoneCommands::Show(cmd) => cmd.take_action(parsed_args, session).await,
            ZoneCommands::Set(cmd) => cmd.take_action(parsed_args, session).await,
        }
    }
}
