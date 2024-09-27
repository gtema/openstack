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

//! IpsecSiteConnection commands

use clap::{Parser, Subcommand};

use openstack_sdk::AsyncOpenStack;

use crate::{Cli, OpenStackCliError};

mod create;
mod delete;
mod list;
mod set;
mod show;

/// Details for the site-to-site IPsec connection, including the peer CIDRs, MTU, authentication
/// mode, peer address, DPD settings, and status.
#[derive(Parser)]
pub struct IpsecSiteConnectionCommand {
    /// subcommand
    #[command(subcommand)]
    command: IpsecSiteConnectionCommands,
}

/// Supported subcommands
#[allow(missing_docs)]
#[derive(Subcommand)]
pub enum IpsecSiteConnectionCommands {
    Create(create::IpsecSiteConnectionCommand),
    Delete(delete::IpsecSiteConnectionCommand),
    List(list::IpsecSiteConnectionsCommand),
    Set(set::IpsecSiteConnectionCommand),
    Show(show::IpsecSiteConnectionCommand),
}

impl IpsecSiteConnectionCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        session: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        match &self.command {
            IpsecSiteConnectionCommands::Create(cmd) => cmd.take_action(parsed_args, session).await,
            IpsecSiteConnectionCommands::Delete(cmd) => cmd.take_action(parsed_args, session).await,
            IpsecSiteConnectionCommands::List(cmd) => cmd.take_action(parsed_args, session).await,
            IpsecSiteConnectionCommands::Set(cmd) => cmd.take_action(parsed_args, session).await,
            IpsecSiteConnectionCommands::Show(cmd) => cmd.take_action(parsed_args, session).await,
        }
    }
}
